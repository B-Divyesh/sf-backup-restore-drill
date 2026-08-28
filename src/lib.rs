//! Core restore, verification, and receipt-chain primitives used by the CLI.

use chrono::{DateTime, Duration, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration as StdDuration, Instant};
use tempfile::Builder;
use wait_timeout::ChildExt;

pub type DrillResult<T> = Result<T, String>;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u8,
    pub name: String,
    pub cadence_days: u32,
    pub receipt_dir: PathBuf,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    pub restore: RestoreConfig,
    #[serde(rename = "sample")]
    pub samples: Vec<SampleConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestoreConfig {
    pub command: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SampleConfig {
    pub path: PathBuf,
    pub sha256: Option<String>,
    pub open_with: Option<Vec<String>>,
}

fn default_timeout() -> u64 {
    900
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SampleResult {
    pub path: String,
    pub status: String,
    pub bytes: Option<u64>,
    pub expected_sha256: Option<String>,
    pub actual_sha256: Option<String>,
    pub open_checked: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptBody {
    pub schema_version: u8,
    pub id: String,
    pub drill_name: String,
    pub created_at: String,
    pub duration_ms: u64,
    pub status: String,
    pub restore_exit_code: Option<i32>,
    pub config_sha256: String,
    pub previous_receipt_sha256: Option<String>,
    pub samples: Vec<SampleResult>,
    pub remediation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    #[serde(flatten)]
    pub body: ReceiptBody,
    pub receipt_sha256: String,
}

#[derive(Debug)]
pub struct RunOutcome {
    pub passed: bool,
    pub receipt_path: PathBuf,
    pub receipt: Receipt,
}

#[derive(Debug, Serialize)]
pub struct StatusOutcome {
    pub status: String,
    pub drill_name: String,
    pub cadence_days: u32,
    pub last_success_at: Option<String>,
    pub due_at: Option<String>,
    pub days_overdue: i64,
    pub message: String,
}

pub fn load_config(path: &Path) -> DrillResult<(Config, String)> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("could not read config {}: {e}", path.display()))?;
    let mut config: Config =
        toml::from_str(&raw).map_err(|e| format!("invalid config {}: {e}", path.display()))?;
    validate_config(&config)?;
    if config.receipt_dir.is_relative() {
        config.receipt_dir = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(&config.receipt_dir);
    }
    Ok((config, sha256_bytes(raw.as_bytes())))
}

pub fn validate_config(config: &Config) -> DrillResult<()> {
    if config.version != 1 {
        return Err(format!(
            "unsupported config version {}; expected 1",
            config.version
        ));
    }
    if config.name.trim().is_empty() {
        return Err("name cannot be empty".into());
    }
    if !(1..=3650).contains(&config.cadence_days) {
        return Err("cadence_days must be between 1 and 3650".into());
    }
    if !(1..=86400).contains(&config.timeout_seconds) {
        return Err("timeout_seconds must be between 1 and 86400".into());
    }
    validate_command(&config.restore.command, "{target}", "restore.command")?;
    if config.samples.is_empty() {
        return Err("configure at least one [[sample]]".into());
    }
    let mut paths = HashSet::new();
    for sample in &config.samples {
        validate_relative_path(&sample.path)?;
        let key = sample.path.to_string_lossy().to_string();
        if !paths.insert(key.clone()) {
            return Err(format!("duplicate sample path: {key}"));
        }
        if sample.sha256.is_none() && sample.open_with.is_none() {
            return Err(format!("sample {key} needs sha256, open_with, or both"));
        }
        if let Some(hash) = &sample.sha256 {
            if hash.len() != 64
                || !hash
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err(format!(
                    "sample {key} sha256 must be 64 lowercase hex characters"
                ));
            }
        }
        if let Some(command) = &sample.open_with {
            validate_command(command, "{file}", &format!("sample {key} open_with"))?;
        }
    }
    Ok(())
}

