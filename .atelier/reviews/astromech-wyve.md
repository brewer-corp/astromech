---
schema: atelier.review
schema_version: 1
id: astromech-wyve
title: approval on astromech-58a7 @4657397b
target: astromech-58a7
identity: review-agent-58a7-1
commit: 4657397bda73c11b22d0216501b72bed0d493a75
created_at: 2026-11-04T23:11:26.873389538Z
event:
  event: approval
---

Read the whole branch 1d2219d7..4657397b. 9f08b20f checked against a fresh EDCD/FDevIDs clone at c7c29f7: the 7 symbols are new (absent before, not moved); AnacondaMkII_Armour_* carry entitlement ELITE_V_ANACONDA_MKII so commander (dropped) is right under EDDN schemas/outfitting-README.md Elisions (non-null sku other than Horizons) and consistent with b8js's cautious choice; Int_LargeCargoRack_Size6_Class1 has a blank entitlement like the size 7/8 class 1 racks already in station. --check: exits 0 at c7c29f7 (1034/184), exits 1 at c356129 naming all 7 and the remedy, usage on bad args; test_gen_eddn_outfitting.py 4 pass. owner() refactor preserves modules() classification; cargo test -p astromech-eddn 152 pass incl. the new unknown_modules test (lib.rs touched first). real_journals on a synthetic folder with an Outfitting.json adding a made-up module plus the two newly listed symbols reports exactly the made-up one as a PROBLEM; skips cleanly without a folder. Workflow: weekly cron + dispatch, read-only perms, checks out FDevIDs default branch, fails with symbol names and the fix. ci.yml step runs on ubuntu. CONTRIBUTING accurate. No journals committed; NOTICE already credits FDevIDs for this file. Two low defer findings.
