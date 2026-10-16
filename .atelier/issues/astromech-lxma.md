---
created_at: "2026-10-15T20:36:13.647655442+00:00"
id: "astromech-lxma"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-huab"
  children:
  - kind: "issue"
    id: "astromech-mmqs"
  - kind: "issue"
    id: "astromech-ny3y"
  - kind: "issue"
    id: "astromech-orel"
  - kind: "issue"
    id: "astromech-we0f"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "M4 exit: onboarding and settings"
updated_at: "2026-10-16T04:33:15.460439623+00:00"
---

## Goal

A fresh install walks the user from first launch to a working "gear down" by voice in under 5 minutes, and voice and calibration are set in Settings, not config files.

## Done when

A timed fresh-install run on a clean Windows account (evidence), plus UI checks. Independent review. Delivers v1.0 S2.

## Scope

Already built: Settings, key tester and Save diagnostics.

## Design

[V1-DESIGN §1.5](docs/V1-DESIGN.md) #1.
