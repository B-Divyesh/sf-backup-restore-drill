# Restore Drill v0.1.0 — repair handoff: PASS

**Repair commit:** `251dec4` (`fix: enforce static response policy`)
**Repaired deployment:** <https://backup-restore-drill.sociobot.in/>
**Verified:** 2026-08-28 UTC
**Release status:** **PASS**

## What changed

Independent verification correctly found that Azure Static Web Apps ignored the
committed Cloudflare/Netlify-style `site/public/_headers` file. The generated
site therefore reached production without the declared CSP, Permissions-Policy,
no-referrer policy, immutable asset caching, or service-worker revalidation.

The repair adds Azure Static Web Apps' native
`site/public/staticwebapp.config.json`. It declares the restrictive global
response policy, explicit default document revalidation, immutable caching for
`/assets/*` and `/restore-path.webp`, and `no-cache` for `/sw.js`. The existing
`_headers` file remains as a portability declaration for hosts that support it.

`npm run test:response-policy` is new regression coverage. It builds the site
and fails unless both the source configuration and deployable
`dist/site/staticwebapp.config.json` contain every required security and cache
rule. `npm test` runs it before the browser suite. The read-only receipt
tampering test now replaces the file through its writable directory instead of
relaxing its mode, so the full clippy gate is clean while preserving the
host-level tampering scenario.

## Verification evidence

```sh
npm ci                                      # PASS: 23 packages audited, 0 vulnerabilities
npm test                                    # PASS: 3 Rust unit + 2 CLI integration + 10 Playwright cases
cargo fmt --check                          # PASS
cargo clippy --all-targets -- -D warnings  # PASS
npm run build                               # PASS: dist/site and 1.1 MB stripped dist/bin/restore-drill
cargo package --allow-dirty                 # PASS: target/package/restore-drill-0.1.0.crate (139 KiB)
```

The packaged crate was unpacked, installed into an empty `cargo install --root`
prefix, and its installed `restore-drill --help` exited successfully. Do not
publish from this worker; the package is ready for the factory's registry flow.

Azure Static Web Apps deployment `2634a764-2c58-41e5-9a4d-665e1b83f446`
succeeded using `dist/site`. Fresh production `HEAD` checks now confirm:

- `/`: CSP `default-src 'self'`, `Permissions-Policy: camera=(), microphone=(), geolocation=()`, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and `Cache-Control: public, max-age=0, must-revalidate`.
- `/assets/main-BuN2cYfe.js` and `/restore-path.webp`: `Cache-Control: public, max-age=31536000, immutable`.
- `/sw.js`: `Cache-Control: no-cache`.

Built and live response identity checks passed exactly:

```text
index.html  2a48ba45eb7a2b6e22fee6c438a438885a797c2e21a9d1e7daddfbfc9ac04702
sw.js       095b8e10ae355738129b01edbfcec19a001be30ccb6452f77084750bb5d5841c
```

`verify-url.sh` against the live URL passed: HTTPS 200, 789 ms load, correct
title/lang/main/one H1, image alt text, no unlabeled buttons, and no console or
page errors. A separate fresh live Playwright run passed at 1366×900 and
390×844: zero horizontal overflow, zero serious/critical Axe violations, no
automatic cross-origin requests, Tab reached the demo action and Enter ran it,
and an offline reload after service-worker registration displayed the offline
banner. Live Lighthouse mobile: **Performance 100**, **Accessibility 100**,
FCP **0.9 s**, LCP **1.2 s**, CLS **0**, TBT **0 ms**.

## How to run and release

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm run build
cargo package --allow-dirty
```

Deploy `dist/site` as an Azure Static Web Apps site. After deployment, run
`npm run test:response-policy` and fresh `curl -sSI` checks for `/`, `/sw.js`,
a hashed asset, and `/restore-path.webp`; the native
`staticwebapp.config.json` must be present at the site root.

## Known gaps / next steps

No release-blocking gaps remain. Receipt files are intentionally tamper-evident
rather than host-immutable; operators who need host-level immutability should
copy the receipt directory to append-only storage, as documented.
