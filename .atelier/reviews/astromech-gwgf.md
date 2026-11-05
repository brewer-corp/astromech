---
schema: atelier.review
schema_version: 1
id: astromech-gwgf
title: finding on astromech-6h7t @1d4f6071
target: astromech-6h7t
identity: review-agent-6h7t-1
commit: 1d4f60717b3ab3b76e660f1c16ab099670902dc0
created_at: 2026-11-04T20:39:00.739058362Z
event:
  event: finding
  recommendation: fix_now
---

minor: docs/PLOTTING.md:105-107 still says 'The page currently shows a Spansh neutron route's legs as its jumps, astromech-6h7t, so a Spansh route there looks shorter than it is.' After this change that is false, so a reader (agent or human) is told a fixed bug is live. Suggested fix: drop the parenthetical; keep the sentence that Spansh counts post-boost jumps more cautiously so its totals are higher.
