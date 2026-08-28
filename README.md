# Restore Drill

Restore Drill is a command-line tool for people who already run scripted backups. It checks that selected files still restore.

It restores into a new temporary folder and compares SHA-256 fingerprints. It can run an application check. It removes the folder and writes a hash-linked receipt.

## Try the bundled demo

Run the real restore path with shipped sample data:

```sh
cargo run -- demo
```

The command creates and removes a disposable workspace. It does not read your configuration, backups, or receipt directory. The browser version is at [the sample demo](https://backup-restore-drill.sociobot.in/demo/).

## Install

Build from source with Rust 1.85 or later:

```sh
cargo install --path .
restore-drill --help
```

The built program has no product network client or runtime package dependency.

## Use your backup command

```sh
restore-drill init --config restore-drill.toml
restore-drill check --config restore-drill.toml
restore-drill run --config restore-drill.toml
restore-drill status --config restore-drill.toml
restore-drill receipts --config restore-drill.toml
```

`run`, `status`, and `receipts` support `--json` for scheduler and alert-tool input. Commands receive direct argument arrays. Restore commands contain `{target}` once. Application checks contain `{file}` once.

```toml
version = 1
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
```

Sample paths must be relative. Restore Drill rejects links and resolved files outside its temporary folder. It does not copy backup-command output into receipts.

| Code | Meaning |
| ---: | --- |
| 0 | The drill passed or status is current. |
| 1 | A restore or check failed. |
| 2 | Configuration or an operational step failed. |
| 3 | The last successful drill is overdue. |
| 4 | No successful receipt exists. |

Receipts are read-only JSON files linked by SHA-256. `restore-drill receipts` detects a later change to a receipt or broken chain. Copy receipts to storage that prevents changes when that risk matters.

## Develop, test, and deploy

```sh
npm ci
npm test
npm run build
cargo package --allow-dirty
```

`npm run build` creates `dist/bin/restore-drill` and `dist/site/`. Deploy `dist/site/` as a static site with the included `staticwebapp.config.json` response policy. Run each command in `.factory/claims.json` from a clean checkout before release.

## Privacy and license

The website has no forms, analytics, cookies, tracking pixels, or third-party scripts. See [Privacy](https://backup-restore-drill.sociobot.in/privacy/) and [Terms](https://backup-restore-drill.sociobot.in/terms/). Restore Drill is free and [MIT licensed](LICENSE).
