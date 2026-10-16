---
created_at: "2026-10-15T20:36:13.656940914+00:00"
id: "astromech-mmqs"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-we0f"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:17:33.241670777+00:00"
status: "done"
title: "Onboarding wizard"
updated_at: "2026-10-16T19:17:33.241670777+00:00"
---

## Goal

Find Elite, detect devices, bind PTT and the kill switch, pick and check the mic, 3-phrase calibration, and a health check with fixes.

## Done when

Each step works against recorded inputs (no Elite install, no devices, no mic) and is covered by tests; a fresh profile walks through every step in the app with screenshots as evidence.

## Close Reason

Onboarding wizard: six steps, each tested against recorded inputs (no Elite, no devices, no mic) in vitest and core tests; fresh-profile walk screenshotted in the Vite preview (docs/design/onboarding). The real-app timed walk on Windows is astromech-we0f. src-tauri changes compile-unverified locally. Independent review outstanding.