fn validate_command(command: &[String], placeholder: &str, label: &str) -> DrillResult<()> {
    if command.is_empty() || command[0].trim().is_empty() {
        return Err(format!("{label} must be a non-empty argument array"));
    }
    let count: usize = command
        .iter()
        .map(|arg| arg.matches(placeholder).count())
        .sum();
    if count != 1 {
        return Err(format!("{label} must contain {placeholder} exactly once"));
    }
    Ok(())
}

fn validate_relative_path(path: &Path) -> DrillResult<()> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(format!(
            "sample path must be non-empty and relative: {}",
            path.display()
        ));
    }
    if path
        .components()
        .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(format!(
            "sample path cannot contain '.', '..', roots, or prefixes: {}",
            path.display()
        ));
    }
    Ok(())
}

pub fn run_drill(config: &Config, config_hash: &str) -> DrillResult<RunOutcome> {
    let started = Instant::now();
    let created = Utc::now();
    let prior = load_receipts(&config.receipt_dir)?;
    let previous_hash = prior
        .last()
        .map(|(_, receipt)| receipt.receipt_sha256.clone());
    let temp = Builder::new()
        .prefix("restore-drill-")
        .tempdir()
        .map_err(|e| format!("could not create temporary restore target: {e}"))?;
    let target = temp.path().to_path_buf();
    let command = substitute(
        &config.restore.command,
        "{target}",
        &target.to_string_lossy(),
    );

    let restore = run_command(&command, config.timeout_seconds);
    let (restore_exit_code, restore_ok, restore_message) = match restore {
        Ok(code) if code == Some(0) => (code, true, None),
        Ok(code) => (
            code,
            false,
            Some(match code {
                Some(value) => format!("restore command exited with code {value}"),
                None => format!(
                    "restore command exceeded {} seconds",
                    config.timeout_seconds
                ),
            }),
        ),
        Err(error) => (None, false, Some(error)),
    };

    let mut results = Vec::new();
    if restore_ok {
        for sample in &config.samples {
            results.push(check_sample(&target, sample, config.timeout_seconds));
        }
    }
    let mut passed = restore_ok && results.iter().all(|result| result.status == "pass");
    let mut remediation = if passed {
        None
    } else {
        Some(restore_message.unwrap_or_else(|| {
            "One or more samples failed. Confirm the backup includes the path, refresh the known-good hash only from a trusted source, then run the drill again.".into()
        }))
    };

    if let Err(error) = temp.close() {
        passed = false;
        remediation = Some(format!(
            "temporary restore target could not be removed: {error}. Remove it manually and inspect local permissions."
        ));
    }

    let id = format!(
        "rd-{}-{}-{:x}",
        created.format("%Y%m%dT%H%M%SZ"),
        created.timestamp_nanos_opt().unwrap_or_default(),
        std::process::id()
    );
    let body = ReceiptBody {
        schema_version: 1,
        id: id.clone(),
        drill_name: config.name.clone(),
        created_at: created.to_rfc3339_opts(SecondsFormat::Millis, true),
        duration_ms: started.elapsed().as_millis().min(u64::MAX as u128) as u64,
        status: if passed { "pass".into() } else { "fail".into() },
        restore_exit_code,
        config_sha256: config_hash.into(),
        previous_receipt_sha256: previous_hash,
        samples: results,
        remediation,
    };
    let receipt = sign_receipt(body)?;
    let receipt_path = write_receipt(&config.receipt_dir, &receipt)?;
    Ok(RunOutcome {
        passed,
        receipt_path,
        receipt,
    })
}

