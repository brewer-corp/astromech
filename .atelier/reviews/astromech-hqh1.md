---
schema: atelier.review
schema_version: 1
id: astromech-hqh1
title: 'Weapons are identified by Name_Localised / ModuleName_Localised (owned.rs:50-53, 133, 142, 175), which the schemas mark optional (not in required) and which follows the game language, while a stable symbol exists in Name / ModuleName (wpn_m_assaultrifle_kinetic_fauto = Karma AR-50, wpn_m_assaultrifle_laser_fauto = TK Aphelion, wpn_m_sniper_plasma_charged = Manticore Executioner). A localised client or a line without the _Localised field drops the weapon entirely. The fixture''s weapon Names are invented (Wpn_M_AssaultRifle_*_FAssault), so nothing tests the real symbols. Fix: map the lower-cased Name/ModuleName symbol to the catalogue weapon (symbol list in odyssey.json), localised name at most a fallback. Evidence: https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/BuyWeapon/BuyWeapon.json, https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/UpgradeWeapon/UpgradeWeapon.json, https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/SuitLoadout/SuitLoadout.json.'
target: astromech-f1us
identity: review-agent (opus, independent)
commit: 723b55273123a5995125fcbd73fa4ea7f96a4ab0
created_at: 2026-10-17T05:24:23.966026731Z
event:
  event: finding
  recommendation: fix_now
---

Weapons are identified by Name_Localised / ModuleName_Localised (owned.rs:50-53, 133, 142, 175), which the schemas mark optional (not in required) and which follows the game language, while a stable symbol exists in Name / ModuleName (wpn_m_assaultrifle_kinetic_fauto = Karma AR-50, wpn_m_assaultrifle_laser_fauto = TK Aphelion, wpn_m_sniper_plasma_charged = Manticore Executioner). A localised client or a line without the _Localised field drops the weapon entirely. The fixture's weapon Names are invented (Wpn_M_AssaultRifle_*_FAssault), so nothing tests the real symbols. Fix: map the lower-cased Name/ModuleName symbol to the catalogue weapon (symbol list in odyssey.json), localised name at most a fallback. Evidence: https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/BuyWeapon/BuyWeapon.json, https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/UpgradeWeapon/UpgradeWeapon.json, https://github.com/jixxed/ed-journal-schemas/blob/main/schemas/SuitLoadout/SuitLoadout.json.
