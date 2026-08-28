use clap::{Parser, Subcommand};
use restore_drill::{EXAMPLE_CONFIG, load_config, load_receipts, run_drill, status};
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "restore-drill",
    version,
    about = "Prove selected backup files can actually be restored",
    long_about = "Runs a configured restore into a new temporary directory, verifies selected samples, cleans up, and records a tamper-evident receipt. No shell, network service, telemetry, or credential storage."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Write a safe starter TOML configuration; never overwrites a file
    Init {
        #[arg(short, long, default_value = "restore-drill.toml")]
        config: PathBuf,
    },
    /// Parse and safety-check configuration without restoring anything
    Check {
        #[arg(short, long, default_value = "restore-drill.toml")]
        config: PathBuf,
    },
    /// Restore samples, verify them, clean up, and write an immutable receipt
    Run {
        #[arg(short, long, default_value = "restore-drill.toml")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Report current, overdue, or never-run status from verified receipts
    Status {
        #[arg(short, long, default_value = "restore-drill.toml")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Verify the receipt chain and list local evidence
    Receipts {
        #[arg(short, long, default_value = "restore-drill.toml")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let code = match execute(Cli::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!(
                "restore-drill: {error}\nHint: run `restore-drill check --config <path>` and correct the reported issue."
            );
            2
        }
    };
    std::process::exit(code);
}

fn execute(cli: Cli) -> Result<i32, String> {
    match cli.command {
        Commands::Init { config } => {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&config)
                .map_err(|e| {
                    format!(
                        "could not create {} without overwriting: {e}",
                        config.display()
                    )
                })?;
            file.write_all(EXAMPLE_CONFIG.as_bytes())
                .map_err(|e| format!("could not write {}: {e}", config.display()))?;
            println!(
                "Created {}\nNext: edit the restore command, sample path, and trusted SHA-256; then run `restore-drill check`.",
                config.display()
            );
            Ok(0)
        }
        Commands::Check { config } => {
            let (config, _) = load_config(&config)?;
            println!(
                "Configuration is safe to run: {} ({} sample{})",
                config.name,
                config.samples.len(),
                if config.samples.len() == 1 { "" } else { "s" }
            );
            Ok(0)
        }
        Commands::Run {
            config,
            json: as_json,
        } => {
            let (config, hash) = load_config(&config)?;
            let outcome = run_drill(&config, &hash)?;
            if as_json {
                println!(
                    "{}",
                    serde_json::to_string(&outcome.receipt).map_err(|e| e.to_string())?
                );
            } else if outcome.passed {
                println!(
                    "PASS  {} sample(s) restored and verified\nReceipt  {}",
                    outcome.receipt.body.samples.len(),
                    outcome.receipt_path.display()
                );
            } else {
                println!(
                    "FAIL  restore drill did not prove recovery\nReceipt  {}\nNext  {}",
                    outcome.receipt_path.display(),
                    outcome
                        .receipt
                        .body
                        .remediation
                        .as_deref()
                        .unwrap_or("Inspect the failed sample and rerun.")
                );
            }
            Ok(if outcome.passed { 0 } else { 1 })
        }
        Commands::Status {
            config,
            json: as_json,
        } => {
            let (config, _) = load_config(&config)?;
            let outcome = status(&config)?;
            if as_json {
                println!(
                    "{}",
                    serde_json::to_string(&outcome).map_err(|e| e.to_string())?
                );
            } else {
                println!(
                    "{}  {}\n{}",
                    outcome.status.to_uppercase(),
                    outcome.drill_name,
                    outcome.message
                );
                if let Some(due) = &outcome.due_at {
                    println!("Due  {due}");
                }
            }
            Ok(match outcome.status.as_str() {
                "current" => 0,
                "failed" => 1,
                "overdue" => 3,
                _ => 4,
            })
        }
        Commands::Receipts {
            config,
            json: as_json,
        } => {
            let (config, _) = load_config(&config)?;
            let receipts = load_receipts(&config.receipt_dir)?;
            if as_json {
                let view: Vec<_> = receipts
                    .iter()
                    .map(|(path, receipt)| json!({"path": path, "receipt": receipt}))
                    .collect();
                println!(
                    "{}",
                    serde_json::to_string(&view).map_err(|e| e.to_string())?
                );
            } else if receipts.is_empty() {
                println!(
                    "No receipts yet. Run `restore-drill run --config {}`.",
                    config.receipt_dir.display()
                );
            } else {
                println!(
                    "Receipt chain verified ({} receipt{})",
                    receipts.len(),
                    if receipts.len() == 1 { "" } else { "s" }
                );
                for (_, receipt) in receipts {
                    println!(
                        "{}  {:4}  {}",
                        receipt.body.created_at,
                        receipt.body.status.to_uppercase(),
                        receipt.receipt_sha256
                    );
                }
            }
            Ok(0)
        }
    }
}
