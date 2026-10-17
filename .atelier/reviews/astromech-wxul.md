---
schema: atelier.review
schema_version: 1
id: astromech-wxul
title: 'R2 [low] app/src/modules/station/View.test.ts: the new test''s vi.spyOn(station, ''loadHere'') is never restored (only window.open is). It is harmless because it is the file''s last test and vitest isolates files, but a test added after it would inherit the undocked mock.'
target: astromech-nkjz
identity: review-agent (opus, independent)
commit: 3dbca50d1488fd3d8c03b86114b8a7aa14f550b5
created_at: 2026-10-17T06:09:38.400660921Z
event:
  event: finding
  recommendation: no_action
---

R2 [low] app/src/modules/station/View.test.ts: the new test's vi.spyOn(station, 'loadHere') is never restored (only window.open is). It is harmless because it is the file's last test and vitest isolates files, but a test added after it would inherit the undocked mock.
