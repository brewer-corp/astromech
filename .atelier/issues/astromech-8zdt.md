---
created_at: "2026-11-07T04:51:22.542097838+00:00"
id: "astromech-8zdt"
issue_type: "task"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xil2"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "UI tests in real Chromium, and the testing rules written down"
updated_at: "2026-11-07T05:31:30.781538806+00:00"
---

## Goal

Frontend tests run in Vitest browser mode on headless Chromium, not jsdom, and AGENTS.md and CONTRIBUTING.md state decision astromech-1zhg's rules, so new tests follow them.

## Done when

- [ ] `app/vite.config.ts` runs DOM tests in Vitest browser mode with Playwright Chromium; pure-logic tests stay in Node; jsdom is gone from the config and the dependencies.
- [ ] CI's frontend job installs Chromium and runs both projects; `pnpm -s test` passes locally and in CI.
- [ ] Tests that only check copy text, static markup or callback wiring are removed, each named in the close reason with the reason; behaviour tests stay.
- [ ] AGENTS.md (short) and CONTRIBUTING.md (full) state: doubles only at true boundaries; recorded raw inputs as fixtures; one case at the lowest layer that fails; what not to test; failing test first for logic bugs; captures for visual changes; no sleeps, polling, wall-clock reads or retries; quarantine needs an issue.
- [ ] An eslint rule for test files rejects `setTimeout`-based waits and `Date.now()`, with a test that it fires.

## Context

Decision astromech-1zhg; reference: T3 Code's apps/web/vite.config.ts browser project and its AGENTS.md lines 105 to 120. Depends on the transport seam for where the fake lives.
