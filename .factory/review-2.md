# Adversarial first-read review 2 — Restore Drill

**Verdict: FAIL**

**Candidate reviewed:** `93ea4c9153b8ac362c9846780989f454e15585cd`
**Live URL:** <https://backup-restore-drill.sociobot.in/>
**Reviewed:** 2026-08-28 UTC
**Blocking findings:** 4

The landing page now explains the job, audience, and first action. The CLI demo also runs safely. The product still fails this round because the phone demo hides the actual result below the first screen, the claim registry leaves published behavior untested, a restore-scope sentence overstates what the CLI enforces, and the earlier route-focus finding is only partly fixed.

## Cold read before scrolling

Fresh Chromium contexts opened `/` at 390×844 and 1366×900 with `scrollY = 0`.

- **What does this do?** It verifies that an important file can be restored from an existing backup.
- **For whom?** People who already have scripted backups and want repeatable evidence.
- **What should I click first?** **Try it with sample data**.

The exact first-screen copy that supplied those answers was:

> “Verify one file restores from your backup”
>
> “For people with scripted backups who need repeatable proof that important files still restore.”
>
> “Try it with sample data”

This passes the three-question cold-read gate at both sizes. At 390 px, however, only two of the required three plain facts fit before the 844 px fold; see F-2-5.

## Findings, ordered by severity

### F-2-1 — BLOCKING — Review 1 BLOCKING 2 remains half-fixed: the phone demo does not show the product result on its first screen

**Quote/location:** After the one-click action, `/demo/` shows “Demo — sample data, nothing is saved,” “See one backup file restore,” and “This completed example mirrors `restore-drill demo`.” At 390×844, **Run sample restore again** begins at y=802 and the terminal and receipt are entirely below the fold.

**Why this fails:** The first screen after clicking does not already look like the product being used. It contains an introduction to a result, not the realistic sample output or the “All checks passed” receipt. The CLI-specific demo contract also calls for a self-hosted recording of the real command; the page provides a hand-authored terminal transcript that says it “mirrors” the command.

**Concrete fix:** Make the persistent banner one compact row on mobile, reduce the introductory height, and place a completed result summary containing `Documents/quarterly-tax-notes.txt`, the SHA-256 match, cleanup result, receipt, and exit code above the 844 px fold. Add a self-hosted terminal recording captured from the real `restore-drill demo` command. Add a 390×844 test that clicks the landing CTA once and asserts that the sample name and completed verification result are visible without scrolling.

### F-2-2 — BLOCKING — Review 1 BLOCKING 3 remains half-fixed: the claim registry is incomplete and one tagged test does not prove its claim

**Quote/location:** `.factory/claims.json` says, “The demo checks the bundled restored file against its SHA-256 fingerprint.” Its tagged test only checks pre-rendered browser text:

> `expect(page.locator("#receipt-hash")).toContainText("match")`

The test never runs the CLI for that claim, changes the bundled file, or observes a calculated digest. A static page containing “match” would pass.

The landing page and README also publish the unlisted claims itemized in F-2-8 through F-2-21. Labels such as “documented command behavior” in `.factory/copy-audit.md` are not entries in `.factory/claims.json`.

**Why this fails:** A visitor is asked to rely on behavior that the release claim suite cannot identify or prove. This is the same claim-coverage problem from review 1, despite the presence of seven new entries.

**Concrete fix:** Make `@claim:restore-verification` run `restore-drill demo` against the shipped sample, independently calculate its SHA-256 value, then corrupt or remove the restored sample and assert failure. Add the missing claim entries and tagged observable tests specified in F-2-8 through F-2-21, or remove those sentences.

### F-2-3 — BLOCKING — Review 1 MAJOR 7 remains half-fixed: deep-link and Back navigation restore position but not focus or announcement

**Quote/location:** Direct `/#setup`, **Start for real** from `/demo/`, and Back from Privacy to `/#how-it-works` all ended with the correct target at the top of the viewport, but `document.activeElement` was `<body>` and `#route-status` was empty. `site/src/main.ts` handles same-document link clicks, but it has no initial-hash handler; its `popstate` handler focuses the page H1 instead of the hash target.

