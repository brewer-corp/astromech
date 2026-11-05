---
schema: atelier.review
schema_version: 1
id: astromech-80jd
title: finding on astromech-pey5 @5c36e669
target: astromech-pey5
identity: reviewer-fable-pey5
commit: 5c36e66901b0dc61bd103cf1c8b4951d00cecb0c
created_at: 2026-11-05T06:45:13.620705420Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/ui/Suggestions.svelte:27 ul[role=listbox] > li > button[role=option]: the li keeps its implicit listitem role between the listbox and its options (a listbox may own only option or group), and the options are tabbable buttons, so Tab from the input lands on the first suggestion, which blurs the input and closes the list under it 150 ms later (OriginInput, PlotBar, CommodityInput onblur). Pre-existing in all four lists this component replaces; not worsened. Follow-up: role=none on the li (or drop ul/li) and tabindex=-1 on the options, since aria-activedescendant already carries the highlight.
