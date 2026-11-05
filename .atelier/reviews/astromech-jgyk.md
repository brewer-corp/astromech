---
schema: atelier.review
schema_version: 1
id: astromech-jgyk
title: finding on astromech-rz0f @2828a4c0
target: astromech-rz0f
identity: reviewer-fable-rz0f
commit: 2828a4c0e6b4301e193a1587c21be434e740b88f
created_at: 2026-11-05T07:28:23.258833911Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/ui/FollowNote.svelte:17,34 the failure text ('Couldn't add it: not running inside the app') is Dim prose, not DESIGN.md States' hard error (red '!' plus a recovery action), and the inline Resume Button overrides the secondary button's 34 px hit area (height: auto). Pre-existing pattern the old goNote line had; fold into astromech-jp36's States work or a small follow-up.
