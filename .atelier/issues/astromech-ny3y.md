---
created_at: "2026-10-15T20:36:13.657068282+00:00"
id: "astromech-ny3y"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:17:32.799554803+00:00"
status: "done"
title: "Mic calibration in Settings"
updated_at: "2026-10-16T19:17:32.799554803+00:00"
---

## Goal

Bring calibration into Settings (CLI-only today: `--calibrate-files`).

## Done when

Calibrating from Settings writes the same per-device gate the CLI does.

## Close Reason

Settings → Microphone → Calibration records through the core and stores the gate with astromech_core::calibration, the same writer and device key as --calibrate-files (test window_and_cli_store_the_same_gate; screenshots in docs/design/onboarding). Recording is Windows-only and compile-unverified on Linux. Independent review outstanding.
