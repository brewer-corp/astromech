---
created_at: "2026-10-17T16:54:23.485268262+00:00"
id: "astromech-flru"
issue_type: "task"
labels:
- "alpha"
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-flru
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Onboarding smoke test to a mock gear-down on Windows CI"
updated_at: "2026-10-17T19:05:03.187563716+00:00"
---

## Goal

Prove the fresh-install path without the game: on the Windows runner, install silently, drive the onboarding wizard, and reach a verified gear-down against `mockgame`, with timings.

## Done when

A CI job (or script run on the Windows runner) reports install-to-gear-down time and passes; the run is linked as evidence.

## Context

`.github/workflows/release.yml` already does a silent per-user install, start, status, quit and uninstall. Voice can't be spoken on the runner: feed a recorded or rendered 'gear down' clip through the core's test input instead of a microphone. astromech-we0f (a timed human run) still follows.
