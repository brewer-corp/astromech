---
schema: atelier.item
schema_version: 3
id: astromech-2dai
title: 'Browser tests in two worktrees at once fail to start: vitest holds port 63315'
status: building
mission: null
labels:
- size-s
- tooling
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A40e7bd6a-75d7-481f-bc07-e18b62a688e1%3Adelegate-task%3A7af34a08-e960-4a02-98c2-b0dd1dd1ba67
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T05:23:28.198951236Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.125189673Z
---

## Goal

Two agents can run `pnpm test` in two worktrees at the same time. Today the second run's browser tests fail to start because Vitest's browser server takes port 63315 in the first; three implementers hit it on 12 Nov 2026 (astromech-drnv, astromech-v4bw, astromech-irrv) and each worked around it by hand.

## Done when

- [ ] `pnpm test` started in two worktrees at once passes in both, with no flag or config outside the repository.
- [ ] The fix is at the cause (the fixed port in the browser test setup, `app/vite.config.ts` or what it loads), not a retry; CONTRIBUTING.md's Tests says nothing about ports because nothing needs saying.

## Scope

Test configuration only.