fn check_sample(target: &Path, sample: &SampleConfig, timeout: u64) -> SampleResult {
    let label = sample.path.to_string_lossy().to_string();
    let candidate = target.join(&sample.path);
    let mut result = SampleResult {
        path: label,
        status: "fail".into(),
        bytes: None,
        expected_sha256: sample.sha256.clone(),
        actual_sha256: None,
        open_checked: false,
        message: String::new(),
    };
    let metadata = match fs::symlink_metadata(&candidate) {
        Ok(value) => value,
        Err(error) => {
            result.message = format!("restored file is missing or unreadable: {error}");
            return result;
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        result.message =
            "restored sample must be a regular file, not a symlink or directory".into();
        return result;
    }
    let canonical_target = match target.canonicalize() {
        Ok(value) => value,
        Err(error) => {
            result.message = format!("could not resolve temporary target: {error}");
            return result;
        }
    };
    let canonical_file = match candidate.canonicalize() {
        Ok(value) => value,
        Err(error) => {
            result.message = format!("could not resolve restored file: {error}");
            return result;
        }
    };
    if !canonical_file.starts_with(&canonical_target) {
        result.message = "restored sample resolves outside the temporary target".into();
        return result;
    }
    result.bytes = Some(metadata.len());
    if let Some(expected) = &sample.sha256 {
        match sha256_file(&canonical_file) {
            Ok(actual) => {
                result.actual_sha256 = Some(actual.clone());
                if &actual != expected {
                    result.message =
                        "SHA-256 mismatch; the restored bytes differ from the trusted value".into();
                    return result;
                }
            }
            Err(error) => {
                result.message = format!("could not hash restored file: {error}");
                return result;
            }
        }
    }
    if let Some(check) = &sample.open_with {
        let command = substitute(check, "{file}", &canonical_file.to_string_lossy());
        result.open_checked = true;
        match run_command(&command, timeout) {
            Ok(Some(0)) => {}
            Ok(Some(code)) => {
                result.message = format!("application-open check exited with code {code}");
                return result;
            }
            Ok(None) => {
                result.message = format!("application-open check exceeded {timeout} seconds");
                return result;
            }
            Err(error) => {
                result.message = error;
                return result;
            }
        }
    }
    result.status = "pass".into();
    result.message = "restored sample passed every configured check".into();
    result
}

fn run_command(command: &[String], timeout: u64) -> DrillResult<Option<i32>> {
    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("could not start configured command: {e}"))?;
    match child
        .wait_timeout(StdDuration::from_secs(timeout))
        .map_err(|e| format!("could not wait for configured command: {e}"))?
    {
        Some(status) => Ok(status.code()),
        None => {
            let _ = child.kill();
            let _ = child.wait();
            Ok(None)
        }
    }
}

fn substitute(command: &[String], placeholder: &str, value: &str) -> Vec<String> {
    command
        .iter()
        .map(|arg| arg.replace(placeholder, value))
        .collect()
}

fn sha256_file(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn sign_receipt(body: ReceiptBody) -> DrillResult<Receipt> {
    let bytes = serde_json::to_vec(&body).map_err(|e| format!("could not encode receipt: {e}"))?;
    Ok(Receipt {
        body,
        receipt_sha256: sha256_bytes(&bytes),
    })
}

fn write_receipt(directory: &Path, receipt: &Receipt) -> DrillResult<PathBuf> {
    fs::create_dir_all(directory).map_err(|e| {
        format!(
            "could not create receipt directory {}: {e}",
            directory.display()
        )
    })?;
    let filename = format!(
        "{}-{}.json",
        receipt.body.created_at.replace([':', '-'], ""),
        receipt.body.id
    );
    let path = directory.join(filename);
    let data =
        serde_json::to_vec_pretty(receipt).map_err(|e| format!("could not encode receipt: {e}"))?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    let mut file = options
        .open(&path)
        .map_err(|e| format!("could not create receipt {}: {e}", path.display()))?;
    file.write_all(&data)
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("could not persist receipt {}: {e}", path.display()))?;
    let mut permissions = file
        .metadata()
        .map_err(|e| format!("could not inspect receipt: {e}"))?
        .permissions();
    permissions.set_readonly(true);
    fs::set_permissions(&path, permissions)
        .map_err(|e| format!("could not make receipt read-only: {e}"))?;
    Ok(path)
}

