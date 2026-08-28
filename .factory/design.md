# Restore Drill — visual thesis

## Direction

**Risograph evidence desk.** Restore verification is a physical-feeling act: pull a sample from the archive, stamp it, and file the receipt. The site therefore resembles a two-ink operator's worksheet rather than a cloud dashboard. Slight ink offsets, coarse dots, clipped paper edges, and a single looping path explain the product's real sequence: archive → temporary bench → checks → receipt. Decoration never pretends that a backup is healthy; the only “proof” mark appears after checks.

The treatment is intentionally single-mode. Warm paper is the workspace and near-black ink provides stable contrast; a dark theme would undermine the paper-receipt metaphor. The background is explicitly painted.

## Palette

| Token | Hex | Role |
| --- | --- | --- |
| paper | `#F3EBD8` | page background, sampled from uncoated stock |
| paper-raised | `#FFF9EA` | receipts and code strips |
| ink | `#18201D` | primary copy and outlines |
| muted-ink | `#525A50` | secondary copy (7.1:1 on paper) |
| cobalt | `#1659A8` | archive/action ink |
| cobalt-dark | `#0B3D79` | link and hover ink |
| persimmon | `#D94B2B` | overdue/failure stamp; paired with text/icons |
| brick | `#A32F18` | accessible persimmon-family text and solid warning fields |
| moss | `#256A45` | verified state; paired with “PASS” |
| ochre | `#A46100` | warning state and focus underlay |

All body combinations meet WCAG AA. Status never depends on hue: every state has a word, symbol, and border treatment.

## Type

- Display: **Arial Black / Franklin Gothic Heavy / sans-serif**, compact and blunt like a stamped carton label. No font download.
- Working copy: **ui-monospace / SFMono-Regular / Consolas / monospace**, connecting the documentation to terminal output while remaining readable at 16px and 1.65 line height.
- Five-step scale: 14, 16, 20, 32, and clamp(48–88) px. Tabular figures are enabled for dates, hashes, and durations.

System families keep the static payload tiny and avoid third-party font requests.

## Spacing and composition

An 8px base rhythm with 4px micro-adjustments. Content caps at 1184px and readable prose at 68ch. Desktop uses an asymmetric 7/5 “desk” split; mobile collapses into document order and drops nonessential registration marks. Every target is at least 44px. Sections group by whitespace first and use rules only where they imply a receipt edge or process boundary.

## Interaction grammar

- Primary actions look like cobalt ink blocks offset over a black keyline; pressing closes the offset.
- The demo advances one physical step at a time and announces its current state. It never claims to touch the visitor's filesystem.
- Copy buttons swap label text immediately and expose an `aria-live` confirmation.
- The navigation stays simple and non-sticky so it cannot obscure zoomed mobile content.
- Empty, running, pass, injected-failure, and offline states all include a next action.

## Motion policy

One 240ms “paper settle” entrance (opacity + translate only) and 160ms press/state transitions. The demo's progress is finite, user-initiated, and cancellable by starting another run. Nothing loops. Under `prefers-reduced-motion: reduce`, transforms and smooth scrolling are removed and state changes are immediate opacity swaps.

## Asset plan and provenance

- `site/public/restore-path.webp`: original generated hero illustration, a text-free two-ink risograph collage showing an archive box, temporary work tray, checked file slips, and an evidence receipt linked by a cobalt path. Generated on 2026-08-28 with the factory image deployment via `/opt/fleet/lib/gen-image.sh`; prompt and model metadata are preserved in `.factory/restore-path.prompt.json`. Optimized locally to WebP under 300 KB. License: project-owned generated asset under the repository MIT license.
- `site/public/restore-drill-og.jpg` and `site/public/apple-touch-icon.png`: local crops composed from the original `restore-path.webp` illustration on 2026-08-28. They add no new imagery or text and inherit its project-owned MIT provenance.
- Registration crosses, dot fields, receipt perforations, and stamps are CSS-built product marks, not stock assets.

No logos, stock icons, third-party imagery, CDN assets, generic gradients, or decorative dashboard charts are used.