**Why this fails:** Keyboard and screen-reader visitors arrive visually at a section while focus and the live region provide no section context. The current test covers only clicking **How it works** within an already loaded home page, so it misses all three broken paths.

**Concrete fix:** On initial load, cross-page hash navigation, and `popstate`, resolve `location.hash`, focus that section's heading with `tabindex="-1"`, and announce it. Fall back to the page H1 only when no valid hash exists. Add tests for direct `/#setup`, `/demo/` → **Start for real**, and home section → Privacy → Back.

### F-2-4 — BLOCKING — the landing page falsely says the backup command restores only selected paths

**Quote/location:** Landing, “Your existing backup command restores only the paths you choose.” The displayed configuration runs `restic restore latest --target {target}` with no `--include`, and the CLI executes the configured argument array without inspecting or restricting its restore scope.

**Why this misleads:** Restore Drill verifies configured sample files, but it cannot guarantee that the operator's backup command restores only those files. The sentence transfers a user configuration responsibility into a product guarantee.

**Concrete fix:** Rewrite it as: “Configure your backup command to restore only the files you want to check.” Add a validation or documented warning when a command has no tool-specific selection argument; do not claim enforcement unless an observable test proves it.

### F-2-5 — MINOR — the third required first-screen fact is below the phone fold

**Quote/location:** Landing facts: “Free and MIT licensed.”, “No usage tracking.”, and “Works offline after the first visit.” At 390×844, the first two end at y=839; the offline fact starts below the captured viewport.

**Why this matters:** The mandatory first-screen shape calls for all three facts, but a phone visitor must scroll to discover the offline fact.

**Concrete fix:** Reduce the mobile headline or vertical gaps so all three facts fit within 390×844, and add a viewport assertion for all three.

### F-2-6 — MINOR — temporary-location terminology has regressed

**Quote/location:** Landing uses “temporary folder” and “temporary restore folder.” README/demo copy switches to “disposable workspace” and “temporary workspace.”

**Why this matters:** The earlier audit established **temporary restore folder** as the single visitor-facing term. Three terms make one safety boundary sound like different mechanisms.

**Concrete fix:** Use “temporary restore folder” everywhere. Rewrite “The command creates and removes a disposable workspace” as “The command creates and removes a temporary restore folder.”

### F-2-7 — MINOR — the 404 route omits canonical and social metadata

**Quote/location:** `/does-not-exist-review-2` correctly returns the designed 404, but its document has no canonical link, Open Graph fields, or Twitter card fields.

**Why this matters:** The standard route metadata is complete on the four public routes but not on the designed error route.

**Concrete fix:** Add the product's canonical home URL, `og:title`, `og:description`, `og:url`, `og:image`, and matching Twitter fields to `site/404.html`; keep `robots=noindex`.

## Unlisted claim findings

Each row is a separate MINOR finding and is also evidence for blocking F-2-2. Existing entries limited to the bundled demo do not cover general CLI behavior.

