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
status: "todo"
title: "Mining hotspots finder in Places, with its voice query"
updated_at: "2026-11-05T06:19:45.358271847+00:00"
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
