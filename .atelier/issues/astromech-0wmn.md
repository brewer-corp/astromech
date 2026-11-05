---
created_at: "2026-11-05T06:47:09.663113633+00:00"
id: "astromech-0wmn"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-pey5"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Button: a flush variant instead of three negative-margin copies of its padding"
updated_at: "2026-11-05T06:47:09.663113633+00:00"
---

## Goal

Pages that need a Button aligned flush with text do it through the component, not by undoing its padding.

## Done when

- About, presets and NeutronFile no longer use `calc(-1 * var(--sp-2))` to cancel Button padding; Button offers the needed alignment once (a prop or class), using tokens only.
- Before/after screenshots of the three places show no layout change; `pnpm lint && pnpm check && pnpm test` pass.

## Context

Review finding astromech-5yze (reviewer-fable-pey5) on astromech-pey5, deferred.