| ID | Exact quote/location | Why it is unproved | Concrete fix |
| --- | --- | --- | --- |
| F-2-8 — MINOR | Landing: “Verify one file restores from your backup”; “Check one selected backup file”; footer: “Check selected files from your existing backup.” README: “It checks that selected files still restore.” | `restore-verification` covers only browser demo text. | Add `selected-file-verification` using the real CLI with pass, corrupt, and missing files, or narrow the copy to the bundled demo. |
| F-2-9 — MINOR | Landing: “Restore Drill runs the restore command you configure.” | No claim entry observes the configured process or its arguments. | Add a tagged integration test with an argument-recording restore stub. |
| F-2-10 — MINOR | Landing: “It cleans the folder before it writes a receipt”; “Restore Drill creates a new folder for each check.” README: “It removes the folder and writes a hash-linked receipt.” | `demo-isolation` is explicitly about the bundled demo, not every `run` result or ordering. | Add a `temporary-folder-lifecycle` claim covering pass, restore failure, missing file, hash mismatch, application-check failure, timeout, and cleanup-before-receipt order. |
| F-2-11 — MINOR | Landing: “You can run an application check too.” README: “It can run an application check.” | No listed claim executes an application check or asserts its failure changes the result. | Add `application-check` with passing and failing programs. Rewrite as “It can check whether an application opens the restored file.” |
| F-2-12 — MINOR | Landing: “No usage tracking.” README: “The website has no forms, analytics, cookies, tracking pixels, or third-party scripts.” | `local-network` only rejects cross-origin requests; same-origin analytics, cookies, or forms would pass. | Expand the claim and test to assert no cookies/storage, no form elements, no tracking endpoints, and no unexpected requests. |
| F-2-13 — MINOR | README demo: “It does not read your configuration, backups, or receipt directory.” | The tagged test checks only that `.restore-drill` is absent in the repository after the command. | Run in a temp directory containing trap config, backup, and receipt files; assert access/write logs and hashes remain unchanged. |
| F-2-14 — MINOR | README: “Build from source with Rust 1.85 or later.” | No claim entry checks the declared minimum supported Rust version. | Add an MSRV build job tagged `@claim:rust-1-85`, or remove the exact floor. |
| F-2-15 — MINOR | Landing: “Runs on your computer.” README: “The built program has no product network client or runtime package dependency.” | No claim entry inspects the packaged binary or captures CLI network activity. | Add `standalone-local-cli`: install the packaged crate in a clean prefix, run offline, inspect dependencies, and capture attempted connections. Rewrite “product network client” in plain words. |
| F-2-16 — MINOR | README: “`run`, `status`, and `receipts` support `--json` for scheduler and alert-tool input.” | No listed claim runs all three JSON modes or validates their schema. | Add `json-output` with parseable output and stable fields for each command. |
| F-2-17 — MINOR | README: “Commands receive direct argument arrays.”; “Restore commands contain `{target}` once.”; “Application checks contain `{file}` once.” | `path-safety` checks unsafe paths and a missing target placeholder, not shell avoidance, repeated placeholders, or application placeholders. | Add `argument-substitution` covering literal shell metacharacters, missing/repeated placeholders, and exact argv received. Rewrite “direct argument arrays” as “Commands run their arguments directly, without a shell.” |
| F-2-18 — MINOR | README: “It does not copy backup-command output into receipts.” | No claim entry injects sensitive stdout/stderr and inspects the receipt. | Add `receipt-redaction` with a unique secret in stdout, stderr, arguments, and restored content. |
| F-2-19 — MINOR | README exit table: codes 0 through 4 and their meanings. | No claim entry observes every documented code and boundary. | Add `exit-codes` covering pass/current, check failure, configuration error, overdue, and never-run states. |
| F-2-20 — MINOR | README: “Receipts are read-only JSON files linked by SHA-256.” | `receipt-integrity` covers later tamper detection, not initial mode or JSON structure. | Expand that claim to assert file mode, parseable JSON, previous-hash linkage, and tamper detection. |
| F-2-21 — MINOR | README: “`npm run build` creates `dist/bin/restore-drill` and `dist/site/`.”; deployment response-policy sentence. | The response-policy test is outside the claim registry, and no claim entry covers both build artifacts. | Add `build-artifacts` and `response-policy` claim entries using the existing build and policy checks. |

### F-2-22 — MINOR — “real restore path” is implementation jargon

**Quote/location:** README, “Run the real restore path with shipped sample data.”

**Why this matters:** A first-time user cannot tell whether “path” means a filesystem path or a code path.

**Concrete fix:** Rewrite it as: “Run the shipped sample through the same restore checks.”

### F-2-23 — MINOR — “resolved files” is unexplained security jargon

**Quote/location:** README, “Restore Drill rejects links and resolved files outside its temporary folder.”

**Why this matters:** “Resolved files” assumes knowledge of link resolution and does not name what the user should avoid.

**Concrete fix:** Rewrite it as: “Restore Drill rejects links and any file that leads outside its temporary restore folder.”

### F-2-24 — MINOR — “response policy” does not name the deployment result

**Quote/location:** README, “Deploy `dist/site/` as a static site with the included `staticwebapp.config.json` response policy.”

**Why this matters:** The phrase names an implementation category, not the security headers or cache rules it provides.

**Concrete fix:** Rewrite it as: “Deploy `dist/site/` with the included security-header and cache settings in `staticwebapp.config.json`.”

## Copy audit

