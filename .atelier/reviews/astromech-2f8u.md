---
schema: atelier.review
schema_version: 1
id: astromech-2f8u
title: finding on astromech-51ue @bfb7f1dd
target: astromech-51ue
identity: reviewer-fable-51ue-plan
commit: bfb7f1ddd9bc83022431ffd734d9a7782c43e41b
created_at: 2026-11-05T06:26:16.846523692Z
event:
  event: finding
  recommendation: fix_now
---

fix_now: astromech-51ue, astromech-kyqw, astromech-sgwi, astromech-wla4, astromech-ibmy. The parent's Done when (recipes from Places, Find near this, Find along route) and the validation child kyqw are blocked by three backlog children and, through sgwi, by the needs-human item astromech-5rm2 (contacting Spansh's maintainer). AGENTS.md merges a parent's branch only when every child is done, so the route strip, result actions, Places home and Find or plot cannot reach main until a human outside the project answers and three someday items ship, while pey5, dfvp, yxtz and sllo rework the same files on main (App.svelte, SearchKit, trade, materials). Fix: split xlbz, sgwi, wla4, ibmy and 5rm2 into a second feature under mission astromech-7q0z with its own Done when and validation child; drop the engine, recipe and along-route bullets from 51ue's Done when and from kyqw's blockers.
