---
created_at: "2026-11-07T11:28:47.118470266+00:00"
id: "astromech-5sp5"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: astromech-5sp5
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Move the CI's Atelier pin to a binary that reads the records this host writes"
updated_at: "2026-11-07T11:28:47.568980904+00:00"
---

## Goal

astromech's Atelier CI job pins `ATELIER_REV` 523a6502, older than the binary every agent on this host now runs. That binary writes `
