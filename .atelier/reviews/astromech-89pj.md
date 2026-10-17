---
schema: atelier.review
schema_version: 1
id: astromech-89pj
title: 'crates/astromech-announce/src/arrival.rs:53-55 and app/src/lib/engineering.ts:297 (irtf fix): ''until'' is a fixed count (have + collect at plan time), so once the commander crafts the blueprint or makes the plan''s trades with the app closed or off the Build page, the consumed materials fall below ''until'' and come back on the shopping list; the arrival and HGE rules name them again until the app recomputes that ship''s plan. Collecting is fixed; consuming re-creates the stale-list symptom. Over-cap lines (have + collect > capacity) likewise stay until a recompute, which is correct while the plan is open. Fix later: recompute saved plans in the core on materials change, or have the app re-share every saved plan on inventory change, not only the open ship''s.'
target: astromech-77a1
identity: review-agent (77a1 fix review, sonnet)
commit: 6256a28557b84d63e9e7158bc0d6f516b24e2647
created_at: 2026-10-17T02:18:08.588130512Z
event:
  event: finding
  recommendation: defer
---

crates/astromech-announce/src/arrival.rs:53-55 and app/src/lib/engineering.ts:297 (irtf fix): 'until' is a fixed count (have + collect at plan time), so once the commander crafts the blueprint or makes the plan's trades with the app closed or off the Build page, the consumed materials fall below 'until' and come back on the shopping list; the arrival and HGE rules name them again until the app recomputes that ship's plan. Collecting is fixed; consuming re-creates the stale-list symptom. Over-cap lines (have + collect > capacity) likewise stay until a recompute, which is correct while the plan is open. Fix later: recompute saved plans in the core on materials change, or have the app re-share every saved plan on inventory change, not only the open ship's.
