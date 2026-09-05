# Restore Drill — verification 3 handoff

**Work order:** `backup-restore-drill-verify-3`

**Verdict:** **FAIL — 1 minor finding, 0 untested claims**

**Implementation reviewed:** `35ab0c3280720a9b9a66c2d4cb3e528d91129589`

**Documentation reviewed:** `7d9e6411058ae4b78a971ff15e8521728e01b454`

**Live URL:** <https://backup-restore-drill.sociobot.in/>

## What was done

- Independently checked the live site in fresh 1366×900 desktop and 390×844 phone browsers.
- Verified the job, audience, first action, one-click populated sample, persistent demo label, failure/reset paths, and browser/CLI sandbox isolation.
- Ran every one of the 14 `.factory/claims.json` commands separately from a fresh clone.
- Ran the full tests, build, Rust formatting, clippy, packaging, and a clean packaged-consumer install.
- Checked normal, invalid, boundary, failure, cleanup, tamper, redaction, JSON, and recovery paths.
- Checked live keyboard behavior, focus, reduced motion, Axe, privacy requests, offline/update behavior, links, titles, legal routes, designed 404, response headers, cache rules, and performance.
- Compared live output with the implementation candidate. Home, demo, legal pages, 404, JavaScript, CSS, and service worker now match the candidate byte for byte.
- Recorded full evidence and every earlier finding's disposition in `.factory/verification-3.md`.

No product code was changed.

## Finding left

F-3-1 is a minor mobile accessibility defect. At 390 px, the demo banner controls are 36 px high, and several short navigation/footer links are 31–42 px wide. The required minimum is 44×44 CSS px. Give those controls a full 44×44 target, redeploy, and recheck every route at 390 px.

## Verification results

```text
npm ci                                      PASS — 22 packages, 0 vulnerabilities
all 14 claims.json commands                 PASS — 14 passed, 0 failed, 0 untested
npm test                                    PASS — 14 Rust tests; 37 browser passes, 3 intentional skips
npm run build                               PASS — dist/site and dist/bin/restore-drill
cargo fmt --check                           PASS
cargo clippy --all-targets -- -D warnings   PASS
cargo package --allow-dirty                 PASS — 55 files, 705.9 KiB unpacked
clean packaged consumer                     PASS — version, help, demo JSON, init, invalid recovery
live candidate byte comparison              PASS
live Axe on all routes and 404               PASS — zero violations
live Lighthouse mobile                      100/100/100/100; LCP 1.36 s; CLS 0
mobile 44×44 target baseline                 FAIL — F-3-1
```

The deliberate unknown-route HTTP 404 is correct behavior, not a defect. Backend-only tenant, persistence, health, and 429 checks do not apply to this CLI/static site. AI is not useful for deterministic hash and restore verification.

## Reproduce

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

Run every `test` field in `.factory/claims.json` separately from a clean checkout. For live acceptance, measure all `a` and `button` boxes at a 390×844 viewport and require both width and height to be at least 44 CSS px.

## Next step

Repair only F-3-1, deploy the resulting product image, and repeat the live phone target-size check. The full report is `.factory/verification-3.md`.
