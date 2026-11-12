---
schema: atelier.review
schema_version: 3
round: 3
commit: 0728fa7ff8bf3d0af750fd9cb468e07a254c6c3d
reviewers:
- thread:delegated-task:command%3Amcp%3A685172b7-45c1-415c-bdbf-c75790f5874c%3Adelegate-task%3A3cce90a0-797f-4655-8049-1db283a9a978
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T09:51:44.066938948Z
---

## Summary

- Complexity: medium. A shared attachment owns positioning, dismissal, descriptions and focus across many components.
- Value: medium. Instant explanations improve controls and values used throughout the current app.
- Risk of merging: medium. Incorrect positioning hides explanations across the window, but the failure is visible and reversible.

The value warrants the implementation's size. I reviewed the full change at `0728fa7ff8bf3d0af750fd9cb468e07a254c6c3d`, the merge since round 2, each Done when line, the Report, all remaining native title attributes, the design and guide updates, and both evidence captures. The earlier fixes remain present. The title migration, title guard, keyboard behavior and documentation meet their claims, but tooltip placement does not account for the root zoom newly merged from main.

Validation passed: `pnpm lint`, `pnpm check`, all three Tip node tests, all 55 browser tests across Tip, SearchFilters, Engineering Build and View, Colonisation, Station and Powerplay, and `python3 scripts/check-guide.py`. The browser test port 63315 was occupied; I ran the browser project's API on 63419 through a temporary in-memory configuration override. An initial configuration attempt did not load the Svelte plugin; the corrected run passed. No project configuration was edited.

I also ran the actual Vite app in headless Chromium at 1200 x 800 and 2560 x 1440 with `?core=ready#/engineering`, hovered Help and a jump value, and measured their drawn rectangles. A separate Chromium probe using the actual Tip implementation and tokens.css reproduced the coordinate error at zoom 1.44, 0.85 and 1.656, while zoom 1 positioned correctly.

I did not repeat the full Vitest suite, production build, Rust checks or journeys, test Windows WebView2 or use a physical screen reader. Accessible descriptions were checked through the Chromium component tests. Only this review file was edited.

## Finding 1: Convert Tip placement to the root zoom's coordinate system
Recommendation: fix now

`app/src/lib/ui/tip.ts:71-84` mixes the viewport coordinates from `getBoundingClientRect()` with unzoomed `offsetWidth`/`offsetHeight`, then writes those coordinates directly to `left`, `top` and `--tip-x`. The fixed tooltip inherits the root CSS zoom set by `startScale()`, so its position is scaled again. The `.tip` viewport-width calculation at `app/src/design/tokens.css:107` also bypasses the new window-size tokens.

This breaks the current app's default scale, not just browser zoom. In the actual app at 2560 x 1440, Normal sets root zoom to 1.44. Hovering Help puts its target at x=2467..2502, y=31..60, but the tooltip at x=3308..3405, entirely outside the 2560 px window. Hovering the first jump value puts the value at x=1295..1307, y=784..815, but its tooltip at x=1761..1987, y=1074..1117. At 1200 x 800 and zoom 1, Help's tooltip stays inside the window as expected. Thus the merged change violates DESIGN.md's placement rule and the Report's claim that explanations remain beside their targets inside the window.

Calculate target coordinates, tooltip size, viewport bounds and pointer position in one coordinate system that accounts for the effective zoom, and use the scaled window width for the CSS clamp. Add a browser regression using the app's non-unit root zoom; the existing five Tip cases run without it and all pass despite this failure.