pub fn load_receipts(directory: &Path) -> DrillResult<Vec<(PathBuf, Receipt)>> {
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut paths: Vec<PathBuf> = fs::read_dir(directory)
        .map_err(|e| {
            format!(
                "could not read receipt directory {}: {e}",
                directory.display()
            )
        })?
        .filter_map(|entry| entry.ok().map(|item| item.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect();
    paths.sort();
    let mut receipts = Vec::new();
    let mut expected_previous: Option<String> = None;
    for path in paths {
        let raw = fs::read(&path)
            .map_err(|e| format!("could not read receipt {}: {e}", path.display()))?;
        let receipt: Receipt = serde_json::from_slice(&raw)
            .map_err(|e| format!("invalid receipt {}: {e}", path.display()))?;
        let computed = sign_receipt(receipt.body.clone())?.receipt_sha256;
        if computed != receipt.receipt_sha256 {
            return Err(format!(
                "receipt integrity check failed: {}",
                path.display()
            ));
        }
        if receipt.body.previous_receipt_sha256 != expected_previous {
            return Err(format!("receipt chain is broken at {}", path.display()));
        }
        expected_previous = Some(receipt.receipt_sha256.clone());
        receipts.push((path, receipt));
    }
    Ok(receipts)
}

pub fn status(config: &Config) -> DrillResult<StatusOutcome> {
    let receipts = load_receipts(&config.receipt_dir)?;
    let last_success = receipts
        .iter()
        .rev()
        .find(|(_, receipt)| receipt.body.status == "pass");
    if receipts
        .last()
        .is_some_and(|(_, receipt)| receipt.body.status == "fail")
    {
        let last_at = last_success
            .map(|(_, receipt)| DateTime::parse_from_rfc3339(&receipt.body.created_at))
            .transpose()
            .map_err(|e| format!("receipt has invalid created_at: {e}"))?
            .map(|time| time.with_timezone(&Utc));
        let due = last_at.map(|time| time + Duration::days(config.cadence_days.into()));
        return Ok(StatusOutcome {
            status: "failed".into(),
            drill_name: config.name.clone(),
            cadence_days: config.cadence_days,
            last_success_at: last_at.map(|time| time.to_rfc3339_opts(SecondsFormat::Secs, true)),
            due_at: due.map(|time| time.to_rfc3339_opts(SecondsFormat::Secs, true)),
            days_overdue: due
                .map(|time| (Utc::now() - time).num_days().max(0))
                .unwrap_or(0),
            message: "The latest restore drill failed. Follow its receipt remediation and run the drill again.".into(),
        });
    }
    let last = receipts
        .iter()
        .rev()
        .find(|(_, receipt)| receipt.body.status == "pass");
    let Some((_, receipt)) = last else {
        return Ok(StatusOutcome {
            status: "never_run".into(),
            drill_name: config.name.clone(),
            cadence_days: config.cadence_days,
            last_success_at: None,
            due_at: None,
            days_overdue: 0,
            message: "No successful receipt exists. Run restore-drill run now.".into(),
        });
    };
    let last_at = DateTime::parse_from_rfc3339(&receipt.body.created_at)
        .map_err(|e| format!("receipt has invalid created_at: {e}"))?
        .with_timezone(&Utc);
    let due = last_at + Duration::days(config.cadence_days.into());
    let now = Utc::now();
    let overdue = now > due;
    Ok(StatusOutcome {
        status: if overdue {
            "overdue".into()
        } else {
            "current".into()
        },
        drill_name: config.name.clone(),
        cadence_days: config.cadence_days,
        last_success_at: Some(last_at.to_rfc3339_opts(SecondsFormat::Secs, true)),
        due_at: Some(due.to_rfc3339_opts(SecondsFormat::Secs, true)),
        days_overdue: if overdue {
            (now - due).num_days().max(0)
        } else {
            0
        },
        message: if overdue {
            "The last successful drill is overdue. Run restore-drill run now.".into()
        } else {
            "The last successful restore drill is current.".into()
        },
    })
}

pub const EXAMPLE_CONFIG: &str = r#"version = 1
name = "monthly essentials"
cadence_days = 30
receipt_dir = ".restore-drill/receipts"
timeout_seconds = 900

[restore]
command = ["restic", "restore", "latest", "--target", "{target}", "--include", "/Documents/tax.pdf"]

[[sample]]
path = "Documents/tax.pdf"
sha256 = "replace-with-known-good-lowercase-sha256"
open_with = ["pdftotext", "{file}", "/dev/null"]
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn config(command: Vec<String>, expected: &str, dir: PathBuf) -> Config {
        Config {
            version: 1,
            name: "fixture".into(),
            cadence_days: 30,
            receipt_dir: dir,
            timeout_seconds: 5,
            restore: RestoreConfig { command },
            samples: vec![SampleConfig {
                path: "docs/proof.txt".into(),
                sha256: Some(expected.into()),
                open_with: Some(vec![
                    "sh".into(),
                    "-c".into(),
                    "test -s \"$1\"".into(),
                    "check".into(),
                    "{file}".into(),
                ]),
            }],
        }
    }

    #[test]
    fn rejects_unsafe_sample_paths_and_missing_target() {
        let cfg = Config {
            version: 1,
            name: "x".into(),
            cadence_days: 30,
            receipt_dir: "receipts".into(),
            timeout_seconds: 5,
            restore: RestoreConfig {
                command: vec!["true".into()],
            },
            samples: vec![SampleConfig {
                path: "../secret".into(),
                sha256: Some("a".repeat(64)),
                open_with: None,
            }],
        };
        assert!(validate_config(&cfg).is_err());
    }

    #[test]
    fn passes_fixture_and_detects_hash_corruption() {
        let root = tempfile::tempdir().unwrap();
        let good = sha256_bytes(b"recover me\n");
        let script = "mkdir -p \"$1/docs\"; printf 'recover me\\n' > \"$1/docs/proof.txt\"";
        let cfg = config(
            vec![
                "sh".into(),
                "-c".into(),
                script.into(),
                "restore".into(),
                "{target}".into(),
            ],
            &good,
            root.path().join("receipts"),
        );
        validate_config(&cfg).unwrap();
        let outcome = run_drill(&cfg, "config-hash").unwrap();
        assert!(outcome.passed);
        assert_eq!(outcome.receipt.body.samples[0].status, "pass");

        let bad = config(
            cfg.restore.command.clone(),
            &"0".repeat(64),
            root.path().join("bad-receipts"),
        );
        let outcome = run_drill(&bad, "config-hash").unwrap();
        assert!(!outcome.passed);
        assert!(outcome.receipt.body.samples[0].message.contains("mismatch"));
    }

    #[test]
    fn detects_missing_sample_and_receipt_tampering() {
        let root = tempfile::tempdir().unwrap();
        let cfg = config(
            vec![
                "sh".into(),
                "-c".into(),
                "true".into(),
                "restore".into(),
                "{target}".into(),
            ],
            &"0".repeat(64),
            root.path().join("receipts"),
        );
        let outcome = run_drill(&cfg, "config-hash").unwrap();
        assert!(!outcome.passed);
        assert!(outcome.receipt.body.samples[0].message.contains("missing"));
        let mut receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(&outcome.receipt_path).unwrap()).unwrap();
        receipt["status"] = serde_json::Value::String("pass".into());
        let mut perms = fs::metadata(&outcome.receipt_path).unwrap().permissions();
        perms.set_readonly(false);
        fs::set_permissions(&outcome.receipt_path, perms).unwrap();
        fs::write(&outcome.receipt_path, serde_json::to_vec(&receipt).unwrap()).unwrap();
        assert!(load_receipts(&cfg.receipt_dir).is_err());
    }
}
