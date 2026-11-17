---
schema: atelier.review
schema_version: 3
round: 1
commit: 47e895e3533705b5fe6ec1302e509e1e3dd3575b
reviewers:
- thread:delegated-task:command%3Amcp%3A73fa4ff9-bf52-41f5-aec3-fbb1a24c241c%3Adelegate-task%3A2a979d51-644e-4890-9b4e-8ca288c7b09f
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T11:18:02.365913412Z
---

## Summary

Complexity: medium. The change coordinates the Rust reply, generated TypeScript, three page components, a shared Meter and the preview/audit tools.

Value: high. The rank, cycle and system answers are the purpose of this page, and the docked-sidebar clipping affected normal use.

Risk of merging: medium. The changes reach the live page and can misstate or hide system information, but are reversible without a data migration.

The value warrants the scope. I found no substantially smaller change that delivers the Board and its three main answers. Two correctness issues need fixing before merge; the narrow not-pledged spacing issue can follow separately.

I reviewed `main...47e895e3533705b5fe6ec1302e509e1e3dd3575b`, the Done when lines, DESIGN.md, the guide, the saved plan/report and representative captures against the reference. `NextRank.from` uses the same rank rules as the next threshold; its standing cases and regenerated fixture agree. The type roles, meters, suggestions, source Tip and lint enrolment are present.

Independent checks passed:

- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p astromech-powerplay`, `cargo test -p astromech-core --lib` (270 tests) and `cargo test -p astromech-app --test preview_fixtures`.
- `pnpm check`, `pnpm lint`, `pnpm test` (766 tests), the seven journeys and `python3 scripts/check-guide.py`.
- The checkpoint audit, using a copy of `docs/design/increment2/plan.json` to protect the before captures, has no entries under any of the six fault lists at its four sizes. A separate Acquire/Reinforce audit also passes at the item's sizes and nearby column/docking breakpoints. Its 1100-wide, 180-percent probe reports document overflow and a status-bar ellipsis outside the changed page.

I reproduced the findings in local headless Chromium. For the first two I changed only the browser's in-memory fixture objects, leaving the checked-in recordings intact. For the third I used the existing `core=empty` preview. The review did not exercise Windows WebView2 or the live game, and did not rerun the entire Rust workspace suite. No implementation files were changed.

## Finding 1: Preserve unknown reinforcement and undermining instead of declaring a tie
Recommendation: fix now

`app/src/modules/powerplay/View.svelte:43` and `:44` replace missing reinforcement/undermining with zero. The bar and lead sentence at `:127` and `:134` then assert an outcome even though the journal did not supply the figures. These fields are nullable in the reply, and `SystemPower::new` preserves their absence; the core's own summary/spoken methods deliberately require both before comparing them.

Reproduction: open the ready Powerplay preview, import `/src/preview/fixtures/powerplay.json?import` in the browser and set `system.reinforcement` and `system.undermining` to null. After the next poll, both printed figures correctly show a dash, but the bar shows a 50/50 split and the page says **Level this cycle**. With only one missing, it instead invents a lead against zero.

Keep unknown values distinct from known zero. Only calculate a split and lead when both figures are known, and cover missing figures as a boundary fault in a behaviour test. A true 0/0 tie should retain the current handling.

## Finding 2: The fixed target track still cuts ordinary longer state lines
Recommendation: fix now

`app/src/modules/powerplay/View.svelte:207` and `:215` fix Where to work at 45.5rem. Moving reinforcement/undermining into their own group in `Targets.svelte:28` solves the recorded examples, but leaves the place/state group only 312 px wide on Reinforce. `targetDetail` still appends every other active power to that single ellipsized line. SearchKit's minimum full-row width guarantees that the groups fit, not that their text fits.

At 1920 x 1080, 100-percent Text size, with the route docked, keep Lembava's recorded figures and set only its `others` array to `["Li Yong-Rui", "Felicia Winters", "Zemina Torval"]`. Select Reinforce. The resulting state is `Li Yong-Rui stronghold 25% · also Felicia Winters, Zemina Torval`: its rendered `.place .text` has `clientWidth=312`, `scrollWidth=335` and `text-overflow: ellipsis`, and the capture ends the line at `Zemina ...`.

This confirms the Report's known limit, but it also leaves the first Done when requirement unresolved for a normal system with two other active powers. Make the grouping accommodate the complete state/active-power information rather than using the fixture's length as its width limit. Preserve the already-clean recorded cases.

## Finding 3: Remove empty standing rows from the narrow not-pledged grid
Recommendation: defer

`app/src/modules/powerplay/View.svelte:203` always defines rank, cycle and merits grid rows, although those sections are absent when not pledged. The `.blocks:not(.standing)` overrides exist only inside the two wider container queries. Below 64rem, the three empty rows remain, with a gap after each.

Reproduction with no modified data: open `?core=empty&route=exact&textsize=115#/powerplay` at 1200 x 800, skip setup and select Powerplay. The computed row tracks begin `0px 0px 0px`, with a 36.8 px gap; This system starts 110.4 physical pixels below the grid's top. The resulting blank band pushes useful blocks down the narrow window. Give the narrow not-pledged grid only the areas it renders. This is a visible spacing defect, but it does not hide information and can follow separately.
