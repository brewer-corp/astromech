---
created_at: "2026-10-15T20:21:44.851745926+00:00"
id: "astromech-0ov4"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-lstz"
  children:
  - kind: "issue"
    id: "astromech-74wq"
  - kind: "issue"
    id: "astromech-9sjx"
  - kind: "issue"
    id: "astromech-ebsc"
  - kind: "issue"
    id: "astromech-eyzg"
  - kind: "issue"
    id: "astromech-pln9"
  - kind: "issue"
    id: "astromech-pt16"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xvhs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDDN sender"
updated_at: "2026-10-15T20:21:44.921986692+00:00"
---

## Outcome

astromech sends commodity, outfitting, shipyard, journal and the other live EDDN schemas from the live game, following the EDCD sender rules, with per-service settings. This is what EDMC does for EDDN.

## Proof

Message tests per schema against the EDDN schema files; a session against the `/test` schemas with no rejections; then a live session with no rejections, recorded as evidence. Independent review.

## Delivers

S6

## Unknowns

Whether the `blackmarket` schema is still live.
