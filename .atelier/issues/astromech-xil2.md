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

Replace the `inTauri` branch in each `app/src/lib/*.ts` data module with one transport, so the fake is in one place and the release bundle carries none of it. The transport has two halves: calls, and pushes from the backend.

## Done when

- [ ] `app/src/lib/transport.ts` exposes `call<T>(cmd, args)` and `subscribe<T>(topic, handler): () => void`. In Tauri, `call` wraps `invoke` (shell commands) and `core_call` (core commands), and `subscribe` wraps Tauri events and channels: `listen("input")` (lib/binds.ts), `listen("route")` (lib/core.ts), the shell's `core-restarted` event (app/src-tauri/src/window.rs) and the neutron download's progress `Channel` (lib/neutron.ts).
- [ ] Outside Tauri the preview transport is loaded only when `import.meta.env.DEV` or `import.meta.env.VITE_PREVIEW` is true. `scripts/preview-publish` sets `VITE_PREVIEW`; a plain `pnpm build` sets neither, so the preview code is dead and dropped, not just split into a chunk.
- [ ] No module outside `app/src/preview/` and `transport.ts` imports `fakecore`, `previewData`, `previewLookup`, `previewQuery` or `lib/fixtures`, or checks `inTauri` for data or events; an eslint `no-restricted-imports` rule (or similar) fails on a new one.
- [ ] fakecore's handlers, `previewGate` (`?fail=`, `?slow=`), `previewQuery` and the fixtures live under `app/src/preview/`. Page-level preview options that are read in components (`?q=`, `?bind=` in KeybindsView.svelte, `?device=`, `?activity=` in controller/View.svelte, `?press=` in lib/binds.ts) are reached through the transport or a preview-only module, not by importing `previewquery` from app code.
- [ ] `app/src/preview/coverage.test.ts` fails when a command or topic the app uses has no fake handler.
- [ ] A build check run in CI's frontend job fails if `app/dist` from `pnpm build` contains a fixture or preview string, for example the fixture route's "Arietis Sector".
- [ ] Every top-level page renders in the preview as before (`scripts/ui` screenshots of each, before and after), and `scripts/preview-publish --page route --query route=exact` still opens on the sample route.

## Scope

Frontend only; the Rust shell and core don't change. Command names, topics and reply shapes stay as they are; generated types come with the command layer (astromech-bp23). If `lib/core-reply-shapes.json` moves, update the Rust test that writes it (`setup_and_calibration_reply_shapes_match_the_window_fake` in crates/astromech-core/src/app.rs) to the new path.

## Context

`app/src/lib/core.ts` (`inTauri`, `coreCall`, the route listener), `app/src/lib/fakecore.ts`, `app/src/lib/previewquery.ts`, `app/src/lib/fixtures/`, `app/src/lib/fakecore.contract.test.ts`, `app/src/main.ts` (the preview start page), `scripts/preview-publish`, CONTRIBUTING.md's UI check. Decision astromech-1zhg.
