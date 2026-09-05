use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

const GOOD_CONTENT: &str = "recover me\n";
const GOOD_HASH: &str = "cd12b55fe3f747e93ed857ac3b3f629cb584dc8dc251ccfbf64233b638c735bd";

fn json_string(value: impl AsRef<str>) -> String {
    serde_json::to_string(value.as_ref()).unwrap()
}

fn command_array(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(json_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn copy_command(source: &Path, record: Option<&Path>) -> String {
    let script = if record.is_some() {
        "printf '%s\\n' \"$1\" > \"$2\"; mkdir -p \"$1/docs\"; cp \"$3\" \"$1/docs/proof.txt\""
    } else {
        "mkdir -p \"$1/docs\"; cp \"$2\" \"$1/docs/proof.txt\""
    };
    let mut values = vec![
        "sh".to_string(),
        "-c".to_string(),
        script.to_string(),
        "restore".to_string(),
        "{target}".to_string(),
    ];
    if let Some(path) = record {
        values.push(path.display().to_string());
    }
    values.push(source.display().to_string());
    command_array(&values)
}

fn write_config(
    root: &Path,
    name: &str,
    command: &str,
    expected_hash: &str,
    open_with: Option<&str>,
    timeout_seconds: u64,
) -> (PathBuf, PathBuf) {
    let receipts = root.join("receipts");
    let config = root.join("drill.toml");
    let open = open_with
        .map(|command| format!("open_with = {command}\n"))
        .unwrap_or_default();
    fs::write(
        &config,
        format!(
            r#"version = 1
name = "{name}"
cadence_days = 30
receipt_dir = "{}"
timeout_seconds = {timeout_seconds}

[restore]
command = {command}

[[sample]]
path = "docs/proof.txt"
sha256 = "{expected_hash}"
{open}"#,
            receipts.display()
        ),
    )
    .unwrap();
    (config, receipts)
}

fn run_json(config: &Path) -> std::process::Output {
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["run", "--json", "--config", config.to_str().unwrap()])
        .output()
        .unwrap()
}

fn receipt_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

#[test]
fn documented_fixture_runs_and_reports_current_status() {
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("drill.toml");
    let receipts = root.path().join("evidence");
    fs::write(
        &config,
        format!(
            r#"version = 1
name = "integration fixture"
cadence_days = 30
receipt_dir = "{}"
timeout_seconds = 5

[restore]
command = ["sh", "-c", "mkdir -p \"$1/docs\"; printf 'recover me\\n' > \"$1/docs/proof.txt\"", "restore", "{{target}}"]

[[sample]]
path = "docs/proof.txt"
sha256 = "cd12b55fe3f747e93ed857ac3b3f629cb584dc8dc251ccfbf64233b638c735bd"
open_with = ["test", "-s", "{{file}}"]
"#,
            receipts.display()
        ),
    )
    .unwrap();

    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["check", "--config", config.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("safe to run"));

    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["run", "--json", "--config", config.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\":\"pass\""));

    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["status", "--json", "--config", config.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\":\"current\""));

    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["receipts", "--config", config.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Receipt chain verified"));
}

#[test]
fn init_never_overwrites_and_missing_config_is_operational_error() {
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("starter.toml");
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["init", "--config", config.to_str().unwrap()])
        .assert()
        .success();
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["init", "--config", config.to_str().unwrap()])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("without overwriting"));
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args([
            "status",
            "--config",
            root.path().join("missing.toml").to_str().unwrap(),
        ])
        .assert()
        .code(2);
}

