---
created_at: "2026-10-16T22:14:30.771392296+00:00"
id: "astromech-h9ch"
issue_type: "task"
labels:
- "needs-human"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-j4c9"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check the Odyssey game data against the game"
updated_at: "2026-10-16T22:14:30.771392296+00:00"
---

## Goal

Confirm in game the values `data/catalog/odyssey.json` is unsure of: its `uncertain` list (weapon upgrade prices the wiki leaves blank, sale prices for a few data and goods items, engineer/mod pairs the wiki contradicts itself on such as Hero Ferrari and Increased Sprint Duration, Rosa Dayette and Greater Range/Scope/Stability), and that the wiki infobox "value" is what a bartender pays.

## Done when

Each uncertain value is confirmed or corrected (in scripts/gen-odyssey.py as an override, or upstream on the wiki and regenerated), and the list in the file is shorter.

## Context

Follow-up of astromech-j4c9.
