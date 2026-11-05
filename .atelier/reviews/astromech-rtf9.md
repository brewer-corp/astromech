---
schema: atelier.review
schema_version: 1
id: astromech-rtf9
title: finding on astromech-0k63 @84b18f26
target: astromech-0k63
identity: reviewer-fable-dfvp
commit: 84b18f26398d85ca5984acce5abcc1a316f7281d
created_at: 2026-11-05T17:19:27.861020913Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/route/Strip.svelte:63,92,117 The same failed clipboard write is now a red hard error with Try again on the Route page (0k63) but still an orange '!' warning with no action in the route strip (the strip's warning() puts clipboard_error through .warn, Orange). DESIGN.md: a failed write is a hard error, red, and each hue keeps one meaning; the strip already renders read errors with <Failure inline>. Out of 0k63's Done when (View.svelte only), so defer: a follow-up moves the strip's clipboard line to Failure inline with the same copyNext recovery.
