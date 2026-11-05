---
schema: atelier.review
schema_version: 1
id: astromech-p5cn
title: finding on astromech-pey5 @5c36e669
target: astromech-pey5
identity: reviewer-fable-pey5
commit: 5c36e66901b0dc61bd103cf1c8b4951d00cecb0c
created_at: 2026-11-05T06:45:13.147502883Z
event:
  event: finding
  recommendation: fix_now
---

low: app/src/modules/route/PlotBar.svelte:82 the plot panel under the header box keeps box-shadow 0 12px 40px, a 2 px orange border-top, a --bg background and an --accent-dim border; DESIGN.md Elevation says popovers are Hull Raised with a 1 px Seam border and no shadow, and the Suggestions popover in the same file already is. Decision 8: fix in pey5, since it is one rule in a file pey5 restyled; 2hbw keeps this panel (its Done when says Plot keeps today's panel). Fix: .panel { background: var(--surface-hi); border: 1px solid var(--line); } and drop box-shadow and border-top. Palette.svelte's shade and box stay for 2hbw, which removes the palette.
