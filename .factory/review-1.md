# Adversarial first-read review 1 — Restore Drill

**Verdict: FAIL**

**Candidate reviewed:** `dfa88e162b750a80ebc1fb7697082118375532dc`

**Live URL:** <https://backup-restore-drill.sociobot.in/>

**Reviewed:** 2026-08-28 UTC

**Blocking findings:** 5

The site is visually distinct and its ordinary test suite passes. It still fails the first-read contract: the first screen does not identify the intended operator or one unambiguous first action, the CLI has no runnable sample-data demo, the required claim registry and tagged tests do not exist, “immutable JSON” overstates the receipt guarantee, and unknown routes silently return the home page.

## Cold read, before scrolling

Fresh Chromium contexts opened the live URL at 390×844 and 1366×900. Both started at `scrollY = 0`; neither produced a console or page error.

My first-screen answers were:

- **What it does:** I can infer that it checks whether some backup recovery works and creates a receipt. The screen does not say that this is a command-line tool that restores selected files into a temporary directory and verifies them.
- **For whom:** Not answerable. “The files you cannot afford to discover are missing” describes the files, not the person or situation. The README later supplies the missing audience: people who already have scripted backups.
- **What to click first:** Not answerable. “Configure a drill” is visually primary, while “See a safe simulation” is beside it. The former only scrolls to a configuration example; the latter only scrolls to an idle simulation.

Exact first-screen text that failed:

> “Backup exists. Recovery proven.”
>
> “A five-minute restore drill for the files you cannot afford to discover are missing. Use the backup tool you already trust; get one quiet, tamper-evident receipt.”
>
> “Configure a drill” / “See a safe simulation”

### BLOCKING 1 — the first screen does not answer all three cold-read questions

**Why this loses a first-time visitor:** The headline is an assertion, not the job. It omits “file,” “restore,” and the intended operator. Two competing actions make the required first click ambiguous, and the visually primary action does not configure anything.

**Concrete fix:** Replace the first-screen copy with:

> **Verify one file restores from your backup**
>
> For people with scripted backups who need repeatable proof that important files still restore.
>
> **Try it with sample data**
>
> Runs a bundled restore in a temporary folder and shows the receipt.
>
> Free and MIT licensed. · Runs on your machine. · No account or usage tracking.

Make the sample action the only primary action. Rename the documentation jump to “Read the setup steps.”

## Findings, ordered by severity

### BLOCKING 2 — there is no one-click product demo or CLI demo command

**Quote:** “See a safe simulation” leads to “Ready for a drill” and “Waiting to run a simulated sample…”

**Evidence:**

- The first click only changes the URL to `/#demo` and scrolls to an idle browser mock. A second click is required before any result appears.
- The browser code animates fixed strings. It does not run the CLI or bundled sample input, despite the heading “Recorded local flow.”
- `restore-drill demo` and `restore-drill --demo`, each run from `/tmp/restore-drill-review.RZskpB`, both exit 2 as unrecognized input.
- `/demo` and `/?demo=1` both return the ordinary landing page and ordinary title.
- There is no `examples/` sample, `.factory/demo.md`, persistent “Demo — sample data, nothing is saved” banner, **Reset demo**, or **Start for real** action.

**Why this loses or misleads a visitor:** A visitor cannot try the CLI without installing Rust, understanding TOML, supplying a backup tool, and constructing a trusted hash. The mock demonstrates copy, not the shipped binary. Calling it a recorded local flow implies stronger provenance than the implementation has.

**Concrete fix:** Ship realistic input under `examples/` and implement `restore-drill demo`. It must copy bundled data into a newly created temporary directory, run the real restore/check/receipt path, clean the restore target, and print where disposable output went. Add a self-hosted recording made from that command. Make `/demo` start with completed realistic sample output on the first click, add the required banner/Reset/Start-for-real controls, document isolation in `.factory/demo.md`, and test that the command creates nothing outside its temporary workspace.

