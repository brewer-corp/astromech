---
created_at: "2026-10-17T04:56:07.943411514+00:00"
id: "astromech-nkjz"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-nkjz
priority: "P3"
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
closed_at: "2026-10-17T05:57:28.031394389+00:00"
status: "done"
title: "Spansh and Inara links on the current system when not docked"
updated_at: "2026-10-17T05:57:28.031394389+00:00"
---

## Goal

In space, show Spansh and Inara links for the current system (the Station page says "Not docked"; Materials and Powerplay show the system without links), using the ExtLinks component from astromech-qiey.

## Done when

A page test shows the system links with no station docked.

## Context

From review finding astromech-ur4h on astromech-qiey.

## Close Reason

Station, Materials and Powerplay link the current system when not docked; Station page test covers no station docked. Gates green at a7735a7; review outstanding.
