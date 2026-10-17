---
schema: atelier.review
schema_version: 1
id: astromech-k104
title: 'No test proves BuySuit takes its grade from the Name suffix, the headline of astromech-zlb7. Mutation: replacing the BuySuit branch of the grade match in crates/astromech-odyssey/src/owned.rs (grade_of(from_name...)) with Some(1) leaves all 15 astromech-odyssey tests passing. The fixture''s UtilitySuit_Class3 purchase (SuitID 1700000000004) is followed by a SwitchSuitLoadout utilitysuit_class3 that resets the grade, and the unit test upgrading_keeps_the_mods_... follows BuySuit UtilitySuit_Class2 with UpgradeSuit Class 3 and only asserts 3. The code is right by inspection; the proof is missing. Fix: assert the grade after the BuySuit alone (e.g. check o.gear() between the BuySuit and UpgradeSuit lines, expecting 2), then re-run the mutation. BuyWeapon''s Class is covered (tkzenith at 2).'
target: astromech-f1us
identity: review-agent (opus, independent)
commit: 9a75d654401c05ed91d14c49ec345b5ce3616a04
created_at: 2026-10-17T05:48:46.201446803Z
event:
  event: finding
  recommendation: fix_now
---

No test proves BuySuit takes its grade from the Name suffix, the headline of astromech-zlb7. Mutation: replacing the BuySuit branch of the grade match in crates/astromech-odyssey/src/owned.rs (grade_of(from_name...)) with Some(1) leaves all 15 astromech-odyssey tests passing. The fixture's UtilitySuit_Class3 purchase (SuitID 1700000000004) is followed by a SwitchSuitLoadout utilitysuit_class3 that resets the grade, and the unit test upgrading_keeps_the_mods_... follows BuySuit UtilitySuit_Class2 with UpgradeSuit Class 3 and only asserts 3. The code is right by inspection; the proof is missing. Fix: assert the grade after the BuySuit alone (e.g. check o.gear() between the BuySuit and UpgradeSuit lines, expecting 2), then re-run the mutation. BuyWeapon's Class is covered (tkzenith at 2).
