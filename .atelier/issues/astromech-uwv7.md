---
created_at: "2026-10-15T20:30:24.596242798+00:00"
id: "astromech-uwv7"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-egtp"
  - kind: "issue"
    id: "astromech-rxv2"
  children:
  - kind: "issue"
    id: "astromech-0cs5"
  - kind: "issue"
    id: "astromech-2bbl"
  - kind: "issue"
    id: "astromech-3fik"
  - kind: "issue"
    id: "astromech-elgr"
  - kind: "issue"
    id: "astromech-lwg0"
  - kind: "issue"
    id: "astromech-qx0m"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-hw3c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "EDDN sender"
updated_at: "2026-10-15T20:30:24.647189306+00:00"
---

## Outcome

astromech sends commodity, outfitting, shipyard, journal and the other live EDDN schemas from the live game, following the EDCD sender rules, with per-service settings. This is what EDMC does for EDDN.

## Proof

Message tests per schema against the EDDN schema files; a session against the `/test` schemas with no rejections; then a live session with no rejections, recorded as evidence. Independent review.

## Delivers

S6

## Unknowns

Whether the `blackmarket` schema is still live.
