---
created_at: "2026-10-16T22:14:30.743942060+00:00"
id: "astromech-ha4m"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-ha4m
priority: "P2"
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
  - kind: "issue"
    id: "astromech-t71i"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "On foot: bartender trade plan for the shopping list"
updated_at: "2026-10-17T05:09:52.536747126+00:00"
---

## Goal

For the short assets on the On foot shopping list, suggest what to trade at a bartender from surplus (held items no picked blueprint needs) using barter values, like the ship materials trader swaps.

## Done when

A fixture with surplus assets and a shopping list short of others shows which surplus to trade, how much barter it gives, and what is still short after the trade.

## Context

Follow-up of astromech-j4c9, which shows each item's barter value and the barter value of the short assets.

## Unknowns

The wiki gives barter values for assets only; check in game whether goods and data can be bartered (values in data/catalog/odyssey.json).
