---
created_at: "2026-10-16T21:04:20.112097882+00:00"
id: "astromech-proz"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-zf3s"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Trade lookups: share the market cache between voice and app; partial buyer fallback per commodity"
updated_at: "2026-10-16T21:04:20.112097882+00:00"
---

## Goal

Voice and the Trade page give the same trade answers, and a large mined load always gets a per-commodity suggestion.

## Done when

- Voice trade questions and the Trade page use one Ardent cache, so an answer given by voice and the page within 15 minutes agree (test: a voice lookup then an app lookup on recorded answers make one request).
- When no market has demand for the whole of a commodity, My cargo shows the best partial buyer (with how much it takes) instead of "No market with demand for all of it"; voice says the same. Recorded-response test.

## Context

From independent review of astromech-zf3s (findings R3, R5). R3: `Assistant` in astromech-core builds its own in-memory `astromech_data::Client`, never shared with the shell's galaxy client, so the two can disagree for up to the 15-minute market TTL. A file-backed `Cache` in the shared data dir is one way. R5: `astromech_market::cargo::plan` per-commodity best uses `single_filters` (demand >= tonnes carried); mined loads of hundreds of tonnes often find none while partial buyers exist.
