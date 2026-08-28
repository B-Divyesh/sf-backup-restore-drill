# Polish 1 — review finding closure

**Repair commits:** `6e5069239637627730ff10fedfd3b45cbb01953b`, `9ab305f`
**Live URL checked cold:** `https://backup-restore-drill.sociobot.in/` on 2026-08-28 UTC

| Finding | Change made | Evidence |
| --- | --- | --- |
| BLOCKING 1 | Replaced the first screen with the seven-word job headline, named people with scripted backups, and made **Try it with sample data** the sole primary action. | `home is semantic…`; `.factory/evidence/polish-1-home-390.png`; live `/` title and cold copy check. |
| BLOCKING 2 | Added shipped `examples/Documents/quarterly-tax-notes.txt`, real `restore-drill demo`, `/demo/`, `?demo=1` redirect, completed sample state, banner, Reset demo, Start for real, and `.factory/demo.md`. | `@claim:demo-isolation`; `cargo run -- demo --json`; `.factory/evidence/polish-1-live-demo-390.png`; live `/demo/`. |
| BLOCKING 3 | Added claims registry and one tagged Playwright test for each listed claim. Removed unsupported marketing promises. | Every command in `.factory/claims.json` passed; `npm test` (20 browser cases). |
| BLOCKING 4 | Replaced every visitor-facing local-receipt “immutable” statement with **hash-linked** and corrected the brief. | `@claim:receipt-integrity`; `rg immutable` finds only immutable HTTP cache directives. |
| BLOCKING 5 | Added real `/demo/` and designed `404.html`; removed the static-site fallback that hid unknown URLs. | live `/does-not-exist-review-1` → HTTP 404, title `Page not found — Restore Drill`; route test. |
| MAJOR 6 | Added canonical, Open Graph, Twitter, 1200×630 product-art image, apple touch icon, and demo sitemap URL. | `routes have metadata…`; live cold route inspection. |
| MAJOR 7 | Made the header/footer consistent, added Param Factory/build text, focused and announced hash destinations, and focuses route headings on history navigation. | `query demo route redirects and keyboard section navigation moves focus`; keyboard test at both viewports. |
| MINOR 8 | Renamed the external source destination to **View source on GitHub (external)**. | live footer and link crawl in browser route test. |
| Copy audit findings | Rewrote the landing and README in plain words, normalized terms, removed “five-minute,” “fixture,” and mock-recording language, and audited all landing copy. | `.factory/copy-audit.md`; 390px screenshot. |
| Earlier response-policy/cache finding | Retained and rechecked production CSP, permissions, referrer, content-type, and cache controls after redeploy. | `npm run test:response-policy`; live `curl -sSI /` headers. |

No review finding remains open.