#[test]
fn selected_file_verification_handles_pass_hash_mismatch_and_missing_file() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("trusted.txt");
    fs::write(&source, GOOD_CONTENT).unwrap();

    let (pass_config, _) = write_config(
        root.path(),
        "selected pass",
        &copy_command(&source, None),
        GOOD_HASH,
        Some(&command_array(&[
            "test".into(),
            "-s".into(),
            "{file}".into(),
        ])),
        5,
    );
    let pass = run_json(&pass_config);
    assert_eq!(pass.status.code(), Some(0));
    let receipt: Value = serde_json::from_slice(&pass.stdout).unwrap();
    assert_eq!(receipt["status"], "pass");
    assert_eq!(receipt["samples"][0]["path"], "docs/proof.txt");
    assert_eq!(receipt["samples"][0]["actual_sha256"], GOOD_HASH);
    assert_eq!(receipt["samples"][0]["open_checked"], true);

    let corrupt_root = tempfile::tempdir().unwrap();
    let corrupt_source = corrupt_root.path().join("trusted.txt");
    fs::write(&corrupt_source, GOOD_CONTENT).unwrap();
    let (corrupt_config, _) = write_config(
        corrupt_root.path(),
        "hash mismatch",
        &copy_command(&corrupt_source, None),
        &"0".repeat(64),
        None,
        5,
    );
    let corrupt = run_json(&corrupt_config);
    assert_eq!(corrupt.status.code(), Some(1));
    let receipt: Value = serde_json::from_slice(&corrupt.stdout).unwrap();
    assert_eq!(receipt["status"], "fail");
    assert!(
        receipt["samples"][0]["message"]
            .as_str()
            .unwrap()
            .contains("mismatch")
    );

    let missing_root = tempfile::tempdir().unwrap();
    let missing_command = command_array(&[
        "sh".into(),
        "-c".into(),
        "true".into(),
        "restore".into(),
        "{target}".into(),
    ]);
    let (missing_config, _) = write_config(
        missing_root.path(),
        "missing sample",
        &missing_command,
        GOOD_HASH,
        None,
        5,
    );
    let missing = run_json(&missing_config);
    assert_eq!(missing.status.code(), Some(1));
    let receipt: Value = serde_json::from_slice(&missing.stdout).unwrap();
    assert!(
        receipt["samples"][0]["message"]
            .as_str()
            .unwrap()
            .contains("missing or unreadable")
    );
}

#[test]
fn configured_command_receives_the_configured_arguments_without_shell_expansion() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("trusted.txt");
    let record = root.path().join("arguments.txt");
    let would_be_created = root.path().join("should-not-exist");
    fs::write(&source, GOOD_CONTENT).unwrap();
    let raw_argument = format!("literal;$(touch {})", would_be_created.display());
    let command = command_array(&[
        "sh".into(),
        "-c".into(),
        "printf '%s\\n' \"$1\" > \"$2\"; printf '%s\\n' \"$3\" >> \"$2\"; mkdir -p \"$1/docs\"; cp \"$4\" \"$1/docs/proof.txt\"".into(),
        "restore".into(),
        "{target}".into(),
        record.display().to_string(),
        raw_argument.clone(),
        source.display().to_string(),
    ]);
    let (config, _) = write_config(
        root.path(),
        "argument capture",
        &command,
        GOOD_HASH,
        None,
        5,
    );
    let outcome = run_json(&config);
    assert_eq!(outcome.status.code(), Some(0));
    let received = fs::read_to_string(&record).unwrap();
    let mut lines = received.lines();
    let temporary_folder = PathBuf::from(lines.next().unwrap());
    assert!(
        temporary_folder
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with("restore-drill-"))
    );
    assert_eq!(lines.next(), Some(raw_argument.as_str()));
    assert!(!would_be_created.exists());

    let repeated_root = tempfile::tempdir().unwrap();
    let repeated = command_array(&["true".into(), "{target}".into(), "{target}".into()]);
    let (repeated_config, _) = write_config(
        repeated_root.path(),
        "repeated",
        &repeated,
        GOOD_HASH,
        None,
        5,
    );
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["check", "--config", repeated_config.to_str().unwrap()])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("exactly once"));

    let file_placeholder_root = tempfile::tempdir().unwrap();
    let source = file_placeholder_root.path().join("trusted.txt");
    fs::write(&source, GOOD_CONTENT).unwrap();
    let bad_open = command_array(&["true".into(), "{file}".into(), "{file}".into()]);
    let (bad_open_config, _) = write_config(
        file_placeholder_root.path(),
        "bad open check",
        &copy_command(&source, None),
        GOOD_HASH,
        Some(&bad_open),
        5,
    );
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["check", "--config", bad_open_config.to_str().unwrap()])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("{file} exactly once"));
}

