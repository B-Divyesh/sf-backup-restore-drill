# Independent verification 3 — Verify a selected backup file restores

**Verdict: FAIL**

**Finding count:** 1 minor

**Untested claim count:** 0

**Implementation candidate:** `35ab0c3280720a9b9a66c2d4cb3e528d91129589`

**Documentation candidate:** `7d9e6411058ae4b78a971ff15e8521728e01b454`

**Live URL:** <https://backup-restore-drill.sociobot.in/>

**Verified:** 2026-09-05 UTC

The implementation works end to end, all 14 declared claims pass from a clean checkout, and the public site now matches the candidate build byte for byte. It does not receive a PASS because several live phone controls are smaller than the required 44×44 CSS-pixel touch target.

## Finding

### F-3-1 — MINOR — Some phone touch targets are smaller than 44×44 CSS pixels

Fresh Chromium contexts at 390×844 measured the live controls after layout:

| Control | Measured box |
| --- | ---: |
| Header `Setup` | 39.0×44 px |
| Header `Demo` | 31.2×44 px |
| Footer `Demo` | 33.6×44 px |
| Footer `Terms` | 42.0×44 px |
| Demo banner `Reset demo` | 72.0×36 px |
| Demo banner `Start for real` | 100.8×36 px |

The same short header and footer links recur on the home, demo, privacy, and terms pages. The controls are visible, keyboard-operable, and separated, but their rendered boxes do not meet the attached accessibility and design baseline of at least 44×44 CSS pixels.

**Required repair:** Give every short navigation link at least 44 px width and increase the demo-banner controls to at least 44 px height. Retest all live routes at 390 px.

No product code was changed during this verification.

## First screen and one-click sample

Fresh desktop (1366×900) and phone (390×844) contexts opened at `scrollY = 0`.

- Job: **Verify one file restores from your backup.**
- Audience: people with scripted backups who need repeatable proof that important files still restore.
- First action: **Try it with sample data.**

All three plain facts were visible before scrolling in both viewports: free and MIT licensed, no usage tracking, and offline after the first visit. One click opened `/demo/`. On the 390 px first screen, the completed result showed the real-looking sample path, SHA-256 match, complete cleanup, hash-linked JSON receipt, and exit `0 / passed`.

The persistent label read **Demo — sample data, nothing is saved**. The missing-file path showed `Recovery not verified`, cleanup complete, exit `1 / failed`, and a specific recovery step. **Reset demo** restored the passing result, and **Start for real** remained available. Browser local storage, session storage, and cookies stayed empty. No cross-origin request or failed request occurred.

The installed `restore-drill demo --json` also returned a passing receipt for `Documents/quarterly-tax-notes.txt` with `workspace: removed`. Its trap-based claim test proved unrelated configuration, backup, and receipt data remained unchanged.

## Declared claims

The clean checkout was `/tmp/restore-drill-verify3-claims.S6X87X/repo` at documentation SHA `7d9e6411058ae4b78a971ff15e8521728e01b454`, containing implementation SHA `35ab0c3280720a9b9a66c2d4cb3e528d91129589`. After `npm ci`, every command exactly as declared in `.factory/claims.json` ran separately.

| Claim | Result | Observable coverage |
| --- | --- | --- |
| `demo-isolation` | PASS | Bundled CLI demo used shipped data and preserved working-directory traps. |
| `restore-verification` | PASS | Passing, hash-mismatched, and missing selected files exercised the real CLI. |
| `configured-command` | PASS | Literal configured arguments reached the restore stub without shell expansion. |
| `temporary-folder-lifecycle` | PASS | Pass, restore failure, missing, mismatch, application failure, and timeout all cleaned before receipt writing. |
| `application-check` | PASS | Passing and exit-7 checks changed the drill result correctly. |
| `path-safety` | PASS | Unsafe configuration and an outside restored link were rejected. |
| `receipt-integrity` | PASS | JSON, read-only mode, SHA-256 linkage, and replacement detection passed. |
| `receipt-redaction` | PASS | Backup-command output, arguments, and restored private bytes were absent from receipts. |
| `json-and-exit-codes` | PASS | Run, status, and receipts JSON plus result codes 0–4 passed. |
| `browser-privacy` | PASS | No forms, cookies, storage, or off-site requests appeared through the demo flow. |
| `offline-reload` | PASS | A fresh first-visited demo reloaded offline in its own browser context. |
| `license` | PASS | Repository and packaged crate carried the MIT license. |
| `build-artifacts` | PASS | The documented build created `dist/bin/restore-drill` and `dist/site/`. |
| `response-policy` | PASS | Built deployment configuration carried the documented security and cache policy. |

