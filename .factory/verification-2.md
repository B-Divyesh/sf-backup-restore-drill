# Independent verification 2 — PASS

**Candidate:** `e4b72c237bf0683c472c07e4e699bb062acd023d`
**Live URL:** <https://backup-restore-drill.sociobot.in/>
**Verified:** 2026-08-28 UTC
**Verdict:** **PASS — candidate and its live deployment meet the Restore Drill acceptance contract.**

This is a fresh independent verification. It specifically retested the earlier deployment-only finding rather than relying on the repair handoff.

## Release gates

| Check | Result | Fresh evidence |
| --- | --- | --- |
| Clean checkout/install | PASS | Checkout was clean at the stated SHA; `npm ci` installed 22 packages and reported 0 vulnerabilities. |
| Repository test suite | PASS | `npm test` passed: 3 Rust library tests, 2 CLI integration tests, the deployed-response-policy check, and 10 Playwright desktop/mobile browser cases. |
| Exact production build | PASS | `npm run build` produced `dist/site/` and stripped `dist/bin/restore-drill` (1.1 MB). |
| Rust quality/package gates | PASS | `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo package --allow-dirty` passed. The package contains 37 files, 253.2 KiB unpacked / 139.4 KiB compressed. |
| Clean consumer install | PASS | Unpacked `target/package/restore-drill-0.1.0.crate`, installed it with `cargo install --root` into a clean `/tmp` prefix, and exercised installed `--help`, `--version` (`0.1.0`), `init`, and validation error handling. |
| Independent CLI end-to-end | PASS | Release binary verified never-run (exit 4), check/pass/current (0), hash mismatch/missing fixture/open-check failure/timeout (1), and invalid cadence/unsafe path (2). Cadence boundaries 1 and 3650 passed. Failure receipts gave specific messages; receipts were mode `0444`; no `/tmp/restore-drill-*` directory remained; deliberately injected private file bytes did not appear in stdout or stderr. |
| Website functionality and accessibility | PASS | Fresh live Chromium checks at 1366x900 and 390x844 found exactly one `h1`, one `main`, `lang=en`, no horizontal overflow, no console/page errors, and no Axe serious/critical findings. Tab reached the healthy-demo control and Enter ran it; Space ran the failed-demo control; computed focus outline was 3px `rgb(164, 97, 0)`. |
| Reduced motion / PWA | PASS | At 390px with reduced motion, animation and transition durations computed to `1e-05s` and root scrolling was `auto`. The live worker registered, `registration.update()` succeeded, it controlled the page with `restore-drill-shell-v1`, and an offline reload showed the reconnect notice and cached home page without errors. |
| Privacy and outbound traffic | PASS | Runtime captured four automatic first-party requests only per viewport; no automatic cross-origin request, analytics, CDN font/script, cookies, or tracking endpoint was observed. Source scan shows the worker fetches only same-origin shell assets; CLI subprocess output is suppressed and the private-fixture check found no content leak. |
| Response policy / deployment identity | PASS | Fresh `HEAD` requests to `/`, `/sw.js`, the hashed JS, hero WebP, `/privacy/`, and `/terms/` served CSP, Permissions-Policy, `Referrer-Policy: no-referrer`, and `X-Content-Type-Options: nosniff`. `/assets/main-BuN2cYfe.js` and `/restore-path.webp` use `public, max-age=31536000, immutable`; `/sw.js` uses `no-cache`; HTML uses `public, max-age=0, must-revalidate`. Built/live SHA-256 matched for `index.html` (`2a48ba45eb7a2b6e22fee6c438a438885a797c2e21a9d1e7daddfbfc9ac04702`) and `sw.js` (`095b8e10ae355738129b01edbfcec19a001be30ccb6452f77084750bb5d5841c`). |
| Budget / live mobile performance | PASS | Built JS: 4,173 bytes raw / 1.66 KB gzip; CSS: 13,255 bytes raw / 3.88 KB gzip; hero WebP: 90,028 bytes. Fresh mobile Lighthouse: Performance 100, Accessibility 100, FCP 0.9 s, LCP 1.4 s, CLS 0, TBT 90 ms. |

## CLI recovery-path evidence

The independent fixture used a shell restore command only as a test double; the product invoked it as an explicit argument array. It restored a known SHA-256 sample and used `test -s` as the open check. It then injected each failure class separately:

- corrupt bytes: `SHA-256 mismatch; the restored bytes differ from the trusted value`;
- absent output: `restored file is missing or unreadable`;
- application check exit 7: `application-open check exited with code 7`;
- restore timeout at one second: `restore command exceeded 1 seconds`.

Every failed run returned exit 1 and wrote an evidence receipt after safely cleaning the created temporary target. The normal run passed, status became current, receipts listed and verified, and a supplied private test string was absent from all CLI output.

## Defects

**None found.**

The original verification's response-header/cache defect is resolved in the live deployment. Receipt files are read-only and tamper-evident rather than host-immutable, which is the documented design and the README appropriately recommends append-only external storage where that threat model applies.

## Reproduce

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
curl -sSI https://backup-restore-drill.sociobot.in/
curl -sSI https://backup-restore-drill.sociobot.in/sw.js
```
