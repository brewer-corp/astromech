---
schema: atelier.review
schema_version: 1
id: astromech-56z2
title: 'SwitchSuitLoadout and CreateSuitLoadout have exactly the SuitLoadout shape (SuitID, SuitName, SuitMods, Modules[] with SuitModuleID, ModuleName, Class, WeaponMods) but are not read (owned.rs:155, lib.rs EVENTS), so mods and grades are refreshed only by the login-time SuitLoadout of the active loadout. Routing all three to loadout() is a one-line change worth folding into the FIX NOW work. Evidence: https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/SwitchSuitLoadout/SwitchSuitLoadout.json, https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/CreateSuitLoadout/CreateSuitLoadout.json.'
target: astromech-f1us
identity: review-agent (opus, independent)
commit: 723b55273123a5995125fcbd73fa4ea7f96a4ab0
created_at: 2026-10-17T05:24:24.196851096Z
event:
  event: finding
  recommendation: defer
---

SwitchSuitLoadout and CreateSuitLoadout have exactly the SuitLoadout shape (SuitID, SuitName, SuitMods, Modules[] with SuitModuleID, ModuleName, Class, WeaponMods) but are not read (owned.rs:155, lib.rs EVENTS), so mods and grades are refreshed only by the login-time SuitLoadout of the active loadout. Routing all three to loadout() is a one-line change worth folding into the FIX NOW work. Evidence: https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/SwitchSuitLoadout/SwitchSuitLoadout.json, https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/CreateSuitLoadout/CreateSuitLoadout.json.
