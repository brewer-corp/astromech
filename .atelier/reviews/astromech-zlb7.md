---
schema: atelier.review
schema_version: 1
id: astromech-zlb7
title: 'BuySuit and BuyWeapon ignore the bought grade and mods (crates/astromech-odyssey/src/owned.rs:110-138 hard-code grade 1, no mods). Real BuySuit has Name with the grade suffix (UtilitySuit_Class3, ExplorationSuit_Class2) and a required SuitMods array; BuyWeapon has required Class (1-3 seen) and WeaponMods. Pioneer Supplies sells G2+ and pre-modded gear, so a bought G3 Maverick shows as G1 and the picker offers G1->G2 until a SuitLoadout corrects it (weapons not in a loadout never are). Fix: take the grade from the Name suffix / Class and the mods from SuitMods/WeaponMods; add these required fields to the fixture. Evidence: https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/BuySuit/BuySuit.json and https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/BuyWeapon/BuyWeapon.json (schemas built from real Odyssey journals).'
target: astromech-f1us
identity: review-agent (opus, independent)
commit: 723b55273123a5995125fcbd73fa4ea7f96a4ab0
created_at: 2026-10-17T05:24:23.707211336Z
event:
  event: finding
  recommendation: fix_now
---

BuySuit and BuyWeapon ignore the bought grade and mods (crates/astromech-odyssey/src/owned.rs:110-138 hard-code grade 1, no mods). Real BuySuit has Name with the grade suffix (UtilitySuit_Class3, ExplorationSuit_Class2) and a required SuitMods array; BuyWeapon has required Class (1-3 seen) and WeaponMods. Pioneer Supplies sells G2+ and pre-modded gear, so a bought G3 Maverick shows as G1 and the picker offers G1->G2 until a SuitLoadout corrects it (weapons not in a loadout never are). Fix: take the grade from the Name suffix / Class and the mods from SuitMods/WeaponMods; add these required fields to the fixture. Evidence: https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/BuySuit/BuySuit.json and https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/BuyWeapon/BuyWeapon.json (schemas built from real Odyssey journals).
