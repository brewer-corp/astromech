---
schema: atelier.review
schema_version: 1
id: astromech-vfvs
title: 'R1 from the re-review: the Option marker is correct but no test caught the original bug. Added a_journal_left_while_empty_is_live_when_reopened; with the mutant .filter(|r| replay && *r > 0) it fails (core pipeline tests: 1 failed), with the fix it passes.'
target: astromech-7uj3
identity: 7uj3-reviewer-2
commit: a6bb2556121981c0f3e9987d041d990678402154
created_at: 2026-10-17T19:07:59.835480282Z
event:
  event: review
---

R1 from the re-review: the Option marker is correct but no test caught the original bug. Added a_journal_left_while_empty_is_live_when_reopened; with the mutant .filter(|r| replay && *r > 0) it fails (core pipeline tests: 1 failed), with the fix it passes.
