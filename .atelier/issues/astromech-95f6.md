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
status: "todo"
title: "Browser preview: lookup sections still answer sample results under ?core=empty"
updated_at: "2026-10-17T22:43:25.171840361+00:00"
---

## Goal

Under ?core=empty the network-lookup sections (Nearby, Sites, Colonies, stations, Where to sell) answer as a PC with no journal would.

## Done when

galaxy.ts, sites.ts, stations.ts and colonies.ts use previewData with an empty answer under ?core=empty (or the preview documents why they do not), so empty-state screenshots of Exploration, Trade, Colonisation and Materials are not cluttered with Sol results. ?fail= and ?slow= already cover them (previewGate).

## Context

Found walking astromech-x81p. These are Spansh and Ardent lookups, not journal data, so the sample answer is defensible; the clutter is the only cost.
