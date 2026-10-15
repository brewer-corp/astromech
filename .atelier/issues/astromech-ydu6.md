---
created_at: "2026-10-15T20:21:44.852344502+00:00"
id: "astromech-ydu6"
issue_type: "epic"
labels: []
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-0ov4"
  - kind: "issue"
    id: "astromech-13nd"
  - kind: "issue"
    id: "astromech-3e84"
  - kind: "issue"
    id: "astromech-ayvu"
  - kind: "issue"
    id: "astromech-hzlf"
  - kind: "issue"
    id: "astromech-kbyt"
  - kind: "issue"
    id: "astromech-osbx"
  children:
  - kind: "issue"
    id: "astromech-5rv8"
  - kind: "issue"
    id: "astromech-l4go"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xvhs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal and companion-file coverage"
updated_at: "2026-10-15T20:21:44.987929663+00:00"
---

## Scope

`astromech-journal` and `astromech-gamestate`. No new user-facing features.

## Outcome

Every journal event and companion file the lookup, EDDN and reaction epics need is parsed into a typed form, with the original JSON kept alongside, and companion files are re-read when their journal event fires.

## Proof

Parser tests on synthetic fixtures for every new event and file, including a raw JSON round-trip test (EDDN needs the original keys). Independent review.

## Delivers

S3, S4, S5, S6, S7, S8
