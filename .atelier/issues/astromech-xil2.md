---
created_at: "2026-11-07T04:51:22.367368678+00:00"
id: "astromech-xil2"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Transport seam: one call(), the fake moved to app/src/preview"
updated_at: "2026-11-07T04:51:22.367368678+00:00"
---

## Goal

Replace the `inTauri` branch in each `app/src/lib/*.ts` data module with one transport, so the fake is in one place and the release bundle carries none of it.

## Done when

- [ ] `app/src/lib/transport.ts` exposes `call<T>(cmd, args)`. In Tauri it wraps `invoke` (shell commands) and `core_call` (core commands). Outside Tauri, `main.ts` loads `app/src/preview/transport.ts` with a dynamic import before mounting.
- [ ] No module outside `app/src/preview/` and `transport.ts` imports `fakecore`, `previewData`, `previewLookup` or `lib/fixtures`, or checks `inTauri` for data; a lint rule (eslint `no-restricted-imports` or similar) fails on a new one.
- [ ] fakecore's handlers, `previewGate` (`?fail=`, `?slow=`), `previewQuery` and the fixtures live under `app/src/preview/`.
- [ ] `app/src/preview/coverage.test.ts` fails when a command the app calls has no fake handler.
- [ ] A build check (a script run in CI's frontend job) fails if `app/dist` from `pnpm build` contains a fixture or preview string, for example the fixture route's "Arietis Sector".
- [ ] Every top-level page renders in the preview as before (scripts/ui screenshots, before and after).

## Scope

Frontend only. The Rust shell and core don't change. Command names and reply shapes stay as they are; generated types come with the command layer increment.

## Context

`app/src/lib/core.ts` (`inTauri`, `coreCall`), `app/src/lib/fakecore.ts`, `app/src/lib/previewquery.ts`, `app/src/lib/fixtures/`, `scripts/preview-publish`, CONTRIBUTING.md's UI check.
