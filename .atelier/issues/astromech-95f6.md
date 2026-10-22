---
created_at: "2026-10-17T22:43:25.171840361+00:00"
id: "astromech-95f6"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Browser preview: lookup sections still answer sample results under ?core=empty"
updated_at: "2026-10-22T21:05:44.518253822+00:00"
---

## Goal

Under ?core=empty the network-lookup sections (Nearby, Sites, Colonies, stations, Where to sell) answer as a PC with no journal would.

## Done when

- galaxy.ts, sites.ts, stations.ts and colonies.ts answer through previewLookup (fakecore.ts): under ?core=empty with no typed system they reject with the shell's own "no position in the journal yet" (search_origin in app/src-tauri/src/main.rs); with a typed system they still answer the sample from it, as the network lookup does on a PC with no journal. ?fail= and ?slow= still apply (previewGate).
- lib/fakecore.preview.test.ts covers all four lookups; before and after screenshots of Exploration, Colonisation and Materials under ?core=empty are in docs/evidence/astromech-95f6/.

## Context

Found walking astromech-x81p. These are Spansh and Ardent lookups, not journal data, so the sample answer is defensible; the clutter is the only cost.

Decision (2026-10-22): mirror the shell rather than answer empty hits, since an empty answer would still say "Nearest to Sol", which a PC with no journal never shows. Trade's own lookups (trade.ts) keep their empty answers; they were not part of this record.
