---
created_at: "2026-10-16T18:12:15.174541300+00:00"
id: "astromech-361l"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-a41c"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Engineering unlock guide as a tech tree"
updated_at: "2026-10-16T22:10:41.812895374+00:00"
---

## Goal

A page showing how to unlock every engineer as a tech tree. Each engineer's requirements (introduction, unlock task, referral) are edges from the engineers or actions that satisfy them. The user's progress comes from the journal's `EngineerProgress`, so the next unlockable engineers stand out, with what to do and where.

## Done when

Given a fixture `EngineerProgress`, the tree marks known, invited, unlocked and locked engineers correctly, and for each locked one names the requirement that is left. Unlock data for every ship and suit engineer is in `data/` with a credit.

## Context

Related: tech broker unlocks (astromech-a41c) and Powerplay module unlocks (astromech-f01x).
