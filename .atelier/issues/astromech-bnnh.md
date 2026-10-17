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
closed_at: "2026-10-17T16:28:50.417993889+00:00"
status: "done"
title: "Should sustained DPS ever exceed burst DPS?"
updated_at: "2026-10-17T16:28:50.417993889+00:00"
---

## Context

Found in review of astromech-ojwr (R2). Time to drain is not affected. The ship detail (astromech-3njr) shows sustained DPS.

## Question

astromech-ship copies Coriolis, whose sustained-DPS factor leaves out charge time, so charged weapons can show sustained DPS above burst DPS (the Fer-de-Lance fixture: DPS 124.5, sustained 136.2 with rail guns). Show Coriolis-identical numbers, or depart from Coriolis so sustained never exceeds burst?

## Options

- Match Coriolis exactly (numbers users can cross-check, but some are impossible).
- Clamp the sustained factor at 1.
- Count charge time in the sustained rate (closer to the game, differs from Coriolis).

## Decision

Count charge time in the sustained rate, departing from Coriolis (Sumeet, 17 Oct 2026). Sustained DPS never exceeds burst DPS. The cause is in `sustained_factor` (`crates/astromech-ship/src/module.rs`, Coriolis's `getSustainedFactor`): the reload cycle counts `(clip - burst) / rof` plus the burst overhead and the reload, but not the last burst's charge, so a one-round clip such as a rail gun cycles in just the reload time. Users cross-checking against Coriolis will see different sustained numbers for charged weapons; the fix is also offered upstream.

## Close Reason

Count charge time; sustained never exceeds burst (Sumeet, 17 Oct)
