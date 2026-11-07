---
created_at: "2026-11-07T04:51:23.385396735+00:00"
id: "astromech-yis2"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
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
status: "backlog"
title: "User journeys end to end in CI, and a release smoke on Windows"
updated_at: "2026-11-07T04:51:23.385396735+00:00"
---

## Goal

A small suite of user journeys runs on every push against the real headless core and a scripted world, in headless Chrome, and the packaged app gets a smoke test on Windows at release (decision astromech-1zhg).

## Done when

- [ ] Journeys, at least: plot and follow a route; jump into the next stop, and it is copied and read out; a lookup fails (recorded 503) and the page shows its error with TRY AGAIN. Others to choose when planning: paste only while the galaxy map is open; set up push-to-talk and the mic; edit a keybind and the right .binds file changes; the kill switch stops input.
- [ ] They run in CI on every push, with no sleeps, wall-clock reads or retries (lint and review), and pass on the mission's last merged commit (success criterion journeys-in-ci).
- [ ] The release workflow launches the packaged app on Windows, sees the window connect, and quits it.

## Unknowns

- Driving the page: scripts/ui (Puppeteer) or Playwright.
- CI time budget for the suite.
