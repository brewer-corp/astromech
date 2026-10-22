---
created_at: "2026-10-22T20:41:32.451119558+00:00"
id: "astromech-0k88"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-0k88
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-6a7c"
  - kind: "issue"
    id: "astromech-6yt1"
  - kind: "issue"
    id: "astromech-k8yk"
  - kind: "issue"
    id: "astromech-lnzj"
  - kind: "issue"
    id: "astromech-zvs9"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T22:16:31.071673074+00:00"
status: "done"
title: "Shell and core hygiene"
updated_at: "2026-10-22T22:16:31.071673074+00:00"
---

## Goal

Smaller shell and core fixes: one URL filter for open_external, the window's config path, the Clock trait's home, GuiFocus panel values, and the window's start time after a core crash.

## Done when

Every child is done, independently reviewed and merged on this branch, with CONTRIBUTING.md's checks passing locally and on the PR's CI.

## Background

Grouped on 22 Oct 2026 so related review leftovers share one branch, one review and one CI run.

## Close Reason

Five shell and core hygiene fixes, independently reviewed (astromech-5ib2); k8yk measured on Windows CI; follow-ups astromech-fyvh, astromech-f2k1.
