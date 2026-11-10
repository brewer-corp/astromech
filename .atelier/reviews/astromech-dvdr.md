---
schema: atelier.review
schema_version: 2
id: astromech-dvdr
title: finding on astromech-xajc @d5f91785
target: astromech-xajc
identity: GPT-6 Astra (round 2)
commit: d5f917856bc09109cfbf4ada97a6b2002ff16820
created_at: 2026-11-10T06:13:58.085918373Z
event:
  event: finding
  recommendation: fix_now
---

P2: The filter revision retains two incompatible close contracts (astromech-drnv Done when 2-4; continuation of astromech-halk). It says the panel closes only on DONE, Escape or CHANGE, never on its own, and a failed search preserves its state, but the explicit trade/Route.svelte consumer must keep it open until its search is run. SearchFilters is assigned internal ownership of open state with only a summary prop specified, while FIND ROUTE and its asynchronous success/failure live in its parent. Choose the same explicit-close behavior for both consumers (the simplest fix), or document the trade exception, whether closing is on submission or success, failure behavior, and the parent/component interface that triggers it. Add the failed-save case requested in round 1 to the browser acceptance cases.
