---
created_at: "2026-10-15T20:21:44.853329850+00:00"
id: "astromech-ejq7"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-13nd"
  - kind: "issue"
    id: "astromech-6ftp"
  - kind: "issue"
    id: "astromech-b15g"
  - kind: "issue"
    id: "astromech-hzlf"
  - kind: "issue"
    id: "astromech-jj0p"
  - kind: "issue"
    id: "astromech-mbnm"
  - kind: "issue"
    id: "astromech-wgv2"
  children:
  - kind: "issue"
    id: "astromech-0uao"
  - kind: "issue"
    id: "astromech-r1pt"
  - kind: "issue"
    id: "astromech-yq5f"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xvhs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Remote data layer and the \"search near me\" kit"
updated_at: "2026-10-15T20:21:45.069603845+00:00"
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
