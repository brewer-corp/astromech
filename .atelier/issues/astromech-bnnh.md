---
created_at: "2026-10-16T19:06:19.464885070+00:00"
id: "astromech-bnnh"
issue_type: "decision"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-z4jb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Should sustained DPS ever exceed burst DPS?"
updated_at: "2026-10-16T19:06:19.464885070+00:00"
---

## Context

Found in review of astromech-ojwr (R2). Time to drain is not affected. The ship detail (astromech-3njr) shows sustained DPS.

## Question

astromech-ship copies Coriolis, whose sustained-DPS factor leaves out charge time, so charged weapons can show sustained DPS above burst DPS (the Fer-de-Lance fixture: DPS 124.5, sustained 136.2 with rail guns). Show Coriolis-identical numbers, or depart from Coriolis so sustained never exceeds burst?

## Options

- Match Coriolis exactly (numbers users can cross-check, but some are impossible).
- Clamp the sustained factor at 1.
- Count charge time in the sustained rate (closer to the game, differs from Coriolis).
