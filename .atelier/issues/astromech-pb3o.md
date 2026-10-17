---
created_at: "2026-10-17T07:47:00.196093328+00:00"
id: "astromech-pb3o"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-pb3o
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T08:30:03.147298368+00:00"
status: "done"
title: "device-overrides carry-over can leave a truncated file"
updated_at: "2026-10-17T08:30:03.147298368+00:00"
---

## Goal

`Paths::carry_over` (astromech-7w5d) copies straight into the final file name, so a crash or I/O error partway through leaves a truncated device-overrides.json in the data folder and the copy is never retried; the user's original stays in the app dir, so nothing is lost, only hidden (finding astromech-glkh). Copy to a temp file and link or rename it into place without overwriting.

## Done when

A test where the copy fails partway leaves no file at the target, and the next call copies it.

## Close Reason

carry_over now copies to a temp file in the data folder and hard-links it into place (fails if the target exists, so never overwrites), then removes the temp; a failed copy leaves no target and is retried. Test covers it.
