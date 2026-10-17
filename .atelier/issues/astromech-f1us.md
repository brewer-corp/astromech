---
created_at: "2026-10-16T22:14:30.718314856+00:00"
id: "astromech-f1us"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-f1us
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-d0zh"
    type: "related"
  - kind: "issue"
    id: "astromech-j4c9"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T05:16:39.401461087+00:00"
status: "done"
title: "On foot: owned suits and weapons with their grades and mods"
updated_at: "2026-10-17T05:16:39.401461087+00:00"
---

## Goal

The On foot page (astromech-odyssey) knows which suits and weapons the commander owns, their current grade and fitted mods, from the journal (`BuySuit`, `BuyWeapon`, `UpgradeSuit`, `UpgradeWeapon`, `SuitLoadout`, `SellSuit`, `SellWeapon`), so the blueprint picker starts from the real grade and hides mods already fitted.

## Done when

A synthetic journal with suit and weapon purchases and upgrades gives each owned item with its grade and mods, and picking an owned Maverick offers upgrades from its current grade.

## Context

Follow-up of astromech-j4c9, which built the picker with a free choice of grade range.

## Close Reason

Owned suits and weapons with grade and mods come from the journal; the picker upgrades owned gear from its current grade and hides fitted mods. Proof at 8c44bd9; review outstanding (orchestrator arranges).
