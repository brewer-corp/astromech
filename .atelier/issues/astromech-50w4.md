---
created_at: "2026-10-15T20:30:24.597823195+00:00"
id: "astromech-50w4"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-7gvj"
  - kind: "issue"
    id: "astromech-ao79"
  - kind: "issue"
    id: "astromech-m7mt"
  - kind: "issue"
    id: "astromech-mwyp"
  - kind: "issue"
    id: "astromech-pj1t"
  - kind: "issue"
    id: "astromech-qzjf"
  - kind: "issue"
    id: "astromech-r5xs"
  children:
  - kind: "issue"
    id: "astromech-8mcc"
  - kind: "issue"
    id: "astromech-fcul"
  - kind: "issue"
    id: "astromech-ug08"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-hw3c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Remote data layer and the \"search near me\" kit"
updated_at: "2026-10-15T20:30:24.822028886+00:00"
---

## Scope

New `astromech-data` crate (seeded from `galaxy::remote`), the shared Svelte search components, and porting the faction-state lookup.

## Outcome

One crate wraps Spansh, Ardent Insight and EDSM behind a cache, and one UI kit runs a search from the current or a typed system and sends a result to the route follower. The existing faction-state lookup runs on both.

## Proof

Crate tests against recorded responses (cache hit, expiry, schema mismatch, offline). The faction-state lookup works unchanged in the app (headless Chrome screenshot). Independent review.

## Delivers

S1, S2, S3, S4, S5

## Unknowns

How Spansh's undocumented search endpoints behave on bad input and under load.

## Design

[ARCHITECTURE §18](docs/ARCHITECTURE.md) (one domain layer); [REPORT §4](docs/REPORT.md) (services).
