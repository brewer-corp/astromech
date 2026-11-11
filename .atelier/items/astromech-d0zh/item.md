---
schema: atelier.item
schema_version: 3
id: astromech-d0zh
title: Check the On foot journal events against the game
status: ready
mission: astromech-7q0z
labels:
- needs-human
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T05:16:27.550532467Z
trivial: null
caused_by: null
created_at: 2026-10-17T05:16:27.550532467Z
---

## Goal

Check the On foot page's owned gear (astromech-f1us) against a real journal from the game. The event shapes come from the community schemas built from real Odyssey journals (jixxed/ed-journal-schemas: `BuySuit`, `BuyWeapon`, `UpgradeSuit`, `UpgradeWeapon`, `SellSuit`, `SellWeapon`, `SuitLoadout`, `SwitchSuitLoadout`, `CreateSuitLoadout`), and the suit and weapon symbols and mod names from EDCD/EDDI's data definitions (`Suit.cs`, `HandWeapon.cs`, `SuitMod.cs`, `HandWeaponMod.cs`); the fixture is synthetic values in those shapes. Nothing has been seen in this commander's own journal.

## Done when

- [ ] A real journal with suit and weapon purchases, upgrades and a loadout with fitted mods gives the right grades and mods on the On foot page, and no `odyssey: unknown ...` line appears on stderr for a mod the game logs (the app reports a journal mod name missing from `data/catalog/odyssey.json` once, on stderr only: the release Windows app has no console, so check with a debug build or `cargo tauri dev`). A wrong symbol or mod name is corrected in `JOURNAL_GEAR` / `JOURNAL_MODS` in `scripts/gen-odyssey.py` and the catalogue.

## Context

Follow-up of astromech-f1us. Gear never logged in a purchase or loadout is unknown; suits are `<symbol>_class<grade>`, weapons are matched by their `Name` / `ModuleName` symbol.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-f1us (On foot: owned suits and weapons with their grades and mods)
