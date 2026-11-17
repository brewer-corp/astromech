---
schema: atelier.item
schema_version: 3
id: astromech-w1lj
title: 'Suit loadouts from the journal: each loadout''s suit and weapons, and which is worn'
status: approved
mission: astromech-xajc
labels:
- size-s
- feature
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A851b640e-dc8c-4e5e-8c48-319b23878357
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-17T10:04:13.697024715Z
---

## Goal

The window knows the commander's suit loadouts: each loadout's name, its suit and the weapon in each slot, and which loadout is worn. Ships > Suits (astromech-nnn0) is drawn around them, as `docs/design/nav-spike/tone5-suits-2560.png` shows. Today the core reads the loadout events only for each suit's and weapon's grade and mods and drops the rest (`crates/astromech-odyssey/src/owned.rs:179`, `fn loadout` below it), so `odyssey_view` has owned gear but no loadouts (`app/src/lib/api/types/OdysseyView.ts`).

## Done when

- [x] `crates/astromech-odyssey` keeps each loadout from `SuitLoadout`, `SwitchSuitLoadout` and `CreateSuitLoadout`, keyed by `LoadoutID`: `LoadoutName`, the suit (`SuitID`, resolved to the owned suit by id) and each entry of `Modules` by `SlotName` (`PrimaryWeapon1`, `PrimaryWeapon2`, `SecondaryWeapon`) to the owned weapon by `SuitModuleID`. The worn loadout is the one named by the latest `SuitLoadout` or `SwitchSuitLoadout`. The Flight Suit's loadout (`flightsuit`, no slots) is not kept, as the Flight Suit is not kept as gear.
- [x] `OdysseyView` gains `loadouts`, each with its id, name, whether it is worn, its suit and its slots, each item as the owned gear it is (catalogue id, name, grade, fitted mods) with its journal id (`SuitID` for the suit, `SuitModuleID` for a weapon), in `LoadoutID` order; `OwnedGear` gains the same `id`, so an item worn in several loadouts is one item with one id; and the reply's generated type and `app/src/lib/api/commands.ts` follow (`ts_rs`).
- [x] A pick can be for one owned item: `Pick` (`crates/astromech-odyssey/src/shopping.rs:13`) gains an optional `item`, the journal id of the suit or weapon it is planned for; a pick without one (every pick saved before this) still costs as today. `shopping` costs each pick once, as it does (`shopping.rs:182`), so two picks of the same recipe for two items cost twice and a pick for an item worn in two loadouts costs once. A table-driven test in `shopping.rs` covers both, and a recipe pick with no `item` beside one with an `item`.
- [x] A loadout event names a suit or weapon the core has no purchase for: it is added as owned gear from the event, as `fn loadout` does today, so the loadout is never missing an item.
- [x] Table-driven tests in `crates/astromech-odyssey` (beside the existing loadout tests in `src/lib.rs`, around line 313) cover: three loadouts created and switched, the worn one following the latest event; a weapon moved between slots; a loadout naming gear never bought; the Flight Suit's loadout left out. Each journal line is in the shape the repo already holds, from `worlds/preview/odyssey/game/Journal.2026-10-14T130000.01.log` lines 13 to 16.
- [x] The preview world `worlds/preview/odyssey` holds four loadouts as the mockup has (a suit and up to three weapons each, one worn), one of them with a Maverick Suit at grade 1 and a Karma P-15 at grade 1, a second loadout wearing the same Maverick (one item in two loadouts), and a second grade-1 Maverick in a third loadout (two items of the same recipe); the fixture's sample picks (its `/picks` input, `app/src-tauri/tests/preview_fixtures.rs:578`) are tied by `item` to the first Maverick and that Karma P-15, using only those event shapes, and `app/src/preview/fixtures/odyssey.json` is dumped again from it (`ASTROMECH_UPDATE_FIXTURES=1 cargo test -p astromech-app --test preview_fixtures`); the fixture drift test passes and `pnpm check` passes with the new field.
- [x] `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test -p astromech-odyssey` and `cargo test -p astromech-app --test preview_fixtures` pass; then `pnpm check` and `pnpm test`, one full run at a time (each agent here has 4 GB).

## Scope

The core's loadouts, each owned item's journal id, and a pick's `item`, so a plan can belong to one suit or weapon. The page is astromech-nnn0. No change to how grades and mods are read.

## Context

