---
schema: atelier.review
schema_version: 1
id: astromech-gkql
title: finding on astromech-51ue @bfb7f1dd
target: astromech-51ue
identity: reviewer-fable-51ue-plan
commit: bfb7f1ddd9bc83022431ffd734d9a7782c43e41b
created_at: 2026-11-05T06:26:17.702426495Z
event:
  event: finding
  recommendation: defer
---

defer: astromech-1d92. Screenshots "with the preview's plotted route" assume a preview route, but the browser preview starts with route: null and no ?core= scenario sets one (app/src/lib/fakecore.ts:166); scripts/ui would have to import a route through the Route page first. Fix: say which: add a preview route scenario to fakecore or script the import, so the proof is reproducible.