#[test]
fn temporary_folder_lifecycle_cleans_before_every_receipt() {
    enum Case {
        Pass,
        RestoreFailure,
        Missing,
        HashMismatch,
        ApplicationFailure,
        Timeout,
    }
    let cases = [
        Case::Pass,
        Case::RestoreFailure,
        Case::Missing,
        Case::HashMismatch,
        Case::ApplicationFailure,
        Case::Timeout,
    ];
    for case in cases {
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("trusted.txt");
        let record = root.path().join("target.txt");
        fs::write(&source, GOOD_CONTENT).unwrap();
        let (command, expected, open_with, timeout, exit_code): (
            String,
            String,
            Option<String>,
            u64,
            i32,
        ) = match case {
            Case::Pass => (
                copy_command(&source, Some(&record)),
                GOOD_HASH.into(),
                None,
                5,
                0,
            ),
            Case::RestoreFailure => (
                command_array(&[
                    "sh".into(),
                    "-c".into(),
                    "printf '%s\\n' \"$1\" > \"$2\"; exit 7".into(),
                    "restore".into(),
                    "{target}".into(),
                    record.display().to_string(),
                ]),
                GOOD_HASH.into(),
                None,
                5,
                1,
            ),
            Case::Missing => (
                command_array(&[
                    "sh".into(),
                    "-c".into(),
                    "printf '%s\\n' \"$1\" > \"$2\"".into(),
                    "restore".into(),
                    "{target}".into(),
                    record.display().to_string(),
                ]),
                GOOD_HASH.into(),
                None,
                5,
                1,
            ),
            Case::HashMismatch => (
                copy_command(&source, Some(&record)),
                "0".repeat(64),
                None,
                5,
                1,
            ),
            Case::ApplicationFailure => (
                copy_command(&source, Some(&record)),
                GOOD_HASH.into(),
                Some(command_array(&[
                    "sh".into(),
                    "-c".into(),
                    "exit 7".into(),
                    "open-check".into(),
                    "{file}".into(),
                ])),
                5,
                1,
            ),
            Case::Timeout => (
                command_array(&[
                    "sh".into(),
                    "-c".into(),
                    "printf '%s\\n' \"$1\" > \"$2\"; sleep 2".into(),
                    "restore".into(),
                    "{target}".into(),
                    record.display().to_string(),
                ]),
                GOOD_HASH.into(),
                None,
                1,
                1,
            ),
        };
        let (config, receipts) = write_config(
            root.path(),
            "lifecycle",
            &command,
            &expected,
            open_with.as_deref(),
            timeout,
        );
        let watcher_record = record.clone();
        let watcher_receipts = receipts.clone();
        let watcher = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(8);
            while !watcher_record.exists() && Instant::now() < deadline {
                thread::sleep(Duration::from_millis(5));
            }
            let target = PathBuf::from(fs::read_to_string(&watcher_record).unwrap().trim());
            while Instant::now() < deadline {
                if watcher_receipts.exists() && !receipt_files(&watcher_receipts).is_empty() {
                    return !target.exists();
                }
                thread::sleep(Duration::from_millis(5));
            }
            false
        });
        let outcome = run_json(&config);
        assert_eq!(outcome.status.code(), Some(exit_code));
        assert!(
            watcher.join().unwrap(),
            "the receipt appeared before temporary-folder cleanup"
        );
        let target = PathBuf::from(fs::read_to_string(&record).unwrap().trim());
        assert!(!target.exists());
        assert_eq!(receipt_files(&receipts).len(), 1);
    }
}

