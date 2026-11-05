---
schema: atelier.review
schema_version: 1
id: astromech-gasg
title: finding on astromech-2hbw @90b989ff
target: astromech-2hbw
identity: reviewer-sonnet-2hbw
commit: 90b989ffe910d1aedca9a21dc413f7355a0138ea
created_at: 2026-11-05T15:32:58.745467813Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src/modules/route/PlotBar.svelte (key, Enter branch; find in lib/find.ts) Enter takes the highlighted row, so a system typed or pasted and submitted before Spansh answers (250 ms debounce plus the request) opens whatever entry fuzzily matches instead of plotting. Reproduced with scripts/ui (?core=ready&route=exact): Ctrl+K, type 'Sol', wait 120 ms, Enter lands on #/places/services/frontline_solutions with no plot panel; with Spansh answered 'Sol' ranks first (exact system, score 0) and plots. Done when says Plot keeps today's behaviour, where Enter plotted the typed system. Fix: while a system lookup is pending (asking), do not let a score>=2 entry beat the typed text on Enter (rank the typed text as a system candidate, or have Enter wait for the answer); add a test that submits before the suggestions resolve.
