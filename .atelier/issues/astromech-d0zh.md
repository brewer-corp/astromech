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

Confirm in game the journal event shapes the On foot page's owned gear reads (astromech-f1us), which were written from the Frontier Journal manual and EDCD docs and a synthetic fixture only: `BuySuit`, `UpgradeSuit` (`Class` is the new grade), `SellSuit`, `BuyWeapon`, `UpgradeWeapon`, `SellWeapon`, `SuitLoadout` (`SuitName` ends `_class<grade>`, `SuitMods`, `Modules[].Class`, `Modules[].WeaponMods`).

## Done when

A real journal with suit and weapon purchases, upgrades and a loadout with fitted mods gives the right grades and mods on the On foot page. Mod names match the catalogue ids after stripping `suit_` / `weapon_` (e.g. whether `weapon_stability`, `weapon_magazinesize` are the real strings); any that don't are added as aliases, and any wrong event field is corrected.

## Context

Follow-up of astromech-f1us. Gear that has never been in a loadout shows no mods. Weapons are matched by their localised name.
