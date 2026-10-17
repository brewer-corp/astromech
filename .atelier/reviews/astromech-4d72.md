---
schema: atelier.review
schema_version: 1
id: astromech-4d72
title: 'Medium-pad limitation is only in a code comment (crates/astromech-market/src/route.rs:14-15), not shown to the user. route.rs:272 sends requires_large_pad only for Pad::L and route.rs:273 always allows planetary, so with pad M (the default for a medium ship such as a Python or Krait; the component test runs with pad M) Spansh can pick a destination with only small pads (some Odyssey settlements). Nothing in app/src/modules/trade/Route.svelte (no pad text near :94 or :121-126) or the docs says so. A commander follows the route and can''t dock. Fix: show a note in Route mode when the pad filter is M, or better, look up the destination''s maxLandingPadSize in Ardent markets (already fetched for loops) and flag or reject a destination that doesn''t fit.'
target: astromech-mljg
identity: review-agent (mljg review)
commit: 517349a4337bddd467c02933a13265ea5c64340f
created_at: 2026-10-16T23:39:53.376761915Z
event:
  event: finding
  recommendation: fix_now
---

Medium-pad limitation is only in a code comment (crates/astromech-market/src/route.rs:14-15), not shown to the user. route.rs:272 sends requires_large_pad only for Pad::L and route.rs:273 always allows planetary, so with pad M (the default for a medium ship such as a Python or Krait; the component test runs with pad M) Spansh can pick a destination with only small pads (some Odyssey settlements). Nothing in app/src/modules/trade/Route.svelte (no pad text near :94 or :121-126) or the docs says so. A commander follows the route and can't dock. Fix: show a note in Route mode when the pad filter is M, or better, look up the destination's maxLandingPadSize in Ardent markets (already fetched for loops) and flag or reject a destination that doesn't fit.
