---
schema: atelier.review
schema_version: 3
round: 2
commit: ed17844107b35fca3eed394ee6b9ef1dae3e5e43
reviewers:
- thread:delegated-task:command%3Amcp%3Af7ab762d-7bc6-4bdd-9234-1ace62c91bb6%3Adelegate-task%3A4a2cd529-944f-48d9-84a4-0f90ff13800d
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T12:04:23.295396484Z
---

## Summary

Complexity: high. The change extends the Rust/TypeScript node contract and adds tree placement, measured connecting lines and a detail panel across several components.

Value: high. Seeing both unlock chains and the selected engineer's requirements and offers is the page's main purpose.

Risk of merging: medium. The page changes route actions and user-facing progress information, but failures are visible and the changes are reversible.

The value justifies the scope. I found no substantially smaller implementation that delivers the requested tree and panel. One route-action race needs fixing before merge.

Reviewed `main...ed17844107b35fca3eed394ee6b9ef1dae3e5e43`, the changes since the previous reviewed commit, the merge conflict resolutions, each Done when line and the Report. Read the core offers, labels and step generation, generated bindings, recorded fixtures, tree placement and lines, selection, distances and engineer-name joins, panel actions, search restoration, sizing rules and guide. Both conflict resolutions preserve the relevant changes from each side. The Report's requirement resolutions remain reasonable: suit modifications belong to the core node, numbered steps avoid parsing prose, and grade colours follow DESIGN.md.

Independently passed on the merged code:

- `cargo fmt --all --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo test -p astromech-engineering -p astromech-api`: 48 engineering and 41 API unit tests, both engineering integration tests and doc tests, including the generated bindings check.
- `cargo test -p astromech-app --test preview_fixtures`: all three tests, including fixture drift.
- `pnpm check`, `pnpm lint` and `pnpm test`: 847 tests in 95 files.
- `cargo build -p astromech-app --bin astromech-core`, then `node --test 'journeys/*.journey.ts'`: all seven tests pass.
- `python3 scripts/check-guide.py`: 23 guide pages, no problems.

Inspected saved after captures, the target mockup and the saved audit report. Independently selected all 38 engineers in headless Chromium on the merged code, with the recorded preview and followed route, at 1100 x 700 / 100 percent, 1920 x 1080 / 100 percent and 2560 x 1440 / 145 percent. Ran the audit tool's actual measurement function after each selection: all 114 selections have empty `sideways`, `clipped`, `overlaps`, `small`, `contrast` and `far` lists outside its exemptions. No panel descendant overflows the panel's width and no page errors were raised.

Separately held a route request at the transport boundary and released its failure after changing selection; this reproduces Finding 1. No production source or test file was changed. I did not exercise the Windows/Tauri window or a live game journal.

## Finding 1: A route retry changes engineers if selection changes while the request is pending
Recommendation: fix now

`app/src/modules/unlocks/Panel.svelte:48-65` clears `note` when `node.name` changes, but an earlier `go()` or `add()` can finish afterward and write its result into the newly selected engineer's panel. Its failure callback calls `go()` or `add()` again using the current reactive `node`, so TRY AGAIN can issue a different route action from the one that failed.

Reproduced deterministically in Chromium using the real page and preview transport: select The Dweller, hold the `route_set` reply after pressing Set destination, select Felicity Farseer, then reject the held request. The failure appears under Felicity's details. Pressing TRY AGAIN sends a second `route_set` with `name: "Deciat"` and a Felicity waypoint, whereas the failed request had `name: "Wyrd"` and a Dweller waypoint. A late success likewise appears under the wrong engineer.

Capture the action's engineer identity and arguments, and prevent its completion from populating another selection's panel. A retry must retain the original action's target. Protect the behaviour with a transport-held response case; clearing the note on selection alone does not address the pending completion.

## Finding 2: The empty preview still misrepresents a commander with no engineer progress
Recommendation: defer

`app/src/preview/shell.ts:417-420` marks every recorded engineer locked and non-actionable for `?core=empty`, while retaining the progressed fixture's `left` and `steps`. The production core correctly marks common-knowledge engineers known and actionable, as its new no-progress test verifies. Previously unlocked sample engineers also retain empty steps despite becoming locked in the preview.

This predates the change, was identified in round 1 and is acknowledged in the Report. It does not block production behaviour, but the preview cannot validate the real no-progress state. Follow up with a genuine empty-world unlock fixture instead of changing only status, rank and next on the progressed reply.
