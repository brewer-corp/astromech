---
schema: atelier.item
schema_version: 3
id: astromech-5lh2
title: 'Three journeys fail on main: their selectors point at the Route page and header as they were before increment 1'
status: building
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

- [ ] `node --test 'journeys/*.journey.ts'` passes on main, run as CONTRIBUTING.md says.
- [ ] The three journeys find what they assert through what the commander sees (a role, a label, the text), not a class name that a restyle changes, so the next redesign does not break them silently.
- [ ] The Report says for each journey whether the product still does what the journey checks, or whether the journey had been hiding a real break since 12 Nov.

## Scope

The journeys and, if one shows a real break, its fix. astromech-uc7i moves the route into the sidebar and keeps these journeys passing there.

## Context

The journeys are not run at merge (astromech-r198), which is how this went unseen for five days.
