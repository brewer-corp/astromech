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
closed_at: "2026-11-04T21:06:02.336912447+00:00"
status: "done"
title: "Setup shows 'Not set' before the first good status read"
updated_at: "2026-11-04T21:06:02.336912447+00:00"
---

## Goal

Until the core first answers, setup's push-to-talk and kill-switch rows say the status hasn't been read yet, not "Not set".

## Done when

Onboarding with a status read that has never succeeded shows a 'Checking...' or 'Could not read' state for those rows, with a component test.

## Context

Review finding R4 (astromech-cbbz) on feature/astromech-jo3l: Onboarding.svelte:32, StepButtons.svelte:65.

## Close Reason

Onboarding.svelte tracks why there is no status yet (checking until a read fails, then failed) and passes it to StepButtons, whose rows say 'Checking…' or 'Could not read' instead of 'Not set' while status is null; the push-to-talk row warns only when known unset or unreadable. Tests in Onboarding.test.ts use the fake core's ?slow=never:core_status and ?fail=core_status; both fail on the old components. StepHealth also receives a null status before the first read and was left as it is (out of scope).
