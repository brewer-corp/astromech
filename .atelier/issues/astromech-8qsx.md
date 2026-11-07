---
created_at: "2026-11-07T22:55:23.421878444+00:00"
id: "astromech-8qsx"
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
title: "Journey harness: node --test drives the page with Playwright against a world core, and a lookup fails with TRY AGAIN"
updated_at: "2026-11-07T22:55:23.421878444+00:00"
---

## Goal

A journey suite runs on every push. Node's test runner drives the page with Playwright. The Vite dev server serves the page, and its dev bridge connects it to a headless core started on a committed world. The test steps that world, then waits on what the core observes and on what the page shows. This child builds the harness, the lint rules, the CI step and the docs. It proves them with the first required journey: a lookup that fails, shows its error with TRY AGAIN, and recovers (success criterion journeys-in-ci, decision astromech-1zhg). Siblings add the other journeys on top of this harness.

## Done when

- [ ] `node --test 'journeys/*.journey.ts'` run in app/ runs every `app/journeys/*.journey.ts`. The glob is quoted, so Node expands it: `node --test journeys/` fails with "Cannot find module" on Node 22.20, and on Node 26 it runs 0 tests and exits 0. The harness fails when it finds no journeys, so a run of zero can't pass; a mutation that empties the glob's match shows it. It uses the `playwright` library, already a devDependency (app/package.json:29), and adds no new test runner. There is no `journeys` script in app/package.json: see Context, Release trigger paths.
  - For each journey the harness starts:
    - one world core, with `startWorld` (app/dev/core.ts:155);
    - a Vite dev server whose bridge sends the page's calls to that same core, with no second core (today the plugin starts its own, app/dev/bridge.ts:106);
    - a fresh Chromium context at the app window's 1200×800.
  - A journey gets the page, the core's `step`, `wait`, `observed` and `callWindow`, and the world's temp folder.
  - The harness ends each journey's core, not the bridge plugin. It calls the core's `quit()` (core.ts:206-216), which must resolve with exit code 0 and an empty home. Then it checks that the temp world folder (`astromech-world-*`, the folder above `appDir`'s `local/astromech/data`) is gone. A journey whose core exits otherwise, or that leaves either folder behind, fails. The dev server only sends calls to that core and never starts or quits one.
- [ ] The page's clock is controlled with Playwright's `page.clock.install` and `runFor` only. The clock is installed at the world header's `start` time before the page loads. The page's timers then fire only when the journey calls `runFor`. That covers the polls (the 2 s status and route polls, app/src/App.svelte:99 and app/src/modules/route/View.svelte:48) and the toast dismissal (the `setTimeout` at app/src/lib/ui/Toasts.svelte:20). It covers debounces too: the plot bar's 250 ms system-name lookup (app/src/modules/route/PlotBar.svelte:90) and the 200 ms searches (app/src/modules/station/View.svelte:51, app/src/modules/logbook/View.svelte:33), so a journey that types calls `runFor` before it waits for the lookup. A harness test shows two things, each firing only on `runFor` and not before:
  - a 2 s status poll, seen through the bridge as one more `status` call posted to `/__astromech/call` (a Playwright `request` event on the page). World mode records no control calls as observations, and this adds none;
  - a toast's dismissal.

- [ ] Journey: a lookup fails and recovers.
  - The world, committed under worlds/journeys/: a Places search goes through SearchKit (app/src/lib/ui/SearchKit.svelte). It is first answered with a recorded 503, then with the recorded answer (`in_turn`, as in worlds/preview/sol/net.json).
  - The page shows DESIGN.md's hard error, a red "!", the reason and TRY AGAIN (app/src/lib/ui/Failure.svelte). After TRY AGAIN it shows the recorded results.
  - With `observed`, the journey checks the core made two requests to that URL.
  - Mutations, each reverted:
    - the world answering 200 the first time fails the wait for the error;
    - TRY AGAIN that repeats nothing fails the wait for the results.
- [ ] Lint enforces determinism.
  - app/eslint.config.js applies `astromech/no-timer-wait` and `astromech/no-wall-clock` (app/eslint-rules/tests.js) to app/journeys/.
  - A new rule there rejects Playwright's `waitForTimeout` and a numeric `polling` interval.
  - Each rule has a case in app/eslint-rules/tests.test.js.
- [ ] No step or test retries: no harness or journey code retries, and `node --test` runs with no retry option. Waits are on the core's observations (`world_wait`, which has its own 30 s hang guard) or on the page (locator waits, or `waitForFunction` with `polling: "mutation"`). Their timeouts only guard against a hang.
- [ ] CI: a step in ci.yml's Linux job runs the journey suite.
  - It goes after "dev bridge answers with the real core" (ci.yml:96-99), so `cargo test` has built the core and the browser project's step has installed Chromium (ci.yml:86-92).
  - It runs `node --test 'journeys/*.journey.ts'` and has `timeout-minutes: 5`.
  - When a journey fails, its Playwright trace, a screenshot and the core's log are uploaded as an artifact.
  - The Report gives the step's time on the runner, with one cold start of the Vite dev server and its first page load measured separately. The budget is under 90 s for the whole suite.
- [ ] The journeys typecheck. `app/journeys/tsconfig.json` extends `../tsconfig.json` and includes `journeys/**/*.ts` (with Node's types), and a ci.yml Linux step runs `pnpm exec tsc -p journeys` in app/. A type error planted in a journey fails it.
- [ ] The Tests section of CONTRIBUTING.md covers:
  - what a journey is: a user outcome that crosses the page, the core and a boundary, never a case a lower layer already tests;
  - how to run one locally;
  - the rules above.

  `scripts/ui` stays the tool for captures and agent checks.

## Scope

In: the harness, the bridge change it needs, this journey's world, the lint rules, the CI step and the docs.

Out:
- The other journeys, which are this record's siblings.
- The shell's push topics (`input`, `route`) through the bridge (app/src/lib/transport.ts:16-18, :93-97). Journeys that need presses in the page are a backlog item.
- Windows: the suite runs in the Linux job.

## Context

- **Release trigger paths.** release.yml runs its Windows Release build on any PR that touches app/package.json, app/vite.config.ts, app/tsconfig*.json or crates/** outside tests and examples (release.yml:37-42 on main). A `journeys` script in app/package.json would make every push to yis2's PR run that build, for nothing that ships. So:
  - the suite is run with `node --test 'journeys/*.journey.ts'`, with no script in app/package.json;
  - the bridge change stays in app/dev/, which is not a trigger path;
  - the journeys' tsconfig is `app/journeys/tsconfig.json`. The trigger glob `app/tsconfig*.json` matches only files in app/ itself, so this file isn't a trigger path.

  A change to a trigger path is fine when the work needs it, but say why in the PR.
- **Order.** This is yis2's first child. The route journeys and the kill switch and voice tester journeys start once this harness is on yis2's branch, which all the children share. Siblings have no blocked_by links (AGENTS.md).
- **What vucf brought.** astromech-vucf merged to main as cac049aa. It brought the world core, the world steps, the recorded network with faults, the devices, the dev bridge and app/dev/core.ts.
- **The driver, yis2's first Unknown, settled here: Playwright, not scripts/ui's Puppeteer.**
  - Playwright 1.60 is already in app/ (app/package.json:25, :29), and CI caches and installs its Chromium for the browser test project (ci.yml:86-92).
  - It has a clock API.
  - scripts/ui is a one-shot command-line tool over puppeteer-core, in its own package. It looks for a system Chrome install (scripts/ui:2-9, scripts/browser/package.json, scripts/browser/run.mjs:18-29) and has no clock control.
- **The runner.** The journeys use node:test rather than Vitest because the bridge plugin turns itself off under Vitest (app/dev/bridge.ts:101). app/dev/core.ts already runs under plain `node` with type stripping, as `pnpm bridge-check` does in CI on Node 22 (ci.yml:97-99).
- **Onboarding.** A fresh world opens onboarding at "/": jkgv's receipt clicked SKIP SETUP. The harness gives journeys one way past it.
- **Lookups and the clock.** In world mode, Spansh jobs are polled with no wait (crates/astromech-core/src/world.rs:655-660), so a lookup needs no clock advance.
- **Allowed guards.** The `world_wait` guard and node:test's timeouts are hang guards that decide nothing, which CONTRIBUTING.md's Determinism rule allows (CONTRIBUTING.md:238).
- **Timings, measured on the dev host on 7 Nov 2026:**
  - after `quit()` a world core exits 0 and its temp world folder is gone (worlds/jump);
  - a world core starts in 12-64 ms and answers `status` in 1-8 ms (worlds jump, network, kill-switch and preview/sol);
  - Playwright's Chromium launches headless in 0.5 s;
  - a page with the clock installed is ready in 0.25 s;
  - CI's Linux job takes 265 s today (run 42417602123).

## Unknowns

- How the bridge gets the journey's core: a plugin option, or the core's contact passed in the environment. Today the plugin starts its own core (app/dev/bridge.ts:106) and quits it when the server closes (:113). Either way, one core per journey serves both the page and the journey's steps, and the harness quits it.
- The cold Vite dev server in CI.
  - With no `node_modules/.vite` cache, Vite pre-bundles dependencies on the first request.
  - If it finds a new dependency mid-run (a module imported only by a later page), it re-optimises and reloads the page, which would break a journey midway.
  - The harness makes sure no page reloads mid-journey: dependencies are pre-bundled or warmed before the first journey, and a reload fails the journey rather than being ignored.
  - Measure one cold start and first page load on the runner, and record it in the Report.
