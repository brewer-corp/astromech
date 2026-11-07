---
created_at: "2026-11-07T04:51:23.385396735+00:00"
id: "astromech-yis2"
issue_type: "feature"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: astromech-yis2
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-11vu"
  - kind: "issue"
    id: "astromech-8qsx"
  - kind: "issue"
    id: "astromech-cwtv"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
  - kind: "issue"
    id: "astromech-vucf"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "User journeys end to end in CI"
updated_at: "2026-11-07T22:56:46.866750944+00:00"
---

## Goal

A small suite of user journeys runs on every push against the real headless core and a scripted world, in headless Chromium driven by Playwright (decision astromech-1zhg).

## Done when

- [ ] The journeys are:
  - plot a route from the page and follow it;
  - jump into the next stop, and the stop after it is copied and read out;
  - a lookup fails with a recorded 503, the page shows its error with TRY AGAIN, and it recovers;
  - the kill switch shows on in the page and goes off after a re-arm;
  - the voice tester runs gear down live, and the page shows the game confirmed it.

  Paste while the galaxy map is open is covered by crates/astromech-core/tests/world_voice.rs (`the_paste_button_pastes_into_the_galaxy_map`) and has no page part. Setting up push-to-talk and editing a keybind need presses in the page, which is backlog item astromech-g0be.
- [ ] The journeys run in CI's Linux job on every push, with no sleeps, no wall-clock reads and no retries, enforced by lint and checked in review. They pass in CI's Linux job on yis2's PR.

## Context

- **The driver: Playwright** (app/package.json:29, with its Chromium already installed in CI at ci.yml:86-92) under `node --test`. Not scripts/ui's Puppeteer, which is a one-shot tool that needs a system Chrome install (scripts/browser/run.mjs:18-29).
- **CI budget:** under 90 s for the suite, with the step capped at 5 minutes. A world core starts in about 60 ms or less, and Chromium launches in 0.5 s. The Linux job takes 265 s today.
- **Order of the children:** the harness and the lookup journey come first. Then the route journeys, and the kill switch and voice tester journeys, in parallel.

## Background

The release smoke on Windows (decision astromech-1zhg: launch, window up, connected, quit) was a Done when line here. It moved to astromech-y6d6 on 7 Nov 2026. y6d6 had already adapted the release.yml smoke step and scripts/onboarding_smoke.py to the one-process app, and its Done when line "The release build and its smoke run pass" now names the four parts. This lets yis2 merge without waiting on y6d6, which waits on decision astromech-lkwj. The run of the journeys on the mission's last merged commit (success criterion journeys-in-ci) is checked at mission close, on astromech-kjfs.
