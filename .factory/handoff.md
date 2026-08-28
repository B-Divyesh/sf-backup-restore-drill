# Restore Drill — adversarial first-read review handoff

**Work order:** `backup-restore-drill-review-1`

**Candidate:** `dfa88e162b750a80ebc1fb7697082118375532dc`

**Review verdict:** **FAIL**

## What was done

Completed the required cold first-read review on the live product at 390×844 and 1366×900. Audited every landing-page and README sentence, entered and exercised the browser simulation, attempted both plausible CLI demo commands from an isolated temporary directory, checked the claim registry, ran the full repository suite, intercepted demo traffic, forced an offline reload, inspected route metadata/focus/history, crawled links, ran the factory URL verifier, and ran Axe against every public route at both viewports.

The complete evidence and concrete fixes are in `.factory/review-1.md`. No product code was modified.

## Result

Five blocking findings prevent acceptance:

1. The first screen does not name the intended operator or one unambiguous first action.
2. There is no one-click real demo, CLI demo command, bundled sample, demo banner, reset, or demo documentation.
3. `.factory/claims.json` and all `@claim:*` tests are missing, leaving every published claim unlisted.
4. The site calls user-controlled, read-only local JSON “immutable.”
5. `/demo` and unknown routes silently return the home page with HTTP 200; there is no real demo route or designed 404.

Metadata and route-focus consistency also need repair. The risograph identity, basic semantics, responsive layout, reduced-motion behavior, offline shell, automated accessibility scan, and discovered links passed.

## How to verify

```sh
npm ci
npm test
npm run build
```

For review-specific reproduction, use the commands and expected results in the verification record at the end of `.factory/review-1.md`.

## Known gaps / next steps

Implement the fixes in severity order in `.factory/review-1.md`, then rerun this review from a fresh browser context and clean temporary directory. Do not accept a repair until `/demo` runs real bundled sample data in one click, every published claim maps to a tagged passing test, unknown paths return a designed 404, and the first screen answers what/for whom/first action without inference.
