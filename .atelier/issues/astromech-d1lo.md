---
created_at: "2026-10-15T20:36:13.661681353+00:00"
id: "astromech-d1lo"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-d1lo
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
closed_at: "2026-10-17T00:28:47.912862643+00:00"
status: "done"
title: "Galaxy-wide best price for a commodity"
updated_at: "2026-10-17T00:28:47.912862643+00:00"
---

## Goal

List the top 100 importers and exporters of a commodity anywhere.

## Done when

A recorded-response test lists them in price order with data age.

## Context

Ardent `commodity/name/{c}/imports` and `/exports`.

## Close Reason

Built in 68d91e8: Ardent galaxy-wide imports/exports (ardent_galaxy), astromech_market::anywhere, shell command trade_anywhere, Trade > Sell/Buy Near/Anywhere switch. Proof: recorded-response test dsng, component tests rjdw, UI check 1u6l, full gates 3rg7. Open: trade_anywhere compiles only on Windows CI; independent review outstanding.
