---
created_at: "2026-11-05T06:19:45.358271847+00:00"
id: "astromech-f7i0"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-nxz9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-wvet"
    type: "supersedes"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T09:33:54.136484379+00:00"
status: "done"
title: "Mining hotspots finder in Places, with its voice query"
updated_at: "2026-11-05T09:33:54.136484379+00:00"
---

## Goal

Add the Mining hotspots finder to Places: rings with a hotspot for a chosen mineral, nearest first, around here, the next stop or a typed system (decision astromech-bi94: mining hotspots become a Places finder). `astromech_market::mining::hotspots_search` exists but nothing in the app or the voice grammar calls it.

## Done when

- `#/places/mining-hotspots/<mineral>` lists rings: main value the hotspot count for the mineral, then the place (ring, system), Also (other hotspots in the ring), ring type, reserve, distance, jumps, arrival; Add stop and Set destination on each row. Ring type and reserve are choices; "Around" offers Here, the route's next stop, and a typed system.
- A Tauri command modelled on `trade_cargo` (`search_origin`) calls the hotspot search; `lib/` gets a wrapper with recorded preview data; the Places home Mining section has its tile.
- A voice query, "nearest <mineral> hotspot", answers from the core with the first ring, its distance and jumps, following the faction-state queries' pattern (`astromech_tools::faction`), with phrase templates.
- Recorded-response Rust tests for the command's origin handling and the voice answer; component tests for the finder; `cargo test --workspace` and `pnpm lint && pnpm check && pnpm build && pnpm test` pass.
- `scripts/ui` screenshot compared with `docs/design/design-system-places-mining-after.png`; docs/guide/mining.md describes the finder and the voice query.

## Context

- Supersedes astromech-wvet. Spike astromech-yn6y: Spansh's bodies search filters `ring_signals` per body; hotspots never change.
- `crates/astromech-market/src/mining.rs`, mockup `#/mockup/ds-mining`, docs/ARCHITECTURE.md §18.2 (voice queries for lookups), docs/strategies/replace-the-toolbox.md (a lookup needs its voice query).

## Close Reason

Implementer report. Mining hotspots finder at #/places/mining-hotspots/<ore>[?ring=&reserve=] with a Mining tile on the Places home (last answer, e.g. "Painite · 4 hotspots") and a Ctrl+K entry per mineral. Main input: the mineral (15 hotspot ores); Ring Metallic/Metal Rich/Rocky/Icy/Any and Reserve Pristine/Major(+Pristine)/Any in the URL; radius 100 ly by default; Around offers Here, the next route stop or a typed system. Ranked by the ring's hotspots for the mineral, then distance, top 10: Spansh bodies search paged by hotspot-count bands, highest first, at most 4 requests (3 for Painite 100 ly from Sol, exact top 10 led by LHS 3602 3 A Ring, 10 hotspots); when the budget runs out the summary names the shorter reach or "too many to rank" (answer fields bodies, least, reach_ly). Rows: hotspots, ring and system, Also, type, reserve, distance, jumps, arrival, Add stop, Set destination. Shell command mining_hotspots. Voice: mining.hotspots with a {mineral} slot ("nearest painite hotspot" and 5 more, "near <system>"), nearest first in one request, ignoring the finder's choices: "Nearest Painite hotspot. Barnard's Star 5 A Ring. 6.0 light years, 1 jump. 2 hotspots, depleted reserves." Deviations: ranked by hotspots, not "nearest first" as the Goal said (DESIGN.md: the main value is the ranking); defaults Any/Any (mockup shows Metallic/Pristine; near Sol no Painite ring is Pristine); two voice-eval phrases swapped to keep the set at 200, audio eval not rerun (astromech-y14z). Also fixed a crumb-navigation TypeError in all five finder pages (shared showing(id) helper + test). Proof: tests fail without each change; cargo test --workspace 1254 passed at 0b6806b9, scoped crates after fixes; clippy -D warnings, fmt, check-deps OK; app lint, svelte-check 0/0, build, 444 tests; check-guide OK; recorded Spansh fixtures (7 live requests in all, tests offline); screenshots docs/design/places-mining-hotspots-1600.png, places-home-mining-1600.png. Not run locally: Tauri shell compile (CI). Review: reviewer-sonnet-f7i0; fix_now okse (nearest-50 cap vs "Within 100 ly") and d2lq (crumb error) fixed in a3a26297, approved astromech-kp4n; deferred geeg → astromech-y14z, otpp → astromech-z9rd, et0g → astromech-k8t4; wydq records the eight decisions (all matching the branch).
