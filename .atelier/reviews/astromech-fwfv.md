---
schema: atelier.review
schema_version: 1
id: astromech-fwfv
title: approval on astromech-rz0f @2828a4c0
target: astromech-rz0f
identity: reviewer-fable-rz0f
commit: 2828a4c0e6b4301e193a1587c21be434e740b88f
created_at: 2026-11-05T07:28:23.461124433Z
event:
  event: approval
---

Reviewed 80063782..2828a4c0 against rz0f's Done when, bi94, g8d5 (items 2 and 3) and DESIGN.md. Read Desk::set/follow/add/add_next and the parked-route interaction with plotted/resume/clear; every route_set caller (Set destination, import, engineer trip, plotter) inherits the park; the changed assertion in resume_brings_the_parked_route_back_and_copies is right (a galaxy-map route is not ours to park, so the parked trip stays); fakecore's follow/addNext mirror the core; icon buttons carry aria-label and tooltip naming the place, visible at rest, lit on row hover, focus-visible outline. Reran: cargo test -p astromech-core --lib (211 pass) and --test gear_down (21 pass), clippy --workspace --all-targets -D warnings clean, fmt clean; app lint, svelte-check 0/0, build, vitest 398 pass; check-guide.py 0 problems; screenshots route-actions-2 and -4 show the actions, the added next stop and the Replaced/RESUME line. Three low defer findings, none blocking.
