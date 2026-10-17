---
created_at: "2026-10-17T17:17:34.412349339+00:00"
id: "astromech-t8hg"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ogq4"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check which Odyssey events rewrite Backpack.json and ShipLocker.json"
updated_at: "2026-10-17T17:17:34.412349339+00:00"
---

## Goal

Find out in the game which on-foot events rewrite Backpack.json or ShipLocker.json, so the companion-file triggers (`crates/astromech-journal/src/companion.rs`) re-read them at the right moments.

## Done when

A note lists, for each of those events, which file changed (watch the files' modified times while doing each action once), and an issue is filed for any trigger that is missing.

## Context

astromech-ogq4 found that the game rewrites Backpack.json with each BackpackChange, with no Backpack event. CollectItems, DropItems, UseConsumable, and micro-resource buy, sell, trade and transfer events may also rewrite one of the files; that can't be checked without the game.