Method: word counts are whitespace-delimited after removing Markdown punctuation; hyphenated terms count as one word. Raw TOML, shell command blocks, table headers, numeric decoration, and CSS-only marks are excluded. Headings, buttons, navigation labels, alt text, metadata, and runtime messages are included. No sentence exceeds 22 words and no banned marketing adjective appears.

### Landing page inventory

| Location | Exact copy | Words | Flag |
| --- | --- | ---: | --- |
| Title | Restore Drill — Check a backup file restores | 7 | — |
| Meta description | Check that an important backup file restores before you need it. | 11 | — |
| Skip link | Skip to content | 3 | — |
| Offline status | Offline. | 1 | — |
| Offline status | This guide and sample demo remain available after the first visit. | 11 | — |
| Brand | Restore Drill | 2 | — |
| Navigation | How it works | 3 | — |
| Navigation | Setup | 1 | — |
| Navigation | Demo | 1 | — |
| Navigation | Privacy | 1 | — |
| Eyebrow | Runs on your computer | 4 | F-2-15 |
| Eyebrow/fact | No usage tracking | 3 | F-2-12 |
| H1 | Verify one file restores from your backup | 7 | F-2-8 |
| Lede | For people with scripted backups who need repeatable proof that important files still restore. | 13 | F-2-8 |
| Primary action | Try it with sample data | 5 | — |
| Action note | Runs a bundled restore and shows its receipt. | 8 | mapped: demo-isolation |
| Fact | Free and MIT licensed. | 4 | mapped: license |
| Fact | No usage tracking. | 3 | F-2-5, F-2-12 |
| Fact | Works offline after the first visit. | 7 | F-2-5; mapped: offline-reload |
| Image alt | A file moves from an archive box to a temporary tray, is checked, then becomes a receipt. | 17 | — |
| Figcaption label | Plate 01 | 2 | — |
| Figcaption | Restore a sample. | 3 | — |
| Figcaption | Compare its fingerprint. | 3 | — |
| Figcaption | Save the receipt. | 3 | — |
| Safeguard | Keep your backup tool. | 4 | — |
| Safeguard | Restore Drill runs the restore command you configure. | 8 | F-2-9 |
| Safeguard | Remove the temporary folder. | 4 | F-2-6, F-2-10 |
| Safeguard | It cleans the folder before it writes a receipt. | 9 | F-2-10 |
| Kicker | How it works | 3 | — |
| H2 | Check one selected backup file | 5 | F-2-8 |
| Method | Use a small check on a schedule. | 7 | — |
| Method | Keep full recovery practice for a separate exercise. | 8 | — |
| H3 | Create a temporary restore folder | 5 | — |
| Step | Restore Drill creates a new folder for each check. | 9 | F-2-10 |
| H3 | Restore selected files | 3 | F-2-8 |
| Step | Your existing backup command restores only the paths you choose. | 10 | F-2-4 |
| H3 | Compare the restored file | 4 | — |
| Step | Compare its SHA-256 fingerprint. | 4 | F-2-8 |
| Step | You can run an application check too. | 7 | F-2-11; jargon |
| H3 | Write a receipt | 3 | — |
| Step | The receipt is hash-linked JSON. | 5 | mapped: receipt-integrity |
| Step | A later change is detected when the chain is checked. | 10 | mapped: receipt-integrity |
| Kicker | Setup | 1 | — |
| H2 | Start with one important file | 5 | — |
| Setup | Choose a file you would notice missing. | 8 | — |
| Setup | Save its SHA-256 fingerprint from a trusted copy. | 8 | — |
| Safety | Rejects links and paths outside the temporary folder. | 8 | mapped: path-safety |
| Region label | Configuration example | 2 | — |
| Button | Copy configuration | 2 | — |
| Code note | Next: restore-drill check && restore-drill run | 5 | — |
| Kicker | Run a sample first | 4 | — |
| H2 | See the bundled restore now | 5 | — |
| Demo note | The demo uses shipped sample data and deletes its temporary workspace. | 10 | F-2-6; mapped: demo-isolation |
| Action | Try it with sample data | 5 | — |
| Footer | Check selected files from your existing backup. | 7 | F-2-8 |
| Footer link | View source on GitHub (external) | 5 | — |
| Footer | Built by Param Factory · build polish-1 | 6 | — |
| Runtime button | Copied configuration | 2 | — |
| Runtime button | Select configuration | 2 | — |
| Runtime status | Configuration copied to clipboard. | 4 | — |
| Runtime error | Clipboard was unavailable. | 3 | — |
| Runtime recovery | The configuration is selected for copying. | 7 | — |

