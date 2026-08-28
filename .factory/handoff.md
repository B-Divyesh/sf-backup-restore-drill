# Restore Drill v0.1.0 — handoff

## What shipped

- A typed Rust/Clap single-binary CLI with `init`, `check`, `run`, `status`, and `receipts` commands, helpful subcommand help, stable exit codes, and JSON output for schedulers.
- Tool-agnostic restore execution using explicit argument arrays and an OS-created temporary target. The target is removed before a receipt is written; cleanup failure fails the drill.
- Verification for regular-file presence, SHA-256, and optional application-open commands. Relative-path traversal, symlinks, duplicate samples, malformed hashes, missing placeholders, and timeouts are rejected or fail loudly.
- Individual read-only JSON receipts with a SHA-256 chain, config fingerprint, per-sample result, duration, cleanup-aware status, and remediation. `receipts` verifies the entire chain. `status` alerts on the latest failed drill, an overdue success, or no prior success.
- A responsive static product/install site at `dist/site/` with a keyboard-accessible recorded pass/fail demo, offline shell and explicit offline state, privacy and terms pages, security headers, cache policy, and no analytics or third-party runtime requests.
- A product-specific risograph evidence-desk visual system and an original generated hero illustration. Prompt/model provenance is in `.factory/restore-path.prompt.json`; the deployed WebP is 88 KB.
- README, MIT license, security model, changelog, package metadata, integration/unit/browser tests, and release build scripts.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo package --allow-dirty
```

`npm test` passed on 2026-08-28:

- 3 Rust unit tests: successful fixture, corrupt hash, missing sample, tampered receipt, failed status, and unsafe configuration.
- 2 Rust CLI integration tests: documented run/check/status/receipt flow, no-overwrite init, and operational exit code.
- 10 Playwright checks across desktop Chromium and a 390 × 844 mobile viewport: semantics, overflow, Axe, pass/fail demo, offline reload, privacy, terms, and console errors.

`npm run build` passed and produced:

- `dist/site/index.html` (deploy root)
- initial JavaScript 4.17 KB raw / 1.66 KB gzip
- CSS 13.26 KB raw / 3.88 KB gzip
- hero WebP 88 KB at 1200 × 800
- stripped Linux release binary `dist/bin/restore-drill` at 1.1 MB

`cargo package --allow-dirty` passed: 240.9 KiB source package / 135.1 KiB compressed. The factory should publish; the worker did not publish.

## Lighthouse-class verification

Lighthouse 13.4.1, mobile defaults, production preview, headless Chromium:

| Category / metric | Result |
| --- | ---: |
| Performance | 97 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP | 1.0 s |
| LCP | 1.5 s |
| CLS | 0 |
| Speed Index | 1.0 s |
| Total Blocking Time | 210 ms |

The synthetic run does not report field INP; the only browser interactions are immediate native button handlers, and the JavaScript payload is 4.17 KB. Axe reported no serious or critical violations in either viewport.

## Deployment

- Build command: `npm ci && npm run build`
- Static deploy directory: `dist/site`
- `index.html` is at that exact root.
- Do not deploy `dist/bin` as the static site. Attach platform-specific binaries to releases when the factory release pipeline is available.

## Known gaps / next steps

- The build container produced and exercised a Linux binary only. Add macOS, Windows, and Linux release-matrix artifacts in factory CI before publishing binaries.
- Local receipts are read-only and tamper-evident, not host-level append-only. Operators with a local-administrator threat model should sync the receipt directory to external append-only storage, as documented.
- Restore behavior is integration-tested with deterministic shell fixtures rather than live restic/Borg repositories. Add tool-specific cookbook examples as operator feedback arrives; the core contract intentionally remains tool-agnostic.
- The landing page links to source until the factory creates signed release assets. No registry or release was published from this worker.
