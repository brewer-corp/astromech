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
status: "todo"
title: "Validate local neutron routes against Spansh"
updated_at: "2026-10-22T19:30:25.162589362+00:00"
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
