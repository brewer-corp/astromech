---
schema: atelier.item
schema_version: 3
id: astromech-70ts
title: 'Suit engineers get distances: their systems'' coordinates in the catalog'
status: draft
mission: astromech-xajc
labels:
- size-s
- feature
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T10:04:13.607866162Z
---

## Goal

A suit engineer has a distance, so the engineer search can name the nearest one and the Engineers tree (astromech-m6gw) can say how far each is. Today the repo holds coordinates only for ship engineers (`data/catalog/engineers.json`, written by `scripts/gen-starmap.py` from the Spansh systems dump), so every suit hit of `engineering_search` has `distance_ly: null` and equal suit hits fall back to alphabetical order (`crates/astromech-engineering/src/search.rs`, `fn hit` and `fn nearer`, around lines 236-254).

Taken from deferred finding astromech-7did/1-4sr2.5, in .atelier/items/astromech-7did/reviews/1-4sr2.md:

> Finding 5: Suit recommendations cannot be ordered by distance
> Recommendation: defer
>
> The Report correctly identifies the missing suit coordinates. `crates/astromech-engineering/src/search.rs:233-243` resolves coordinates only through the ship-engineer catalog, so every suit hit has `distance_ly = None` and equal-status suit hits fall back to alphabetical order. The guide discloses the missing distances, but a suit recommendation also cannot implement the nearest-first rule. Preserve this as a follow-up item to generate the suit-system coordinates and use them for distance and selection; the missing catalog data predates this search.

Round 2's finding astromech-7did/2-85d3.5 is the same and was dropped as taken here.

## Done when

- [ ] Each on-foot engineer (EDCD/FDevIDs `engineers.csv` ids 4000xx, the 13 in `data/catalog/odyssey.json`'s `engineers`) has its system's coordinates in a committed catalog file, read from the Spansh systems dump by the engineer's `id64` the way `scripts/gen-starmap.py` reads the ship engineers' (it filters to `SHIP_ENGINEER_IDS` at line 24 and 59 today). The file's `source` says so.
- [ ] `odyssey_view`'s engineers (`EngineerLine`, `app/src/lib/api/types/EngineerLine.ts`) carry `coords`, and `engineering_search` gives suit hits a `distance_ly` from the commander's current system, so equal suit hits order nearest first, as ship hits do.
- [ ] A row in the search's existing table tests (`search.rs`) covers two suit engineers of equal status ordered by distance, and the test that every engineer resolves (`search.rs`'s check over `engineers.json` and the unlock data) also checks every suit engineer has coordinates.
- [ ] The recorded fixtures that show suit engineers are dumped again (`engineer-search.json`'s range search and `odyssey.json`, `ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-app --test preview_fixtures`); the drift test passes.
- [ ] `docs/guide/engineering.md` no longer says suit engineers have no distance.
- [ ] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p astromech-engineering -p astromech-odyssey` and the fixture test pass; then `pnpm check` and `pnpm test`, one full run at a time (each agent here has 4 GB).

## Scope

The coordinates, the reply field and the search's distances. The Engineers tree shows them in astromech-m6gw.

## Context

The dump is `https://downloads.spansh.co.uk/systems.json.gz` (about 6.4 GB; Spansh's dumps are public and fine to process). Reading it is a long single job, so it runs on boat (the `boat` skill), not on this machine, which serves T3 Code; only the regenerated catalog file is committed. Either script may write the field: `gen-starmap.py` already reads the dump and `engineers.csv`; `gen-odyssey.py` writes the rest of each suit engineer's entry and their `id64` (line 334). Ship-only users of `engineers.json` (`engineering_engineers`, the engineer trip) must keep seeing only ship engineers.
