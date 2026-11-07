---
schema: atelier.review
schema_version: 2
id: astromech-1qa8
title: finding on astromech-l5c6 @9af0c06b
target: astromech-l5c6
identity: claude-reviewer-l5c6
commit: 9af0c06b2de026fe1110a8df9df885c82c22b459
created_at: 2026-11-07T03:07:32.251629194Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/keybinds/KeybindsView.svelte:18,83 (?q=, ?bind=) and app/src/modules/controller/View.svelte:25,35 (?device=, ?activity=) still read location.search directly, so a preview build can't bake those in via --query, though CONTRIBUTING now says preview options go through it. Also previewQuery (app/src/lib/previewquery.ts:8) is all-or-nothing: any query string on the page (e.g. one the artifact host adds) drops the whole baked-in query, core included. Fix: route those readers through previewQuery and merge per key (page query overrides baked keys) rather than replacing.
