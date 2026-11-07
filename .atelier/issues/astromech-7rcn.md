---
created_at: "2026-11-07T04:51:22.113467059+00:00"
id: "astromech-7rcn"
issue_type: "feature"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-7rcn
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
closed_at: "2026-11-07T06:56:14.651535630+00:00"
status: "done"
title: "The window reaches its backend through one transport"
updated_at: "2026-11-07T06:56:14.651535630+00:00"
---

## Goal

The Svelte app calls its backend through one `call()` transport picked once at startup, so the fake lives apart from the app, never ships, and UI behaviour tests run in real Chromium against it. This is the window side of decision astromech-1zhg, and it needs no Rust change.

## Done when

- [x] Every child is done.
- [x] `cd app && pnpm -s check && pnpm -s lint && pnpm -s test` pass, and the browser preview (`?core=ready`, `?route=exact`, `?fail=`, `?slow=`) and `scripts/preview-publish` behave as before: every page opens with its sample data, checked with `scripts/ui` screenshots of each top-level page before and after.

## Context

Strategy docs/strategies/one-tested-process.md; decision astromech-1zhg.

## Close Reason

Both children done: astromech-xil2 (one transport, fake in app/src/preview, dist check, lint rule) and astromech-8zdt (Vitest browser mode on Chromium, testing rules in AGENTS.md and CONTRIBUTING.md). pnpm check, lint and test pass (528 tests); scripts/ui screenshots of every page before and after are identical except Mining with no ?core=, accepted as astromech-nvi0; preview-publish route=exact unchanged. Reviewed: astromech-9jbl, astromech-mqa9. Also moves CI's Atelier pin to 523a6502 so atelier doctor reads finding dispositions (failing on main since fb155297).
