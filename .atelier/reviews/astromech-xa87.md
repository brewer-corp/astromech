---
schema: atelier.review
schema_version: 1
id: astromech-xa87
title: finding on astromech-51ue @bfb7f1dd
target: astromech-51ue
identity: reviewer-fable-51ue-plan
commit: bfb7f1ddd9bc83022431ffd734d9a7782c43e41b
created_at: 2026-11-05T06:26:17.512818637Z
event:
  event: finding
  recommendation: defer
---

defer: astromech-kwoi. It describes the Materials side panel as holding "Find a trader", "Find sites" and the HGE pool. In app/src/modules/materials/View.svelte:155 Sites renders in the raw tab's main column, not the side panel; the side panel (line 174) embeds Nearby, a faction-state lookup keyed by the selected material's source query (faction.<query>); no "Find a trader" control exists (trader finding is Galaxy's Services lookup). Fix: name the real embeds so the implementer finds them: Sites under the raw grid, Nearby in the side panel for a material's source query.
