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
closed_at: "2026-11-07T06:27:55.488319424+00:00"
status: "done"
title: "UI tests in real Chromium, and the testing rules written down"
updated_at: "2026-11-07T06:27:55.488319424+00:00"
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

## Close Reason

Outcome: app/vite.config.ts has two Vitest projects: node (pure logic, src/**/*.test.ts and eslint-rules/*.test.js) and browser (src/**/*.browser.test.ts in Playwright's headless Chromium, @vitest/browser-playwright 5.0.5 and playwright 1.60.0, at the window's 1200x800). jsdom is gone from the config, package.json and the lockfile (pnpm why jsdom is empty). 54 browser files, 19 node files, 524 tests pass locally (pnpm -s check, lint, test; build and check-dist too). CI's linux job caches ~/.cache/ms-playwright, runs `playwright install --with-deps --only-shell chromium`, then pnpm test (both projects). AGENTS.md has a short Tests section; CONTRIBUTING.md a full Tests section with every rule in Done when plus how to run, the helper and the exemptions. A local eslint plugin (app/eslint-rules/tests.js) rejects setTimeout and Date.now()/performance.now() in every src test file; eslint-rules/tests.test.js (RuleTester plus a check that eslint.config.js turns both on) fails when the rule is disabled in the config or broken, and passes restored.

Transport: UI tests now change the fake's answers through app/src/test/transport.ts (answer, watch, previewAnswer, startPreview, idle, sent), which installs the preview's transport with per-test overrides and resets the fake core, neutron file, page query and timers before each test. vi.mock of an app module with importOriginal deadlocks in the browser when the preview imports that module (engineering, search, trade, odyssey, core, hotspots, stations), and vi.spyOn on an ES module namespace fails, so those were all moved behind answer/watch; fakeCall and resetFakeCore calls went to previewAnswer and startPreview. The xil2 test-file exemption is narrowed: app code keeps the full rule; test files may import preview/fixtures/* (recorded inputs, as data) and from fakecore only fakeJump, fakePlotInGame, fakeClipboard, fakePreviews and the Scenario type (the fake world's own events); everything else in preview/, Tauri's API and __TAURI_INTERNALS__ are rejected in tests. Still exempt: src/preview/** tests (they test the fake), and staleTauri and tauriFailure (they fake Tauri's invoke to test the app's side of the transport). Still using vi.mock of an app module, because it works and the conversion is not 1:1 (core replies need reshaping): voicetester, colonisation, powerplay/Targets, route/Plot, staleMocked, settings/announce, lib/mining.test.ts; CONTRIBUTING.md names them and forbids new ones.

Removed tests: (1) datasharing/View "says plainly what gets sent": copy text only. (2) trade/View.test.ts, its only test "links Sell, Buy and Sell my cargo into their Places finders instead of listing them, and keeps trade routes": static link hrefs and markup; App's "links each page a lookup moved from into its finder with the input filled in" covers the same #/trade links by following them.

Deviations and surprises: every setTimeout wait became an event wait: idle() (all transport calls answered, then Svelte's tick), vi.waitFor on sent, or fake timers (setInterval polls in Onboarding, Settings, Route, the voice tester; the preview's ?slow= gate). Per-call findBy timeouts were removed; one ceiling, testing-library's asyncUtilTimeout 5000 ms, is set in src/test/setup.ts because parallel files on a loaded machine took over 1 s to first render. Tests that read the wall clock now fix Date (SearchKit, Powerplay); Onboarding's press timestamps are a counter. Real Chromium exposed four test races jsdom hid (FactionStates, RawSites, SearchKit, Powerplay asserted before the second render); they now wait for the end state. Vitest's default 414 px viewport hid table columns, so the viewport is the window's size. The App finder test ignores Chromium's benign "ResizeObserver loop" error. jsdom stubs (scrollIntoView, the no-navigation comment) are gone; the App test now clicks the link. svelteTesting() needs resolveBrowser: false in browser mode (it otherwise empties resolve.conditions and Svelte resolves to its server build). @tauri-apps/api/core mocks need a Channel export (an ES module import fails without it). Playwright 1.60 refuses to install on Ubuntu 26.04; CONTRIBUTING.md gives PLAYWRIGHT_HOST_PLATFORM_OVERRIDE=ubuntu24.04-x64. pnpm re-resolved a few transitive patch versions (babel runtime, magic-string, tinybench) when jsdom left the lockfile.

Proof of failure: breaking Market.svelte's error path (err = null) failed "says why when there is no market to compare" in Chromium; reverted, passes.

Open items: "passes in CI" is unproven until the branch is pushed (stand-in: the full local run above and a yaml.safe_load of ci.yml; actionlint is not installed). No Chromium processes were left behind. Review outstanding.
