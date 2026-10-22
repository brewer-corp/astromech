---
created_at: "2026-10-22T20:41:31.754884800+00:00"
id: "astromech-uuqt"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-uuqt
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-550b"
  - kind: "issue"
    id: "astromech-joma"
  - kind: "issue"
    id: "astromech-sdg7"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Voice and callout copy fixes"
updated_at: "2026-10-22T22:50:00.408465434+00:00"
---

## Goal

Spoken lines read names and numbers correctly, stay within the callout word budget with long system names, the speech thread stops promptly, and the synthetic eval's weak phrase groups are fixed.

## Done when

Every child is done, independently reviewed and merged on this branch, with CONTRIBUTING.md's checks passing locally and on the PR's CI.

## Background

Grouped on 22 Oct 2026 so related review leftovers share one branch, one review and one CI run.
