# Restore Drill

Restore Drill is a local, backup-tool-agnostic CLI for people who already have scripted backups but want repeatable evidence that selected files can be restored. It restores into a newly created temporary directory, checks presence and SHA-256 hashes, optionally asks the file's real application to open or validate it, removes the temporary directory, and writes a tamper-evident receipt.

It does not move backups, store repository credentials, inspect file contents, or provide disaster-recovery orchestration.

## Install

Download a release binary when releases are available, or build from source:

```sh
cargo install --path .
restore-drill --help
```

Rust 1.85+ is required to build. The compiled CLI has no runtime dependencies.

## Usage

Create a starter file and edit its explicit argument arrays for your backup tool:

```sh
restore-drill init --config restore-drill.toml
restore-drill check --config restore-drill.toml
restore-drill run --config restore-drill.toml
restore-drill status --config restore-drill.toml
restore-drill receipts --config restore-drill.toml
```

Use `--json` on `run`, `status`, or `receipts` for schedulers. Commands never invoke a shell. `{target}` is replaced only with the temporary directory Restore Drill created; `{file}` in an open-check is replaced with the restored sample's absolute path.

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

The restore command must contain `{target}` exactly once. Sample paths must be relative and cannot contain `..`. Restore Drill refuses symlinks and any resolved file outside its temporary target. It deletes that target after every result, including failures.

Exit codes are stable:

| Code | Meaning |
| ---: | --- |
| 0 | Command completed; drill passed or status is current |
| 1 | A restore or verification failed |
| 2 | Invalid configuration or operational error |
| 3 | Last successful drill is overdue |
| 4 | No successful receipt exists yet |

Receipts are individual read-only JSON files linked by SHA-256. `restore-drill receipts` verifies the complete chain before displaying it. Copy or sync the receipt directory to append-only storage if host-level immutability is required.

## Scheduler examples

Run daily and alert only when the drill itself fails:

```cron
17 4 * * * /usr/local/bin/restore-drill run --config /etc/restore-drill.toml --json || /usr/local/bin/notify-admin "Restore drill failed"
```

Or perform drills separately and alert only when overdue:

```cron
20 8 * * * /usr/local/bin/restore-drill status --config /etc/restore-drill.toml --json || /usr/local/bin/notify-admin "Restore drill needs attention"
```

## Develop and verify

```sh
npm install
npm test
npm run build
npm run build:site
cargo package --allow-dirty
```

`npm run build` creates release binaries in `dist/bin/` and the deployable documentation site in `dist/site/`. The static site can be developed with `npm run dev`.

The deployable site includes `staticwebapp.config.json`, which is the Azure Static Web Apps response-policy contract: it sends the restrictive CSP, permissions and referrer policies, caches only hashed assets and the immutable hero for a year, and keeps `sw.js` revalidating. Verify that contract after a production deployment with:

```sh
npm run test:response-policy
curl -sSI https://backup-restore-drill.sociobot.in/
curl -sSI https://backup-restore-drill.sociobot.in/sw.js
```

## Privacy and security

Everything runs locally. There is no telemetry, account, cloud API, or credential storage. Restore subprocess output is deliberately not copied into receipts because tools sometimes echo repository locations or sensitive arguments. Prefer a read-only repository credential scoped outside this config file.

See [SECURITY.md](SECURITY.md) for the threat model. This software is MIT licensed; see [LICENSE](LICENSE).
