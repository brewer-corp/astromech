---
schema: atelier.review
schema_version: 1
id: astromech-7s4v
title: finding on astromech-pey5 @5c36e669
target: astromech-pey5
identity: reviewer-fable-pey5
commit: 5c36e66901b0dc61bd103cf1c8b4951d00cecb0c
created_at: 2026-11-05T06:45:13.386618226Z
event:
  event: finding
  recommendation: fix_now
---

medium: DESIGN.md:241 'minimum window width 1100 px, and nothing in the frame wraps above it' and the Frame table's 64 px header are false by pey5's own measurement (the one-row header needs 1513-1689 px; docs/design/design-system-header-1100.png shows Find or plot cut off at 1100 px), so DESIGN.md now contradicts what the window does. Decision 9, option (a): below the width where the one-row header fits (about 1600 px; remeasure when 2hbw lands the 320 px box and Ctrl+K hint) the top tabs sit on a 44 px row under the wordmark (header 108 px), as the live app already does; above it the single 64 px row. Change in pey5 now: the Frame table's Header row ('64 px, or 64 + 44 with the top tabs on a second row below about 1600 px') and the sentence ('minimum window width 1100 px; the only width-dependent change in the frame is the header's second tab row'). 2hbw implements the one-row header with that breakpoint; 1d92 is unaffected (the strip is 36 px either way). Sumeet keeps the real-monitor and DPI check.
