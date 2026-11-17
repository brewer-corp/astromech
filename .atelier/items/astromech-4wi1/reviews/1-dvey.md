---
schema: atelier.review
schema_version: 3
round: 1
commit: b75655688eddf657c7f6ba1c7a7744b8ae78f40e
reviewers:
- thread:delegated-task:command%3Amcp%3A9d5bc0dc-205a-4ad6-a0d5-0abb49c85d4c%3Adelegate-task%3Afc293540-850f-411f-a570-5979f9f11d2d
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T08:14:20.001572487Z
---

## Summary

Complexity: medium. Three new lint rules share value parsing and now run against both Svelte/PostCSS and a newly added CSS language parser; the migration touches many components.
Value: high. Enforcing the design system is an explicit mission outcome, and these checks catch the common mistakes automatically.
Risk of merging: medium. The spacing changes reach many visible screens, and the new rules can both miss violations and block valid edits; each change is reversible.

The value warrants the implementation and migration. I found no substantially smaller change that delivers the required enforcement. Changes are needed because the font and colour checks still accept ordinary off-system values, and the CSS path treats comments as values.

Reviewed main...b75655688eddf657c7f6ba1c7a7744b8ae78f40e against all four Done when lines, DESIGN.md, tokens.css, the mission, config, tests and report. Checked the spacing substitutions by resolving their tokens back to rem values and inspected the committed Materials, Cargo and calibration-dialog before/after captures. The exemption list has reasons and pending-item references, the stale-entry test runs, and CONTRIBUTING.md describes the lint. The page-sizing rule is unchanged.

Independently passed pnpm lint, pnpm check, pnpm build, scripts/check-dist.py, scripts/check-deps.py and the full pnpm test suite (85 files, 685 tests). Ran additional lintText probes through the real ESLint config for new, non-exempt .css and .svelte paths. For accepted-value findings below, also used headless Chromium's CSS.supports and computed styles to confirm that the examples are valid and actually set the off-system values. No source files were changed. Did not rerun Windows/Tauri or Rust checks, or reproduce the report's historical claim that the first migration commit had 11 byte-identical page captures.

## Finding 1: [P2] Preserve and check font-size fallbacks inside var()
Recommendation: fix now

app/eslint-rules/tokens.js:80 removes the entire var() call before inspecting a font declaration. Both `font-size: var(--missing, 17px)` and `font: 400 var(--missing, 17px)/1.3 sans-serif` produce no lint messages in either a .css file or a Svelte style block. Chromium renders the first at 17px. The component writes that raw size directly, without a declared exemption, so the font-size Done when line is not met. Preserve and inspect fallback values, as the colour and spacing checks already do, while keeping legitimate shorthand line-height lengths valid. Add regression cases for both properties and parsers.

## Finding 2: [P2] Check named colours in filter and text-fill properties
Recommendation: fix now

app/eslint-rules/tokens.js:42 and :59 restrict named-colour detection to an incomplete property list. `filter: drop-shadow(0 0 2px red)`, `-webkit-text-fill-color: red` and `-webkit-text-stroke: 1px red` all produce no lint messages in both parsers. Chromium accepts all three and computes their colour to rgb(255, 0, 0). Hex colours in the same properties are caught, so merely choosing a colour name bypasses the first Done when line. Cover these colour-bearing values, with tests that still allow non-colour identifiers such as animation names.

## Finding 3: [P2] Exclude CSS comments before inspecting declaration values
Recommendation: fix now

app/eslint-rules/tokens.js:145 passes source text, including comments, to bare() at :47-52, which removes strings and URLs but leaves comments. In a .css file, `color: var(--accent) /* was red */` incorrectly fails tokens/colour, and `padding: var(--sp-2) /* used to be 10px */` incorrectly fails tokens/spacing. Both declarations use valid tokens. Equivalent Svelte style blocks pass because their PostCSS declaration values omit those comments. Read the CSS value without comment tokens, or handle comments in the shared normalisation, and test that both parsers agree. An explanatory comment must not force an exemption or block pnpm lint.

## Finding 4: [P2] Reject fractional scaling of font-size tokens
Recommendation: fix now

app/eslint-rules/tokens.js:77-86 checks only explicit lengths, percentages and keywords. `font-size: calc(var(--fs) * 1.25)` therefore passes both parsers: after var() is removed there is nothing this rule recognises. With the actual 15px --fs scale step, Chromium computes 18.75px, which is not a font-size token or type role. Spacing already rejects this fractional-factor pattern at :98; font-size needs equivalent protection against off-scale arithmetic, with valid direct token uses retained and regression coverage. This is a numeric multiplier, not the report's acknowledged arithmetic between tokens.

## Finding 5: [P3] Recognise valid CSS exponent numbers and root-relative lengths
Recommendation: defer
Taken: astromech-mwlt by 65a8ff7c-e901-48a1-be52-323bbbd8e80c

app/eslint-rules/tokens.js:64-66 recognises only plain decimal numbers and omits root-relative font units such as rcap. `padding: 1e1px`, `font-size: 1e1px` and `padding: 2rcap` all pass lint in .css and Svelte. Chromium accepts them; the first two compute to 10px. These are literal lengths, so the rule's stated coverage is broader than its implementation. This is rare syntax with no observed production use and can follow the ordinary-value fixes above. Extend the dimension handling and tests so all supported CSS numeric forms and fixed/font-relative units are checked.