### BLOCKING 3 — claims cannot be verified because `.factory/claims.json` and all `@claim:*` tests are missing

**Quote:** The repository contains no `.factory/claims.json`; `rg '@claim:'` returns no matches.

**Evidence:** `npm test` passed 3 Rust unit tests, 2 CLI integration tests, the response-policy check, and 10 browser cases. Those are useful regressions, but none is tagged to a published claim and there is no list of claim commands to run. Therefore the required “run every listed test” operation has no valid input.

**Why this misleads a visitor:** The live page and README make dozens of behavioral, privacy, compatibility, timing, cleanup, and security statements. A green general-purpose suite does not identify which promise is protected or expose an untested promise.

**Concrete fix:** Add `.factory/claims.json` and one uniquely tagged observable test per claim. At minimum, add:

| Required test | Sentences it must prove |
| --- | --- |
| `@claim:demo-isolation` | The demo never accesses user files, writes only inside its generated temp directory, cleans it, and leaves configured receipt storage untouched. |
| `@claim:local-network` | The CLI and complete browser demo make no unexpected network request; the site makes only same-origin requests. |
| `@claim:tool-compatibility` | Explicit argument-array stubs for the named backup-tool patterns receive the correct target and selected path. |
| `@claim:five-minute` | A clean user can finish the stated sample flow in at most five minutes, or remove “five-minute.” |
| `@claim:restore-verification` | Presence, SHA-256, and optional application checks affect the real command result and receipt. |
| `@claim:temp-cleanup` | The generated target is removed after pass, restore failure, missing file, hash mismatch, open-check failure, and timeout. |
| `@claim:receipt-chain` | Every result writes a read-only, hash-linked receipt and later tampering is detected. |
| `@claim:log-redaction` | File bytes, passphrases, restore stdout/stderr, repository locations, and sensitive arguments do not enter output or receipts. |
| `@claim:path-safety` | Absolute paths, `..`, symlinks, target escapes, and missing or repeated `{target}` are rejected. |
| `@claim:exit-codes` | Every documented exit code and alert condition is observed from the packaged binary. |
| `@claim:failure-classes` | Missing, corrupt, stale, and application-invalid samples produce the stated failures and remediation. |
| `@claim:cadence` | The 30-day due date and overdue boundary are calculated from the last successful receipt. |
| `@claim:offline-reload` | After one visit, home/privacy/terms and the sample demo reload offline with network disabled. |
| `@claim:no-runtime-service` | A packaged install runs with no account, cloud API, telemetry, or product-managed credential store. |
| `@claim:install-requirements` | A source build enforces the documented Rust floor, and the packaged program runs without Rust or another application package installed. |
| `@claim:license` | The repository and packaged artifact contain the stated MIT license and no paid gate. |
| `@claim:build-artifacts` | `npm run build` creates both documented output paths from a clean checkout. |
| `@claim:response-policy` | A deployed response check observes the exact stated security and cache headers, not only the source configuration. |

Every behavioral sentence marked `U` in the copy audit below is an individual unlisted-claim finding until it is removed or mapped to one of these tests.

### BLOCKING 4 — “immutable JSON” is a false description of a local receipt

**Quote:** “Restore evidence / immutable JSON”

**Why this misleads a visitor:** The implementation writes a read-only local file in a user-controlled directory. The same user can replace or chmod it. A hash chain can reveal later modification; it cannot make local JSON immutable. The README correctly uses the narrower term “tamper-evident.”

**Concrete fix:** Rewrite the label as “Restore evidence / hash-linked JSON.” Keep “tamper-evident” only where a tagged test changes a receipt and confirms chain verification fails. Reserve “immutable” for a separately verified append-only storage system.

### BLOCKING 5 — catch-all routing hides missing pages and breaks the required demo/404 routes

**Quote:** `GET /does-not-exist-review-1` and `GET /demo` both return HTTP 200 with the home-page title and H1.

**Why this loses a visitor:** A mistyped or stale link looks valid, and the advertised demo deep link cannot identify or restore demo state. There is no designed 404 or path back from an error because the application never admits the route is missing.

