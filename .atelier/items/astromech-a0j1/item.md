---
schema: atelier.item
schema_version: 3
id: astromech-a0j1
title: audit.mjs fails its last step for a plan outside the repository, and one run stopped at 19 of 30 captures
status: ready
mission: null
labels:
- tooling
- size-s
priority: medium
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T12:23:38.509294697Z
trivial: null
caused_by: null
created_at: 2026-11-17T12:23:38.359426859Z
---

## Goal

The capture and measure tool finishes every run. `scripts/browser/audit.mjs` (from astromech-8e22) is what every Glow-up screen item and reviewer uses. astromech-7vxi's round 4 reviewer hit two faults on 19 Nov 2026:

- With a plan file outside the repository, the last step fails: it runs `git rev-parse` from the plan's folder to stamp the commit (around line 476).
- A second run stopped after 19 of 30 captures with Puppeteer's "Navigating frame was detached". The cause is not known.

## Done when

- [ ] A run with a plan in a scratch folder outside the repository finishes and stamps the commit of the script's own repository.
- [ ] The "Navigating frame was detached" stop is reproduced and its cause named in the Report, then fixed at the cause. If it cannot be reproduced in ten full runs, the Report says so and what was tried, and the tool reports which capture it stopped on and why instead of a bare Puppeteer error.
- [ ] Every committed capture plan runs to its report on main. `docs/design/ship-loadout/plan.json`'s `no-fleet` shot times out today (astromech-59n6's reviewer, 19 Nov 2026): an empty fleet now opens on onboarding and the plan's header clicks cannot get past it. The plan reaches the empty Ships page another way, and the Report lists each plan run.
- [ ] No retry is added to hide a failed navigation.

## Scope

`scripts/browser/audit.mjs` and the capture plans under `docs/design/`.
