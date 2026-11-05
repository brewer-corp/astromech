---
schema: atelier.review
schema_version: 1
id: astromech-5yze
title: finding on astromech-pey5 @5c36e669
target: astromech-pey5
identity: reviewer-fable-pey5
commit: 5c36e66901b0dc61bd103cf1c8b4951d00cecb0c
created_at: 2026-11-05T06:45:13.871684591Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/ui/About.svelte:264, app/src/modules/presets/View.svelte:699 and app/src/modules/route/NeutronFile.svelte:747 each undo the secondary Button's 8 px padding with margin-left calc(-1 * var(--sp-2)) to line a stacked action word up with the text above it: three copies of Button.svelte's padding value, which drift if the Button changes. A stacked/flush variant on Button (or one shared class) keeps that knowledge in one place.
