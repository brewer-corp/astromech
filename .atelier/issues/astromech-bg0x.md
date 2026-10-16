---
created_at: "2026-10-15T20:36:13.659477764+00:00"
id: "astromech-bg0x"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-tqdh"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:17:12.227557488+00:00"
status: "done"
title: "Session summary"
updated_at: "2026-10-16T20:17:12.227557488+00:00"
---

## Goal

On demand or at logout: credits earned by activity, jumps, distance, bounties and data sold. Covers EDMC's hourly-income plugin.

## Done when

A fixture session gives the expected totals.

## Notes

- 2026-10-16 [note] From review of astromech-aonb: (R3) CarrierJump visits carry jump_dist 0.0 and look like FSD jumps; store NULL or a via column before counting jumps and distance. (R7) decide which credit events count as earnings (PowerplaySalary, ModuleSell, ship sales, SellDrones are left out today).

## Close Reason

History::session totals (credits by activity, per hour, trade profit, FSD jumps/distance, carrier jumps apart, bounties awarded, data sold) tested on fixture sessions; earnings list explicit (R7); carrier jumps via/NULL distance (R3); Session tab in the app. Voice/logout split to astromech-oo8y. e1a22ac.
