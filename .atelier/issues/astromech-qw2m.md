---
created_at: "2026-10-15T20:36:13.661950695+00:00"
id: "astromech-qw2m"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-qw2m
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
closed_at: "2026-10-17T02:43:19.338962034+00:00"
status: "done"
title: "Compare this market with nearby prices"
updated_at: "2026-10-17T02:43:19.338962034+00:00"
---

## Goal

When docked, compare each item carried or for sale here with the best nearby price: "you can sell gold for +3,400/t 12 ly away".

## Done when

A fixture `Market.json` plus recorded nearby prices gives the expected differences.

## Context

`Market.json` plus the commodity sell lookup.

## Close Reason

Docked market and hold compared with the best nearby prices (astromech-market::here, market_compare, Station page Market section); Done-when test crates/astromech-market/tests/here.rs passes
