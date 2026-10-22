---
schema: atelier.review
schema_version: 1
id: astromech-9iap
title: 'R2 low docs/V1-DESIGN.md:695-702 and crates/astromech-core/tests/copy.rs:745: the rule says a name inside a sentence counts as spoken and gives ''Col 285 Sector AB-C d14-123, for neutron boost and fuel.'' as 15 of 16, implying the next-waypoint answer leaves room for any name. Real nebula sectors are longer: ''NGC 7822 Sector AB-C d14-123, for neutron boost and fuel.'' normalises to 17 words (IC 1805 Sector: 16), over the answer budget; joma''s goal says ''any system name''. Fix: either say the name as its own sentence in the answer when a label follows, or state the real bound in 6.5 (and add an NGC name to the copy test) so the rule Sumeet approves is accurate.'
target: astromech-uuqt
identity: review-agent-uuqt-1
commit: 70f1895d6b71686794c8ef3f4fd2a6885d716b46
created_at: 2026-10-22T22:59:21.621731127Z
event:
  event: finding
  recommendation: fix_now
---

R2 low docs/V1-DESIGN.md:695-702 and crates/astromech-core/tests/copy.rs:745: the rule says a name inside a sentence counts as spoken and gives 'Col 285 Sector AB-C d14-123, for neutron boost and fuel.' as 15 of 16, implying the next-waypoint answer leaves room for any name. Real nebula sectors are longer: 'NGC 7822 Sector AB-C d14-123, for neutron boost and fuel.' normalises to 17 words (IC 1805 Sector: 16), over the answer budget; joma's goal says 'any system name'. Fix: either say the name as its own sentence in the answer when a label follows, or state the real bound in 6.5 (and add an NGC name to the copy test) so the rule Sumeet approves is accurate.
