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
    id: "astromech-n1ij"
    type: "related"
  - kind: "issue"
    id: "astromech-zs2q"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:20:15.736208158+00:00"
status: "done"
title: "Remote data layer and the \"search near me\" kit"
updated_at: "2026-10-16T19:27:17.429887072+00:00"
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

## Notes

- 2026-10-16 [note] Goal correction (review of 1d1921b): the kit does not send a result to the route follower. That part was split out to astromech-5akj (Go button), because the route follower is not on main yet. What qz9u delivered is the data crate, the search kit with current or typed origin, and remembered filters; the faction-state lookup runs on both.

## Close Reason

All children done: astromech-data crate with cache, schema checks, rate limits and plain-words errors (h74l); one search kit with current or typed origin and Spansh autocomplete, the faction-state lookup on it (pxy2); per-lookup remembered filters (1imw). Go to the route follower is astromech-5akj.
