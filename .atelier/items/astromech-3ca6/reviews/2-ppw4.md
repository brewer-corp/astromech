---
schema: atelier.review
schema_version: 3
round: 2
commit: e5ff9876ca960c64cdd96ed19b476c28bf355ba5
reviewers:
- thread:delegated-task:command%3Amcp%3A9c1e4e92-f0ca-4c55-a5dd-43940aad39d4%3Adelegate-task%3A635bcc6c-8f82-46ed-8864-9a11367a850f
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T08:53:13.703688207Z
---

## Summary

Complexity: medium. The shared row renderer and adapters change across several pages, and the hold figures cross the Rust/window boundary.
Value: high. Grouping results and qualifying demand against the searched commodity directly serve the requested finder workflow.
Risk of merging: medium. Incorrect grouping could hide or misrepresent facts across several user-facing pages; the change is reversible and writes no player data.

The value warrants the implementation. I reviewed the full `main...e5ff9876ca960c64cdd96ed19b476c28bf355ba5` change and the changes since round 1, the Done when lines, the adapters for each shared-row consumer, the inventory derivation and existing sellable-cargo rules, the guide and DESIGN.md, and the supplied captures at both sizes. The disclosed row-density reduction meets the requirement to measure and document density. The new icon slot also addresses the previous round's deferred alignment finding, although the older supplied captures predate that correction.

Verification passed: `pnpm test` (87 files, 719 tests), `pnpm lint`, `pnpm check`, `cargo test -p astromech-core ship_status_tells_an_unknown_hold_from_an_empty_one`, `cargo test -p astromech-gamestate -p astromech-market`, `cargo test -p astromech-app --test preview_fixtures`, and `python3 scripts/check-guide.py`. These cover the demand threshold edges, mixed and empty holds, unknown versus zero inventory, hold updates without another trade search, pad and stale marks, fresh ages on panel-less lists, and the new compact Colonisation regression.

I also ran headless Chromium against the preview at 1100 x 700, waited for the fonts, and inspected Colonisation's folded rows at 145, 160 and 180 percent Text size. The stock remains available through its full-text title, and the trip figures and report age are restored. The minor clipping below does not remove the distance or age figures in the recorded case and can follow separately. I did not rerun the full Rust workspace, clippy, dependency/security audits, production build or journeys, or test live Elite on Windows. The Report's suspicion of duplicate searches in other finders is not established for all the named pages: Services, Shipyard, Traders, Raw sites and Faction states assign primitive values, unlike Market's former object replacement.

## Finding 1: Keep stock and the trip reachable when Colonisation rows become compact
Recommendation: fix now
Was: 1-79uh.1
Response: fixed in 052c792f: a compact row in a list with no side panel folds its own group and the trip into a third line under the place; a browser test on the recorded Colonisation list at 1100 x 700 and 145 percent checks the stock, trip and age.
Now: fixed

Confirmed in `SearchKit.svelte:525-533`: compact lists without a detail panel now render their own facts and trip in the folded line. The new recorded Colonisation test passes, and the live preview at 1100 x 700 / 145 percent shows Efremov Plant's stock, 1 jump, 8.6 ly and report age. The original loss of those fields is fixed.

## Finding 2: Keep folded trip units and age suffixes from shrinking into their neighbours
Recommendation: defer

`app/src/lib/ui/SearchKit.svelte:579-580` makes the folded line a nonwrapping flex container and permits every child to shrink to zero. Its stock text correctly ellipsizes and has a full-text title, but the trip's unit, distance and age spans also shrink while their text overflows. On the recorded Hah Industrial Moulding row at 1100 x 700 / 145 percent, the folded line ends at x=1039.6 while the `42 d old` text extends to x=1045.2 and is clipped; at 180 percent it extends to x=1059.0 past the line's x=1035.4 edge, and the jump unit overlaps its following separator. This is visible after scrolling to that row. The new test checks only Efremov Plant's text content, so it cannot detect this geometry.

Reserve the trip's required width and let only the prose stock shrink, or give the trip another line when it cannot fit. This can follow as layout polish: the stock's complete title remains reachable, and the numeric distance and age are visible in this recorded case, so it does not reopen the earlier finding that those facts disappeared altogether.
