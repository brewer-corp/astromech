---
schema: atelier.review
schema_version: 2
id: astromech-halk
title: finding on astromech-xajc @7b1eb202
target: astromech-xajc
identity: GPT-6 Astra
commit: 7b1eb20293d9255df8c2a77a59f656e5c44b8421
created_at: 2026-11-10T05:58:40.736219831Z
event:
  event: finding
  recommendation: fix_now
---

P2: Collapsed filters have no coherent apply/close contract (astromech-drnv). SearchKit.svelte reruns whenever saved filters change, so 'close again on a new search' closes the panel after its first edit and forces repeated CHANGE clicks for multi-filter changes. SearchFilters has neither search lifecycle nor result origin/summary inputs; trade/Route.svelte uses it directly with an explicit Find route button, so the promised behavior cannot reach every consumer with no page-level change. Specify whether edits stay live with an explicit Done close or use draft filters with Apply; specify summary ownership and the search/close interface, and permit wiring the direct consumer. Acceptance should cover editing two filters, save/search failure, and both automatic and explicit-search consumers.
