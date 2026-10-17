---
created_at: "2026-10-17T06:37:15.759516983+00:00"
id: "astromech-7w5d"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-7w5d
priority: "P2"
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
closed_at: "2026-10-17T07:38:35.720148792+00:00"
status: "done"
title: "Carry device-overrides.json over when the data folder isn't the app dir"
updated_at: "2026-10-17T07:38:35.720148792+00:00"
---

## Goal

astromech-4yds made the shell open device-overrides.json (saved binding overrides) in the core's data folder. A user with a data_dir override or a portable install who already saved overrides would find them gone: the file stays in the old app dir (finding astromech-wekk). Copy it across once when the new location has none. Also: the shell's portable detection uses its own exe folder, which matches the core only when both exes sit together, and the shell wiring has no test (finding astromech-dphg).

## Done when

A test: an overrides file in the old app dir with a data_dir override set is found after the change, and the copy happens once.

## Close Reason

Paths::carry_over copies once, never overwrites; shell calls it in make_api
