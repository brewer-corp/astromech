---
created_at: "2026-10-17T05:16:27.550532467+00:00"
id: "astromech-d0zh"
issue_type: "task"
labels:
- "needs-human"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-f1us"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check the On foot journal events against the game"
updated_at: "2026-10-17T05:16:27.550532467+00:00"
---

## Goal

Check the On foot page's owned gear (astromech-f1us) against a real journal from the game. The event shapes come from the community schemas built from real Odyssey journals (jixxed/ed-journal-schemas: `BuySuit`, `BuyWeapon`, `UpgradeSuit`, `UpgradeWeapon`, `SellSuit`, `SellWeapon`, `SuitLoadout`, `SwitchSuitLoadout`, `CreateSuitLoadout`), and the suit and weapon symbols and mod names from EDCD/EDDI's data definitions (`Suit.cs`, `HandWeapon.cs`, `SuitMod.cs`, `HandWeaponMod.cs`); the fixture is synthetic values in those shapes. Nothing has been seen in this commander's own journal.

## Done when

A real journal with suit and weapon purchases, upgrades and a loadout with fitted mods gives the right grades and mods on the On foot page, and no `odyssey: unknown ...` line appears on stderr for a mod the game logs (the app reports a journal mod name missing from `data/catalog/odyssey.json` once). A wrong symbol or mod name is corrected in `JOURNAL_GEAR` / `JOURNAL_MODS` in `scripts/gen-odyssey.py` and the catalogue.

## Context

Follow-up of astromech-f1us. Gear never logged in a purchase or loadout is unknown; suits are `<symbol>_class<grade>`, weapons are matched by their `Name` / `ModuleName` symbol.
