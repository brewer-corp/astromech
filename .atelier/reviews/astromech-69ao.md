---
schema: atelier.review
schema_version: 1
id: astromech-69ao
title: finding on astromech-51ue @bfb7f1dd
target: astromech-51ue
identity: reviewer-fable-51ue-plan
commit: bfb7f1ddd9bc83022431ffd734d9a7782c43e41b
created_at: 2026-11-05T06:26:18.087400406Z
event:
  event: finding
  recommendation: defer
---

defer: astromech-nxz9. Its Done when makes Galaxy's page tabs Places and Route, but the mockups it asks to compare screenshots against show Places, Route, System (app/src/mockups/ds/Finder.svelte:87, PlacesHome.svelte:33); System lives in the Exploration group today (exploration/manifest.ts). Fix: a line in nxz9 saying the mockups' System tab is not built here and why, so the screenshot comparison does not flag it.
