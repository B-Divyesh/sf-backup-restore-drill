# Restore Drill v0.1.0 — verification handoff: FAIL

**Candidate:** `0d405d027fdab791d64a6b3aec9859ad0b2d8275`
**Live URL:** <https://backup-restore-drill.sociobot.in/>
**Verified:** 2026-08-28 UTC
**Release status:** **FAIL**

The CLI, package, static build, desktop/mobile UI, accessibility, offline reload, privacy behavior, and live HTML all passed independent verification. The live home page is byte-for-byte identical to the freshly built candidate (`2a48ba45eb7a2b6e22fee6c438a438885a797c2e21a9d1e7daddfbfc9ac04702`).

Release is blocked by deployment-only policy defects:

- **Medium:** Production does not serve the committed `Content-Security-Policy` or `Permissions-Policy`. It serves host-default `Referrer-Policy: strict-origin-when-cross-origin` rather than the committed `no-referrer` policy.
- **Medium:** Production serves hashed JS, the hero WebP, and `sw.js` as `public, must-revalidate, max-age=30`, not the committed immutable asset cache policy and `sw.js` `no-cache` policy.

Apply equivalent response-header/cache rules in the deployment platform and rerun the live header checks. No source-code redeploy is necessary to establish candidate identity; the built and deployed HTML already match.

## Verified commands and results

```sh
npm ci                         # PASS, 0 vulnerabilities
npm test                       # PASS, 15 total Rust/CLI/browser cases
npm run build                  # PASS, dist/site and dist/bin/restore-drill
cargo package --allow-dirty    # PASS, package verification build
```

Independent packaged-consumer installation and `restore-drill --help` passed. Independent CLI fixtures verified pass, missing-file fail/remediation, never-run, current, invalid-config, JSON, receipt-chain listing, cleanup, and exit codes 0/1/2/4. A live 390 px reduced-motion browser run and desktop run had no console/page errors or Axe serious/critical findings; live offline reload passed. Live Lighthouse recorded Performance 100 and Accessibility 100 (FCP 0.9 s, LCP 1.4 s, CLS 0, TBT 80 ms). Initial JS/CSS/image payloads are 4.17 KB / 13.26 KB / 90 KB raw.

See `.factory/verification.md` for exact commands, evidence, and the full defect record.

## Follow-up

1. Configure the host to emit the committed CSP, Permissions-Policy, no-referrer policy, immutable hashed-asset/WebP caching, and `sw.js` no-cache behavior.
2. Re-run the live `curl -I` checks and update the verification verdict only after they pass.
3. Optional maintenance: make the receipt-tampering test avoid `Permissions::set_readonly(false)` so `cargo clippy --all-targets -- -D warnings` passes.
