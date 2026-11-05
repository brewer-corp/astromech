---
created_at: "2026-11-05T06:19:46.350866500+00:00"
id: "astromech-5rm2"
issue_type: "task"
labels:
- "needs-human"
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-os70"
    type: "related"
  - kind: "issue"
    id: "astromech-yn6y"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ask Spansh's maintainer about linked-place query volume"
updated_at: "2026-11-05T06:19:46.350866500+00:00"
---

## Goal

Ask Spansh's maintainer whether astromech may run linked-place queries against Spansh's search API at the volume the engine needs (spike astromech-yn6y open item). Asked in parallel: the recipes ship within the spike's budget without waiting for the answer (decision astromech-g8d5, item 9). Each recipe costs 2 to 7 Spansh requests per query at 1 request/s, and recipe 1 moved 14 to 27 MB of station records per query; a local station file built from the public dumps would bring that to about 1 request.

## Done when

Spansh's maintainer has answered (or a stated time has passed with no answer), and the answer is recorded as a note here with what it means for the recipes (astromech-sgwi) and the local station file (astromech-ibmy).

## Context

Spike close reason in astromech-yn6y (request counts per recipe, User-Agent, rate limit); the engine, astromech-xlbz, in astromech-os70.