**Claim summary:** 14 passed, 0 failed, 0 untested. The landing page and README were cross-checked against the registry; no unlisted or materially incomplete public claim was found.

## CLI and clean consumer evidence

| Check | Result |
| --- | --- |
| `npm ci` | PASS — 22 packages installed, 0 vulnerabilities. |
| `npm test` | PASS — 3 Rust unit tests, 11 CLI integration tests, 37 Playwright passes, and 3 intentional duplicate-project skips. |
| `npm run build` | PASS — static site and stripped single binary produced. |
| `cargo fmt --check` | PASS. |
| `cargo clippy --all-targets -- -D warnings` | PASS. |
| `cargo package --allow-dirty` | PASS — 55 files, 705.9 KiB unpacked, 494.8 KiB compressed. |
| Clean packaged consumer | PASS — package installed to `/tmp/restore-drill-verify3-consumer.Wqw6zY/install`. |
| Installed `--version` and `--help` | PASS — version `0.1.0`; commands and result behavior documented. |
| Installed demo | PASS — JSON parsed, sample passed, disposable workspace removed. |
| Installed init and invalid recovery | PASS — starter configuration created; invalid fingerprint exited 2 with a specific correction hint and no receipt state. |

The Rust tests cover normal success; corrupt, missing, restore-failure, timeout, and application-failure paths; unsafe paths and links; invalid placeholders and configuration; cadence boundaries; receipt tampering; redaction; JSON modes; and recovery hints. This is a CLI/static-site product, so backend tenant isolation, restart persistence, health checks, and HTTP 429 allowances do not apply.

## Live site, accessibility, privacy, and recovery

| Check | Result |
| --- | --- |
| Basic URL verifier | PASS — HTTP 200, title, `lang=en`, one H1, one main, alt text, and no console errors. |
| Home and demo at desktop/phone | PASS apart from F-3-1 — no horizontal overflow, page errors, request failures, or off-origin requests. |
| Axe | PASS — zero violations on home, demo, privacy, terms, and the designed 404 in fresh live contexts. |
| Keyboard and focus | PASS — Enter and Space operated demo controls; the tested focus ring was 3 px ochre; no trap occurred. |
| Route focus/history | PASS — direct hashes and Back focused and announced the destination heading. |
| Reduced motion | PASS — animation and transition were `1e-05s`; scroll behavior was `auto`. |
| Offline/update | PASS — the live service worker updated, controlled `/demo/`, and reloaded the completed demo offline with its notice. |
| Privacy | PASS — no forms, cookies, browser storage, analytics/CDN requests, or third-party runtime requests were observed. |
| Links | PASS — all discovered internal routes and the labeled GitHub source link returned 200. |
| Route titles/metadata | PASS — home, demo, privacy, terms, and 404 had route titles, canonical data, social image data, one H1, and one main. |
| Deliberate unknown route | PASS — returned HTTP 404 with the designed page and a return link; this expected 404 is not a defect. |
| Visual identity | PASS — the two-ink evidence-desk system, original restore path art, receipt treatment, and single-mode paper palette match `.factory/design.md`. |

The deterministic restore and hash checks do not benefit from adding an AI feature. No missed AI leverage finding applies.

## Live candidate identity and response policy

Production changed during this verification and now serves `build repair-2`. The following live resources matched the clean candidate build byte for byte: home, demo, privacy, terms, 404, hashed JavaScript, hashed CSS, and `sw.js`.

Representative SHA-256 values:

- home: `c836759d096a78eb3e7b37d84a96bb3e38886232dceb4a33bafe0a5162b19e5e`
- demo: `129cdc70b3ab11589068232feb7ee9064270af82d56c8b0b545e13c5fca9c306`
- JavaScript: `37cc2ff47bf7e4614f4da75d667910e3b4ca7f7b11cca3e17ce2ab95bbc36f10`
- CSS: `806aeae14972050cd2a8561910e6f119f84ae925f43e145a4171c296ef94601d`
- service worker: `e5f3208f601033c472608b072c4663ade8978847180bb29aa150c1253440113e`

