---
created_at: "2026-10-15T20:36:13.648377218+00:00"
id: "astromech-qz9u"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-qz9u
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-5akj"
  - kind: "issue"
    id: "astromech-7ni2"
  - kind: "issue"
    id: "astromech-94j4"
  - kind: "issue"
    id: "astromech-m30a"
  - kind: "issue"
    id: "astromech-qdjp"
  - kind: "issue"
    id: "astromech-sfys"
  - kind: "issue"
    id: "astromech-zf3s"
  - kind: "issue"
    id: "astromech-znhx"
  children:
  - kind: "issue"
    id: "astromech-1imw"
  - kind: "issue"
    id: "astromech-h74l"
  - kind: "issue"
    id: "astromech-pxy2"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-zs2q"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Remote data layer and the \"search near me\" kit"
updated_at: "2026-10-16T19:02:54.519761424+00:00"
---

## Goal

One crate wraps Spansh, Ardent Insight and EDSM behind a cache, and one UI kit runs a search from the current or a typed system and sends a result to the route follower. The existing faction-state lookup runs on both.

## Done when

Crate tests against recorded responses (cache hit, expiry, schema mismatch, offline). The faction-state lookup works unchanged in the app (headless Chrome screenshot). Independent review. Delivers S1, S2, S3, S4, S5.

## Scope

New `astromech-data` crate (seeded from `galaxy::remote`), the shared Svelte search components, and porting the faction-state lookup.

## Unknowns

How Spansh's undocumented search endpoints behave on bad input and under load.

## Design

[ARCHITECTURE §18](docs/ARCHITECTURE.md) (one domain layer); [REPORT §4](docs/REPORT.md) (services).
