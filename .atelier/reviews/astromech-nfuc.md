---
schema: atelier.review
schema_version: 1
id: astromech-nfuc
title: 'R2 [medium] core/app.rs outbox_status + edsm/mod.rs pass: ERRORS_CLEARED is set to the pass''s wall second and the 203 that the same pass logs (same wall) is filtered by ''e.at > c'', so saving a key that is STILL wrong shows no error in Settings until the next retry a minute+ later - the exact flow this clearing serves. Reproduced: cleared=1800000006, last row at=1800000006 ''203 not found'' -> hidden. The app.rs test asserts the hiding at equal seconds. Fix: compare by send_log row id (store the last id at clearing) or use >= (an error logged in the clearing second is shown).'
target: astromech-ky3d
identity: review-agent (opus, independent)
commit: 769a9dea096cd5fb96e00ce173c87c92fc0134b2
created_at: 2026-10-17T05:37:39.417609385Z
event:
  event: finding
  recommendation: fix_now
---

R2 [medium] core/app.rs outbox_status + edsm/mod.rs pass: ERRORS_CLEARED is set to the pass's wall second and the 203 that the same pass logs (same wall) is filtered by 'e.at > c', so saving a key that is STILL wrong shows no error in Settings until the next retry a minute+ later - the exact flow this clearing serves. Reproduced: cleared=1800000006, last row at=1800000006 '203 not found' -> hidden. The app.rs test asserts the hiding at equal seconds. Fix: compare by send_log row id (store the last id at clearing) or use >= (an error logged in the clearing second is shown).
