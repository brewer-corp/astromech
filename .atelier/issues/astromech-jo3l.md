---
created_at: "2026-10-22T20:41:31.580037529+00:00"
id: "astromech-jo3l"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-jo3l
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-83og"
  - kind: "issue"
    id: "astromech-95f6"
  - kind: "issue"
    id: "astromech-ak62"
  - kind: "issue"
    id: "astromech-cvxf"
  - kind: "issue"
    id: "astromech-nhyb"
  - kind: "issue"
    id: "astromech-v65y"
  - kind: "issue"
    id: "astromech-wv9x"
  - kind: "issue"
    id: "astromech-zkz8"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "App polish from the alpha reviews"
updated_at: "2026-10-22T20:57:49.767769698+00:00"
---

## Goal

The app window's leftover review findings are fixed: errors that vanish or fail silently, preview states, narrow layouts, keyboard focus, and stale lines on the Mining, Route and Settings pages.

## Done when

Every child is done, independently reviewed and merged on this branch, with CONTRIBUTING.md's checks passing locally and on the PR's CI.

## Background

Grouped on 22 Oct 2026 so related review leftovers share one branch, one review and one CI run.
