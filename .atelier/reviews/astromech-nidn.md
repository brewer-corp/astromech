---
schema: atelier.review
schema_version: 1
id: astromech-nidn
title: 'app/src/modules/trade/View.svelte:94-113 and crates/astromech-market/src/trade.rs:299-321: the pad filter starves the Anywhere list silently. Ardent ignores the pad and returns a fixed 100 rows, and the pad (default ''Ship''s'', so a large ship filters to L pads) is applied only locally. A large-ship commander sees a few rows, or none, with no note that most of the 100 were dropped and no count (''12 of 100''). Only static copy (''best 100 that pass the filters'') hints at it. Fix: put the kept/fetched counts in the result footer (SearchKit.svelte:114) or the empty state. Also, Prices.truncated (trade.rs:318) is computed before filtering, is always true for real data, and the UI never reads it, here or for Near.'
target: astromech-d1lo
identity: review-agent (d1lo review, sonnet)
commit: 9b7d288da903b76e2d6df0de91c4f4ba1de60958
created_at: 2026-10-17T00:31:04.655577422Z
event:
  event: finding
  recommendation: defer
---

app/src/modules/trade/View.svelte:94-113 and crates/astromech-market/src/trade.rs:299-321: the pad filter starves the Anywhere list silently. Ardent ignores the pad and returns a fixed 100 rows, and the pad (default 'Ship's', so a large ship filters to L pads) is applied only locally. A large-ship commander sees a few rows, or none, with no note that most of the 100 were dropped and no count ('12 of 100'). Only static copy ('best 100 that pass the filters') hints at it. Fix: put the kept/fetched counts in the result footer (SearchKit.svelte:114) or the empty state. Also, Prices.truncated (trade.rs:318) is computed before filtering, is always true for real data, and the UI never reads it, here or for Near.
