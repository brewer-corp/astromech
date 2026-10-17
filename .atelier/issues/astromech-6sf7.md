---
created_at: "2026-10-17T07:38:09.240690092+00:00"
id: "astromech-6sf7"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-e8ux"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Voice query for nearby Boom, Famine and War systems"
updated_at: "2026-10-17T07:38:09.240690092+00:00"
---

## Goal

The faction-state system lookups that feed the engineering shopping list and HGE farming (`astromech_galaxy::remote::QUERIES`: Famine, Boom, War and the others) can be asked by voice from the current system, e.g. "nearest famine system", "where is there a boom near me". Today they exist only in the app, so S3 of the toolbox strategy ("in the app and as a voice query"; ARCHITECTURE §18.7: a module is not done until its voice_exposed queries have phrase templates) is not met.

## Done when

Phrase templates in the grammar for each state query, dispatched to the same Spansh search with the remembered filters (`faction.<id>`), and a spoken answer with system, distance and data age. Tests: phrase-to-command tests and an answer test against the recorded Spansh fixtures (e.g. `spansh-famine.json`). Independent review.

## Context

Found by the S1-S8 validation (astromech-e8ux).
