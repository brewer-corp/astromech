---
created_at: "2026-10-15T20:30:24.596853457+00:00"
id: "astromech-qe9i"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-kbsk"
  - kind: "issue"
    id: "astromech-m7mt"
  - kind: "issue"
    id: "astromech-mipe"
  - kind: "issue"
    id: "astromech-mwyp"
  - kind: "issue"
    id: "astromech-r2zw"
  - kind: "issue"
    id: "astromech-tel1"
  - kind: "issue"
    id: "astromech-uwv7"
  children:
  - kind: "issue"
    id: "astromech-o9d4"
  - kind: "issue"
    id: "astromech-ygpn"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-hw3c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal and companion-file coverage"
updated_at: "2026-10-15T20:30:24.720832911+00:00"
---

## Scope

`astromech-journal` and `astromech-gamestate`. No new user-facing features.

## Outcome

Every journal event and companion file the lookup, EDDN and reaction epics need is parsed into a typed form, with the original JSON kept alongside, and companion files are re-read when their journal event fires.

## Proof

Parser tests on synthetic fixtures for every new event and file, including a raw JSON round-trip test (EDDN needs the original keys). Independent review.

## Delivers

S3, S4, S5, S6, S7, S8
