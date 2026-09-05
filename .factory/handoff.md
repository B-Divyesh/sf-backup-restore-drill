# Restore Drill — repair 2 handoff

**Work order:** `backup-restore-drill-repair-2`
**Implementation SHA:** `35ab0c3280720a9b9a66c2d4cb3e528d91129589`
**Documentation revision:** the commit that adds this handoff follows the implementation SHA.
**Product URL:** <https://backup-restore-drill.sociobot.in>

## What changed

- Put the completed demo result first on the 390px demo screen. It shows the sample path, SHA-256 match, cleanup, receipt type, and exit code before scrolling.
- Added `restore-drill-demo-recording.svg`, a self-hosted visual capture of the real bundled command. Its provenance is recorded in the visual thesis and the page keeps an accessible text transcript.
- Reduced the landing layout so all three plain facts appear before scrolling at both 390×844 and 1366×900.
- Made direct hashes, cross-page hash links, and browser Back focus and announce the destination heading.
- Corrected restore-scope wording: the operator configures selection; Restore Drill checks the selected samples and does not claim to constrain a backup tool.
- Normalized visitor-facing safety wording to **temporary restore folder** and replaced the remaining implementation jargon.
- Completed 404 canonical, Open Graph, and Twitter metadata. The 404 remains a deliberate HTTP 404 with a return link.
- Replaced the incomplete seven-claim registry with 14 outcome-based claims. The CLI fixtures now exercise selected-file pass/mismatch/missing behavior, argument handling, cleanup ordering across failure modes, application failure, path/link safety, receipt mode/chain tampering, redaction, JSON, and all documented result codes.
- Added browser checks for the one-click demo, phone viewport, keyboard activation, designed focus, reduced motion, privacy/storage/request behavior, offline reload, route metadata, and history focus.

## Review-2 disposition

| Finding | Status | Evidence |
| --- | --- | --- |
| F-2-1 | Fixed | Completed summary and real-command recording; 390px first-screen test. |
| F-2-2, F-2-8–F-2-21 | Fixed | 14 registered claims; each has a tagged outcome test and all commands pass in a clean clone. |
| F-2-3 | Fixed | Direct, cross-page, and Back focus/announcement browser test. |
| F-2-4 | Fixed | Copy now assigns restore scope to the configured backup command and documents the limitation. |
| F-2-5 | Fixed | Desktop and phone before-fold regression test. |
| F-2-6 | Fixed | Copy audit and visitor-facing product copy use “temporary restore folder.” |
| F-2-7 | Fixed | Designed 404 has canonical, Open Graph, Twitter, noindex, and route test coverage. |
| F-2-22–F-2-24 | Fixed | README uses plain descriptions for the shipped sample, safe links, and deployment settings. |

The earlier review-1 demo, claims, immutable-receipt, routing, metadata, skeleton, and copy findings remain fixed. Local receipts are described accurately as read-only, hash-linked JSON; they are not described as host-immutable. AI is intentionally not included because deterministic restore verification is the product’s job and a network summary would not improve the proof.

## Verified locally

```text
npm ci                                                   PASS
npm test                                                 PASS — 14 Rust tests; 37 Playwright passes and 3 intentional duplicate-scope skips
npm run build                                            PASS — dist/site and dist/bin/restore-drill
cargo fmt --check                                        PASS
cargo clippy --all-targets -- -D warnings                PASS
cargo package --allow-dirty                              PASS — 55 files, 701.9 KiB unpacked
all 14 claims.json commands from a fresh clone           PASS
clean packaged consumer install                           PASS — --version, --help, demo, init, expected invalid-config recovery
```

Fresh-claim clone: `/tmp/restore-drill-clean.qsj0fg/repo` (all commands reached the final `passed` record). Consumer sandbox: `/tmp/restore-drill-consumer.urdjVV`.

Local cold browser checks opened desktop 1366×900 and phone 390×844. Before scrolling, the job is “verify one file restores,” the audience is people with scripted backups, and the first action is “Try it with sample data.” The demo banner persisted through reset; demo storage and cookies stayed empty; and the real-data traps in the CLI sandbox stayed unchanged.

Local mobile Lighthouse, with the bundled Chromium executable and full-page screenshots disabled, reported **Performance 100**, **Accessibility 100**, FCP **0.92 s**, LCP **1.67 s**, CLS **0**, and TBT **0 ms**. Built assets: JavaScript **2,045 B gzip**, CSS **4,404 B gzip**, hero WebP **90,028 B**.

## Deployment status

`35ab0c3` was pushed to `origin/main`. The static site uses the committed durable `staticwebapp.config.json` and does not introduce state, volumes, environment changes, or replicas.

At 2026-09-05 20:23 UTC, HTTPS still served the prior artifact (`Last-Modified: 2026-08-28`, 6,911-byte home document), so the new implementation could not be cold-validated on the public URL in this session. The prior live response continued to return HTTPS, CSP, Permissions-Policy, `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and revalidation caching. No repository deployment command or workflow is present; deployment remains a factory-owned external step. Recheck the live build marker `build repair-2`, the completed demo summary, and headers after the factory static deployment consumes `main`.

## Release artifacts and next steps

- Publish only through the factory registry process when desired: `cargo package --allow-dirty` has produced the ready-to-publish crate.
- The catalog description is verb-first, 61 characters, and has been copied to `/work/.evidence/catalog-description.txt`.
- The only remaining gap is external deployment propagation and its cold HTTPS verification; no product-code gap is known.
