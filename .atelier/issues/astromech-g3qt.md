---
created_at: "2026-10-22T20:41:31.444458042+00:00"
id: "astromech-g3qt"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-01c0"
  - kind: "issue"
    id: "astromech-2h0q"
  - kind: "issue"
    id: "astromech-54q6"
  - kind: "issue"
    id: "astromech-5gxp"
  - kind: "issue"
    id: "astromech-nrhv"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal pipeline and replay fixes"
updated_at: "2026-10-22T20:41:33.845826974+00:00"
---

## Goal

The journal pipeline treats replayed and live lines correctly at every hand-over: a touched older journal, a scheme change, a retried Market trigger, and background lookups that wait for late answers.

## Done when

Every child is done, independently reviewed and merged on this branch, with CONTRIBUTING.md's checks passing locally and on the PR's CI.

## Background

Grouped on 22 Oct 2026 so related review leftovers share one branch, one review and one CI run.