**Concrete fix:** Add a real `/demo` route titled “Demo — Restore Drill.” Add a styled 404 route with a concise explanation and “Return to Restore Drill.” Configure the host to serve it with 404 status for unknown routes. Add direct-load and reload tests for `/demo`, `/privacy/`, `/terms/`, and an unknown path.

### MAJOR 6 — route metadata is incomplete

**Quote:** Home, Privacy, and Terms have descriptions and route-appropriate titles, but none has a canonical URL, Open Graph fields, Twitter card fields, or apple-touch icon. Only `/favicon.svg` exists. The sitemap lists only `/`, `/privacy/`, and `/terms/`.

**Why this matters:** Shared links have no product-authored preview or canonical identity. The missing demo route is also absent from the route inventory; unknown-route behavior is not covered separately.

**Concrete fix:** Add per-route canonical URLs, `og:title`, `og:description`, `og:url`, a product-specific 1200×630 `og:image`, Twitter card fields, and a 180px apple-touch icon. Add real public routes to the sitemap and verify each route title exactly.

### MAJOR 7 — navigation does not restore or announce route context

**Quote:** After activating “Method,” `document.activeElement` is `<body>`, not the target heading. After opening Privacy and using Back, focus is still `<body>` and the page restores near the footer rather than the previously selected method section.

The header and footer also change by route. The mobile home header exposes only Source; legal headers use Product/Terms or Product/Privacy. Footers omit “Built by Param Factory” and a version/build ID, and legal footers expose different link sets.

**Why this loses a keyboard or screen-reader visitor:** Scrolling changes visually while focus and announced context do not. A visitor cannot rely on one navigation skeleton across pages.

**Concrete fix:** Use consistent header/footer link sets on all routes. On route/hash changes, focus the destination heading with `tabindex="-1"` and announce its text in a dedicated polite live region. Preserve the originating scroll position on Back. Add keyboard tests for home → section → Privacy → Back and direct `/demo` navigation.

### MINOR 8 — external links are not identified as external

**Quote:** “Source” and “Get Restore Drill on GitHub” give no “opens GitHub” cue.

**Why this matters:** The destination changes from the product site to GitHub without being stated.

**Concrete fix:** Use “View source on GitHub” and “Get Restore Drill on GitHub (external).” All crawled destinations currently return 200; this is a labeling issue, not a dead-link issue.

## Demo, privacy, and offline observations

- The browser simulation itself did not write `localStorage`, `sessionStorage`, or cookies before or after a healthy run.
- Network interception observed only same-origin HTML, JS, CSS, image, service-worker, Privacy, and Terms requests. No cross-origin request occurred.
- After service-worker registration, a forced offline reload returned the cached home page and displayed: “Offline copy. The guide still works; downloads and external links may not. Reconnect, then reload to update.”
- Reload reset the browser mock to “Ready for a drill.” That avoids persistence, but it is not evidence about the missing CLI demo.

These checks confirm the current browser mock is locally contained. They do not satisfy the demo or claim contracts because the real CLI path is absent and the promises are unregistered.

## Structure and accessibility results

| Check | Result | Evidence |
| --- | --- | --- |
| Home at 390px and desktop | PASS | No horizontal overflow or console/page error; first screen captured cold. |
| `<title>`, `lang`, one H1, one main | PASS on `/`, `/privacy/`, `/terms/` | `verify-url.sh` passed; titles are 45, 23, and 21 characters respectively. |
| Axe automated scan | PASS | Zero violations on all three routes at 390px and 1366px. |
| Image alt | PASS | The one meaningful hero image has a purpose-specific alt sentence. |
| Reduced motion | PASS | The demo transition resolves immediately under `prefers-reduced-motion: reduce`. |
| Link crawl | PASS | Home, Privacy, Terms, favicon, robots, sitemap, GitHub repository, and GitHub LICENSE all returned 200. |
| Visual identity | PASS | The two-ink risograph evidence-desk art, type, offset controls, receipt shapes, and paper palette are product-specific rather than a generic SaaS template. |
| Titles by route | FAIL | `/demo` and unknown paths reuse the home title because they reuse the home page. |
| Canonical/OG/Twitter/apple icon | FAIL | Absent on every checked route. |
| Designed 404 | FAIL — BLOCKING | Unknown paths return the home page with HTTP 200. |
| Route-change focus/announcement | FAIL | Focus remains on `<body>`; no route-heading announcement exists. |
| Consistent header/footer | FAIL | Link sets vary; factory credit and build ID are absent. |

