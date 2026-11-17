---
schema: atelier.review
schema_version: 3
round: 1
commit: 9b1ae4495558ff5427a4cd2c07514ae1bfb1e982
reviewers:
- thread:delegated-task:command%3Amcp%3A40e7bd6a-75d7-481f-bc07-e18b62a688e1%3Adelegate-task%3A504ae37f-cfd4-4d9a-b5ca-f111997e140d
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T05:23:57.065121066Z
---

## Summary

Complexity: high. The amendment changes the frame, navigation, sizing and colour contracts across every page, and adds engineer-search logic.

Value: high. It addresses the owner's stated problems on pages in daily use and follows the recorded feedback at the increment 1 stop.

Risk of merging it: high. The visual changes are reversible, but the new grade and market-capacity recommendations can give silently wrong advice, and text-size changes must handle existing settings.

The value warrants the work. I found no substantially smaller substitute that delivers the approved sidebar, navigation and hierarchy changes. The incremental implementation and independent checkpoint are appropriate. The four gaps below should be settled before increment 2 starts; the later screen items can remain drafts for subsequent planning.

Reviewed the mission, design, notes and all its item specifications at commit 9b1ae4495558ff5427a4cd2c07514ae1bfb1e982 against README.md, CONTRIBUTING.md, DESIGN.md and the current frame, route UI, settings/configuration, lookup adapters and cargo model. Checked the nearest-step calculation directly with Node. This is a plan review: I did not run the app or the implementation test suite, inspect the hosted artifact links, or verify Windows/WebView2 behaviour. No implementation exists for these amended items yet.

## Finding 1: Keep Import reachable before a route exists
Recommendation: fix now

`.atelier/items/astromech-uc7i/item.md:29-35` renders the full sidebar only while following a route, replaces the empty state with a rail whose sole action focuses Find or plot, and removes the Route page. That leaves no specified way to import the first route. Today Import is in `route/Title.svelte`'s menu, available even with no active route; `route/View.svelte` renders that title in the empty state. Find or plot (`route/PlotBar.svelte` and `route/Plot.svelte`) offers plotting and following systems, but no Import action.

Specify an entry point for Import when there is no active route, including below the dock width. The rail can stay narrow; its action could open route tools, or Import can live in another reachable menu. Add an empty-state import journey to the item's acceptance criteria so preserving the feature does not depend on first plotting a disposable route. The mission explicitly excludes removing existing features.

## Finding 2: Define migration and the unset state for Text size
Recommendation: fix now

`.atelier/items/astromech-b2cz/item.md:30-31` replaces three persisted values with six absolute percentages and initializes only when there is no saved choice, but gives no migration rule for existing choices. The current config stores `smaller`, `normal` or `larger` (`crates/astromech-core/src/config.rs:136-157`), and `window_settings` returns `normal` even when the field is absent (`app.rs:1236-1237`). Those values multiply a height-dependent zoom; they are not absolute percentages. In particular, simply treating saved `normal` as 100 percent shrinks an existing 1440p installation from about 144 percent to 100 percent. Replacing the enum without accepting its old strings instead makes `Config::parse_report` drop the value as invalid.

State how each legacy value migrates, how an absent setting is distinguished from an explicit new choice, and which layer persists the one-time result. Require configuration upgrade cases for all three old strings and the missing field, plus a restart that preserves the migrated/new choice. This can remain one setting change; it needs a compatibility contract rather than another item.

## Finding 3: Make the first-run formula agree with the 1080p default
Recommendation: fix now

`.atelier/items/astromech-b2cz/item.md:31` selects the step nearest display height / 1000. For 1080 that is 1.08: 115 percent is closer than 100 percent (0.07 versus 0.08). The mission's fills-the-window criterion (`mission.md:70-73`) instead says a 1920 x 1080 display starts at 100 percent, and b2cz and the increment 2 checkpoint mandate captures at that value. The prescribed implementation and its validation therefore describe different initial layouts.

Choose explicit default thresholds that produce the intended 100/145/180 percent results, or amend the expected defaults and capture matrix consistently. The existing requested 1080/1440/2160 test should assert the selected values, not just exercise the formula.

## Finding 4: Use the selected commodity's cargo for Sell demand, and separate Buy's empty-hold rule
Recommendation: fix now

`.atelier/items/astromech-3ca6/item.md:30` compares Sell demand with "the cargo aboard" and "the hold", without restricting it to the commodity being sold, and says bars have no warning colour when no cargo is aboard. A station's demand is per commodity (`crates/astromech-market/src/trade.rs:91`); the existing cargo model explicitly compares carried tonnes per commodity (`cargo.rs:5-9`). The recorded preview already carries 300 t of diamonds, 200 t of painite and 100 t of platinum, plus kept goods (`app/src/preview/fixtures/trade.json:515`). A 250 t painite buyer can take all 200 t; comparing it with the whole hold would incorrectly label it short. The general no-cargo exception also suppresses Buy shortages precisely when an empty hold has the most free space.

Specify Sell's baseline as the sellable quantity of the selected commodity, using the existing cargo eligibility rules, and limit its zero-quantity exception to Sell. Buy should compare supply with known free space even with an empty hold. Distinguish unknown cargo/capacity from a known zero. Name the reply supplying these quantities: `ship_status` has totals only, and `trade_search` has no inventory. Protect the contract with a mixed-hold case and an empty-hold Buy case, in addition to the numeric threshold-edge tests already planned.
