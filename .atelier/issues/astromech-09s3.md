---
created_at: "2026-10-15T20:36:13.661131337+00:00"
id: "astromech-09s3"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-ah0i"
  - kind: "issue"
    id: "astromech-fq4o"
  - kind: "issue"
    id: "astromech-uxyq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:46:02.673510784+00:00"
status: "done"
title: "Settings → Data sharing"
updated_at: "2026-10-16T20:46:02.673510784+00:00"
---

## Goal

Per-service on/off, keys, status (last send, queue depth, last error) and a plain "what gets sent" text.

## Done when

Turning EDDN off stops sends (the outbox stays empty during a replay), and the status row shows the last error from a failed send.

## Notes

- 2026-10-16 [note] From the astromech-tyjz review (R7): when a service is switched off, purge its queued messages or stop queuing for it, and cap the outbox by row count and age so a long outage can't grow it without limit.
- 2026-10-16 [note] Also decide whether giving up after about 2 h of retries (60+...+3600 s, 8 tries) is the intended limit, or whether a long gateway outage should keep data queued longer (review of el1v, R7).
- 2026-10-16 [note] Also (review of xvjs, R6): Diagnostics shows the newest 50 send-log rows mixed, so skips push gateway answers off screen; add a filter or a separate list.
- 2026-10-16 [note] Decisions: (1) give-up limit: the 30-minute age cap (xvjs review R3) is intended and now governs (it ends retries after about 5 tries); EDDN wants live data, so a long outage drops data rather than sending it late. (2) R7: switching off purges the queue (worker, before any post) and stops queuing (feed); the outbox is capped at 2000 rows plus the 30-minute age. (3) Keys: EDDN has none; EDSM and Inara rows with keyring-stored keys come with astromech-ah0i and astromech-fq4o, so only EDDN is shown. (4) The feed now always runs (opens eddn.sqlite even while off) so switching on mid-session has the right header and location. (5) R6: Diagnostics lists gateway answers and skips separately.

## Close Reason

Done-when met at 5ec8728: turning EDDN off stops sends (feed queues nothing during live or replayed lines, worker posts nothing and discards the queue; tests), and the Data sharing status row shows the last error from a failed send (core eddn_log and app tests). Needs independent review.
