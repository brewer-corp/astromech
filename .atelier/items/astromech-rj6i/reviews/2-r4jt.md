---
schema: atelier.review
schema_version: 3
round: 2
commit: a4c6c286f92e9edda7b973a5be26eef4393c4e72
reviewers:
- thread:delegated-task:command%3Amcp%3A73fa4ff9-bf52-41f5-aec3-fbb1a24c241c%3Adelegate-task%3A20db8163-4132-4e14-9e7a-f4b245fa4c87
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T11:28:55.799862355Z
---

## Summary

Complexity: medium. The change coordinates the Rust reply, generated TypeScript, three page components, a shared Meter, and preview/audit tools.

Value: high. The rank, cycle and current-system answers are the purpose of the page; docked-sidebar clipping affected normal use.

Risk of merging: medium. These changes reach the live page and shared search rows, but can be reverted without changing stored data.

The value warrants the scope. I found no substantially smaller change that delivers the Board and its three answers. Both carried findings are fixed; there are no new findings.

I reviewed `main...a4c6c286f92e9edda7b973a5be26eef4393c4e72` and the round-2 delta against every Done when line, DESIGN.md, the reference image, guide, saved audit plan/report and representative before/after captures. The rank start uses the same rank rules as the next threshold and has standing coverage; the generated binding and recorded fixture agree. The Board, type roles, meter bounds, suggestions and Tips, empty/loading/error states, and page-sizing/token lint enrolment are present.

Independent checks passed:

- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p astromech-powerplay`, `cargo test -p astromech-core --lib` (270 tests), and `cargo test -p astromech-app --test preview_fixtures`.
- `pnpm check`, `pnpm lint`, journey typechecking (`pnpm exec tsc -p journeys`), all seven journeys (`node --test 'journeys/*.journey.ts'` from `app/`), and `python3 scripts/check-guide.py`.
- The page audit's 22 captures and the checkpoint's four Powerplay captures all have empty `sideways`, `clipped`, `overlaps`, `small`, `contrast` and `far` lists. I ran copies of both plans with output under `target/review-rj6i/` to preserve the checked-in captures. The 1920 and 3840 captures show complete system names and states. The main rank value and chosen work option retain Orange Lit; secondary figures are grey. The Report calls the row actions orange, but the shared icon buttons actually rest in Label Grey, as DESIGN.md specifies.

The first `pnpm test` run passed 766 tests and timed out in the unchanged Outfitting partial-route case (5-second limit). Another worktree was running a full Vitest suite concurrently. The isolated Outfitting file then passed all nine tests in 228 ms without changing its timeout or code. The final plain `pnpm test` run passed all 767 tests in 91 files (20.32 s); its output is saved under `target/review-rj6i/final-tests.log`. No code or timeout was changed.

Browser probes changed only in-memory fixtures. They checked both missing cycle figures, each missing independently, a known 0/0 tie and a known lead; longer Reinforce/Undermine states with all bundled powers active; and not pledged with the ready preview's route still followed at 1100/100%, 1920/100%, 2560/145% and 3840/180%. State text fits at each size and its Tip retains the other powers' names. The grid starts at its first rendered block. The narrow not-pledged empty-area override also fixes the earlier deferred spacing issue.

I did not exercise the live game or Windows WebView2, and did not run the entire Rust workspace test suite. No implementation files were changed.

## Finding 1: Preserve unknown reinforcement and undermining instead of declaring a tie
Recommendation: fix now
Was: 1-fhom.1
Response: fixed in f936f991
Now: fixed

`View.svelte` now returns no comparison when either figure is missing and renders neither the split bar nor the lead sentence. The new missing-figure browser case passes. Independent probes confirm null/null, null/9 and 0/null produce no result, while 0/0 retains the split and “Level this cycle”, and 20/9 reports reinforcement ahead by 11.

## Finding 2: The fixed target track still cuts ordinary longer state lines
Recommendation: fix now
Was: 1-fhom.2
Response: fixed in f936f991
Now: fixed

`targetDetail` omits the controlling power when it is the pledged power and counts other powers; `targetOthers`, the row's `notes`, and SearchKit's Tip preserve their names. Reinforcement and undermining remain their own figures group. The updated target behaviour case passes. In independent probes, “Arissa Lavigny-Duval stronghold 100% · 10 other powers” fits without ellipsis at every required size, including mixed rows that prevent SearchKit from lifting a shared state into its summary. Its Tip names all ten other powers. All recorded cases remain clean in both audits.
