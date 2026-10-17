---
created_at: "2026-10-17T04:56:08.557454751+00:00"
id: "astromech-zc0v"
issue_type: "bug"
labels: []
priority: "P3"
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
status: "todo"
title: "EDDN outbox migrations can run twice when two connections open an old file together"
updated_at: "2026-10-17T04:56:08.557454751+00:00"
---

## Goal

The outbox reads `user_version` before its migration transaction and sets it after commit, so two connections opening an old file at once can both run a migration (ids purged between the runs could be reused once). Run each migration in `BEGIN IMMEDIATE`, re-read `user_version` inside it, and set it before commit, as astromech-n1ij does for astromech-store; share the helper if it fits.

## Done when

A test with two connections migrating the same old file at once runs each migration once.

## Context

From review finding astromech-1ca1 on astromech-8ozh (found by reading, not reproduced).