The fields, from `worlds/preview/odyssey/game/Journal.2026-10-14T130000.01.log:13-16`: `SuitID`, `SuitName` (`utilitysuit_class1`; grade in the suffix), `SuitMods`, `LoadoutID` (4293000001), `LoadoutName` ("Explore"), `Modules[]` with `SlotName`, `SuitModuleID`, `ModuleName`, `Class` (the weapon's grade) and `WeaponMods`. Those lines were written from the community schemas jixxed/ed-journal-schemas, as `owned.rs`'s header says.

## Unknowns

- The repo holds no `RenameSuitLoadout`, `DeleteSuitLoadout`, `LoadoutEquipModule` or `LoadoutRemoveModule` line, so a renamed or deleted loadout, or a weapon equipped without a switch, is not followed until the next loadout event names it. Following them needs their shapes from the community schemas and a check in the game (astromech-d0zh checks the On foot events; astromech-t8hg which events rewrite the on-foot files).
- Whether the game writes `SuitLoadout` for every loadout at start or only for the worn one is not known (astromech-t8hg). If only the worn one, a loadout the commander has not worn since the journals began is missing until it is worn; the page says how many it knows.

## Report

The core now keeps each suit loadout and which one is worn. The preview sample has four loadouts and picks tied to the items they are for, so astromech-nnn0 can draw Ships > Suits around them. Commits `b3e981e1` and `34c06f9c`.

**What changed**

- `crates/astromech-odyssey/src/owned.rs`: each loadout event (`SuitLoadout`, `SwitchSuitLoadout`, `CreateSuitLoadout`) records the loadout under its `LoadoutID`: its `LoadoutName`, its suit's `SuitID`, and each `Modules` entry by `SlotName` and `SuitModuleID`. The latest event for an id replaces the whole loadout, so a weapon moved between slots follows. `worn` is the `LoadoutID` of the latest `SuitLoadout` or `SwitchSuitLoadout`. `CreateSuitLoadout` does not change it. The suit and weapons are still recorded as owned gear from the event, as before, so gear never bought still appears. A loadout whose suit the catalogue lacks (the Flight Suit) is not kept. If one was kept under that id, it is removed. `Owned::loadouts()` returns `SuitLoadout { id, name, worn, suit, slots: [LoadoutSlot { slot, item }] }` in `LoadoutID` order, and `OwnedGear` gains `id` (the `SuitID` or `SuitModuleID`).
- `OdysseyView` gains `loadouts`. The bindings were regenerated (`SuitLoadout.ts` and `LoadoutSlot.ts` are new; `OdysseyView`, `OwnedGear` and `Pick` changed). `commands.ts` needed no change, because the commands are the same.
- `Pick::Upgrade` and `Pick::Mod` gain `item: Option<i64>`. It uses `serde(default)` and is skipped when `None`, and is `item?: number` in TS, so every pick saved before this still parses and costs as before. `shopping` is unchanged and still costs each pick once. `each_pick_is_costed_once_whatever_item_it_is_for` in `shopping.rs` covers one item, two items of one recipe, and a pick with no item beside one with an item.
- `loadouts_from_the_journal` in `src/lib.rs` is a table of five cases: three loadouts created and switched, a weapon moved between slots, gear never bought, one suit in two loadouts, and the Flight Suit's loadout. Each case also checks that every loadout item is in the owned gear exactly once.
- Preview world: a new `worlds/preview/odyssey/game/Journal.2026-10-14T133000.01.log` buys a Karma P-15 G1 (`1710000000004`) and a second G1 Maverick (`1700000000005`). It creates "Spare" (the second Maverick and the P-15), switches "Raid" to the first Maverick (`1700000000001`), then switches to "Explore": the first Maverick with the AR-50, the Aphelion and the P-15, which is worn. With "Fight" (the Dominator) that makes four loadouts. The sample picks (Maverick 1→3, Night Vision, P-15 1→2) carry `item` for the first Maverick and the P-15, and `odyssey.json` was dumped again.
- `docs/V1-DESIGN.md`: the On foot row now describes the loadouts and a pick's `item`, and "Loadouts" is no longer in its not-in-v1 column, since astromech-nnn0 draws them.

**Decisions a reviewer should check**

- `pickKey` in `app/src/lib/odyssey.ts` now includes `item`. Without it, two picks of one recipe for two items would have one key. The picker would then treat the second pick as a duplicate, and the preview's `samePicks` would match picks that name different items. Picks without `item` keep their old key, plus a trailing `:`, so nothing saved changes meaning.
- I took the first Maverick's Night Vision out of the preview world: `130000` line 13 and my two `SwitchSuitLoadout` lines. That lets the sample's Night Vision pick be tied to it, as in the mockup, where the plan is "Grade 3 · Night Vision" on a suit without it. The crate's own `tests/fixtures` were not touched, so the preview world no longer equals them.
- The preview P-15 is bought and worn with Stability. `View.browser.test.ts` asserts that Stability is fitted on every owned weapon, so it is not offered. Without it, that sample assumption breaks. The mockup shows the P-15 with no mods.
- Moving "Raid" to the first Maverick leaves the G3 Maverick (`1700000000004`) in no loadout. It is still owned gear. Four loadouts, two sharing a Maverick and one wearing the second G1 Maverick, needed one of the existing three to change.
- If a loadout's suit is sold later, `loadouts()` leaves the whole loadout out. If a slotted weapon is sold, only that slot is left out. Following deleted or renamed loadouts still waits on the events listed under Unknowns.

**Checks run**

- `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings` are clean.
- These passed: `cargo test -p astromech-odyssey` (18), `cargo test -p astromech-app --test preview_fixtures` (drift test included), `cargo test -p astromech-app --test every_window_command`, `cargo test -p astromech-api`, `cargo test -p astromech-core --lib bindings`, and `scripts/check-deps.py` and `scripts/check-guide.py`.
- In `app/`, run one at a time after `pnpm install --frozen-lockfile`: `pnpm lint`, `pnpm check` (0 errors), and `pnpm test` (758 passed).
- Not run: the whole `cargo test --workspace`, `pnpm build`, and the journeys. No journey uses the odyssey world.

