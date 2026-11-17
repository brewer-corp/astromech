---
schema: atelier.item
schema_version: 3
id: astromech-w1lj
title: 'Suit loadouts from the journal: each loadout''s suit and weapons, and which is worn'
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
created_at: 2026-11-17T10:04:13.697024715Z
---

## Goal

The window knows the commander's suit loadouts: each loadout's name, its suit and the weapon in each slot, and which loadout is worn. Ships > Suits (astromech-nnn0) is drawn around them, as `docs/design/nav-spike/tone5-suits-2560.png` shows. Today the core reads the loadout events only for each suit's and weapon's grade and mods and drops the rest (`crates/astromech-odyssey/src/owned.rs:179`, `fn loadout` below it), so `odyssey_view` has owned gear but no loadouts (`app/src/lib/api/types/OdysseyView.ts`).

## Done when

- [ ] `crates/astromech-odyssey` keeps each loadout from `SuitLoadout`, `SwitchSuitLoadout` and `CreateSuitLoadout`, keyed by `LoadoutID`: `LoadoutName`, the suit (`SuitID`, resolved to the owned suit by id) and each entry of `Modules` by `SlotName` (`PrimaryWeapon1`, `PrimaryWeapon2`, `SecondaryWeapon`) to the owned weapon by `SuitModuleID`. The worn loadout is the one named by the latest `SuitLoadout` or `SwitchSuitLoadout`. The Flight Suit's loadout (`flightsuit`, no slots) is not kept, as the Flight Suit is not kept as gear.
- [ ] `OdysseyView` gains `loadouts`, each with its id, name, whether it is worn, its suit and its slots, each item as the owned gear it is (catalogue id, name, grade, fitted mods), in `LoadoutID` order; and the reply's generated type and `app/src/lib/api/commands.ts` follow (`ts_rs`).
- [ ] A loadout event names a suit or weapon the core has no purchase for: it is added as owned gear from the event, as `fn loadout` does today, so the loadout is never missing an item.
- [ ] Table-driven tests in `crates/astromech-odyssey` (beside the existing loadout tests in `src/lib.rs`, around line 313) cover: three loadouts created and switched, the worn one following the latest event; a weapon moved between slots; a loadout naming gear never bought; the Flight Suit's loadout left out. Each journal line is in the shape the repo already holds, from `worlds/preview/odyssey/game/Journal.2026-10-14T130000.01.log` lines 13 to 16.
- [ ] The preview world `worlds/preview/odyssey` holds four loadouts as the mockup has (a suit and up to three weapons each, one worn), one of them with the Maverick Suit at grade 1 and a Karma P-15 at grade 1 so the recorded sample picks apply to it, using only those event shapes, and `app/src/preview/fixtures/odyssey.json` is dumped again from it (`ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-app --test preview_fixtures`); the fixture drift test passes and `pnpm check` passes with the new field.
- [ ] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p astromech-odyssey` and `cargo test -p astromech-app --test preview_fixtures` pass; then `pnpm check` and `pnpm test`, one full run at a time (each agent here has 4 GB).

## Scope

The core's loadouts and the reply. The page is astromech-nnn0. No change to how grades and mods are read.

## Context

The fields, from `worlds/preview/odyssey/game/Journal.2026-10-14T130000.01.log:13-16`: `SuitID`, `SuitName` (`utilitysuit_class1`; grade in the suffix), `SuitMods`, `LoadoutID` (4293000001), `LoadoutName` ("Explore"), `Modules[]` with `SlotName`, `SuitModuleID`, `ModuleName`, `Class` (the weapon's grade) and `WeaponMods`. Those lines were written from the community schemas jixxed/ed-journal-schemas, as `owned.rs`'s header says.

## Unknowns

- The repo holds no `RenameSuitLoadout`, `DeleteSuitLoadout`, `LoadoutEquipModule` or `LoadoutRemoveModule` line, so a renamed or deleted loadout, or a weapon equipped without a switch, is not followed until the next loadout event names it. Following them needs their shapes from the community schemas and a check in the game (astromech-d0zh checks the On foot events; astromech-t8hg which events rewrite the on-foot files).
- Whether the game writes `SuitLoadout` for every loadout at start or only for the worn one is not known (astromech-t8hg). If only the worn one, a loadout the commander has not worn since the journals began is missing until it is worn; the page says how many it knows.
