---
schema: atelier.review
schema_version: 1
id: astromech-s55s
title: finding on astromech-51ue @249bf97a
target: astromech-51ue
identity: reviewer-fable-51ue-final
commit: 249bf97a81ab3640206b4a73b02f7eeaf70a95de
created_at: 2026-11-05T16:09:11.413363336Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/route/PlotBar.svelte key() Enter branch and app/src/lib/find.ts sure: a system typed in full and entered before Spansh answers (250 ms debounce plus the request; a paste and Enter, or Enter straight after the last letter) is taken by an entry whose label starts with the text, because a label-start match (score 1) counts as sure. 'Colonia' + Enter opens the Colonisation page ("colonisation".startsWith("colonia")) instead of the plot panel; once Spansh has answered the exact system scores 0 and plots. The pre-2hbw box plotted the typed text in that window, so 'Plot keeps today's behaviour' has a timing hole for systems whose names start an entry label (Colonia is the common one). Not blocking: the window is a few hundred ms, nothing is lost (the box is cleared, Back returns), and the gasg fix chose this trade-off. Fix: while asking, have Enter wait for Spansh's answer and then decide, or count only a whole-label match or a finder phrase as sure, not a label start; add 'Colonia' entered at once to PlotBar.test.ts.