## Copy audit method

Counts use whitespace-delimited words; a hyphenated compound counts as one. Raw commands, TOML, URLs, table numbers, data-field labels, and decorative marks are not sentences and are excluded. Headings, buttons, alt text, status/error text, and meaningful fragments are included because the plain-words contract explicitly covers them.

Flags: `>22` exceeds the hard cap; `J` jargon; `V` vague/marketing wording; `H` heading unclear out of context; `A` action does not name its result; `I` inconsistent or inaccurate term; `U` unlisted claim.

Metadata copy is also visitor-facing in search and link contexts:

| Location | Exact copy | Words | Flags |
| --- | --- | ---: | --- |
| `<title>` | Restore Drill — proof your backup can restore | 7 | V, U |
| Meta description | Restore Drill is a free local CLI that proves selected files can restore from the backups you already run. | 19 | J, U |

## Landing-page sentence inventory

| # | Exact copy | Words | Flags |
| ---: | --- | ---: | --- |
| 1 | Skip to content | 3 | — |
| 2 | Offline copy. | 2 | U |
| 3 | The guide still works; downloads and external links may not. | 10 | U |
| 4 | Reconnect, then reload to update. | 5 | — |
| 5 | Restore Drill | 2 | — |
| 6 | Method | 1 | H |
| 7 | Configure | 1 | A |
| 8 | Try the flow | 3 | A, V |
| 9 | Source | 1 | — |
| 10 | Local | 1 | U, V |
| 11 | Tool-agnostic | 1 | J, U |
| 12 | Zero telemetry | 2 | J, U |
| 13 | Backup exists. | 2 | H |
| 14 | Recovery proven. | 2 | H, U |
| 15 | A five-minute restore drill for the files you cannot afford to discover are missing. | 14 | U |
| 16 | Use the backup tool you already trust; get one quiet, tamper-evident receipt. | 12 | J, V, U |
| 17 | Configure a drill | 3 | A |
| 18 | See a safe simulation | 4 | A, V |
| 19 | Free · MIT licensed · No account · Runs on your machine | 9 | U |
| 20 | Risograph collage: one file moves from an archive box into a temporary tray, is checked, and becomes a receipt. | 19 | — |
| 21 | Plate 01 | 2 | — |
| 22 | Restore a sample. | 3 | — |
| 23 | Check the bytes. | 3 | J |
| 24 | Keep the evidence. | 3 | — |
| 25 | Nothing to migrate. | 3 | V, U |
| 26 | Works around restic, Borg, rsync, tar, or your own restore script. | 11 | J, U |
| 27 | Nothing left behind. | 3 | V, U |
| 28 | Every drill uses and removes a fresh temporary target. | 9 | J, U |
| 29 | The method / 01–04 | 3 | H |
| 30 | Small enough to run. | 5 | H, V |
| 31 | Strict enough to trust. | 4 | H, V |
| 32 | A full disaster simulation is easy to postpone. | 8 | — |
| 33 | Restore Drill samples the recovery path you actually need, on a cadence you can sustain. | 15 | J, U |
| 34 | Create a clean target | 4 | J |
| 35 | The CLI—not your restore command—creates a random temporary directory. | 9 | J, U |
| 36 | Restore chosen samples | 3 | — |
| 37 | Your existing tool restores only configured paths with your read-only credentials. | 11 | J, U |
| 38 | Verify real files | 3 | V |
| 39 | Check SHA-256 and optionally ask the real application to open or parse each file. | 14 | J, U |
| 40 | File a receipt | 4 | — |
| 41 | Cleanup happens first. | 3 | U |
| 42 | Then a read-only, hash-chained JSON receipt records the result. | 9 | J, U |
| 43 | Your first drill | 3 | — |
| 44 | One file. | 2 | H |
| 45 | One known hash. | 3 | H, J |
| 46 | One command. | 2 | H |
| 47 | Start with a file you would immediately miss: a tax PDF, password-vault export, database dump, or family photo. | 18 | — |
| 48 | Restore Drill never reads its contents into logs. | 8 | U |
| 49 | Relative sample paths only | 4 | J, U |
| 50 | Explicit argument arrays, no implicit shell | 6 | J, U |
| 51 | Symlinks and path escapes refused | 5 | J, U |
| 52 | Subprocess output excluded from receipts | 5 | J, U |
| 53 | Copy config | 2 | J |
| 54 | Then: `restore-drill check && restore-drill run` | 5 | J |
| 55 | Recorded local flow | 3 | I, U |
| 56 | Hear the alarm before it matters. | 6 | H, V |
| 57 | This browser simulation uses a fixed fixture and never touches your files. | 12 | J, U |
| 58 | Compare a healthy run with the exact failure a scheduler would receive. | 13 | U |
| 59 | Run healthy sample | 3 | V |
| 60 | Inject missing file | 3 | — |
| 61 | Ready for a drill | 4 | — |
| 62 | Choose a fixture. | 3 | J |
| 63 | No local files are accessed. | 6 | U |
| 64 | Waiting to run a simulated sample… | 7 | — |
| 65 | Restore evidence / immutable JSON | 4 | I, J, U |
| 66 | No receipt yet | 3 | — |
| 67 | Awaiting drill | 2 | — |
| 68 | Quiet by default | 3 | H, V |
| 69 | Alert on two things only. | 5 | H, U |
| 70 | Failed — the restore command, hash, or open-check did not pass. | 11 | J, U |
| 71 | Overdue — no successful receipt exists inside your chosen cadence. | 9 | J, U |
| 72 | Everything else exits quietly with code 0. | 7 | U |
| 73 | JSON output is built for cron, systemd timers, and your existing notifier. | 12 | J, U |
| 74 | Operator notes | 2 | H, J |
| 75 | Before you put it on a timer | 8 | H |
| 76 | Does it need my backup password? | 6 | — |
| 77 | No. | 1 | U |
| 78 | Restore Drill inherits the environment of your configured command. | 9 | J, U |
| 79 | Keep credentials in your backup tool's existing mechanism and use read-only repository access. | 13 | J |
| 80 | Can a local receipt prove everything? | 6 | — |
| 81 | No. | 1 | — |
| 82 | It proves what this binary observed on this host. | 9 | J, U |
| 83 | Copy receipts to append-only external storage if a local attacker is in your threat model. | 15 | J |
| 84 | Why not run a full restore? | 6 | — |
| 85 | You should still rehearse full recovery. | 6 | — |
| 86 | A small monthly sample catches missing, corrupt, stale, and unusable files between larger exercises. | 14 | U |
| 87 | What happens to restored data? | 5 | — |
| 88 | It stays in an OS-created temporary directory only for verification. | 10 | J, U |
| 89 | Cleanup finishes before the receipt is written. | 8 | U |
| 90 | Cleanup failure makes the drill fail loudly. | 7 | V, U |
| 91 | The next useful backup task | 5 | H |
| 92 | Prove one file today. | 4 | V |
| 93 | Then let the receipt—not your memory—tell you when it is time again. | 12 | V, U |
| 94 | Get Restore Drill on GitHub | 5 | — |
| 95 | Local proof for the backups you already have. | 8 | V, U |
| 96 | Privacy | 1 | — |
| 97 | Terms | 1 | — |
| 98 | MIT license | 2 | U |

