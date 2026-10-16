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
status: "todo"
title: "Run the EDDN sender in the core"
updated_at: "2026-10-16T18:47:49.293873074+00:00"
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
