---
created_at: "2026-10-15T20:36:13.660178701+00:00"
id: "astromech-lt2b"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-uxyq"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "EDDN commodity/3"
updated_at: "2026-10-16T19:15:44.543727774+00:00"
---

## Goal

Send `commodity/3` from `Market.json` on the `Market` event, with the prohibited list and economies.

## Done when

A fixture `Market.json` produces a message that validates against the live `commodity/3` schema.

## Context

This is the data Inara, Spansh and Ardent all run on.