### Landing runtime/status copy

| # | Exact copy | Words | Flags |
| ---: | --- | ---: | --- |
| 99 | Copied | 1 | — |
| 100 | Select code | 2 | — |
| 101 | Configuration copied to clipboard. | 4 | — |
| 102 | Clipboard was unavailable. | 3 | — |
| 103 | The configuration is selected for copying. | 7 | — |
| 104 | Drill running | 2 | — |
| 105 | A fresh temporary target has been created. | 7 | J, U |
| 106 | Receipt pending | 2 | — |
| 107 | Failure caught loudly | 3 | V |
| 108 | Next: confirm the backup includes this path, then rerun. | 9 | — |
| 109 | Recovery not proven | 3 | — |
| 110 | Recovery proven | 2 | U |
| 111 | Next drill is due in 30 days. | 7 | U |
| 112 | No alert needed. | 3 | U |
| 113 | All checks passed | 3 | U |
| 114 | `CREATE temporary target /tmp/restore-drill-••••••` | 4 | J, U |
| 115 | `RESTORE Documents/tax.pdf` | 2 | J, U |
| 116 | `MISSING Documents/tax.pdf` | 2 | J, U |
| 117 | `CLEAN temporary target removed` | 4 | J, U |
| 118 | `FAIL sample was not restored [exit 1]` | 7 | J, U |
| 119 | `HASH sha256 8b51…a02f matched` | 4 | J, U |
| 120 | `OPEN pdftotext accepted file` | 4 | J, U |
| 121 | `PASS receipt chain advanced [exit 0]` | 6 | J, U |

