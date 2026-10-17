---
schema: atelier.review
schema_version: 1
id: astromech-abmz
title: 'Fix commit 1b6f53d reviewed. 4d72: pad looked up from Ardent per stop (single hop too), warning shown as alert, lookup failure on a single hop leaves pad None and returns the route; tests cover S vs M and fail without the fix. uwvo: single hop adds the empty return leg; test shows single equals a loop with no cargo back. yz2l: ROUTE_BUSY swap guard is created after the swap and only on the winning path, so a rejected call never clears the winner''s flag; Drop clears on error, early return and panic unwind; no locks so no deadlock; untested (shell does not compile on Linux). Nit: no test for the pad lookup failing on a single hop. fmt, clippy, workspace tests, cargo metadata, pnpm lint/check/test (95) all pass.'
target: astromech-mljg
identity: review-agent (mljg fix review, sonnet)
commit: fc45f9ce90ea69f710ba0387d0801ea9644fa3bf
created_at: 2026-10-16T23:45:56.586956630Z
event:
  event: approval
---

Fix commit 1b6f53d reviewed. 4d72: pad looked up from Ardent per stop (single hop too), warning shown as alert, lookup failure on a single hop leaves pad None and returns the route; tests cover S vs M and fail without the fix. uwvo: single hop adds the empty return leg; test shows single equals a loop with no cargo back. yz2l: ROUTE_BUSY swap guard is created after the swap and only on the winning path, so a rejected call never clears the winner's flag; Drop clears on error, early return and panic unwind; no locks so no deadlock; untested (shell does not compile on Linux). Nit: no test for the pad lookup failing on a single hop. fmt, clippy, workspace tests, cargo metadata, pnpm lint/check/test (95) all pass.
