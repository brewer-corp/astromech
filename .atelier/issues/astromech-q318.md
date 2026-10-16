---
created_at: "2026-10-16T18:12:14.888713530+00:00"
id: "astromech-q318"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-q318
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-1b43"
  - kind: "issue"
    id: "astromech-snqt"
  children:
  - kind: "issue"
    id: "astromech-fa3g"
  - kind: "issue"
    id: "astromech-g5rw"
  - kind: "issue"
    id: "astromech-kbcm"
  - kind: "issue"
    id: "astromech-pxeh"
  - kind: "issue"
    id: "astromech-rhfn"
  - kind: "issue"
    id: "astromech-vmot"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-54hd"
    type: "related"
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-xtly"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:25:49.729591449+00:00"
status: "done"
title: "v1.0 UI refinement"
updated_at: "2026-10-16T19:25:49.729591449+00:00"
---

## Goal

Many screens still look rough. Before v1.0, fix the specific layout and interaction problems Sumeet reported on 2026-10-16, then have Sumeet walk every screen and sign it off.

## Done when

Every child is done and the work has had independent review. Sumeet's walkthrough of every screen (astromech-snqt) runs after this merges.

## Close Reason

All six fixes done: keybinds panels open on their own (kbcm, rhfn), My controller and Activities merged (g5rw), Materials detail below the grid (pxeh), system-name chips (vmot), voice command tester with dry run and a guarded live run (fa3g). Sumeet's screen walk is astromech-snqt, after the merge.
