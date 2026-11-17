---
schema: atelier.item
schema_version: 3
id: astromech-5lh2
title: 'Three journeys fail on main: their selectors point at the Route page and header as they were before increment 1'
status: in_review
mission: null
labels:
- bug
- size-s
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Af257f609-54d7-4e2e-bfce-45bbbe671617%3Adelegate-task%3A87df7a9b-2f2a-4ed9-8aa8-1f0d594e4129
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T07:00:31.053192082Z
trivial: null
caused_by: astromech-qwvs
created_at: 2026-11-17T07:00:30.914113742Z
---

## Goal

The journeys pass on main again. Three time out today: "Plot a route from the page and follow it", "a jump into the next stop copies and reads out the one after" (`app/journeys/route.journey.ts`) and "the kill switch shows on in the page and goes off after a re-arm" (`app/journeys/voice.journey.ts`). Their selectors (`.next .sys`, `header .state`) match nothing: increment 1 of astromech-xajc rebuilt the Route page as a timeline (astromech-qwvs, astromech-irrv) and moved the app's status from the header to the status bar (astromech-q79q). Found by astromech-b2cz's implementer on 17 Nov 2026; they fail the same way on the commit before it.

## Done when

- [x] `node --test 'journeys/*.journey.ts'` passes on main, run as CONTRIBUTING.md says.
- [x] The three journeys find what they assert through what the commander sees (a role, a label, the text), not a class name that a restyle changes, so the next redesign does not break them silently.
- [x] The Report says for each journey whether the product still does what the journey checks, or whether the journey had been hiding a real break since 12 Nov.

## Scope

The journeys and, if one shows a real break, its fix. astromech-uc7i moves the route into the sidebar and keeps these journeys passing there.

## Context

The journeys are not run at merge (astromech-r198), which is how this went unseen for five days.

## Report

The three journeys pass again, and the product never broke: each one failed only because its selector matched nothing after increment 1. The fix changes the journeys alone.

Per journey, whether the product still does what it checks:

- **"plot a route from the page and follow it"**: still works. The Route page's timeline shows PSR J1752-2806 as the next stop after the plot, the core copies Nova Aquila No 3 at once on the jump, and the page moves on only after its 2 s route poll. `.next .sys` was the old Route page's markup and matched nothing in Timeline.svelte.
- **"a jump into the next stop copies and reads out the one after"**: still works. Alpha Centauri shows as the next stop; the jump copies Barnard's Star and says "Next waypoint. Barnard's Star."; the page shows it after the poll. Same dead selector.
- **"the kill switch shows on in the page and goes off after a re-arm"**: still works. The state line (`headerState`) moved from the header to StatusBar.svelte's `<footer>` (astromech-q79q), so `header .state` matched nothing; the line still reads "Elite running · Voice ready", gains " · Kill switch on" from the next status poll and loses it after the re-arm.

How they find things now (`app/journeys/route.journey.ts`, `app/journeys/voice.journey.ts`):

- The next stop is the row of the list "Stops and the jumps between them" that says "Next stop", found by its copy button's accessible name, `Copy <system>`. Scoping to that list keeps out the route strip's button of the same name. The "moved on before its route poll" checks now assert the old stop's button is still the next stop's.
- The core's state is found by its exact text inside the page's `contentinfo` (the status bar's `<footer>`).

A separate commit fixes `headerState`'s comment in `app/src/lib/core.ts`, which still said it fed the header. It is outside this item's Scope but one line, and it is what misled anyone reading the journeys against the code. The function keeps its name.

Checks, in `app/`: `pnpm lint`, `pnpm check` and `pnpm exec tsc -p journeys` exit 0. `node --test 'journeys/*.journey.ts'` passes 7 of 7; before the change the route and voice files failed 3 of 4 with the timeouts this item names. The core was built from this worktree with `CARGO_TARGET_DIR=/home/supadava/projects/astromech/target cargo build -p astromech-app --bin astromech-core` (which replaced that checkout's 6 Nov binary) and the journeys ran with `ASTROMECH_CORE` pointing at it, as CONTRIBUTING allows. `pnpm exec playwright install --only-shell chromium` fails on this host ("does not support chromium-headless-shell on ubuntu26.04-x64"); the already cached `chromium_headless_shell-1223` ran the journeys.

