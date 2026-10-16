---
created_at: "2026-10-15T20:36:13.652051298+00:00"
id: "astromech-2u1x"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-2u1x
priority: "P1"
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
closed_at: "2026-10-16T22:29:24.654652876+00:00"
status: "done"
title: "Command palette (Ctrl+K)"
updated_at: "2026-10-16T22:29:24.654652876+00:00"
---

## Goal

Search across modules: systems, stations, modules, commodities, materials and blueprints, the fastest path to a lookup in two keystrokes.

## Done when

From any page, Ctrl+K then a module name opens its nearby search (UI check).

## Context

[ARCHITECTURE §18.2](docs/ARCHITECTURE.md).

## Close Reason

Ctrl+K from any page opens a palette of every page plus materials and the fleet's ships, fitted modules, blueprints and experimentals; Enter opens the route with the item selected, a material landing on its nearby search (UI-checked in the browser preview: Powerplay -> 'proto heat' -> #/materials/protoheatradiators with Nearest shown). Systems, stations and commodities have no landing page yet (astromech-4c6q, xqdl, tw2y). Review outstanding.
