---
schema: atelier.review
schema_version: 3
round: 5
commit: 1879238dfd927276ab81230297c35800b3eb4e13
reviewers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A2d97da16-d743-4997-8ba4-3b139a1f1e24
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T10:36:09.609704349Z
---


## Summary

Approve the plan at `1879238dfd927276ab81230297c35800b3eb4e13`. All seven carried findings are fixed; I found no new problem requiring a plan change.

- Complexity: high. The remaining work changes several page bodies and adds loadout identity, per-item picks and reply fields across the Rust/window boundary.
- Value: high. The owner approved the direction for pages in current use, and the work completes the mission's navigation and readability outcome.
- Risk of merging the planned work: high. Incorrect item ownership or a stale cargo answer can silently misstate costs or recommend the wrong destination; the revised plan requires focused tests for those contracts, alongside per-page captures and independent final checks.

The value warrants the work. Separate screen items, shared layouts and the existing checkpoint tool keep the changes reviewable and reversible; shrinking this to colours and spacing would leave the loadout and live-home outcomes unfulfilled.

I read the mission, design, notes and item briefs, checked the seven carried findings against their amendments, and checked the mission dependency graph for cycles and missing items (none). I cross-checked the affected contracts against the owned-gear reader, shopping calculation, generated reply types, cargo calculation, finder formatter and preview fixture handlers. I also verified that the audit script and capture plan are tracked at this commit and that the script accepts the required flags and emits the required report keys.

This is a plan review. I did not run application tests, recapture the UI or verify the unbuilt screens in WebView2. The per-page implementation checks, final sizing captures and blind glance test remain acceptance work, not results established by this review.

## Finding 1: Let 8e22 close with the checkpoint failures already handed to later items
Recommendation: fix now
Was: 4-ayza.1
Now: fixed

`astromech-8e22/item.md:37` now explicitly allows the three named screen handoffs and the older standalone bug. Its Report discloses each disposition. The dropped `lf3a`, `8og8` and `5g9w` Scope paragraphs no longer require the checkpoint to wait for them; the receiving screens retain the fixes and their measurements as Done when lines.

## Finding 2: Make landing the audit tool a Done when of 8e22
Recommendation: fix now
Was: 4-ayza.2
Now: fixed

The prerequisite has been delivered: `scripts/browser/audit.mjs` and `docs/design/increment2/plan.json` are tracked at this commit, brought in by the checkpoint merge `faeb2878`. The script supports `--base`, `--plan` and `--only` and reports `sideways`, `clipped`, `overlaps`, `small`, `far`, `contrast` and `bright`. `design.md:208` now identifies it as landed. A further acceptance-line edit is unnecessary once the required tool is on the branch the screens inherit.

## Finding 3: Decide whether wo8s is in the mission
Recommendation: fix now
Was: 4-ayza.3
Now: fixed

`astromech-wo8s/item.md` now has `mission: null`; its Scope explicitly makes it a standalone Controls bug. `design.md:192` agrees. The mission's Controls exclusion is preserved.

## Finding 4: Reconcile the checkpoint gate with the three fixes assigned after it
Recommendation: fix now
Was: 4-pi08.1
Now: fixed

The checkpoint's second Done when line permits the named handoffs, its Report records them, and `design.md:186` describes the completed checkpoint and the receiving screens. The screen items retain the original cases and final sizing checks. This removes the dependency deadlock described by the finding.

## Finding 5: A suit pick cannot be treated as a plan for every compatible owned item
Recommendation: fix now
Was: 4-pi08.2
Now: fixed

`astromech-w1lj/item.md:27` carries journal identity through owned gear and loadout slots; line 28 adds optional `Pick.item`, preserves old unassigned picks and requires once-per-pick costing tests. `astromech-nnn0/item.md:30` displays plans only for the matching item, and line 31 includes item identity in pick keys while leaving old picks explicitly unassigned until chosen. The fixture and acceptance cases cover two compatible items and one item shared by loadouts. The amendment addresses both the false row implication and multiplicity.

## Finding 6: Refresh Now's cargo answer when its origin or sellable hold changes
Recommendation: fix now
Was: 4-pi08.3
Now: fixed

`astromech-coq6/item.md:37` now defines a change key over the per-commodity sellable hold, system, ship id, pad and jump range. It requires re-asking on changes and no request for an unchanged key. Line 38 dims the old answer and disables its destination actions during refresh. The pure-function cases include changed cargo at equal tonnage and changed origin; the browser case exercises a jump without a cargo change.

## Finding 7: Identify a partial cargo sale beside Now's recommended total
Recommendation: fix now
Was: 4-pi08.4
Now: fixed

`astromech-coq6/item.md:32` always qualifies the total with the tonnes taken against the sellable load, beside that total. It shares the qualifier with the finder, tests whole and partial loads including a mixed hold, and explicitly omits recommendation actions for an empty station list. This matches the existing `StationSale.sold` and `CargoPlan.load` contract.
