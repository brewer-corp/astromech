---
created_at: "2026-10-17T05:19:02.900011860+00:00"
id: "astromech-t71i"
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
    id: "astromech-ha4m"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Check the bartender's barter rules against the game"
updated_at: "2026-10-17T05:19:02.900011860+00:00"
---

## Goal

Check in game how the bartender's barter works, which the On foot page's trade plan (astromech-ha4m) assumes without having seen: that goods and data cannot be bartered (the wiki gives barter values for assets only; `data/catalog/odyssey.json` has none for goods or data), that a swap takes equal barter value both ways (no margin or fee), that the bartender will hand over whatever is asked for (no stock limit), and what happens to a trade that overshoots.

## Done when

Each assumption is confirmed or corrected. If goods or data can be bartered, their barter values are added to `scripts/gen-odyssey.py` and the catalogue regenerated so the plan covers them; if the rate differs, `crates/astromech-odyssey/src/barter.rs` follows it.

## Context

Follow-up of astromech-ha4m. Like astromech-h9ch, it needs the game.
