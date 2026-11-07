---
created_at: "2026-11-07T04:51:22.113467059+00:00"
id: "astromech-7rcn"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-8zdt"
  - kind: "issue"
    id: "astromech-xil2"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "The window reaches its backend through one transport"
updated_at: "2026-11-07T04:51:22.542630080+00:00"
---

## Goal

The Svelte app calls its backend through one `call()` transport picked once at startup, so the fake lives apart from the app, never ships, and UI behaviour tests run in real Chromium against it. This is the window side of decision astromech-1zhg, and it needs no Rust change.

## Done when

- [ ] Every child is done.
- [ ] `cd app && pnpm -s check && pnpm -s lint && pnpm -s test` pass, and the browser preview (`?core=ready`, `?route=exact`, `?fail=`, `?slow=`) and `scripts/preview-publish` behave as before: every page opens with its sample data, checked with `scripts/ui` screenshots of each top-level page before and after.

## Context

Strategy docs/strategies/one-tested-process.md; decision astromech-1zhg.
