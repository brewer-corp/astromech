---
created_at: "2026-10-15T20:36:13.652916321+00:00"
id: "astromech-mycc"
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
title: "Fleet page"
updated_at: "2026-10-15T20:36:13.652916321+00:00"
---

## Goal

Every ship with location, value, loadout and engineering, plus stored modules with location: "where's my Krait?"

## Done when

Fixture `StoredShips`, `StoredModules` and `Loadout` give the full fleet and module list.

## Context

`StoredShips` and `Loadout` already feed the engineering module (partly built). Add `StoredModules` and a fleet page.
