---
schema: atelier.review
schema_version: 1
id: astromech-qwzi
title: finding on astromech-jp36 @e2a7b6bb
target: astromech-jp36
identity: reviewer-sonnet-jp36
commit: e2a7b6bb7dfdbdef096bb343d095fa592db019c8
created_at: 2026-11-05T08:24:34.714598589Z
event:
  event: finding
  recommendation: fix_now
---

low: app/src/lib/ui/Allegiance.svelte:1 is now unused: SearchKit was its only importer and this change replaced the allegiance badge with the text Faction column (grep finds no other reference in app/src, docs or DESIGN.md). Dead component left by the change; delete it.
