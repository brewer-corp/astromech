---
schema: atelier.item
schema_version: 3
id: astromech-3ca6
title: 'Finder results read as results: grouped two-line rows, demand against your hold'
status: building
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-3kf5
implementers:
- thread:delegated-task:command%3Amcp%3Affec3634-009b-40bd-8764-e19cb778e43b%3Adelegate-task%3A98f91929-7d51-4fb1-9847-d5cbef3b3ffd
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.425029229Z
---

## Goal

A finder result reads as a result, not a spreadsheet row with a column per field. Sumeet, 17 Nov 2026: "table carcinisation again"; price belongs with demand, and a station with its distance from the star. Demand is coloured by whether it will take the commander's cargo.

## Done when

- [x] The shared lookup result row is two lines in three groups, as `tone5-sell-2560.png`: the ranked value over what qualifies it; the place (station, pad, arrival distance) over its system with copy; the trip (jumps over light years). Row actions stay at the row's end. Facts the commander does not choose a row by, such as Updated, move to the side panel where the list has one. A list with no side panel (Powerplay's `Targets.svelte` and Colonisation's `View.svelte` use SearchKit outside List with detail) keeps them on the row: the report's age stays in the trip group at any age, grey when fresh and Orange past 30 days. No fact a row shows today becomes unreachable on any consumer of the shared row.
- [x] Sell and Buy show demand or supply under the price with the market's three bars and DESIGN.md's colours, measured for Sell against the tonnes of the searched commodity aboard that are the commander's to sell, by the rules `crates/astromech-market/src/cargo.rs` already applies (mission cargo and stolen goods are left out): short below that figure, tight below five times it, plenty above. For Buy, supply is measured against the free cargo space, including with an empty hold. The page states the figure it measures against ("200 t of Painite aboard", "136 t free"). Sell with none of the commodity aboard, and either finder when the cargo or capacity is not known (Elite not running), shows the bars with no warning colour; a known zero and an unknown are told apart. The figures come from `ship_status`, which gains the sellable tonnes per commodity and the free space; `trade_search` carries no inventory. A Tip on a short row says what happens. The thresholds are constants with a test at each edge, plus a mixed hold (300 t of one commodity, 200 t of the searched one: a 250 t buyer is not short) and Buy with an empty hold, on the recorded preview world.
- [x] A pad too small for the current ship is marked and struck through; a report older than 30 days shows its age in the trip group in Orange. Neither relies on colour alone.
- [x] Every Places finder uses the grouped row with groups that fit its job (Outfitting, Shipyard, Services, Traders, Mining hotspots, Raw sites, Faction states, Cargo); none keeps a column per field. A text cell that is the same on every row is left out and stated once in the summary line (this takes in astromech-rfn8).
- [x] Rows visible are measured at 1100 x 700 and 2560 x 1440 and DESIGN.md's figure updated. Browser tests on recorded markets cover the three demand states, the pad mark and the stale mark. Captures of Sell, Outfitting, Mining hotspots, Powerplay's targets and Colonisation's where-to-buy list at both sizes; a browser test shows a 12-day-old report's age on a panel-less list. The Places guide page describes the row.

## Scope

The result row and the Places finders. The Engineers search results are astromech-7did.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). The demand thresholds and the list of coloured things are in the spike's final report and in DESIGN.md revision 5.

## Report

Finder results now read as results. Every consumer of the shared row (the ten Places finders,
Powerplay's targets, Colonisation's where-to-buy and candidates, Nearby) draws DESIGN.md's
grouped row: the ranked value over what qualifies it, the place with its pad and arrival over its
system and copy icon, the finder's own group where it has one, the trip (jumps over light years),
then the actions. Rows are 60 px (`--row-result` 3.75rem). Sell and Buy colour demand or stock
against the hold. Captures before and after: `docs/design/grouped-rows/`.

**One objection, reflected in DESIGN.md.** DESIGN.md's figure promised about 5 whole rows at
1100 x 700 and about 11 at 2560 x 1440, from the spike. Measured on the built row (Sell anywhere,
preview, no route sidebar), the page opens with 2 whole rows at 1100 x 700 and 8 at 2560 x 1440.
Scrolled so the list starts at the top of the page column, 7 and 13 fit. The old one-line rows
gave 3 and 11 on opening, 10 and 17 scrolled. The gap to the spike is the origin box and range
choice above the list, which the spike's mockup did not draw. DESIGN.md now states the measured
figures. If 2 rows on opening at 1100 x 700 is too few, the fix is a shorter query band, not a
shorter row.

What changed:

- **`ship_status`** gains `sellable` (tonnes per commodity by `cargo::sellable`, so mission
  cargo, stolen goods and limpets are left out; null until a Cargo.json is read, via a new
  `GameState.cargo_known`), `cargo_free` (capacity less everything aboard, 0 when full), and
  `pad` (`Pad::for_ship` on the Loadout's ship, as the voice lookups resolve it). `trade_search`
  is unchanged. The run function never reads the hold, so a hold change does not re-ask Ardent:
  SearchKit asks `ship_status` with each search and the colour is worked out at render.
- **Recorded worlds.** The ship world's 120 t gains a Cargo.json (100 t Painite, 20 t Low
  Temperature Diamonds). The sol world, which the trade samples come from, now starts Elite and
  is recorded with its mixed hold (`trade.json` `/ship`: 300 t LTD, 200 t Painite, 100 t
  Platinum, 0 free since its Cargo.json says 652 t in a 200 t Asp) and once emptied
  (`/ship_emptied`: 200 t free). These are the last parts of trade.json, because the fixture
  recorder steps each world forward only once.
