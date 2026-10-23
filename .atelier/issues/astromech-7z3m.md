---
created_at: "2026-10-22T19:30:25.162589362+00:00"
id: "astromech-7z3m"
issue_type: "validation"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-3x0x"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T23:46:46.243772197+00:00"
status: "done"
title: "Validate local neutron routes against Spansh"
updated_at: "2026-10-22T23:46:46.243772197+00:00"
---

## Goal

Check independently that local neutron routes are at least as good as Spansh's and that they are followed like any other route.

## Done when

Each claim has an explicit result and linked evidence receipt; the handoff states failures, blocked checks, and proof limits. Claims:

1. For at least six endpoint pairs (inside the bubble, bubble to Colonia, bubble to Beagle Point, one starting off the plane, one ending in sparse space, one under 500 ly) and two ships (an ordinary drive, k = 4, and a Caspian Explorer with the Mk II drive, k = 6), the local route has no more jumps than Spansh's at efficiency 60 with the same range and multiplier.
2. Every stop but the destination is a neutron star, and each leg's jump count is reachable at that range by the stated model.
3. A local route loads into the follower: the first stop is on the clipboard, and arriving at a stop moves the route on (core tests or the browser preview with a recorded route).
4. With no neutron file, the page plots through Spansh and says so.

## Context

Increment astromech-1842 and its children. Spansh's `/api/route` takes `supercharge_multiplier` (astromech-vlsz). Flying a route in the game is astromech-rk4s, not this item.

## Notes

- 2026-10-22 [note] 2026-10-22 handoff, validator-7z3m (agent-factory validate), at 69680437.

  Outcome: all four claims pass. Receipts: claim 1 astromech-yqu6, claim 2 astromech-yh3g, claim 3 astromech-4f1z, claim 4 astromech-zoee.

  Claim 1 (Spansh live, efficiency 60; ordinary R 58.37 k4, Caspian Mk II R 81.62 k6; jumps local / Spansh in the per-leg model / Spansh's own; local stops vs Spansh stops; search):
  | Pair | ly | Ship | Local | Spansh model | Spansh own | Stops L/S | Proof | Search |
  | Achenar-Sothis (bubble) | 576 | k4 | 10 | 10 | 11 | 1/1 | fewest | 0.00 s |
  | Achenar-Sothis | 576 | k6 | 5 | 8 | 8 | 2/1 | fewest | 0.00 s |
  | Sol-Maia (<500 ly) | 383 | k4 | 7 | 7 | 8 | 1/1 | fewest | 0.00 s |
  | Sol-Maia | 383 | k6 | 4 | 5 | 6 | 2/1 | fewest | 0.00 s |
  | Shinrarta Dezhra-Colonia | 22001 | k4 | 116 | 129 | 133 | 105/109 | near | 0.38 s |
  | Shinrarta Dezhra-Colonia | 22001 | k6 | 52 | 52 | 53 | 49/48 | near | 0.76 s |
  | Sol-Beagle Point | 65279 | k4 | 397 | 467 | 502 | 287/301 | near | 0.43 s |
  | Sol-Beagle Point | 65279 | k6 | 159 | 168 | 176 | 146/147 | near | 0.95 s |
  | Braufooe MI-B d0 (y +1823)-Sol | 2362 | k4 | 37 | 46 | 53 | 8/6 | fewest | 0.11 s |
  | Braufooe MI-B d0-Sol | 2362 | k6 | 16 | 23 | 26 | 8/6 | fewest | 0.01 s |
  | Sol-Qiedeau GM-V c2-0 (sparse) | 6157 | k4 | 64 | 80 | 90 | 32/22 | near | 0.58 s |
  | Sol-Qiedeau GM-V c2-0 | 6157 | k6 | 23 | 36 | 40 | 18/14 | fewest | 0.16 s |
  Spansh's total_jumps field equals its leg count in all 12 (astromech-6h7t). File load 0.13 s. Load average ~9.6.

  Proof: scratch harness and data in /tmp/v7z3m.JmR4 (spansh/*.json raw results, run1.log, core-test.log, route-nofile-*.png); nothing committed but these records.

  Limits: claims 1-2 are judged in the plotter's model (normal jumps of a full R through any space, no fuel); local routes lean on sparse d0/d1 neutron stars far from the plane, so in-game jump counts there may exceed the model (astromech-zrbk). Two of Spansh's 411 neutron stops (Dryao Chrea GA-A d8629, Phua Aub YP-G d10-5502) are not in the file; Spansh has no body data for them. One timing run per route. Claim 3 checked by a core test, not in the game (astromech-rk4s). Claim 4's page check was the browser preview, which mocks the plot call; the real fallback path was run live separately.

  Issues filed: none. Seen: Spansh-plotted routes show the leg count as jumps (live Sol-Maia: 1 shown, 8 real), already astromech-6h7t.

## Close Reason

All four claims pass with fresh evidence (receipts astromech-yqu6, astromech-yh3g, astromech-4f1z, astromech-zoee); see the 2026-10-22 handoff note.
