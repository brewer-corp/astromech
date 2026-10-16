---
created_at: "2026-10-15T20:36:13.647402360+00:00"
id: "astromech-tpdb"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-tpdb
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-09s3"
  - kind: "issue"
    id: "astromech-5el6"
  - kind: "issue"
    id: "astromech-7ni2"
  - kind: "issue"
    id: "astromech-cmhv"
  - kind: "issue"
    id: "astromech-ipb5"
  - kind: "issue"
    id: "astromech-k3wp"
  - kind: "issue"
    id: "astromech-lt2b"
  - kind: "issue"
    id: "astromech-p4ls"
  - kind: "issue"
    id: "astromech-thau"
  - kind: "issue"
    id: "astromech-w20r"
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
  - kind: "issue"
    id: "astromech-ogq4"
    type: "related"
  - kind: "issue"
    id: "astromech-rjww"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:47:49.778334288+00:00"
status: "done"
title: "Journal and companion-file coverage"
updated_at: "2026-10-16T18:47:49.778334288+00:00"
---

## Goal

Every journal event and companion file the lookup, EDDN and reaction work needs is parsed into a typed form, with the original JSON kept alongside, and companion files are re-read when their journal event fires.

## Done when

Parser tests on synthetic fixtures for every new event and file, including a raw JSON round-trip test (EDDN needs the original keys). Independent review. Delivers S3, S4, S5, S6, S7, S8.

## Scope

`astromech-journal` and `astromech-gamestate`. No new user-facing features.

## Close Reason

Both children done: 60+ journal events typed with raw JSON kept byte-for-byte (qgjp, scope note for Carrier*/Colonisation*; rest in astromech-rjww), and the seven companion files read on their events into game state, stale or missing files reported (9uqb). Review findings fixed in e71c2a3; R5/R7 in astromech-ogq4.
