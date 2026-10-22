---
created_at: "2026-10-22T21:34:32.397967654+00:00"
id: "astromech-9kp2"
issue_type: "task"
labels:
- "size-s"
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
status: "todo"
title: "Setup shows 'Not set' before the first good status read"
updated_at: "2026-10-22T21:34:32.397967654+00:00"
---

## Goal

Until the core first answers, setup's push-to-talk and kill-switch rows say the status hasn't been read yet, not "Not set".

## Done when

Onboarding with a status read that has never succeeded shows a 'Checking...' or 'Could not read' state for those rows, with a component test.

## Context

Review finding R4 (astromech-cbbz) on feature/astromech-jo3l: Onboarding.svelte:32, StepButtons.svelte:65.
