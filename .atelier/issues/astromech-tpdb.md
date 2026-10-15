---
created_at: "2026-10-15T20:36:13.647402360+00:00"
id: "astromech-tpdb"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-7ni2"
  - kind: "issue"
    id: "astromech-cmhv"
  - kind: "issue"
    id: "astromech-ipb5"
  - kind: "issue"
    id: "astromech-k3wp"
  - kind: "issue"
    id: "astromech-p4ls"
  - kind: "issue"
    id: "astromech-tyjz"
  - kind: "issue"
    id: "astromech-znhx"
  children:
  - kind: "issue"
    id: "astromech-9uqb"
  - kind: "issue"
    id: "astromech-qgjp"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal and companion-file coverage"
updated_at: "2026-10-15T20:36:13.770967873+00:00"
---

## Scope

`astromech-journal` and `astromech-gamestate`. No new user-facing features.

## Outcome

Every journal event and companion file the lookup, EDDN and reaction epics need is parsed into a typed form, with the original JSON kept alongside, and companion files are re-read when their journal event fires.

## Proof

Parser tests on synthetic fixtures for every new event and file, including a raw JSON round-trip test (EDDN needs the original keys). Independent review.

## Delivers

S3, S4, S5, S6, S7, S8
