---
schema: atelier.review
schema_version: 1
id: astromech-qqws
title: finding on astromech-dfvp @84b18f26
target: astromech-dfvp
identity: reviewer-fable-dfvp
commit: 84b18f26398d85ca5984acce5abcc1a316f7281d
created_at: 2026-11-05T17:20:02.344337386Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/controller/View.svelte:258, app/src/modules/engineering/Build.svelte:513, app/src/modules/keybinds/KeybindsView.svelte:216 Selected rows that a side panel explains (My controller row, the shopping list's material row, the selected binding) still use a 1 px orange outline; DESIGN.md States says a selected row is solid orange with On Orange text while a side panel or keyboard focus shows the selection. Not a cell question, so outside dfvp; a follow-up selection pass over the three pages (the Keybinds one must not collide with the Listening fill).
