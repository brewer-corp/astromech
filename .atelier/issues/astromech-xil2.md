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
closed_at: "2026-11-07T05:23:10.732391877+00:00"
status: "done"
title: "Transport seam: one call(), the fake moved to app/src/preview"
updated_at: "2026-11-07T05:23:10.732391877+00:00"
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

## Notes

- 2026-11-07 [decision] lib/transport.ts imports the preview transport statically and picks it on first use when !inTauri && (DEV || VITE_PREVIEW); a release build folds the condition and drops app/src/preview entirely (index chunk 379 KB to 182 KB, scripts/check-dist.py passes).
  Reason: Synchronous previewOption needs the preview loaded; a dynamic import with top-level await would deadlock on the preview <-> lib import cycle, and without it options read at component init would race the load.
  Rejected: dynamic import() of the preview behind the same condition
- 2026-11-07 [decision] Without ?core= the Mining page now says the core did not answer, as the app does with no core; with ?core=ready or empty it is unchanged. mining.ts reads the run from core_status, which the no-core preview answers null (the header's Not running).
  Reason: One command cannot answer null for the header and the sample run for Mining; the old code special-cased Mining outside Tauri. previewstates/stale/mining View tests now run Mining under ?core=ready.
  Rejected: a sample status without ?core= (changes the header from Not running)
- 2026-11-07 [decision] The no-restricted-imports rule exempts *.test.ts: tests still import preview/fakecore helpers (resetFakeCore, fakeCall) and fixtures until astromech-8zdt moves them behind the transport. lib/core-reply-shapes.json stays in lib so the Rust test path is unchanged; crates/astromech-core/tests/mining_status.rs include_str path and the Rust dump examples' doc paths follow the fixtures to app/src/preview/fixtures.
  Reason: Moving the fixtures is required by Done when; the mining_status test includes the fixture by path and would not compile otherwise.

## Close Reason

The window reaches its backend through app/src/lib/transport.ts (call, subscribe, previewOption): Tauri invoke, core_call, events and the neutron progress Channel in the app, and the browser preview in app/src/preview/ (fakecore, shell and keybinds handlers, query, fixtures) only when DEV or VITE_PREVIEW is set. No lib module branches on inTauri; a plain pnpm build carries none of the fake. Commits 56c08e16 and 191e4dec on feature/astromech-7rcn.

Proof (all after the last change):
- cd app && pnpm check (0 errors, 0 warnings), pnpm lint (clean), pnpm test (73 files, 515 tests pass).
- eslint: importing ../preview/fakecore, ../preview/fixtures/materials.json and @tauri-apps/api/core into lib/materials.ts, and ../../preview/query into KeybindsView.svelte, each fail no-restricted-imports; "__TAURI_INTERNALS__" fails no-restricted-syntax; reverted, lint clean.
- preview/coverage.test.ts fails naming materials_inventory (lib/materials.ts) when its shell handler is removed, neutron_progress (lib/neutron.ts) when the topic is removed, and callouts when the core handler is removed; passes when restored.
- pnpm build then scripts/check-dist.py app/dist: pass (74 files, no preview). On a scripts/preview-publish build: fails on "Arietis Sector LD-H a11-1", "failed on purpose" and "preview: no fake for". CI's frontend job runs it after pnpm build.
- scripts/ui screenshots of all 24 pages under ?core=ready, ?core=empty and no ?core= plus route?route=exact, setup?core=new (76 shots) before (commit 86f40d62) and after: 75 byte-identical; the one difference is Mining without ?core= (decision note). scripts/preview-publish --page route --query route=exact opens on #/route with the sample route, byte-identical to before. ?press=, ?q= checked in the dev preview.
- cargo test -p astromech-core --test mining_status: pass (fixture path moved).

Deviations: Mining without ?core= now says the core did not answer (decision note); three preview tests run Mining under ?core=ready. Outside the one allowed test, crates/astromech-core/tests/mining_status.rs changes its include_str path and five Rust dump examples change doc-comment paths, because the fixtures moved. The lint rule exempts tests (8zdt).

Open items: independent review outstanding. CI not run (not pushed). A preview build is now one chunk over Vite's 500 kB warning (all fixtures load with the preview); harmless for the artifact. Tauri path not run on Windows here: the transport keeps invoke/core_call/listen/Channel argument shapes, covered by staleTauri and tauriFailure tests with a mocked invoke.
