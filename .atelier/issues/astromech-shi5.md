---
created_at: "2026-10-17T04:57:57.648156899+00:00"
id: "astromech-shi5"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Upload to EDSM from a live session"
updated_at: "2026-10-17T04:57:57.648156899+00:00"
---

## Goal

Confirm the EDSM journal upload (astromech-ah0i) against EDSM from the real game with a real commander and API key.

## Done when

With the EDSM commander name and API key set in Settings → Data sharing and upload on, a live session is accepted by EDSM: the Data sharing page shows sent batches and no last error, and the jumps appear in the EDSM flight log (the page's Show button, or edsm.net) (evidence).

## Context

- The fixture proof (requests exclude everything on the discard list) is in `crates/astromech-eddn/src/edsm/tests.rs`; this checks the real API, the key in Windows Credential Manager, and real journals.
- Also check: whether EDSM answers per-event codes for every batched event as assumed (msgnum 100 with an `events` list the length of the batch); that saving the key works on Windows (Credential Manager entry `edsm-api-key.astromech`); and that turning EDMC's EDSM upload off first avoids duplicates (EDSM answers 101/103 for duplicates, counted as sent).
