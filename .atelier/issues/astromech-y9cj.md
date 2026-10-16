---
created_at: "2026-10-16T19:48:02.936492475+00:00"
id: "astromech-y9cj"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-k3wp"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Shopping list: follow carrier transfers and deaths"
updated_at: "2026-10-16T19:48:02.936492475+00:00"
---

## Goal

The colonisation shopping list's hold ignores `CargoTransfer` (Transfers[] Type/Count/Direction toship, tocarrier, tosrv), the main carrier-hauling workflow, so moving 200 steel from the carrier to the ship still says to buy 200. It also keeps cargo after `Died`.

## Done when

Tests: a carrier-to-ship transfer adds to the hold and ship-to-carrier subtracts; `Died` clears the hold.

## Context

`crates/astromech-colonisation/src/shopping.rs`. Found in review of astromech-i9w7 (round 2, R1, R2).