#[test]
fn application_check_controls_the_result() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("trusted.txt");
    fs::write(&source, GOOD_CONTENT).unwrap();
    let passing_check = command_array(&["test".into(), "-s".into(), "{file}".into()]);
    let (pass_config, _) = write_config(
        root.path(),
        "passing application check",
        &copy_command(&source, None),
        GOOD_HASH,
        Some(&passing_check),
        5,
    );
    assert_eq!(run_json(&pass_config).status.code(), Some(0));

    let failing_root = tempfile::tempdir().unwrap();
    let failing_source = failing_root.path().join("trusted.txt");
    fs::write(&failing_source, GOOD_CONTENT).unwrap();
    let failing_check = command_array(&[
        "sh".into(),
        "-c".into(),
        "exit 7".into(),
        "open-check".into(),
        "{file}".into(),
    ]);
    let (fail_config, _) = write_config(
        failing_root.path(),
        "failing application check",
        &copy_command(&failing_source, None),
        GOOD_HASH,
        Some(&failing_check),
        5,
    );
    let failed = run_json(&fail_config);
    assert_eq!(failed.status.code(), Some(1));
    let receipt: Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert!(
        receipt["samples"][0]["message"]
            .as_str()
            .unwrap()
            .contains("application-open check exited with code 7")
    );
}

#[test]
fn path_safety_rejects_unsafe_configuration_and_restored_links() {
    let unsafe_root = tempfile::tempdir().unwrap();
    let unsafe_command = command_array(&["true".into()]);
    let (unsafe_config, _) = write_config(
        unsafe_root.path(),
        "unsafe configuration",
        &unsafe_command,
        GOOD_HASH,
        None,
        5,
    );
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["check", "--config", unsafe_config.to_str().unwrap()])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("{target} exactly once"));

    let root = tempfile::tempdir().unwrap();
    let outside = root.path().join("outside.txt");
    fs::write(&outside, GOOD_CONTENT).unwrap();
    let command = command_array(&[
        "sh".into(),
        "-c".into(),
        "mkdir -p \"$1/docs\"; ln -s \"$2\" \"$1/docs/proof.txt\"".into(),
        "restore".into(),
        "{target}".into(),
        outside.display().to_string(),
    ]);
    let (config, _) = write_config(root.path(), "restored link", &command, GOOD_HASH, None, 5);
    let outcome = run_json(&config);
    assert_eq!(outcome.status.code(), Some(1));
    let receipt: Value = serde_json::from_slice(&outcome.stdout).unwrap();
    assert!(
        receipt["samples"][0]["message"]
            .as_str()
            .unwrap()
            .contains("not a symlink")
    );
}

#[test]
fn bundled_demo_leaves_unrelated_working_directory_data_unchanged() {
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("restore-drill.toml");
    let backup = root.path().join("backup-trap.bin");
    let receipt = root
        .path()
        .join(".restore-drill/receipts/do-not-touch.json");
    fs::create_dir_all(receipt.parent().unwrap()).unwrap();
    fs::write(&config, "this is intentionally not a valid config").unwrap();
    fs::write(&backup, "backup trap bytes").unwrap();
    fs::write(&receipt, "receipt trap bytes").unwrap();
    let before = [
        fs::read(&config).unwrap(),
        fs::read(&backup).unwrap(),
        fs::read(&receipt).unwrap(),
    ];

    let output = Command::cargo_bin("restore-drill")
        .unwrap()
        .current_dir(root.path())
        .args(["demo", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        serde_json::from_slice::<Value>(&output.stdout).unwrap(),
        serde_json::json!({
            "status": "pass",
            "sample": "Documents/quarterly-tax-notes.txt",
            "receipt_count": 1,
            "workspace": "removed"
        })
    );
    assert_eq!(fs::read(&config).unwrap(), before[0]);
    assert_eq!(fs::read(&backup).unwrap(), before[1]);
    assert_eq!(fs::read(&receipt).unwrap(), before[2]);
}

#[test]
fn receipt_files_are_read_only_json_linked_and_tamper_detected() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("trusted.txt");
    fs::write(&source, GOOD_CONTENT).unwrap();
    let (config, receipts) = write_config(
        root.path(),
        "receipt chain",
        &copy_command(&source, None),
        GOOD_HASH,
        None,
        5,
    );
    assert_eq!(run_json(&config).status.code(), Some(0));
    assert_eq!(run_json(&config).status.code(), Some(0));
    let files = receipt_files(&receipts);
    assert_eq!(files.len(), 2);
    let first: Value = serde_json::from_slice(&fs::read(&files[0]).unwrap()).unwrap();
    let second: Value = serde_json::from_slice(&fs::read(&files[1]).unwrap()).unwrap();
    assert_eq!(first["status"], "pass");
    assert_eq!(second["previous_receipt_sha256"], first["receipt_sha256"]);
    assert!(fs::metadata(&files[0]).unwrap().permissions().readonly());
    assert!(fs::metadata(&files[1]).unwrap().permissions().readonly());

    fs::remove_file(&files[1]).unwrap();
    fs::write(&files[1], b"{\"not\":\"a valid receipt\"}").unwrap();
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["receipts", "--config", config.to_str().unwrap()])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("receipt"));
}

