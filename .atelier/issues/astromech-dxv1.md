---
created_at: "2026-10-17T22:52:51.528292386+00:00"
id: "astromech-dxv1"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "EDSM: three or more poisoned events due in one pass still stop it; test the reset on stored"
updated_at: "2026-10-22T20:58:06.713982705+00:00"
---

## Goal

After astromech-ix2p, the outage guard stops a pass after three single failures in a row, so three or more poisoned events due together still hold the rest of that pass. Also, removing the 'reset on stored' line breaks no test (re-review R1).

## Done when

A test with three poisoned events separated by good ones, all due in one pass, completes the pass (and fails without the reset); and either the three-in-a-row limit is lifted without letting a sustained outage burn the queue, or the limit is documented as accepted for best-effort upload.

## Notes

- 2026-10-22 [note] 22 Oct 2026: lifted the limit by ordering. Outbox::due now returns events never refused before those refused before (ORDER BY refusals > 0, id), so the good events due in a pass go before any known-poisoned ones; three or more poisoned events failing in a row still trip the outage guard, but by then only other refused events are left to wait. Outage: never-refused events fail first and stop the pass, so refused ones only use up refusals once every other event has one; the 21-event, 20-pass outage test now gives up 3 (was 6) and asserts at most OUTAGE_AFTER. Tests: an_event_stored_between_poisoned_ones_resets_the_outage_guard (fails with the reset removed: stores [1, 3], not [1, 3, 5]) and three_poisoned_events_side_by_side_do_not_hold_the_good_ones_behind (failed before the reorder: nothing stored). Side effect: during an outage the pass rate stays at one a minute while never-refused events remain (each pass gives OUTAGE_AFTER of them a first refusal), instead of slowing with the head events' back-off.
