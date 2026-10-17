---
created_at: "2026-10-15T20:36:13.649822543+00:00"
id: "astromech-kv9k"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-kv9k
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
closed_at: "2026-10-17T03:03:30.565472264+00:00"
status: "done"
title: "\"This market has N things the project needs\""
updated_at: "2026-10-17T03:03:30.565472264+00:00"
---

## Goal

On docking, say and show how many listed commodities this market sells.

## Done when

A docking fixture with a matching `Market.json` produces one line with the right count.

## Context

`Market.json` plus the construction tracker.

## Close Reason

Voice line 'Sells N project commodities.' once per dock on a fresh Market.json; page display not done
