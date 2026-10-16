---
created_at: "2026-10-15T20:36:13.647655442+00:00"
id: "astromech-lxma"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-lxma
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-huab"
  - kind: "issue"
    id: "astromech-we0f"
  children:
  - kind: "issue"
    id: "astromech-mmqs"
  - kind: "issue"
    id: "astromech-ny3y"
  - kind: "issue"
    id: "astromech-orel"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-c9em"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:56:26.120724320+00:00"
status: "done"
title: "M4 exit: onboarding and settings"
updated_at: "2026-10-16T19:56:26.120724320+00:00"
---

## Goal

A fresh install walks the user from first launch to a working "gear down" by voice in under 5 minutes, and voice and calibration are set in Settings, not config files.

## Done when

UI checks. Independent review. The timed fresh-install run on a clean Windows account (astromech-we0f) runs after this merges. Delivers v1.0 S2.

## Scope

Already built: Settings, key tester and Save diagnostics.

## Design

[V1-DESIGN §1.5](docs/V1-DESIGN.md) #1.

## Close Reason

All children done: mic calibration in Settings writes the same per-device gate as the CLI (ny3y), onboarding wizard from first launch through health check (mmqs), Settings voice responses with Kokoro v1.0 speaker, colour, speed and preview (orel). The timed fresh install on Windows is astromech-we0f, after the merge.
