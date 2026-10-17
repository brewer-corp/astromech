---
schema: atelier.review
schema_version: 1
id: astromech-9gtn
title: 'Not approved: 2 FIX NOW (R1 astromech-bo8g, R2 astromech-nfuc). Read git diff main...HEAD at 769a9de against ky3d (69wx, 3pkl, nhhq). Ran cargo test -p astromech-eddn -p astromech-core: all pass. Mutations in a scratch copy (dropping 429 from the hold list, off-by-one on the give-up limit, removing make_due) each failed a new test, so the tests catch what they claim. Scratch tests reproduced R1 (7 held tries then one 403 gives up the whole batch as ''refused 8 times'') and R2 (a still-wrong key''s new 203 is logged in the same second as errors_cleared_at and is hidden). Checked: the classification (transport/408/429/5xx -> hold; other statuses, unreadable 200 body, wrong-length list -> refused; 201-203 -> hold), the wake only on changed credentials (not on re-save or turning off), the flight log''s 10 s against the shell''s 15 s per-read socket timeout, the timeout message carrying no URL or key, and the shared outbox and worker additions (make_due, waker) being unused by EDDN. Not run: the pnpm gates and a live EDSM run.'
target: astromech-ky3d
identity: review-agent (opus, independent)
commit: 769a9dea096cd5fb96e00ce173c87c92fc0134b2
created_at: 2026-10-17T05:37:46.208882827Z
event:
  event: review
---

Not approved: 2 FIX NOW (R1 astromech-bo8g, R2 astromech-nfuc). Read git diff main...HEAD at 769a9de against ky3d (69wx, 3pkl, nhhq). Ran cargo test -p astromech-eddn -p astromech-core: all pass. Mutations in a scratch copy (dropping 429 from the hold list, off-by-one on the give-up limit, removing make_due) each failed a new test, so the tests catch what they claim. Scratch tests reproduced R1 (7 held tries then one 403 gives up the whole batch as 'refused 8 times') and R2 (a still-wrong key's new 203 is logged in the same second as errors_cleared_at and is hidden). Checked: the classification (transport/408/429/5xx -> hold; other statuses, unreadable 200 body, wrong-length list -> refused; 201-203 -> hold), the wake only on changed credentials (not on re-save or turning off), the flight log's 10 s against the shell's 15 s per-read socket timeout, the timeout message carrying no URL or key, and the shared outbox and worker additions (make_due, waker) being unused by EDDN. Not run: the pnpm gates and a live EDSM run.
