use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

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