## README sentence inventory

| # | Exact copy | Words | Flags |
| ---: | --- | ---: | --- |
| 1 | Restore Drill | 2 | — |
| 2 | Restore Drill is a local, backup-tool-agnostic CLI for people who already have scripted backups but want repeatable evidence that selected files can be restored. | 24 | **>22**, J, U |
| 3 | It restores into a newly created temporary directory, checks presence and SHA-256 hashes, optionally asks the file's real application to open or validate it, removes the temporary directory, and writes a tamper-evident receipt. | 33 | **>22**, J, U |
| 4 | It does not move backups, store repository credentials, inspect file contents, or provide disaster-recovery orchestration. | 15 | J, U |
| 5 | Install | 1 | — |
| 6 | Download a release binary when releases are available, or build from source: | 12 | J |
| 7 | Rust 1.85+ is required to build. | 6 | U |
| 8 | The compiled CLI has no runtime dependencies. | 7 | J, U |
| 9 | Usage | 1 | — |
| 10 | Create a starter file and edit its explicit argument arrays for your backup tool: | 14 | J |
| 11 | Use `--json` on `run`, `status`, or `receipts` for schedulers. | 9 | J, U |
| 12 | Commands never invoke a shell. | 5 | J, U |
| 13 | `{target}` is replaced only with the temporary directory Restore Drill created. | 11 | J, U |
| 14 | `{file}` in an open-check is replaced with the restored sample's absolute path. | 12 | J, U |
| 15 | The restore command must contain `{target}` exactly once. | 8 | J, U |
| 16 | Sample paths must be relative and cannot contain `..`. | 9 | J, U |
| 17 | Restore Drill refuses symlinks and any resolved file outside its temporary target. | 12 | J, U |
| 18 | It deletes that target after every result, including failures. | 9 | U |
| 19 | Exit codes are stable: | 4 | U |
| 20 | Command completed; drill passed or status is current | 8 | U |
| 21 | A restore or verification failed | 5 | U |
| 22 | Invalid configuration or operational error | 5 | U |
| 23 | Last successful drill is overdue | 5 | U |
| 24 | No successful receipt exists yet | 5 | U |
| 25 | Receipts are individual read-only JSON files linked by SHA-256. | 9 | J, U |
| 26 | `restore-drill receipts` verifies the complete chain before displaying it. | 9 | J, U |
| 27 | Copy or sync the receipt directory to append-only storage if host-level immutability is required. | 14 | J |
| 28 | Scheduler examples | 2 | J |
| 29 | Run daily and alert only when the drill itself fails: | 10 | U |
| 30 | Or perform drills separately and alert only when overdue: | 10 | U |
| 31 | Develop and verify | 3 | — |
| 32 | `npm run build` creates release binaries in `dist/bin/` and the deployable documentation site in `dist/site/`. | 15 | J, U |
| 33 | The static site can be developed with `npm run dev`. | 10 | J, U |
| 34 | The deployable site includes `staticwebapp.config.json`, which is the Azure Static Web Apps response-policy contract: it sends the restrictive CSP, permissions and referrer policies, caches only hashed assets and the immutable hero for a year, and keeps `sw.js` revalidating. | 38 | **>22**, J, U |
| 35 | Verify that contract after a production deployment with: | 8 | J |
| 36 | Privacy and security | 3 | — |
| 37 | Everything runs locally. | 3 | V, U |
| 38 | There is no telemetry, account, cloud API, or credential storage. | 10 | J, U |
| 39 | Restore subprocess output is deliberately not copied into receipts because tools sometimes echo repository locations or sensitive arguments. | 18 | J, U |
| 40 | Prefer a read-only repository credential scoped outside this config file. | 10 | J |
| 41 | See `SECURITY.md` for the threat model. | 6 | J |
| 42 | This software is MIT licensed; see `LICENSE`. | 7 | U |

