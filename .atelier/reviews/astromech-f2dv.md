---
schema: atelier.review
schema_version: 1
id: astromech-f2dv
title: 'R1 [medium] crates/astromech-engineering/src/links.rs large_loadout_urls_stay_openable_and_short: the 40-module loadout gives Coriolis 6374 / EDSY 5899 chars, and the test only asserts < 32000 while its own comment names a ~2k limit for some Windows handlers. open_external hands the URL to explorer.exe, so whether a 6k URL survives that path on Windows is unproven by this test. Recommendation: DEFER to a needs-human check (open a large engineered ship in Coriolis/EDSY from the app on the PC) and tighten the comment/assertion to the limit that actually applies.'
target: astromech-qiey
identity: review-agent (opus, independent)
commit: 6c75c7e172df1b5811ae53b2b6f54fb737016ba5
created_at: 2026-10-17T04:54:50.447432405Z
event:
  event: finding
  recommendation: defer
---

R1 [medium] crates/astromech-engineering/src/links.rs large_loadout_urls_stay_openable_and_short: the 40-module loadout gives Coriolis 6374 / EDSY 5899 chars, and the test only asserts < 32000 while its own comment names a ~2k limit for some Windows handlers. open_external hands the URL to explorer.exe, so whether a 6k URL survives that path on Windows is unproven by this test. Recommendation: DEFER to a needs-human check (open a large engineered ship in Coriolis/EDSY from the app on the PC) and tighten the comment/assertion to the limit that actually applies.
