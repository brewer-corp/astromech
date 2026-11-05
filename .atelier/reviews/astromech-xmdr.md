---
schema: atelier.review
schema_version: 1
id: astromech-xmdr
title: finding on astromech-c4i0 @79f35aaf
target: astromech-c4i0
identity: review-agent (sonnet, independent)
commit: 79f35aafb0887665d1463f6b2b04f0f148a3ee2d
created_at: 2026-11-05T00:52:28.404447851Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/astromech-yxtz.md:41 Context still says DESIGN.md "adds --alarm red for stale report ages and hard errors", contradicting its own Done when (no red for stale) and 5p9c item 11 / DESIGN.md Narrow Red Rule. The revision claims yxtz is made consistent; an implementer reading Context would paint stale ages red. Fix: change to "adds --alarm red for hard errors only; staleness is Orange Mid facts with the age in Orange".
