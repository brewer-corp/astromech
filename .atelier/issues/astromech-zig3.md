---
created_at: "2026-10-15T20:36:13.655361500+00:00"
id: "astromech-zig3"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
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
title: "Docked-here facts and an outfitting history"
updated_at: "2026-10-15T20:36:13.655361500+00:00"
---

## Goal

When docked, show services, economies, faction and state, pad size, and what this outfitting sells that the current build needs. Keep every `Outfitting.json` seen so "where did I see X?" works offline.

## Done when

Replaying a docking fixture shows the facts; a later offline query for a module seen there returns that station.

## Context

Journal and companion files only.