- **Row model** (`search.ts` `groups`): a column says where it sits (`at: "qual" | "row"`, else
  the side panel). Without a side panel, panel facts stay on the row and the report's age stays
  in the trip group at any age ("10.3 ly · 12 d old"), grey until 30 days. A prose fact the same
  on every row (two or more) leaves the row and joins the summary line (takes in astromech-rfn8).
  DESIGN.md's Lookup result row now has the table of each finder's groups.
- **Demand** (`demand.ts`): `TIGHT_TIMES = 5`. Short is Red Low with one bar and a Tip, tight is
  Orange Mid with two, plenty is Prose Grey with three. None aboard, a full hold, or not known
  give three grey bars, and the page states which ("No Painite aboard", "Hold full", "Cargo not
  known: Elite isn't running"). The bars' accessible name carries the words.
- **Pad and stale.** A pad smaller than `ship_status.pad` is Red Low, struck through, named
  "Medium pad: too small for your ship". A report older than 30 days as of the fetch puts "34 d
  old" in Orange in the trip group and steps the ranked value to Orange Mid. This now applies
  whatever the age filter is; before, rows were only marked with the filter at "Any". DESIGN.md's
  stale definition changed to match the item.
- **Compact rows** stay one line only inside a list with a side panel. Powerplay's narrow Where
  to work column used to go compact and lose the power state, distance and age with no panel to
  hold them. It now keeps its first two groups' second lines. That fix is its own commit.

No fact became unreachable. Before, then after, per consumer:

| Consumer | Before (one line, a column each) | After |
|---|---|---|
| Sell, Buy | price, station, system, demand, ly, jumps, pad, arrival, age | price over demand; station, pad, arrival over system; jumps over ly. Panel: age (and all the rest) |
| Sell my cargo | total, station, system, takes, ly, jumps, pad, arrival, age | total over takes; place; trip. Panel: age |
| Outfitting, Shipyard | jumps, station, system, module, price, faction, ly, pad, arrival, age | jumps over ly; place; module over price (module to the summary when every row shares it). Panel: faction, age |
| Services, Material traders | jumps, station, system, faction, ly, pad, arrival, age | jumps over ly; place. Panel: faction, age |
| Mining hotspots | hotspots, ring, system, also, type, reserve, ly, jumps, arrival | hotspots; ring, arrival over system; type over reserve (type to the summary when shared); trip. Panel: also |
| Raw sites | jumps, body, system, site, ly, arrival, age | jumps over ly; body, arrival over system; site. Panel: age |
| Faction states | score, system, population, faction, ly, jumps, age | score over population (or jumps over ly); system over faction; trip. Panel: age |
| Powerplay targets, Colonies, Nearby (no panel) | jumps or score, system, fact, ly, age | first group over ly and age; system over its fact. Compact keeps both lines |
| Colonisation where to buy (no panel) | sells, station, system, stock, ly, jumps, pad, arrival, age | sells; place; stock; jumps over ly and age |

The side panel repeats nothing: demand sits under the price and is not listed again.

Tests: `demand.test.ts` covers each threshold edge and the recorded holds: the mixed hold, where
a 250 t buyer is tight for 200 t of Painite but short for 300 t of LTD; Buy against the emptied
hold; full; unknown; and none aboard (stolen and mission gold). `search.test.ts` covers the
groups, the panel and the shared-fact summary. The browser tests on recorded markets are:

- Market: the three demand states on Sell anywhere against the sol hold (15 and 193 t short,
  220 t tight, 1,070 t plenty), the Tip, Buy tritium against the emptied hold (31, 385, 1,872),
  none aboard, not known, and the pad mark (Cousteau Vista's M under the Anaconda).
- Services: the stale mark on the recorded 2021 report.
- Targets: a 12-day-old report's age, grey, on the panel-less list.

The existing row tests were updated to the new heads. Market's stacked-panel test now sets
145 percent Text size, because at 1100 px and 100 percent the narrower grouped list leaves room
for the panel beside it.

Checks run: `cargo fmt --check`, `cargo clippy --workspace --all-targets -D warnings`,
`cargo test` for core, gamestate, market and app (the preview fixtures included), `pnpm lint`,
`pnpm check`, `pnpm test` (653 tests), `scripts/check-guide.py`, and the 7 journeys. I did not
run `pnpm build`, `check-dist.py` or `cargo deny`.

Notes for review:

- The age shown is measured from now, but staleness is judged as of the fetch, as DESIGN.md
  already said. In the preview's old fixtures a row can therefore read "32 d old" in grey. Live
  answers are fetched moments before they are read, so this does not show there.
- The 1100 x 700 captures have no route sidebar (astromech-uc7i, in review). With it, the page
  column narrows and Sell's side panel will stack below the list at 1100 px, as in the spike.
- `ship_status.pad` comes from the journal's Loadout ship. The window's search filters resolve
  the pad from the engineering fleet's current ship (`filters::current_pad`). Both read the same
  Loadout, but they are two derivations.

