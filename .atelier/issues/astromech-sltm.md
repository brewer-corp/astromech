---
created_at: "2026-10-16T19:21:14.595412647+00:00"
id: "astromech-sltm"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-sltm
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-rhfn"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "A click in the bind dialog captures Mouse 1 as the new binding"
updated_at: "2026-10-17T00:15:31.525486589+00:00"
---

## Goal

Clicking inside the bind dialog in the app (for example the copy-name box shown for every preset that can't be edited) no longer records Mouse 1 as the new binding.

## Done when

A Mouse_1 press that arrives while the bind dialog is capturing (the window's own click, forwarded by Raw Input with INPUTSINK) is not taken as the binding, and Save never writes Mouse_1 from a click inside the dialog; a component test shows it, and binding a real Mouse_4 still works.

## Context

`app/src/lib/ui/BindDialog.svelte:26-31`. Same root cause as astromech-rhfn (the window's clicks reach Raw Input as Mouse_1-3); found in the re-review of feature/astromech-q318. Already on main.
