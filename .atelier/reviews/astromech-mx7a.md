---
schema: atelier.review
schema_version: 1
id: astromech-mx7a
title: finding on astromech-j75v @9f516d3e
target: astromech-j75v
identity: review-agent-j75v-1
commit: 9f516d3e479fae0aa610989b7b29db820e240589
created_at: 2026-11-04T21:25:05.662604285Z
event:
  event: finding
  recommendation: fix_now
---

minor: docs/ARCHITECTURE.md:683 Section 5.12 lists the matching steps and slot rules in detail but does not mention the new step, where a trailing 'near/from <system>' is split off for takes_origin commands, kept in Match::origin, and the split is kept only when it scores at least as well as the whole transcript. Agents rely on this section when they change the matcher. Fix: add a bullet beside the Slots bullet, together with the R1 change.