## Copy findings and proposed rewrites

Each flag above is a finding. These replacements resolve the repeated patterns without hiding technical detail:

| Flagged copy | Proposed rewrite |
| --- | --- |
| “Local · Tool-agnostic · Zero telemetry” | “Runs on your machine · Uses your current backup tool · No usage tracking” |
| “Method” / “The method / 01–04” | “How Restore Drill checks one file” |
| “Configure” | “Setup” |
| “Backup exists. Recovery proven.” | “Verify one file restores from your backup” |
| “Use the backup tool you already trust; get one quiet, tamper-evident receipt.” | “Keep your current backup tool. Restore Drill checks one file and records a receipt that reveals later changes.” |
| “Configure a drill” | “Read the setup steps” |
| “See a safe simulation” / “Try the flow” | “Try it with sample data” |
| “Nothing to migrate. Works around…” | “Keep your current backup tool. Restore Drill runs its existing restore command.” |
| “Nothing left behind.” | “Restore Drill removes its temporary restore folder after every result.” |
| “Small enough to run. Strict enough to trust.” | “Restore one selected file and verify the result” |
| “cadence” | Use “schedule” everywhere outside the TOML field name. |
| “Restore Drill samples the recovery path you actually need…” | “Restore Drill checks selected files on a schedule you can keep.” |
| “Create a clean target” | “Create a temporary restore folder” |
| “Check SHA-256…” | “Compare each restored file with its known SHA-256 fingerprint. You can also ask its application to open it.” |
| “One file. One known hash. One command.” | “Start with one important file and its known SHA-256 fingerprint” |
| “Verify real files” | “Verify the restored files” |
| “Explicit argument arrays, no implicit shell” | “Runs command arguments directly, without a shell” |
| “Symlinks and path escapes refused” | “Rejects links and paths outside the temporary folder” |
| “Subprocess output excluded from receipts” | “Does not copy backup-command output into receipts” |
| “Recorded local flow” | “Illustrated browser example” until it is replaced by a recording of the real demo command. |
| “Hear the alarm before it matters.” | “See a passing restore and a missing-file alert” |
| “fixture” | Use “sample data.” |
| “Restore evidence / immutable JSON” | “Restore evidence / hash-linked JSON” |
| “Alert on two things only.” | “Alerts report a failed restore or an overdue drill” |
| “Everything else exits quietly with code 0…” | “Successful runs exit with code 0. JSON output works with schedulers and alert tools.” |
| “Operator notes / Before you put it on a timer” | “Questions before scheduling a drill” |
| “Run healthy sample” | “Run sample restore” |
| “Failure caught loudly” | “Missing file detected” |
| “Cleanup failure makes the drill fail loudly.” | “If cleanup fails, the command exits with code 1 and names the temporary folder.” |
| “The next useful backup task” / “Prove one file today.” | “Run your first drill” / “Check one important file today.” |
| “Then let the receipt—not your memory—tell you when it is time again.” | “Check the receipt to see when the next drill is due.” |
| “Local proof for the backups you already have.” | “Verify selected files from your existing backup.” |
| “Check the bytes.” | “Compare the restored file with its saved fingerprint.” |
| README opening, sentence 1 (24 words) | “Restore Drill is a local command-line tool for people who already run scripted backups. It checks that selected files still restore.” |
| README opening, sentence 2 (33 words) | “It restores into a new temporary directory and verifies presence and SHA-256 fingerprints. It can ask the file's application to validate it. It removes the directory and writes a hash-linked receipt.” |
| README response-policy sentence (38 words) | “The deployable site includes an Azure Static Web Apps response policy. It sets CSP, permissions, and referrer headers. Hashed assets cache for one year. The service worker always revalidates.” |
| “Everything runs locally.” | “The CLI runs on your machine and has no product network client.” |

