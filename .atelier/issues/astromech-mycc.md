---
created_at: "2026-10-15T20:36:13.652916321+00:00"
id: "astromech-mycc"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-3njr"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Ships page: dense card view of the fleet"
updated_at: "2026-10-16T18:12:14.677258702+00:00"
---

## Goal

Replace the Builds page (`app/src/modules/engineering`) with a Ships page: one dense card per ship showing its name and ID, type, where it is, value and rebuy, a few headline stats (jump range, shield, DPS), and how far its engineering is done. Stored modules are listed with where they are, so the user can answer "where's my Krait?". Opening a card leads to the ship detail (the live stats task).

## Done when

A fixture fleet made from `StoredShips`, `StoredModules` and `Loadout` renders every ship and stored module. The old `#/engineering` route opens the Ships page. A screenshot of the fixture fleet is attached as evidence.

## Scope

Take the game's card idea, but make it denser. Headline stats come from the ship calculations (astromech-ojwr); until that lands, use the journal's `MaxJumpRange` and leave the other stats out.

## Context

`StoredShips` and `Loadout` already feed the engineering module. Add `StoredModules`.
