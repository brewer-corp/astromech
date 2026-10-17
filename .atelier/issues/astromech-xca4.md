---
created_at: "2026-10-17T17:13:42.109076861+00:00"
id: "astromech-xca4"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-xca4
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T22:41:37.729773234+00:00"
status: "done"
title: "Sustained EPS and clamp: charged weapons and engineered reloads"
updated_at: "2026-10-17T22:41:37.729773234+00:00"
---

## Goal

Sustained EPS and time to drain use the same reload cycle as sustained DPS for charged weapons, and sustained DPS never exceeds burst for engineered builds.

## Done when

- Sustained EPS and time to drain use a cycle consistent with sustained DPS for charged weapons (found in review of astromech-tgar; today sustained EPS counts clip/rof plus reload, the DPS cycle counts charge but leaves out the last shot's interval), or the difference is documented as deliberate.
- An engineered build with the reload below the fire interval no longer shows sustained above burst (clamp the factor at 1, or count the last shot's interval), with a test on an engineered rail gun.

## Notes

- 2026-10-17 [note] Departures from Coriolis (decision astromech-bnnh): charged weapons (today only the three rail guns) draw over the DPS cycle, so the Fer-de-Lance fixtures' time to drain moves from Coriolis's 0:21 and 0:19 to 0:15 and 0:14; the sustained factor is clamped at 1. Non-charged weapons keep Coriolis's timeToDrainWep cycle on purpose (its draw cycle counts the last shot's interval and its damage cycle does not, so the two differ by that interval; cross-checking with Coriolis is the point). Sustained EPS old vs new, from the review: small rail gun 0.951 to 1.223, medium 1.686 to 2.323, medium burst 2.194 to 2.195 (per-shot draw per cycle).

## Close Reason

Charged weapons draw over the same cycle as their sustained damage and the sustained factor is clamped at 1; non-charged weapons keep Coriolis's draw cycle on purpose (documented). Tests for the rail gun draw and an engineered reload below the fire interval fail on main; Fer-de-Lance time to drain moves from 0:21/0:19 to 0:15/0:14. Review recorded.
