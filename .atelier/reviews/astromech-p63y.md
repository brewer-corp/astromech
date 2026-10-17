---
schema: atelier.review
schema_version: 1
id: astromech-p63y
title: 'R2 [low] The label still goes stale on the Trade page''s Route mode. app/src/modules/trade/Route.svelte:94 renders the same SearchFilters with the filters it loaded once (line 44) and passes no shipPad, but tradeRoute re-resolves the pad on every search (main.rs:916 via search_origin). After a ship change, Route shows ''Ship''s (M)'' while routing for L: the bug this issue fixes, still present in the component''s other consumer. Fix: refresh the pad after each tradeRoute run, the way SearchKit does, or pass the resolved pad back with the result.'
target: astromech-ssyo
identity: review-agent (opus, independent)
commit: 7d5fa6d67b38f916be0bf4bc76d95f87e7d884d0
created_at: 2026-10-17T06:09:22.816662480Z
event:
  event: finding
  recommendation: fix_now
---

R2 [low] The label still goes stale on the Trade page's Route mode. app/src/modules/trade/Route.svelte:94 renders the same SearchFilters with the filters it loaded once (line 44) and passes no shipPad, but tradeRoute re-resolves the pad on every search (main.rs:916 via search_origin). After a ship change, Route shows 'Ship's (M)' while routing for L: the bug this issue fixes, still present in the component's other consumer. Fix: refresh the pad after each tradeRoute run, the way SearchKit does, or pass the resolved pad back with the result.
