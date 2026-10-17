---
created_at: "2026-10-17T21:32:44.090774443+00:00"
id: "astromech-fg9l"
issue_type: "bug"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-fg9l
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Materials ledger counts collections past the cap"
updated_at: "2026-10-17T21:32:47.885902866+00:00"
---

## Goal

The materials ledger never counts a material above its grade's cap: the game stops at the cap but still logs the whole amount picked up.

## Done when

A ledger test from an anonymised excerpt: a `Materials` snapshot with a grade 5 material at 99, `MaterialCollected` `Count: 3`, then a `MaterialTrade` paying 96 of it leaves 4 (the next login's snapshot), not 6. Replaying the real journals (astromech-yin0 harness) shows no drift between the ledger and each login's `Materials` snapshot.

## Context

Found replaying real journals (astromech-yin0). The harness checks the ledger against every later `Materials` snapshot (ARCHITECTURE §18.5's self-check): 11 counts in 24 snapshots were off, every one a material collected past its cap (152 → 150, 102 → 100, 252 → 250, 117 → 100) or traded down after that (6 where the game had 4). The Materials page, its farm list and the cap callout read the inflated counts until the next login.
