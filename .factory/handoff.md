# Restore Drill — polish 1 handoff

**Work order:** `backup-restore-drill-polish-1`  
**Repair commits:** `6e5069239637627730ff10fedfd3b45cbb01953b`, `9ab305f`  
**Deployment:** `https://backup-restore-drill.sociobot.in/` redeployed with `/opt/fleet/lib/deploy-static.sh backup-restore-drill dist/site` on 2026-08-28 UTC.

## Delivered

- A real bundled CLI demo: `restore-drill demo` runs the normal restore, SHA-256, application-check, cleanup, and receipt path inside a temporary workspace, then removes it.
- A direct browser demo at `/demo/` and `?demo=1`, with completed sample output, the required isolated-demo banner, reset, and start-for-real controls.
- Plain first-screen copy, a full claim registry, tagged observable claim tests, mobile navigation, consistent legal/footer links, metadata, product-derived share art, and real 404 behavior.
- The risograph evidence-desk identity remains intact. The new share image and apple icon are crops of the existing project-owned illustration; provenance is recorded in `.factory/design.md`.

## Verification evidence

From a clean dependency install:

```text
npm ci                                      PASS (23 packages, 0 vulnerabilities)
npm test                                    PASS (5 Rust tests, 20 Playwright tests)
npm run build                               PASS (dist/site and dist/bin/restore-drill)
cargo fmt --check                           PASS
cargo clippy --all-targets -- -D warnings  PASS
cargo package --allow-dirty                 PASS (51 files, 579.5 KiB unpacked)
```

Every command listed in `.factory/claims.json` passed individually. This includes demo isolation, restore verification, same-origin browser traffic, offline reload, MIT license, receipt tamper detection, and path safety.

Live cold verification passed at 390×844:

- `/`, `/demo/`, `/privacy/`, and `/terms/` have their required titles, one `h1`, and zero Axe serious/critical findings.
- `/does-not-exist-review-1` returns HTTP 404 with `Page not found — Restore Drill`.
- `/demo/` reloads offline after service-worker setup and shows its offline notice and persistent demo banner.
- Mobile overflow was `0`; normal route loads produced no console/page errors.
- Production headers include CSP, Permissions-Policy, Referrer-Policy, X-Content-Type-Options, and HTML revalidation caching.

Screenshots: `.factory/evidence/polish-1-home-390.png`, `.factory/evidence/polish-1-demo-390.png`, and `.factory/evidence/polish-1-live-demo-390.png`.

## Run and publish

```sh
npm ci
npm test
npm run build
cargo run -- demo
cargo package --allow-dirty
```

Publish the ready crate with `cargo publish` from a credentialed release environment; this work order did not publish it.

## Known gaps

None. The static site and CLI are buildable and deployed. The `navigationFallback` was intentionally removed because this is a real multi-page static site; keeping it made unknown paths return the home page instead of the required HTTP 404.
