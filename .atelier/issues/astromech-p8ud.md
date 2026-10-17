---
created_at: "2026-10-16T20:14:13.392846043+00:00"
id: "astromech-p8ud"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-cmhv"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Prohibited goods in the scan callout"
updated_at: "2026-10-16T20:14:13.392846043+00:00"
---

## Goal

A cargo scan also warns when goods aboard are prohibited in the current system ("Cargo scan. Prohibited goods aboard."), not only when they are stolen.

## Done when

Replay test: with a fixture cargo and a prohibited list for the system, the cargo scan line gets the prohibited suffix; without, it does not.

## Context

astromech-0zcu built the scan callout with the stolen-goods suffix only. The journal and its companion files (`Market.json` has no prohibited flag) do not say which goods are prohibited where; a source is needed first (Frontier cAPI market `prohibited`, or EDDN/Spansh market data).