### README inventory

| Location | Exact copy | Words | Flag |
| --- | --- | ---: | --- |
| H1 | Restore Drill | 2 | — |
| Opening | Restore Drill is a command-line tool for people who already run scripted backups. | 13 | — |
| Opening | It checks that selected files still restore. | 7 | F-2-8 |
| Opening | It restores into a new temporary folder and compares SHA-256 fingerprints. | 11 | F-2-6, F-2-8, F-2-10 |
| Opening | It can run an application check. | 6 | F-2-11; jargon |
| Opening | It removes the folder and writes a hash-linked receipt. | 9 | F-2-10 |
| H2 | Try the bundled demo | 4 | — |
| Demo | Run the real restore path with shipped sample data: | 9 | F-2-22 |
| Demo | The command creates and removes a disposable workspace. | 8 | F-2-6; mapped: demo-isolation |
| Demo | It does not read your configuration, backups, or receipt directory. | 10 | F-2-13 |
| Demo | The browser version is at the sample demo. | 8 | — |
| H2 | Install | 1 | — |
| Install | Build from source with Rust 1.85 or later: | 8 | F-2-14 |
| Install | The built program has no product network client or runtime package dependency. | 12 | F-2-15; jargon |
| H2 | Use your backup command | 4 | — |
| Use | run, status, and receipts support --json for scheduler and alert-tool input. | 11 | F-2-16 |
| Use | Commands receive direct argument arrays. | 5 | F-2-17; jargon |
| Use | Restore commands contain {target} once. | 5 | F-2-17 |
| Use | Application checks contain {file} once. | 5 | F-2-17 |
| Safety | Sample paths must be relative. | 5 | mapped: path-safety |
| Safety | Restore Drill rejects links and resolved files outside its temporary folder. | 11 | F-2-23; mapped: path-safety |
| Privacy | It does not copy backup-command output into receipts. | 8 | F-2-18 |
| Exit 0 | The drill passed or status is current. | 8 | F-2-19 |
| Exit 1 | A restore or check failed. | 5 | F-2-19 |
| Exit 2 | Configuration or an operational step failed. | 6 | F-2-19 |
| Exit 3 | The last successful drill is overdue. | 6 | F-2-19 |
| Exit 4 | No successful receipt exists. | 5 | F-2-19 |
| Receipt | Receipts are read-only JSON files linked by SHA-256. | 8 | F-2-20 |
| Receipt | restore-drill receipts detects a later change to a receipt or broken chain. | 12 | mapped: receipt-integrity |
| Guidance | Copy receipts to storage that prevents changes when that risk matters. | 11 | — |
| H2 | Develop, test, and deploy | 4 | — |
| Build | npm run build creates dist/bin/restore-drill and dist/site/. | 7 | F-2-21 |
| Deploy | Deploy dist/site/ as a static site with the included staticwebapp.config.json response policy. | 12 | F-2-21, F-2-24 |
| Release | Run each command in .factory/claims.json from a clean checkout before release. | 11 | — |
| H2 | Privacy and license | 3 | — |
| Privacy | The website has no forms, analytics, cookies, tracking pixels, or third-party scripts. | 12 | F-2-12 |
| Links | See Privacy and Terms. | 4 | — |
| License | Restore Drill is free and MIT licensed. | 7 | mapped: license |

## Demo, sandbox, privacy, and offline evidence

- One landing click reaches `/demo/` with a completed sample state and the required banner, **Reset demo**, and **Start for real** controls.
- Reset starts a fresh 220 ms pass run and returns to the completed state. It does not write browser storage.
- Before the demo, after landing, after reset, and after running it, `localStorage`, `sessionStorage`, and cookies remained empty.
- Live request capture across landing, demo, reset, and Start for real contained only `https://backup-restore-drill.sociobot.in` requests. No cross-origin request occurred.
- After service-worker installation and an online reload, the live `/demo/` reloaded offline and displayed both the offline notice and demo banner.
- `cargo run --quiet --manifest-path … -- demo --json` was run from `/tmp/restore-drill-cli-review-2.E62B39`. A pre-existing `.restore-drill/receipts/do-not-touch.txt` retained SHA-256 `d3d5da…b6ce`; no other file appeared in that directory and no `restore-drill-demo-*` folder remained.

