---
schema: atelier.review
schema_version: 1
id: astromech-5vo7
title: 'low: app/src/modules/materials/View.svelte:12-13 decodeURIComponent throws URIError on a malformed hash like #/materials/%E0%A4%A, and readSel runs in the component initialiser, so the Materials page fails to render. Wrap in try/catch. Also Palette.svelte:30-33 pick() does not restore focus (page ends with focus on body); acceptable but note.'
target: astromech-2u1x
identity: review-agent (2u1x review, sonnet)
commit: 75aae4400dc5ad652ba7f629a31f99ad41e2cfbd
created_at: 2026-10-16T22:31:24.139323272Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/materials/View.svelte:12-13 decodeURIComponent throws URIError on a malformed hash like #/materials/%E0%A4%A, and readSel runs in the component initialiser, so the Materials page fails to render. Wrap in try/catch. Also Palette.svelte:30-33 pick() does not restore focus (page ends with focus on body); acceptable but note.
