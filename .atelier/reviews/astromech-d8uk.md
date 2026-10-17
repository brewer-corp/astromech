---
schema: atelier.review
schema_version: 1
id: astromech-d8uk
title: 'Checked: visits_before SQL (strictly-before ts, kind=system, name+legacy=0 via commanders, uses visits_address index), try_lock + 50ms busy timeout on a WAL read-only connection, silent catch-up (Engine::fire returns early when not live), from_carrier and first-visit silence, unit rounding (30d month, 360d year, floor), 8-word budget incl. 99,999 visits in copy test, check-deps. Tests re-run green. Residual low risks: lookup runs synchronously on the pipeline thread (bounded to ~50 ms worst case, not zero); same-name commanders on different FIDs are merged; carrier-jump arrivals count as earlier visits.'
target: astromech-2myu
identity: review-agent (sonnet, independent)
commit: fb7df4dea7bf43c6474e42c60c43349f2cfbfa9a
created_at: 2026-10-17T02:45:12.319102435Z
event:
  event: approval
---

Checked: visits_before SQL (strictly-before ts, kind=system, name+legacy=0 via commanders, uses visits_address index), try_lock + 50ms busy timeout on a WAL read-only connection, silent catch-up (Engine::fire returns early when not live), from_carrier and first-visit silence, unit rounding (30d month, 360d year, floor), 8-word budget incl. 99,999 visits in copy test, check-deps. Tests re-run green. Residual low risks: lookup runs synchronously on the pipeline thread (bounded to ~50 ms worst case, not zero); same-name commanders on different FIDs are merged; carrier-jump arrivals count as earlier visits.
