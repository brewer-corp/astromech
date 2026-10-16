---
created_at: "2026-10-16T18:47:49.293873074+00:00"
id: "astromech-xvjs"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-uxyq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:02:14.277330776+00:00"
status: "done"
title: "Run the EDDN sender in the core"
updated_at: "2026-10-16T20:02:14.277330776+00:00"
---

## Goal

The core feeds the live journal into `astromech-eddn` and delivers the outbox in the background, without network calls ever holding up journal processing.

## Done when

Tests show:
- each journal is replayed into the sender from its `Fileheader`, so the header, flags, commander and last system are right before any message is built;
- a delivery worker owns its own `Outbox` and `HttpTransport`: no lock is held across a network call, and `observe`/`submit` never wait on HTTP (review R2 on astromech-el1v);
- a pass stops at the first connection error, `408` or `5xx` instead of making one request per queued message (R3);
- the 1-minute minimum before a retry doesn't depend on the wall clock alone, so a clock jump can't cause an early retry (R4);
- the Diagnostics send log shows the day or date, and `dropped` rows can't push sent, rejected and retry entries out of the kept log (R8).

## Context

`astromech-eddn` (astromech-el1v) provides `Sender`, `Outbox::deliver_due` and the `eddn_log` query, but nothing in the core runs them yet. Found in the independent review of feature/astromech-tyjz.

## Notes

- 2026-10-16 [note] From review of lt2b/thau: (R1) don't enable outfitting/2 until PowerPlay and tech-broker modules are filtered: journal Outfitting.json has no sku, so the README's commander-status rule never fires; add a denylist of those module names (wiki-visible data) or file a decision. (R2) send companion-file messages only for live tailed events, not replay; if real logs show file and event timestamps always equal, require equality. (R5) write each companion::Unusable to the send log like Dropped results.
- 2026-10-16 [note] Decisions: nothing from the startup catch-up is sent (Developers.md has no explicit rule; EDDN consumers read messages as current, so replayed lines only rebuild the header/commander/location); a journal the game starts while astromech runs is live from its Fileheader. EDDN is off unless [eddn] enabled = true in astromech.toml (09s3 should move this into Settings); debug builds always use /test. outfitting/2 stays off (companion::SEND_OUTFITTING) and each Outfitting visit logs a skip. One companion trigger rule (event, MarketID, file >= event) for commodity/outfitting/shipyard/navroute/fcmaterials; uxyq decides equality. The Diagnostics view still shows the newest 50 rows mixed; the kept log caps dropped rows (200) apart from gateway answers (500). Shutdown can wait up to the 30 s HTTP timeout if a post is in flight.

## Close Reason

Done-when met at 06e57df (evidence astromech-ckvr, astromech-ut2b, astromech-04bk): replay from Fileheader, worker with its own outbox and transport (no lock across HTTP), pass stops at first outage, monotonic retry minimum, dated log and separate drop cap; plus live-only sending, outfitting gated, Unusable/skips logged. Needs independent review.
