---
created_at: "2026-10-15T20:36:13.646802031+00:00"
id: "astromech-tyjz"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-huab"
  children:
  - kind: "issue"
    id: "astromech-09s3"
  - kind: "issue"
    id: "astromech-5el6"
  - kind: "issue"
    id: "astromech-el1v"
  - kind: "issue"
    id: "astromech-lt2b"
  - kind: "issue"
    id: "astromech-thau"
  - kind: "issue"
    id: "astromech-w20r"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDDN sender"
updated_at: "2026-10-15T20:36:13.697686132+00:00"
---

## Outcome

astromech sends commodity, outfitting, shipyard, journal and the other live EDDN schemas from the live game, following the EDCD sender rules, with per-service settings. This is what EDMC does for EDDN.

## Proof

Message tests per schema against the EDDN schema files; a session against the `/test` schemas with no rejections; then a live session with no rejections, recorded as evidence. Independent review.

## Delivers

S6

## Unknowns

Whether the `blackmarket` schema is still live.
