---
schema: atelier.review
schema_version: 1
id: astromech-hdj3
title: finding on astromech-2hbw @90b989ff
target: astromech-2hbw
identity: reviewer-sonnet-2hbw
commit: 90b989ffe910d1aedca9a21dc413f7355a0138ea
created_at: 2026-11-05T15:32:59.594564264Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/route/PlotBar.svelte listbox markup Combobox wiring is right (role=combobox on the input, aria-expanded, aria-controls, aria-activedescendant, aria-selected, listbox name) but each system option contains focusable Plot, Add stop and Set destination buttons and a tabindex=-1 button, which ARIA says option children cannot be (nested-interactive). Accepted for now, see decision 2 in astromech-wydq-style note; revisit with an axe run if a screen-reader pass is wanted. Tab from the input walks Plot, Add, Set for every system row in turn (up to ten rows, thirty stops).
