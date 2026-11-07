---
schema: atelier.review
schema_version: 1
id: astromech-b3f4
title: finding on astromech-ja5a @27fd64e6
target: astromech-ja5a
identity: claude-reviewer-ja5a
commit: 27fd64e65f24e4623e6ace3ee29140d9bc58e4a0
created_at: 2026-11-07T00:08:42.673133353Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.4.md:97 (astromech-4ytn) still says 'then plot the same routes on the Route page'. The Route page has no plotter any more (app/src/modules/route/View.svelte imports no Plot; plotting is the Find or plot panel, guide routes.md 'Plot a route'), the same stale-UI class as R1/R2, missed in my first pass. Fix: 'then plot the same routes from Find or plot in the header, from the same starting system (it always plots from where you are)'.
