---
schema: atelier.review
schema_version: 1
id: astromech-uwvo
title: 'Single-hop profit per hour leaves out the flight back. crates/astromech-market/src/route.rs:130 documents per_hour as ''Estimated profit per hour of repeating the route'', but route.rs:301-304 divides a single route''s profit by the one-way leg time only. You can''t repeat a single hop without returning, so its cr/h is about double that of the same hop as a loop with an empty way back. The page shows them side by side (Route.svelte:122), so Single looks twice as good as it is (preview: 2,206,109 cr/h single). Fix: count an empty return leg in the single-hop time, or label the single figure as one-way and correct the doc comment.'
target: astromech-mljg
identity: review-agent (mljg review)
commit: 517349a4337bddd467c02933a13265ea5c64340f
created_at: 2026-10-16T23:39:53.431488253Z
event:
  event: finding
  recommendation: fix_now
---

Single-hop profit per hour leaves out the flight back. crates/astromech-market/src/route.rs:130 documents per_hour as 'Estimated profit per hour of repeating the route', but route.rs:301-304 divides a single route's profit by the one-way leg time only. You can't repeat a single hop without returning, so its cr/h is about double that of the same hop as a loop with an empty way back. The page shows them side by side (Route.svelte:122), so Single looks twice as good as it is (preview: 2,206,109 cr/h single). Fix: count an empty return leg in the single-hop time, or label the single figure as one-way and correct the doc comment.
