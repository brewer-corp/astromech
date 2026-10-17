---
created_at: "2026-10-16T20:41:03.298907879+00:00"
id: "astromech-gtza"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-gtza
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T02:42:03.989787375+00:00"
status: "done"
title: "Speak the system's Powerplay state on arrival"
updated_at: "2026-10-17T02:42:03.989787375+00:00"
---

## Goal

Speak the system's Powerplay state when arriving by jump, when the commander has the callout enabled. Split from astromech-h1dz: the arrival state, its summary and the spoken line are built (`astromech_powerplay::arrival_line`, `GameState.system_power`); the reaction engine with per-rule toggles is on feature/astromech-cmhv, not main.

## Done when

A callout rule (`powerplay_arrival`, off by default unless Sumeet decides otherwise) speaks `arrival_line` on `FSDJump`/`CarrierJump` into a system with Powerplay activity; a replay test shows it speaks with the setting on and nothing is spoken when the setting is off, and replay catch-up is silent.

## Context

`astromech_powerplay::arrival_line(&event, pledged)` returns the line or `None` (no Powerplay activity; `Location` is not an arrival). The pledged power is `GameState.powerplay.power`. The game shows this state only in the system and galaxy maps, not on the HUD.

## Close Reason

powerplay_arrival rule shipped, off by default; replay tests cover on, off and catch-up
