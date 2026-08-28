# Independent verification — FAIL

**Candidate:** `0d405d027fdab791d64a6b3aec9859ad0b2d8275`
**Live URL:** <https://backup-restore-drill.sociobot.in/>
**Verified:** 2026-08-28 UTC
**Verdict:** **FAIL — do not release this deployment until its response-policy and immutable-cache configuration is applied.**

The candidate source and live HTML are otherwise the same: SHA-256 of the freshly built `dist/site/index.html` and of the live `/` response was `2a48ba45eb7a2b6e22fee6c438a438885a797c2e21a9d1e7daddfbfc9ac04702` (11,949 bytes).

## Release gates

| Check | Result | Evidence |
| --- | --- | --- |
| Clean install | PASS | `npm ci`: 23 packages audited, 0 vulnerabilities. |
| Unit/integration/browser suite | PASS | `npm test`: 3 Rust unit + 2 CLI integration + 10 Playwright cases, all passed. |
| Exact production build | PASS | `npm run build`: static site and stripped `dist/bin/restore-drill` (1.1 MB) produced. |
| Package readiness | PASS | `cargo package --allow-dirty`: package and verification build passed; 244.8 KiB unpacked / 136.6 KiB compressed. |
| Consumer install | PASS | Installed the packaged crate into an empty `cargo install --root` prefix; its sole `restore-drill` binary ran `--help` successfully. |
| CLI end-to-end | PASS | Independent fixtures exercised never-run (exit 4), invalid config (2), safe pass with SHA-256 and open check (0), current status (0), missing sample with remediation (1), and failed status (1). Receipts were verified and `0444`; no `/tmp/restore-drill-*` target remained. |
| Desktop/mobile/accessibility | PASS | Fresh live Chromium checks at desktop and 390×844 mobile: one `<h1>` and `<main>`, no horizontal overflow, keyboard pass-demo activation, no console/page errors, no Axe serious/critical findings, and only first-party automatic requests. Reduced-motion timing was `0.00001s` on mobile. |
| Offline/PWA | PASS | Live service worker registered at `/sw.js`; after registration and an online reload, offline reload showed the offline banner and the cached home page without errors. |
| Performance/budgets | PASS | Built JS 4.17 KB raw / 1.66 KB gzip; CSS 13.26 KB raw / 3.88 KB gzip; hero WebP 90,028 bytes. Live mobile Lighthouse: Performance 100, Accessibility 100, FCP 0.9 s, LCP 1.4 s, CLS 0, TBT 80 ms. |
| Privacy/outbound traffic | PASS | No analytics, fonts, scripts, or automatic requests leave the site origin. Source and runtime inspection show local-only CLI operation and suppressed subprocess output. |
| Live deployment policy | **FAIL** | See blocking defects below. |

`cargo fmt --check` passed. An extra non-gating `cargo clippy --all-targets -- -D warnings` check failed only on test code that calls `Permissions::set_readonly(false)` while tampering with a test receipt; it does not affect the release artifact, but should be cleaned up in a later maintenance change.

## Blocking defects

### Medium — committed response protections are absent in production

`site/public/_headers` specifies a restrictive CSP, `Permissions-Policy`, `Referrer-Policy: no-referrer`, and `X-Content-Type-Options: nosniff`. Fresh `curl -I` checks on `/`, `/sw.js`, `/assets/main-BuN2cYfe.js`, `/restore-path.webp`, `/privacy/`, and `/terms/` showed only the host's `Referrer-Policy: strict-origin-when-cross-origin` and `X-Content-Type-Options: nosniff`; **no `Content-Security-Policy` and no `Permissions-Policy`** were served.

This is a deployment configuration failure, not a source-build mismatch. Apply equivalent platform response-header configuration and recheck the live URL before release.

### Medium — immutable asset caching is absent in production

The same committed `_headers` sets `/assets/*` and `/restore-path.webp` to `public, max-age=31536000, immutable`, and `sw.js` to `no-cache`. The live host serves all sampled paths, including hashed JS, WebP, and `sw.js`, as `cache-control: public, must-revalidate, max-age=30`.

Apply the declared asset and service-worker cache controls at the deployment platform, then verify with fresh HEAD requests.

## Non-blocking observations

- Source, documentation, visual thesis, privacy/terms, MIT license, changelog, help text, JSON output, stable exit codes, and no-interactive CI behavior meet the CLI contract.
- Read-only local receipts are tamper-evident rather than host-level immutable; this is accurately documented and external append-only storage remains the appropriate operator mitigation.
- The live HTML exactly matches this candidate, so a source redeploy is not indicated; the deployment policy layer is the required fix.

## Reverification commands

```sh
npm ci
npm test
npm run build
cargo package --allow-dirty
curl -sSI https://backup-restore-drill.sociobot.in/
curl -sSI https://backup-restore-drill.sociobot.in/assets/main-BuN2cYfe.js
curl -sSI https://backup-restore-drill.sociobot.in/sw.js
```
