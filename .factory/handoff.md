# Restore Drill — adversarial review 2 handoff

**Work order:** `backup-restore-drill-review-2`
**Candidate:** `93ea4c9153b8ac362c9846780989f454e15585cd`
**Verdict:** **FAIL**

## Done

- Reviewed the live site cold at 390×844 and 1366×900.
- Audited landing and README copy, demo behavior, browser/CLI isolation, live offline behavior, request traffic, claims, prior findings, routes, links, metadata, accessibility, and visual identity.
- Recorded the full evidence and 24 findings in `.factory/review-2.md` without modifying product code.

## Verification

```text
npm ci                         PASS
npm run build                  PASS (dist/site and dist/bin/restore-drill)
npm test                       PASS (5 Rust tests, response policy, 20 Playwright tests)
7 claims.json commands         PASS individually from a clean clone
Live Axe, 390px + desktop      0 serious/critical violations
Live link crawl                no dead destination
Live offline demo reload       PASS
Browser demo storage/requests  empty storage; same-origin requests only
CLI demo in temp directory     PASS; marker unchanged; demo workspace removed
```

Clean claim clone: `/tmp/restore-drill-review-2.QEbqAl/repo`. CLI sandbox: `/tmp/restore-drill-cli-review-2.E62B39`.

## Blocking gaps

- The one-click phone demo hides its sample output and receipt below the first viewport and has no recording of the real CLI.
- The claim registry remains incomplete; the SHA-256 claim test only reads static browser text.
- Landing copy says the configured backup command restores only selected paths, which the CLI does not enforce.
- Direct/cross-page hashes and Back restore visual position without moving focus or announcing the section.

See `.factory/review-2.md` for exact quotes, all copy counts, unlisted claims, prior-finding verification, and concrete fixes.
