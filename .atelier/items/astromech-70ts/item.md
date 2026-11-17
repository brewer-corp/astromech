---
schema: atelier.item
schema_version: 3
id: astromech-70ts
title: 'Suit engineers get distances: their systems'' coordinates in the catalog'
status: in_review
mission: astromech-xajc
labels:
- size-s
- feature
priority: high
blocked_by:
- astromech-w1lj
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Aaf0def7e-3fd3-4188-9338-4a38284d0387
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
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

- [x] Each on-foot engineer (EDCD/FDevIDs `engineers.csv` ids 4000xx, the 13 in `data/catalog/odyssey.json`'s `engineers`) has its system's coordinates in `data/catalog/engineers.json`, in a new `onfoot` list beside `engineers` (id, name, system, id64, coords), written by `scripts/gen-starmap.py` from the Spansh systems dump the way it writes the ship engineers' (it filters to `SHIP_ENGINEER_IDS` at lines 24 and 59 today). `engineers` stays ship engineers only, so `engineering_engineers` and the engineer trip are unchanged. The file's `source` says so.
- [x] `odyssey_view`'s engineers (`EngineerLine`, `app/src/lib/api/types/EngineerLine.ts`) carry `coords`, and `engineering_search` gives suit hits a `distance_ly` from the commander's current system, so equal suit hits order nearest first, as ship hits do.
- [x] A row in the search's existing table tests (`search.rs`) covers two suit engineers of equal status ordered by distance, and the test that every engineer resolves (`search.rs`'s check over `engineers.json` and the unlock data) also checks every suit engineer has coordinates.
- [x] The recorded fixtures that show suit engineers are dumped again (`engineer-search.json`'s range search and `odyssey.json`, `ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-app --test preview_fixtures`); the drift test passes.
- [x] `docs/guide/engineering.md` no longer says suit engineers have no distance.
- [x] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p astromech-engineering -p astromech-odyssey` and the fixture test pass; then `pnpm check` and `pnpm test`, one full run at a time (each agent here has 4 GB).

## Scope

The coordinates, the reply field and the search's distances. The Engineers tree shows them in astromech-m6gw.

## Context

The dump is `https://downloads.spansh.co.uk/systems.json.gz` (about 6.4 GB; Spansh's dumps are public and fine to process). Reading it is a long single job, so it runs on boat (the `boat` skill), not on this machine, which serves T3 Code; only the regenerated catalog file is committed. `gen-starmap.py` writes the coordinates because it already reads the dump and `engineers.csv`; `gen-odyssey.py` (which writes the rest of each suit engineer's entry, and their `id64` at line 334) regenerates `odyssey.json` whole and would drop a field another script added there.

## Report

Suit hits of `engineering_search` now have a `distance_ly`, so equal suit hits order nearest first, as ship hits do. In the recorded range search, Domino Green is 119.57 ly from the fixture's system, Rosa Dayette 21,998.88 ly and Wellington Beck 128.49 ly; all three were `null` before.

- Data: `scripts/gen-starmap.py` now keeps both EDCD/FDevIDs id ranges (`SHIP_ENGINEER_IDS` and a new `ONFOOT_ENGINEER_IDS`). It writes ship engineers to `engineers` exactly as before and the on-foot engineers to a new `onfoot` list (id, name, system, id64, coords). The `source` names both lists. I ran it on a throwaway boat VM (`boat run --type default`, streaming the dump: 204,435,975 systems, 8 min 59 s, $0.0054, sandbox deleted). The 25 ship entries came out byte-identical to the committed ones, so `engineering_engineers` and the engineer trip don't change. All 13 on-foot id64s match `odyssey.json`.
- Search: `engineers::onfoot_info` reads `onfoot`, and `search::hit` looks a suit node up there and a ship node in `engineers`. `engineers::catalog()` still returns ship engineers only.
- Odyssey: `catalog::coords(id)` reads the same `onfoot` list by id. `EngineerLine` has `coords: [f64; 3]` (the binding is regenerated). The value is not optional because the data is compiled in, and `catalogue_is_complete_and_consistent` now checks that every on-foot engineer has coordinates. `lines()` `expect`s it rather than defaulting to Sol, since a silent Sol would be a wrong distance.
- Tests: the new `equal_suit_engineers_go_nearest_first` puts Domino Green and Uma Laszlo at equal status (both unlocked, then both known) and expects Uma Laszlo (75.76 ly) before Domino Green (119.57 ly), which is the reverse of alphabetical order. `the_table_and_the_catalogs_agree` also checks that every suit node in the unlock data has coordinates.
- Fixtures: `engineer-search.json` and `odyssey.json` were dumped again with `ASTROMECH_UPDATE_FIXTURES=1`, and the drift test passes without it.
- Guide: `docs/guide/engineering.md` drops "A suit engineer's distance isn't shown yet". It now says how suit hits are ordered and where the positions come from. The page already shows `distance_ly` whenever it is set, so the window needed no change.

Checks: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p astromech-engineering -p astromech-odyssey`, the fixture test, `pnpm check` (0 errors, 0 warnings) and `pnpm test` (90 files, 758 tests) all pass. The worktree had no `node_modules`, so I ran `pnpm install --frozen-lockfile` first.
