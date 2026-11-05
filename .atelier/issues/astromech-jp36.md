---
created_at: "2026-11-05T06:19:44.708270656+00:00"
id: "astromech-jp36"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-pey5"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-rz0f"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T08:26:59.816381188+00:00"
status: "done"
title: "Lookup result row: main value first, aligned columns, DESIGN.md states"
updated_at: "2026-11-05T08:26:59.816381188+00:00"
---

## Goal

Make the search kit's results the DESIGN.md Lookup result row, so every lookup (and every Places finder after it) shares one grammar: the ranked value first and largest, the place with its copy icon, fixed fact columns, then the actions. Today the ranked value is the smallest text in a 65 to 100 px row.

## Done when

- `SearchKit.svelte` rows are one line, 44 px: main value column (112 px, right-aligned, Value L, Orange Lit, unit in Orange Mid), place (name, system, copy), fixed right-aligned fact columns with Label heads over a Seam hairline (distance, jumps, pad for stations, arrival, report age, plus lookup-specific ones such as demand), then the actions from astromech-rz0f. A one-line constraint summary above the heads ("Within 100 ly of Sol · large pad · reports up to 30 days old").
- Each lookup names its main value and extra columns: price for Sell and Buy, jumps where nothing else ranks (Services, Outfitting, Shipyard, sites, Powerplay, colonisation), the HGE score for the High Grade Emissions lookups. Tabular figures for every number.
- States per DESIGN.md States: stale rows step down with the age in Orange; with an age filter set, stale rows are hidden and the summary names the limit (amended 5 Nov after review astromech-akom: sources apply the filter in the query, so a count of what they dropped is not available); Empty is one prose line plus one widening action; Loading is one Orange Mid line naming the source and keeps old rows Dim; errors are a red "!" with a recovery action.
- `SearchKit.test.ts` and the lookup tests cover the column set per lookup and each state; `pnpm lint && pnpm check && pnpm build && pnpm test` pass.
- `scripts/ui` screenshots of Trade Sell, Services and a sites lookup, before and after, compared with `docs/design/design-system-places-sell-after.png`.

## Scope

`lib/ui/SearchKit.svelte`, `lib/search.ts` (`Row`/`Results` gain what the columns need), each lookup's adapter. Pages keep their place; moving them into Places is the next child.

## Context

- DESIGN.md: Lookup result row, States, Content widths and rows, The Aligned Columns Rule; mockup `app/src/mockups/ds/Finder.svelte` (`#/mockup/ds-sell`).
- docs/ARCHITECTURE.md §18.2 Search kit.

## Notes

- 2026-11-05 [note] 2026-11-05: deferred finding astromech-jgyk from the rz0f review: FollowNote.svelte (app/src/lib/ui) shows failure text as Dim prose rather than DESIGN.md's hard-error state (red ! + words + recovery action), and its inline Resume overrides the 34 px hit area. Fold into this child's States work.
- 2026-11-05 [note] 2026-11-05: deferred findings from the 1d92 review for this child's States work: astromech-w52v (Strip.svelte's hard error has no recovery action; DESIGN.md's example is '! Could not read the route. TRY AGAIN') and astromech-aeub (route/View.svelte's .err is a filled orange block, which DESIGN forbids; make it the States table's hard-error line).

## Close Reason

Implementer report. SearchKit draws DESIGN.md's Lookup result row: the ranked value first in Value L (price, total, HGE score, "N of M" sold, else jumps), the place with its copy icon, the lookup's own columns, then distance, jumps, pad, arrival and report age under Label heads over a Seam hairline, the rz0f actions, and a one-line constraint summary; tabular figures. search.ts Results/Row gained place, main, columns and facts (replacing lead/detail), filled by every adapter; columns per lookup in the jp36 handoff (Sell/Buy price + demand/stock, My cargo total + takes, Services/Outfitting/Shipyard jumps + faction (+ module, price), sites jumps + site, faction states, HGE score, Powerplay, colonisation). States: stale rows step down with the age in Orange; with an age filter rows past it are hidden and the summary names the limit, counting only rows that came back past it; empty is one prose line plus one widening action that never relaxes the pad; loading is an Orange Mid line naming the source over Dim rows; failures use the new shared Failure.svelte (red "!" + TRY AGAIN repeating the action), which also closes folded findings jgyk (FollowNote), w52v (route strip) and aeub (Route page's filled block). Narrow side panels get a compact row (value, place, actions). Deviations: the Done when's hidden-row count was amended (review akom: sources apply the age filter in the query, so no count of dropped reports exists); staleness is judged as of the answer's fetch time; Colonisation's main column widened to the 1200 px list width; the kit resets inherited caps. DESIGN.md now records text-column truncation, the compact row and the age-filter rule; ARCHITECTURE.md and the guide updated. Proof: 34 new or changed tests fail without the change; app lint, svelte-check 0/0, build, 427 tests; check-guide 0 problems; no overflow at 1100/1600 px on every page hosting a lookup; screenshots docs/design/lookup-row-*.png. Review: reviewer-sonnet-jp36; fix_now akom, qwzi (unused Allegiance.svelte removed), nnj6 fixed in 509d84e3; a400 deferred to astromech-rfn8; d8w6, wxh1 no action; approved astromech-ai0k. Reviewer decisions (reversible): no backend stale count; keep the Faction column as plain text; keep the compact panel row; let text columns shrink, later omit identical ones (rfn8); fetch-time staleness; record it in DESIGN.md now.
