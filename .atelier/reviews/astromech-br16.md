---
schema: atelier.review
schema_version: 1
id: astromech-br16
title: 'O1: retry_pending ran in on_status before the batch''s journal lines, so a newer file could be applied to an older pending event and read twice'
target: astromech-ogq4
identity: ogq4-reviewer-1
commit: 4af226494ac2d098eadcfb2b6b1d61cbeba17832
created_at: 2026-10-17T18:09:11.410288212Z
event:
  event: finding
  recommendation: fix_now
---

O1: retry_pending ran in on_status before the batch's journal lines, so a newer file could be applied to an older pending event and read twice
