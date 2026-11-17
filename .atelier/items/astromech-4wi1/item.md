---
schema: atelier.item
schema_version: 3
id: astromech-4wi1
title: 'Lint: a raw colour or an off-scale size fails in any component'
status: in_review
mission: astromech-xajc
labels:
- size-m
- tooling
priority: medium
blocked_by:
- astromech-3kf5
implementers:
- thread:delegated-task:command%3Amcp%3Affec3634-009b-40bd-8764-e19cb778e43b%3Adelegate-task%3Ae5d8edc3-906e-4dfb-9895-e1ecc0d0969e
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.275900160Z
---

## Goal

The design system is enforced, not remembered: a component cannot use a colour or a size the system does not have. Sumeet, 16 Nov 2026: "the principled design system is what I'm after".

## Done when

- [x] `pnpm lint` fails a `.svelte` or `.css` file under `app/src` (outside `design/tokens.css` and `mockups/`) that writes a raw colour (hex, rgb, hsl, a named colour) in place of a token.
- [x] It fails a font size, and a margin, padding or gap, that is not a token or a role class. Exemptions are listed by file with a reason each, as `PAGE_SIZING_EXEMPT` is.
- [x] Every existing violation is fixed or listed; the list is short enough to read, and each entry names the item that removes it.
- [x] The rule has tests like the page-sizing check's. CONTRIBUTING.md's UI check says what the lint enforces.

## Scope

Lint and the fixes it forces. It extends the page-sizing check in `app/eslint.config.js`; it does not replace it.

## Report

A component can no longer use a colour, font size or spacing the design system lacks without `pnpm lint` failing it. The rule is `app/eslint-rules/tokens.js`, with three parts that can be switched off separately: `tokens/colour`, `tokens/font-size` and `tokens/spacing`. It is on for every `.svelte` and `.css` file under `app/src` except `design/tokens.css` and `mockups/`. `.css` files are read through `@eslint/css` (new dev dependency). The shared script configs are now scoped to script files, because they crashed on a CSS AST. The page-sizing check is unchanged.

What fails:
- Colour: a hex, `rgb()`/`hsl()`/`lab()` and the like, anywhere in a value, a custom property or a `var()` fallback included. A named colour fails in a property that takes a colour. `transparent`, `currentColor` and a `color-mix()` of tokens pass.
- Font size: a length, a percentage or a size keyword in `font-size` or the `font` shorthand.
- Spacing: a margin, padding or gap written as a length other than 0, or a token scaled by a factor that is not a whole number. A percentage or a viewport unit counts as a share of the window and passes; so do whole factors like `calc(-1 * var(--sp-2))`.

Known gaps. I grepped `app/src` for each one and found no current use:
- A custom property the component declares itself is not followed, so `--pad: 10px; padding: var(--pad)` passes. The rule's header says so.
- Arithmetic between tokens, like `calc(var(--sp-4) / 3)` or `calc(var(--sp-2) + var(--gap))`, passes.
- Interpolated style text, like `style="padding: {n}px"`, passes, because only literal text is read.
- SVG presentation attributes (`fill="#…"`, `stroke=`) and colours in `<script>` are not read.

The fixes, in three commits:
1. `9830d2b2`: 218 spacing literals that already equalled a step now name it (`--sp-*`, `--gap`, `--gutter`). Captures of the 11 pages before and after this commit are byte-identical.
2. `d7a374ad`: off-scale spacing in `lib/ui`, onboarding, the route frame parts, Materials, Settings and the Cargo and Outfitting finders now takes the nearest step. A tie (6, 10, 14, 20 or 28 px) goes to the smaller step. Each value moves 1 to 4 px. Most shrink, but 22 px is nearer 24 than 16, so three grow by 2 px: onboarding's lead margin and the vertical padding of the bind and calibration dialogs. The commit body says "nothing grows", which is wrong for those three. Timeline's node padding moves 1 px (3 to 2). The three dialogs' `rgb(0 0 0 / 0.7)` is the new `--scrim` token, added to DESIGN.md's Token map. SearchKit's unit span now uses the `.type-unit` role, which writes the same 3 px. Before and after captures at 1920 x 1080 are in `docs/design/tokens-*-{before,after}.png`: 13 pairs, including the calibration dialog and the route import dialog.
3. `204cb661`: the rule, the config, the tests and CONTRIBUTING.md.

`TOKENS_EXEMPT` in `app/eslint.config.js` has 14 entries, all spacing except Sheet:
- Lasting (3):
  - Controls (the four Controls modules, plus About and PresetPicker, which appear only there).
  - Button. DESIGN.md Buttons gives the primary button 20 px padding, which the scale lacks.
  - Sheet. Its labels scale with the controller image (`--k`), so they stay px.
- Pending (11): each names in `until` the mission item that rebuilds the page: m07p, 8wr4, m6gw, nnn0, evj3, dpaz, fzhp, rj6i, 7vxi, el3i and gvy0. gvy0's entry includes `places/Home.svelte`.

Tests (`eslint-rules/tokens.test.js`, 45 cases): RuleTester cases for each part, in Svelte and in CSS. The config cases check that a new `.svelte` or `.css` file fails, that tokens.css and the mockups don't, and that an exemption turns off only its own parts. Every entry must have a reason and point at an existing item. **An entry its files no longer need fails the suite**, so a page rebuilt clean must leave the list. I checked that this test fails with a bogus entry.

Checks run: `pnpm lint`, `pnpm check`, `pnpm build`, `scripts/check-dist.py`, `scripts/check-deps.py` and `pnpm test` (85 files, 685 tests) all pass. `eslint --print-config` lists the same script rules as on main for a `.ts`, a `.svelte.ts` and a `.svelte` file, plus the three token rules on the `.svelte` file.

For the lead:
- **Controls are exempt from spacing for good.** The mission gives Controls only the frame, Text size, colours and Tip, so I didn't move their spacing. If the system should hold there too, that entry becomes a pending one with a new item.
- **DESIGN.md disagrees with its own spacing scale.** It gives the primary button 20 px padding, a step the scale lacks. Settle it by adding a step or changing the button, not in lint.
- **The 11 named items should each get a Done-when line**, such as "its `TOKENS_EXEMPT` entry is gone". I didn't edit other items from this branch. The stale-entry test already forces removal once a page is clean.
- **On foot (odyssey) waits on nnn0, not 5a01.** 5a01 moves On foot's parts whole into Ships > Suits, Materials and Engineers, which removes no spacing violations. 5a01 must carry the `TOKENS_EXEMPT` entry to wherever the files land, or Materials, which is clean now, will fail lint.
- **uc7i and 3ca6 are building in parallel**, and both touch `eslint.config.js` and components. The second to merge must rerun `pnpm lint`, since a new raw value now fails.
