---
schema: atelier.review
schema_version: 3
round: 2
commit: 8f79bc00dd1aeda69dbaf6945b653c1e5a4aef92
reviewers:
- thread:delegated-task:command%3Amcp%3A9d5bc0dc-205a-4ad6-a0d5-0abb49c85d4c%3Adelegate-task%3A48d6ae10-d481-4b97-9729-665735cf94f1
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T08:19:31.783126412Z
---

## Summary

Complexity: medium. Three lint rules share value parsing across Svelte/PostCSS and a new CSS language parser; their migration touches many components.
Value: high. Enforcing the design system is an explicit mission outcome and catches ordinary styling mistakes automatically.
Risk of merging: medium. The spacing migration reaches visible screens, and a lint error can block valid edits; the changes are reversible.

The value warrants this implementation. I found no substantially smaller change that delivers the required enforcement. All four carried fix-now findings are fixed. The decimal-factor false positive below is a minor follow-up, with no current occurrence found, and does not block approval.

Reviewed main...8f79bc00dd1aeda69dbaf6945b653c1e5a4aef92 against the four Done when lines, the mission, DESIGN.md, tokens.css, the config, tests and Report. Also read the full round-2 code/test delta. Checked the migration by parsing component declarations and resolving spacing tokens back to rem values: the remaining numerical changes agree with the reported rounding. Inspected the committed Materials and calibration-dialog before/after captures and the Cargo after capture. The exemption list is short, gives reasons and pending-item references, and its stale-entry tests pass. The permanent Controls exemption agrees with the mission, and Button's 20 px exception agrees with DESIGN.md. CONTRIBUTING.md describes the checks, and page-sizing remains unchanged. Compared calculated ESLint rules with main for .ts, .svelte.ts, .svelte and .js paths: existing rules are unchanged.

Independently passed pnpm lint, pnpm check, pnpm build, scripts/check-dist.py, scripts/check-deps.py and the full pnpm test suite (85 files, 702 tests). The targeted token-rule suite passes all 62 cases. Ran additional lintText probes through the actual config for non-exempt .css and .svelte paths, covering every carried finding, valid shorthand line heights, and the earlier deferred exponent/root-relative-length cases. Used headless Chromium to confirm the decimal-factor examples below compute to permitted spacing values. No source files were changed. Did not rerun Rust or Windows/Tauri checks, recapture all pages, or reproduce the historical claim of 11 byte-identical captures after the first migration commit. The Report explicitly lists the remaining custom-property, interpolated-style, SVG/script-colour and spacing-arithmetic limits; this approval does not claim those paths are enforced.

## Finding 1: [P2] Preserve and check font-size fallbacks inside var()
Recommendation: fix now
Was: 1-dvey.1
Response: fixed in dc327678
Now: fixed

The shared normalization now retains var() fallbacks. Both font-size: var(--missing, 17px) and font: 400 var(--missing, 17px)/1.3 sans-serif produce tokens/font-size errors through both parsers. Legitimate token sizes with a length-valued shorthand line height still pass. Regression coverage is present.

## Finding 2: [P2] Check named colours in filter and text-fill properties
Recommendation: fix now
Was: 1-dvey.2
Response: fixed in dc327678
Now: fixed

The colour-bearing property list now includes filters and the WebKit text properties. The three original examples fail through both parsers, while a token-valued drop-shadow and an animation named red pass. The added tests cover the examples.

## Finding 3: [P2] Exclude CSS comments before inspecting declaration values
Recommendation: fix now
Was: 1-dvey.3
Response: fixed in dc327678
Now: fixed

bare() now removes comments. The original commented colour and spacing declarations pass in both .css and Svelte. CSS regression cases also cover comments containing an old raw font size.

## Finding 4: [P2] Reject fractional scaling of font-size tokens
Recommendation: fix now
Was: 1-dvey.4
Response: fixed in dc327678; any calc(), min(), max() or clamp() in a font size now fails, not just a fractional factor
Now: fixed

Font-size checking now rejects arithmetic/comparison functions. Both calc(var(--fs) * 1.25) and max(var(--fs), var(--fs-s)) fail through both parsers; direct token sizes still pass. Regression cases cover the fractional scaling and comparison.

## Finding 5: [P3] Compare spacing factors numerically before rejecting decimals
Recommendation: defer

app/eslint-rules/tokens.js:102 treats any decimal multiplier as fractional, including whole numbers written with a decimal point. padding: calc(var(--sp-2) * 2.0) and padding: calc(var(--sp-2) * 0.0) both fail tokens/spacing in .css and Svelte. Chromium computes these to 16px and 0px with the actual 8px --sp-2 token, the same values accepted when written with factors 2 and 0. This disagrees with the rule's documented allowance for whole factors and zero. Parse the multiplier and check whether its numeric value is an integer, with regression cases in both operand orders. Deferred because these spellings have no current use and the equivalent integer spelling works; it need not delay the enforcement fixes.