Terminology should be fixed to one term per concept:

| Concept | Use | Replace |
| --- | --- | --- |
| Runnable example | sample data / demo | fixture, simulation, recorded local flow, flow |
| Temporary location | temporary folder | target, clean target, temporary directory in visitor-facing copy |
| File identity | SHA-256 fingerprint | hash on first mention; define technical field later |
| Receipt protection | hash-linked / tamper-evident | immutable |
| Timing | schedule | cadence outside configuration/API names |
| Backup process output | backup-command output | subprocess output |
| Command-line product | command-line tool | CLI on first mention |
| Installed executable | prebuilt program | release binary on first mention |
| Application validation | application check | open-check |
| Machine-readable output | JSON output | JSON without a first-use explanation |
| Local machine | this computer | host in visitor-facing copy |
| Security assumptions | security risks | threat model on first mention |
| Backup access | credentials that can only read backups | read-only repository credentials on first mention |
| Scheduling | scheduler / alert tool | cron, systemd, notifier until the technical example |
| Protected external receipt store | storage that prevents changes | append-only storage on first mention |

For every `U` flag, the concrete resolution is to put that exact sentence and location in `.factory/claims.json`, map it to the closest tagged test in BLOCKING 3, and assert the observable result. Remove the sentence when no such test can honestly prove it. In particular, remove the five-minute number unless `@claim:five-minute` measures it, replace every immutable-receipt claim rather than testing it, and map privacy/local/offline statements to `@claim:local-network`, `@claim:no-runtime-service`, and `@claim:offline-reload`.

## Verification record

Commands and checks run from the clean base worktree before this report was added:

```text
npm ci                                                     PASS (22 packages, 0 vulnerabilities)
npm test                                                   PASS (5 Rust + 10 browser tests)
rg '@claim:'                                               FAIL contract: no tagged tests
test -f .factory/claims.json                               FAIL contract: missing
test -f .factory/demo.md                                   FAIL contract: missing
cargo run --manifest-path /work/repo/Cargo.toml -- demo    exit 2, unrecognized subcommand
cargo run --manifest-path /work/repo/Cargo.toml -- --demo  exit 2, unexpected argument
/opt/fleet/lib/verify-url.sh <live> <temp evidence dir>     PASS basic URL checks
live Axe at 390px and 1366px, all three real routes        PASS, zero violations
live link crawl                                             PASS, all discovered targets 200
live unknown-route check                                    FAIL, home page returned as 200
live offline reload after service-worker registration      PASS
live network interception through browser demo              PASS, same-origin only
```

The PASS results above are retained as evidence, not used to soften the verdict. The release rule requires zero blocking findings; this review found five.