These checks confirm isolation. They do not cure F-2-1's first-screen presentation failure or the missing automated coverage in F-2-2.

## Claim test results

All commands were run exactly as listed after `npm ci` in clean clone `/tmp/restore-drill-review-2.QEbqAl/repo`.

| Claim | Command result | Evidence |
| --- | --- | --- |
| demo-isolation | PASS | 2 Playwright projects passed; CLI returned `status=pass`, `receipt_count=1`, `workspace=removed`. |
| restore-verification | PASS but insufficient | 2 projects passed by reading browser strings; see F-2-2. |
| local-network | PASS | 2 projects captured same-origin requests only. |
| offline-reload | PASS | 2 projects reloaded home/demo offline. |
| license | PASS | 2 projects found the MIT license and Cargo metadata. |
| receipt-integrity | PASS | 2 projects invoked the tamper test successfully. |
| path-safety | PASS | 2 projects invoked unsafe-path and placeholder validation successfully. |

The repository-wide `npm test` also passed 5 Rust tests and 20 Playwright cases. `npm run build` passed and produced `dist/site/` plus `dist/bin/restore-drill`. A green suite does not change the verdict because claim coverage and the mobile demo contract are incomplete.

## Earlier finding verification

| Earlier finding | Status now | Fresh evidence |
| --- | --- | --- |
| Review 1 BLOCKING 1 — first-screen clarity | FIXED | Cold mobile and desktop reads answer job, audience, and action. |
| Review 1 BLOCKING 2 — one-click/CLI demo | **HALF-FIXED; BLOCKING again as F-2-1** | CLI command, route, banner, reset, and isolation exist; real output is not on the phone's first screen and no terminal recording exists. |
| Review 1 BLOCKING 3 — claims registry/tests | **HALF-FIXED; BLOCKING again as F-2-2** | Seven entries exist and commands pass, but a key test checks static text and published claims remain unlisted. |
| Review 1 BLOCKING 4 — “immutable JSON” | FIXED | Live and source use “hash-linked JSON”; `immutable` remains only in cache directives. |
| Review 1 BLOCKING 5 — demo/404 routing | FIXED | `/demo/` returns 200 with its own title; unknown path returns the designed page with HTTP 404. |
| Review 1 MAJOR 6 — public-route metadata | FIXED for public routes | Home, Demo, Privacy, and Terms have route titles, canonical, OG/Twitter fields, favicon, and apple icon. F-2-7 is limited to 404 metadata. |
| Review 1 MAJOR 7 — route focus/history | **HALF-FIXED; BLOCKING again as F-2-3** | Same-page click focuses correctly; direct hashes, cross-page hashes, and Back do not. |
| Review 1 MINOR 8 — external-link naming | FIXED | “View source on GitHub (external)” is explicit. |
| Verification response-policy/cache findings | FIXED | Live headers include CSP, Permissions-Policy, no-referrer, nosniff, HTML revalidation, immutable hashed assets, and no-cache service worker. |
| Prior copy flags | PARTLY REGRESSED | Earlier overlong/marketing copy is gone; temporary-location terminology and technical phrases remain as documented above. |

### Review 1 copy-finding ledger

