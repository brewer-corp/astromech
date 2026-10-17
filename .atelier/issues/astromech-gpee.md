---
created_at: "2026-10-15T20:36:13.664784849+00:00"
id: "astromech-gpee"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T05:00:10.350579971+00:00"
status: "done"
title: "High-value discovery alert"
updated_at: "2026-10-17T05:00:10.350579971+00:00"
---

## Goal

On discovery, call out ELW, WW, AW, terraformables and high-value mapping targets; not "all bodies found", which the FSS shows.

## Done when

Replay: an ELW scan gets one line; FSSAllBodiesFound gets none.

## Context

A reaction rule.

## Close Reason

discovery callout rule built and replay-tested: ELW scan gets one line, FSSAllBodiesFound none.
