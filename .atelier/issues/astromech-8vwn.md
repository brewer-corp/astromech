---
created_at: "2026-10-16T23:17:08.503987161+00:00"
id: "astromech-8vwn"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Test the truncated Ardent answer in Where to buy"
updated_at: "2026-10-16T23:17:08.503987161+00:00"
---

## Goal

Ardent sorts exports cheapest first, so a truncated answer drops dearer markets and can understate how many of the list a station stocks; the 'Too many markets' warning is the only signal, and nothing tests it.

## Done when

A Rust test covers a truncated answer in market::list and a component test renders the warning.

## Context

Deferred from the astromech-05bi review (crates/astromech-market/src/list.rs:151, app/src/lib/colonisation.ts:140).