| Earlier flagged wording or rule | Status on live/source copy |
| --- | --- |
| “Local · Tool-agnostic · Zero telemetry” | FIXED; replaced with plain facts. |
| “Method” / “The method / 01–04” | FIXED; now “How it works.” |
| “Configure” | FIXED; now “Setup.” |
| “Backup exists. Recovery proven.” | FIXED; job-first H1 now used. |
| “quiet, tamper-evident receipt” sentence | FIXED; removed. |
| “Configure a drill” | FIXED; removed as the primary action. |
| “See a safe simulation” / “Try the flow” | FIXED; now “Try it with sample data.” |
| “Nothing to migrate” / named-tool sentence | FIXED; removed. |
| “Nothing left behind” | FIXED; concrete cleanup copy now used. |
| “Small enough to run. Strict enough to trust.” | FIXED; removed. |
| “cadence” in visitor copy | FIXED outside the required config field name. |
| “samples the recovery path” | FIXED; removed. |
| “Create a clean target” | FIXED; now “Create a temporary restore folder.” |
| “Check SHA-256…” | FIXED; fingerprint copy is split into short sentences. |
| “One file. One known hash. One command.” | FIXED; removed. |
| “Verify real files” | FIXED; now “Compare the restored file.” |
| “Explicit argument arrays” | **UNFIXED in README** as “direct argument arrays”; see F-2-17. |
| “Symlinks and path escapes refused” | PARTLY FIXED; landing is plain, README says “resolved files”; see F-2-23. |
| “Subprocess output excluded from receipts” | FIXED; now “backup-command output.” |
| “Recorded local flow” | PARTLY FIXED; renamed, but the required real recording is still absent; see F-2-1. |
| “Hear the alarm before it matters” | FIXED; removed. |
| “fixture” | FIXED; now “sample data.” |
| “immutable JSON” | FIXED; now “hash-linked JSON.” |
| “Alert on two things only” / quiet-exit copy | FIXED; removed from landing. |
| “Operator notes” / timer heading | FIXED; removed. |
| “Run healthy sample” | FIXED; now “Run sample restore again.” |
| “Failure caught loudly” | FIXED; now “Missing file detected.” |
| “Cleanup failure makes the drill fail loudly” | FIXED; removed from landing. |
| “The next useful backup task” / “Prove one file today” | FIXED; removed. |
| “let the receipt—not your memory” | FIXED; removed. |
| “Local proof for the backups…” | FIXED; replaced with a concrete footer sentence. |
| “Check the bytes” | FIXED; now “Compare its fingerprint.” |
| Overlong README opening sentences | FIXED; all current sentences are at most 13 words. |
| Overlong response-policy sentence | FIXED for length; remaining jargon is F-2-24. |
| “Everything runs locally” | FIXED for precision; replacement jargon/claim coverage is F-2-15. |

## Structure, links, accessibility, and identity

| Check | Result | Evidence |
| --- | --- | --- |
| Titles, language, H1, main | PASS on public routes and 404 | Each tested route has `lang=en`, one H1, and one main. Titles follow the product pattern. |
| Canonical/OG/Twitter/favicon | PASS public routes; FAIL 404 | See F-2-7. Share art is a real 1200×630 image; apple icon is 180×180. |
| Designed 404 | PASS | Unknown URL returns HTTP 404 with a styled route and **Return to Restore Drill**. |
| Deep links/back/focus | **FAIL — BLOCKING** | See F-2-3. |
| Link crawl | PASS | All internal destinations and the GitHub source link returned 200; the unknown route's self skip-link retains the expected 404 page. |
| Header/footer | PASS | Link sets, factory credit, and build ID are consistent. |
| Heading outline | PASS | No skipped heading levels on tested pages. |
| Automated accessibility | PASS | Axe found zero serious/critical violations at 390×844 and 1366×900. No horizontal overflow occurred. |
| Console | PASS on normal routes | No console/page errors on Home, Demo, Privacy, or Terms. The browser reports the expected 404 document response on the unknown route. |
| Visual identity | PASS | The two-ink paper, registration marks, offset controls, receipt shapes, and risograph art are recognizably specific to restore evidence, not a generic SaaS template. |

## Missed leverage

No AI feature is warranted. Backup verification must be deterministic, and an AI summary would add network/privacy risk without improving proof. JSON receipts already provide exportable evidence; remote sync is not implied by the brief and should remain an operator-controlled choice.

## What would make this perfect

1. Show the completed real sample result above the phone fold and include a recording captured from the CLI demo.
2. Replace the static-text SHA claim test and register every published behavioral/privacy/build claim.
3. Correct the restore-scope sentence so it describes operator configuration, not product enforcement.
4. Focus and announce hash targets on direct load, cross-page navigation, and Back.
5. Fit all three plain facts on the first phone screen, normalize temporary-folder wording, and complete 404 metadata.

Until every item is closed and retested from a fresh context and clean clone, the correct verdict is **FAIL**.