#[test]
fn receipts_exclude_restore_output_arguments_and_restored_bytes() {
    let root = tempfile::tempdir().unwrap();
    let secret = "unique-private-restore-content-7b2c";
    let expected = format!("{:x}", Sha256::digest(format!("{secret}\n").as_bytes()));
    let command = command_array(&[
        "sh".into(),
        "-c".into(),
        format!(
            "mkdir -p \"$1/docs\"; printf '{secret}\\n' > \"$1/docs/proof.txt\"; printf '{secret}\\n' >&2; printf '{secret}\\n'"
        ),
        "restore".into(),
        "{target}".into(),
    ]);
    let (config, receipts) = write_config(root.path(), "redaction", &command, &expected, None, 5);
    let output = run_json(&config);
    assert_eq!(output.status.code(), Some(0));
    assert!(!String::from_utf8_lossy(&output.stdout).contains(secret));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(secret));
    let receipt = fs::read_to_string(&receipt_files(&receipts)[0]).unwrap();
    assert!(!receipt.contains(secret));
}

#[test]
fn json_modes_and_exit_codes_cover_current_failed_overdue_and_never_run() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("trusted.txt");
    fs::write(&source, GOOD_CONTENT).unwrap();
    let (config, receipts) = write_config(
        root.path(),
        "exit codes",
        &copy_command(&source, None),
        GOOD_HASH,
        None,
        1,
    );

    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["status", "--config", config.to_str().unwrap()])
        .assert()
        .code(4);
    let pass = run_json(&config);
    assert_eq!(pass.status.code(), Some(0));
    let pass_receipt: Value = serde_json::from_slice(&pass.stdout).unwrap();
    assert_eq!(pass_receipt["status"], "pass");

    let status = Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["status", "--json", "--config", config.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(status.status.code(), Some(0));
    assert_eq!(
        serde_json::from_slice::<Value>(&status.stdout).unwrap()["status"],
        "current"
    );
    let listed = Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["receipts", "--json", "--config", config.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(listed.status.code(), Some(0));
    assert_eq!(
        serde_json::from_slice::<Value>(&listed.stdout)
            .unwrap()
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let receipt_path = receipt_files(&receipts).remove(0);
    let mut receipt: restore_drill::Receipt =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt.body.created_at = "2020-01-01T00:00:00.000Z".into();
    receipt.receipt_sha256 = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&receipt.body).unwrap())
    );
    fs::remove_file(&receipt_path).unwrap();
    fs::write(&receipt_path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["status", "--config", config.to_str().unwrap()])
        .assert()
        .code(3);

    let failed_root = tempfile::tempdir().unwrap();
    let failed_command = command_array(&[
        "sh".into(),
        "-c".into(),
        "exit 7".into(),
        "restore".into(),
        "{target}".into(),
    ]);
    let (failed_config, _) = write_config(
        failed_root.path(),
        "failed",
        &failed_command,
        GOOD_HASH,
        None,
        5,
    );
    assert_eq!(run_json(&failed_config).status.code(), Some(1));
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["status", "--config", failed_config.to_str().unwrap()])
        .assert()
        .code(1);

    let invalid_root = tempfile::tempdir().unwrap();
    let invalid = command_array(&["true".into()]);
    let (invalid_config, _) =
        write_config(invalid_root.path(), "invalid", &invalid, GOOD_HASH, None, 5);
    Command::cargo_bin("restore-drill")
        .unwrap()
        .args(["check", "--config", invalid_config.to_str().unwrap()])
        .assert()
        .code(2);
}
