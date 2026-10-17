---
created_at: "2026-10-17T16:53:48.322291671+00:00"
id: "astromech-mf5e"
issue_type: "feature"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-mf5e
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-basj"
  - kind: "issue"
    id: "astromech-koso"
  - kind: "issue"
    id: "astromech-md1k"
  - kind: "issue"
    id: "astromech-stxg"
  - kind: "issue"
    id: "astromech-uvs6"
  - kind: "issue"
    id: "astromech-wi7q"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Alpha: a user guide for every feature, and a current README"
updated_at: "2026-10-17T17:04:03.942733519+00:00"
---

## Goal

A player can find out what astromech does, how to install it and how to use each feature without reading the design docs. Today there is no user documentation: the README is a developer page last shaped around the M0 preview and the gear-down slice, and ARCHITECTURE.md and V1-DESIGN.md are design records, not guides.

## Done when

Every child issue is done, and an independent docs check (someone who did not write the pages) has compared each guide page with the app built from main.

## Design

The guide is Markdown in the repo under `docs/guide/`, one page per area, rendered by GitHub and linked from the README and from each app page. Not the GitHub wiki: it is a separate repository outside review, and would drift from the code, which in-repo pages avoid because a feature PR updates its page in the same diff. Write for players, in the voice of the app's own copy (V1-DESIGN §6.5 copy rules): what it does, how to use it, what it needs (network, game data, settings), and its limits. Screenshots from `scripts/app-shots.sh` where they help.