Live HTML uses `public, max-age=0, must-revalidate`; the service worker uses `no-cache`; hashed JavaScript and the hero WebP use one-year immutable caching. HTTPS responses include the matching CSP, Permissions-Policy, `Referrer-Policy: no-referrer`, and `X-Content-Type-Options: nosniff`.

## Performance

Fresh mobile Lighthouse against the live home page scored Performance 100, Accessibility 100, Best Practices 100, and SEO 100. FCP was 0.99 s, LCP 1.36 s, CLS 0, and TBT 52 ms.

Built payloads remain below budget: JavaScript 2,045 bytes gzip, CSS 4,404 bytes gzip, and the hero WebP 90,028 bytes.

## Earlier finding disposition

### Review 1 and the first deployment verification

| Earlier finding | Current disposition |
| --- | --- |
| Review 1 blocking 1 — unclear job, audience, action | FIXED — cold desktop and phone first screens answer all three before scrolling. |
| Review 1 blocking 2 — no one-click/CLI demo | FIXED — one-click populated browser demo and installed bundled CLI demo both passed. |
| Review 1 blocking 3 — no claims registry | FIXED — 14 declared commands passed; none is untested. |
| Review 1 blocking 4 — “immutable JSON” | FIXED — public copy says read-only, hash-linked JSON and accurately limits the guarantee. |
| Review 1 blocking 5 — catch-all routing | FIXED — demo is real and an unknown path deliberately returns the designed 404. |
| Review 1 major 6 — route metadata | FIXED — route-specific titles, canonical/social metadata, art, favicon, and apple icon are live. |
| Review 1 major 7 — route focus and inconsistent shell | FIXED — headers/footers are consistent and hash/Back focus is announced. |
| Review 1 minor 8 — unlabeled external links | FIXED — source links say GitHub and external. |
| First verification — missing response headers/cache | FIXED — headers and cache rules are live and match the candidate configuration. |

### Review 2

| Finding | Current disposition |
| --- | --- |
| F-2-1 | FIXED — completed populated output is visible on the 390 px demo first screen. |
| F-2-2 | FIXED — registry has 14 outcome tests; all passed separately from a clean checkout. |
| F-2-3 | FIXED — direct, cross-page, and Back hash routes focus and announce their headings. |
| F-2-4 | FIXED — copy assigns restore scope to the operator's backup command. |
| F-2-5 | FIXED — all three facts are above the phone fold. |
| F-2-6 | FIXED — visitor copy consistently uses “temporary restore folder.” |
| F-2-7 | FIXED — the 404 has canonical, Open Graph, Twitter, noindex, and live HTTP 404 behavior. |
| F-2-8 | FIXED — real CLI pass, corrupt, and missing selected-file outcomes passed. |
| F-2-9 | FIXED — configured literal arguments reached the restore process. |
| F-2-10 | FIXED — cleanup-before-receipt passed across six result classes. |
| F-2-11 | FIXED — application pass/failure affects the result. |
| F-2-12 | FIXED — browser demo has no forms, cookies, storage, or third-party requests. |
| F-2-13 | FIXED — bundled demo preserved configuration, backup, and receipt traps. |
| F-2-14 | FIXED — the unsupported exact public Rust-floor claim was removed. |
| F-2-15 | FIXED — unsupported standalone/network copy was removed from the landing page and README. |
| F-2-16 | FIXED — run, status, and receipts JSON plus documented result codes passed. |
| F-2-17 | FIXED — placeholder/path safety and literal no-shell arguments passed. |
| F-2-18 | FIXED — injected process output and restored bytes were absent from receipts. |
| F-2-19 | FIXED — all documented codes 0–4 occurred. |
| F-2-20 | FIXED — receipt JSON, mode, linkage, and tamper detection passed. |
| F-2-21 | FIXED — documented build artifacts and deployment policy passed. |
| F-2-22 | FIXED — “real restore path” was replaced with plain sample wording. |
| F-2-23 | FIXED — unexplained “resolved files” wording was removed. |
| F-2-24 | FIXED — README names security-header and cache settings directly. |

## Final decision

**FAIL — 1 minor finding, 0 untested claims.** Repair F-3-1 and repeat the live 390 px target-size check before declaring PASS.
