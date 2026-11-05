---
schema: atelier.review
schema_version: 1
id: astromech-u270
title: finding on astromech-e0im @b71a0261
target: astromech-e0im
identity: review-agent (opus, independent)
commit: b71a02618ae815b628e7eaca817f7e0b452d7885
created_at: 2026-11-05T00:55:42.893367878Z
event:
  event: finding
  recommendation: defer
---

Low: .atelier/issues/astromech-e0im.md:48 records the R2 and R3 choices as a plain [note] ('Decisions (R2, R3): ...'), not as [decision] notes, so 'atelier issue show astromech-e0im' lists no decision of its own under Decisions (astromech-54k4 records its EVERYDAY choice with --kind decision). The content is accurate (checked against a scratch probe: margin-term removal flips all three near misses to Execute; the 'everyday - best.score >= MARGIN' NoMatch rule turns both 'this worth' phrases into system_value Ask and keeps the ship phrases NoMatch), but the next agent looking for the decision will not find it where atelier surfaces decisions. Suggest re-recording as two 'atelier issue note --kind decision --reason ... --rejected ...' entries (R2: keep margin, rejected drop the term; R3: NoMatch, rejected Ask system value via a NoMatch-rule change), which can land on main as a planning commit.
