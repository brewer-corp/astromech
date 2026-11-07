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
status: "in_progress"
title: "Journey harness: node --test drives the page with Playwright against a world core, and a lookup fails with TRY AGAIN"
updated_at: "2026-11-07T23:13:51.260098222+00:00"
---

## Goal

A journey suite runs on every push. Node's test runner drives the page with Playwright. The Vite dev server serves the page, and its dev bridge connects it to a headless core started on a committed world. The test steps that world, then waits on what the core observes and on what the page shows. This child builds the harness, the lint rules, the CI step and the docs. It proves them with the first required journey: a lookup that fails, shows its error with TRY AGAIN, and recovers (success criterion journeys-in-ci, decision astromech-1zhg). Siblings add the other journeys on top of this harness.

## Done when

- [x] `node --test 'journeys/*.journey.ts'` run in app/ runs every `app/journeys/*.journey.ts`. The glob is quoted, so Node expands it: `node --test journeys/` fails with "Cannot find module" on Node 22.20, and on Node 26 it runs 0 tests and exits 0. The harness fails when it finds no journeys, so a run of zero can't pass; a mutation that empties the glob's match shows it. It uses the `playwright` library, already a devDependency (app/package.json:29), and adds no new test runner. There is no `journeys` script in app/package.json: see Context, Release trigger paths.
  - For each journey the harness starts:
    - one world core, with `startWorld` (app/dev/core.ts:155);
    - a Vite dev server whose bridge sends the page's calls to that same core, with no second core (today the plugin starts its own, app/dev/bridge.ts:106);
    - a fresh Chromium context at the app window's 1200×800.
  - A journey gets the page, the core's `step`, `wait`, `observed` and `callWindow`, and the world's temp folder.
  - The harness ends each journey's core, not the bridge plugin. It calls the core's `quit()` (core.ts:206-216), which must resolve with exit code 0 and an empty home. Then it checks that the temp world folder (`astromech-world-*`, the folder above `appDir`'s `local/astromech/data`) is gone. A journey whose core exits otherwise, or that leaves either folder behind, fails. The dev server only sends calls to that core and never starts or quits one.
- [x] The page's clock is controlled with Playwright's `page.clock.install`, `pauseAt` at the same instant, and `runFor` only (install alone leaves the page clock running in real time). The clock is installed and paused at the world header's `start` time before the page loads. The page's timers then fire only when the journey calls `runFor`. That covers the polls (the 2 s status and route polls, app/src/App.svelte:99 and app/src/modules/route/View.svelte:48) and the toast dismissal (the `setTimeout` at app/src/lib/ui/Toasts.svelte:20). It covers debounces too: the plot bar's 250 ms system-name lookup (app/src/modules/route/PlotBar.svelte:90) and the 200 ms searches (app/src/modules/station/View.svelte:51, app/src/modules/logbook/View.svelte:33), so a journey that types calls `runFor` before it waits for the lookup. A harness test shows two things, each firing only on `runFor` and not before:
  - a 2 s status poll, seen through the bridge as one more `status` call posted to `/__astromech/call` (a Playwright `request` event on the page). World mode records no control calls as observations, and this adds none;
  - a toast's dismissal.

- [x] Journey: a lookup fails and recovers.
  - The world, committed under worlds/journeys/: a Places search goes through SearchKit (app/src/lib/ui/SearchKit.svelte). It is first answered with a recorded 503, then with the recorded answer (`in_turn`, as in worlds/preview/sol/net.json).
  - The page shows DESIGN.md's hard error, a red "!", the reason and TRY AGAIN (app/src/lib/ui/Failure.svelte). After TRY AGAIN it shows the recorded results.
  - With `observed`, the journey checks the core made two requests to that URL.
  - Mutations, each reverted:
    - the world answering 200 the first time fails the wait for the error;
    - TRY AGAIN that repeats nothing fails the wait for the results.
- [x] Lint enforces determinism.
  - app/eslint.config.js applies `astromech/no-timer-wait` and `astromech/no-wall-clock` (app/eslint-rules/tests.js) to app/journeys/.
  - A new rule there rejects Playwright's `waitForTimeout` and a numeric `polling` interval.
  - Each rule has a case in app/eslint-rules/tests.test.js.
- [x] No step or test retries: no harness or journey code retries, and `node --test` runs with no retry option. Waits are on the core's observations (`world_wait`, which has its own 30 s hang guard) or on the page (locator waits, or `waitForFunction` with `polling: "mutation"`). Their timeouts only guard against a hang.
- [ ] CI: a step in ci.yml's Linux job runs the journey suite.
  - It goes after "dev bridge answers with the real core" (ci.yml:96-99), so `cargo test` has built the core and the browser project's step has installed Chromium (ci.yml:86-92).
  - It runs `node --test 'journeys/*.journey.ts'` and has `timeout-minutes: 5`.
  - When a journey fails, its Playwright trace, a screenshot and the core's log are uploaded as an artifact.
  - The Report gives the step's time on the runner, with one cold start of the Vite dev server and its first page load measured separately. The budget is under 90 s for the whole suite.
- [x] The journeys typecheck. `app/journeys/tsconfig.json` extends `../tsconfig.json` and includes `journeys/**/*.ts` (with Node's types), and a ci.yml Linux step runs `pnpm exec tsc -p journeys` in app/. A type error planted in a journey fails it.
- [x] The Tests section of CONTRIBUTING.md covers:
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

## Notes

- 2026-11-07 [decision] The bridge gets the journey's core as a plugin option: astromechBridge({ core }), added to app/vite.config.ts's plugins by the harness's createServer call. With a core given the plugin applies without ASTROMECH_WORLD, never starts or quits a core, and the harness refuses to run with ASTROMECH_WORLD set, so the config's own plugin starts no second core.
  Reason: One owner for the core's life (the harness), no contact string to parse from the environment, and vite.config.ts (a Release trigger path) stays untouched because inline plugins merge with the config file's.
  Rejected: The core's contact in an environment variable read by the config's plugin: a second way to find a core beside the stdout line, and process-wide state
- 2026-11-07 [decision] Each journey's dev server pre-bundles at start only what optimizeDeps.include lists (vite-plugin-svelte adds svelte's entries) and discovers no other dependency (optimizeDeps.noDiscovery), into a cache of its own: cacheDir is <the journey's temp world>/vite, an absolute path, removed with the temp world when the core quits. It watches no files and listens on 127.0.0.1 from port 5180 with strictPort off. A second page load fails the journey at once (the page is closed, the waits reject). Corrected after finding astromech-o2x5: the first version said there was no pre-bundling, and its servers shared and rewrote app/node_modules/.vite.
  Reason: noDiscovery means a dependency first met mid-journey is never optimised, so nothing re-optimises and reloads the page; a cache per journey means the side-by-side journey processes node --test runs share no cache and never write app/node_modules/.vite. A relative cacheDir would resolve against app/.
  Rejected: The shared app/node_modules/.vite with a warm-up before the first journey: needs a step outside the glob, and the parallel journey processes would still rewrite one cache
- 2026-11-07 [decision] The page clock is page.clock.install({ time: start }) followed by page.clock.pauseAt(start), before the one page.goto; journeys then move it only with runFor.
  Reason: Installed alone, Playwright's clock runs on with real time (its controller resumes after install), so the 2 s poll fired on its own: with pauseAt removed the harness test fails with 'a status call before the poll was due'. pauseAt at the same instant moves no time and stops that.
- 2026-11-07 [decision] The way past onboarding is the window's own onboarding_done (what Skip setup sends), called on the core by open() before the page loads; the page then opens straight at the journey's route.
  Reason: One page load per journey, and the same effect as the button; clicking SKIP SETUP would land on the controller page and need a second navigation.
- 2026-11-07 [decision] A run of zero journeys: harness.journey.ts's first test fails when globSync('journeys/*.journey.ts') finds no journey but itself, and the CI step lists the files once (files=$(ls journeys/*.journey.ts), then node --test $files), so a glob that matches nothing at all fails the step.
  Reason: node --test with a glob matching nothing exits 0 on Node 22 and 26 and loads no file, so no check inside the harness can run then; the in-harness test covers the journeys renamed or moved, and the ls covers the whole folder gone in CI.
- 2026-11-07 [decision] The lookup journey's world, worlds/journeys/lookup-retry, is the Services finder's Black Market search on worlds/preview/sol-nov's game folder, answered in turn by a 503 and by sol-nov's recorded Spansh page (referenced by path, not copied); EDSM's sphere lookup is recorded as the 503 sol-nov holds.
  Reason: A recorded answer already in the repository (decision astromech-1zhg: fixtures are recorded raw inputs), and the journey can also check every request the core made was in the recording.
- 2026-11-07 [decision] Failure artifacts go to app/journeys/results/<journey>/ (trace.zip, page.png, the core's astromech-core.log copied before quit, and its stderr), removed when the journey passes; the repository's results/ ignore rule already covers it.
  Reason: A known path for CI's upload-artifact step, kept out of git with no .gitignore change.
- 2026-11-07 [handoff] Implementer handoff: see ## Report. Commits d7792acd (code, worlds, CI, docs) and 15ec04ca (records) on astromech-yis2. Six lines ticked; line 2 holds but uses pauseAt beside install (wording for the plan owner), and line 6 waits on the PR's CI run for the runner timing. Review outstanding.

## Report

Seven of the eight Done when lines hold and are ticked; line 6 waits on the PR's CI run. Commits d7792acd and f093bf71 (review fixes o2x5, 7yw1, 96mk, 35vx) on astromech-yis2.
- Line 1 (the glob, one core, server and context per journey, the core's end): `node --test 'journeys/*.journey.ts'` in app/ runs harness.journey.ts and lookup.journey.ts, 3 tests pass. Each journey has one world core: sampling `ps -C astromech-core`, the lookup journey alone peaks at 1 and the suite at 2 (its two journeys run side by side). The harness quits the core and fails on an exit other than 0, a written home (core.ts `quit`) or a temp world left behind. After 30 suite runs no world core, `astromech-world-home-*` or temp world of the harness's was left. Mutation: lookup.journey.ts renamed to lookup.ts fails "the suite finds journeys" with "no journey matches journeys/*.journey.ts in app/: a run of none would pass". No new runner and no app/package.json script.
- Line 3 (the lookup journey, worlds/journeys/lookup-retry): the page shows "! Spansh is having trouble (HTTP 503). Try again later." with TRY AGAIN. After the click it lists the recorded 10 stations. `observed` shows two requests to Spansh's stations search, and none unrecorded. Mutations, each reverted: with the world's first answer 200, the wait for the error times out ("waiting for getByRole('alert').filter({ hasText: 'Spansh is having trouble (HTTP 503)' })"); with SearchKit's TRY AGAIN a no-op, the wait for the "Rescue Ship - Li Qing Jao" row times out.
- Line 2 (the clock, reworded in 910de95e to name pauseAt): `install` alone lets Playwright's clock run on with real time, so the harness calls `page.clock.pauseAt(start)` at the same instant before the page loads. The harness test, harness.journey.ts: the 2 s status poll, where `runFor(1999)` adds no `status` request on `/__astromech/call` and `runFor(1)` adds exactly one, counted after a request barrier (`settled()`); and the toast (worlds/journeys/clock, a DockingDenied that says and toasts "Docking denied. No free pad."), absent until the poll that brings it, still up after `runFor(7999)`, gone after `runFor(1)`. Mutations: with pauseAt removed, "a status call before the poll was due"; with SHOW_MS 7999, "the toast dismissed before its 8 s were up"; with 8001, "the toast still up after its 8 s". Debounces are faked by the same install, but no journey here types.
- Line 4 (lint): eslint.config.js applies no-timer-wait, no-wall-clock and the new astromech/no-playwright-time to journeys/**/*.ts. no-playwright-time rejects `waitForTimeout`, any `polling` but "mutation", `"networkidle"` (`waitForLoadState` and `waitUntil`), and, outside journeys/harness.ts (which the config gives `{ clock: true }`), the page clock's `resume`, `setSystemTime`, `setFixedTime` and `fastForward` (finding astromech-96mk). The timer and wall-clock messages name the journey way too. tests.test.js has 6 valid and 10 invalid cases for it plus the harness option, and a config test that journeys/x.journey.ts gets the sleep and clock errors and journeys/harness.ts only the sleep; 34 tests pass. With the harness block's option removed that config test fails. Mutations: a journey with waitForTimeout, `polling: 50`, setTimeout and Date.now gives 4 lint errors, one per rule; the journeys block pointed at another folder fails both config tests.
- Line 5 (no retries): no harness or journey code retries, and node:test is run with no retry option. The waits are locators, `waitForRequest` and `world_wait`, and the 15 s page and 90 s journey timeouts are hang guards.
- Line 7 (typecheck): `pnpm exec tsc -p journeys` exits 0, checking journeys/*.ts and the app/dev modules they import. A planted `open(42)` fails it with TS2345. The ci.yml Linux job runs it after "frontend typecheck".
- Line 8 (CONTRIBUTING): the Tests section has a Journeys paragraph: what a journey is, the API, how to run all or one, the results folder, the rules and the lint. "Before a PR" lists the tsc step and the suite.
- Proof after the review fixes (f093bf71): `pnpm -s check` 0 errors, `pnpm -s lint` clean, `pnpm -s test` 563 passed, `pnpm exec tsc -p journeys` 0, `pnpm build` plus check-dist ("no browser preview or dev bridge"), check-guide 0 problems, `cargo test -p astromech-core --test committed_worlds` 2 passed (the new worlds). The suite ran 10/10 on Node 26 (2.2-2.3 s a run by node:test) and 10/10 on Node 22.23 via `npx node@22` (2.4-2.6 s), after 5 more runs; before the fixes it had run 20/20 on Node 26. After those 25 runs no `playwright-tracing-*` folder was left in TMPDIR (finding astromech-7yw1; the 74 empty ones the first version had left were removed), and app/node_modules/.vite was untouched (its _metadata.json still stamped 18:21, before the runs), while a run's deps metadata appeared under /tmp/astromech-world-*/vite/deps (finding astromech-o2x5). No Rust change, so `cargo test --workspace` was not run.

### Deviations
- Line 6 (CI): the step is added. It sits after bridge-check, has timeout-minutes 5, runs `files=$(ls journeys/*.journey.ts)` then `node --test $files` (finding astromech-35vx), and uploads journeys/results/ on failure. Its time on the runner is unmeasured, since nothing was pushed, so the line is left unticked. On the dev host:
  - one cold dev server start (an empty cache folder) takes 340-480 ms (measured before the per-journey cache, which is always cold, so the same case);
  - the first page load with the clock installed takes about 750 ms, and the first 503 error shows at about 860 ms after Chromium launched;
  - the whole suite takes 2.1-2.4 s against the 90 s budget.
- A run of zero: node --test with a glob matching nothing exits 0 and loads no file, so no harness check can run. The CI step's `ls` fails the step when the glob matches nothing. The in-harness test catches the journeys renamed or moved (decision note).

### Decisions
Seven decision notes on this record:
- the plugin option `astromechBridge({ core })`, which settles the first Unknown;
- each journey's server pre-bundles svelte's entries into its own cache, discovers nothing else, watches no files, and a second load fails the journey, which settles the second (corrected after o2x5);
- install plus pauseAt;
- onboarding_done as the way past onboarding;
- the zero-journey guards;
- the lookup world reusing sol-nov's recorded page;
- the results folder.

### Surprises
- The first loop ran from the wrong folder: a backgrounded `cd`. node --test found nothing and "passed" 20 times. That is exactly the zero-journey case the CI `ls` guards.
- vite-plugin-svelte adds svelte's entries to optimizeDeps.include, so `include: []` did not turn pre-bundling off (finding astromech-o2x5); the timing comparison that seemed to show no difference was between two pre-bundled runs.

### Open items
- Line 6 needs the PR's CI run: the step's time and one cold start on the runner.
- core.ts gained `stderr: <fd>`, used by the harness for the core's stderr log.
- No Release trigger path is touched. app/journeys/tsconfig.json is outside `app/tsconfig*.json`.
- Review is outstanding.
