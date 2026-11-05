---
schema: atelier.review
schema_version: 1
id: astromech-pron
title: finding on astromech-pey5 @5c36e669
target: astromech-pey5
identity: reviewer-fable-pey5
commit: 5c36e66901b0dc61bd103cf1c8b4951d00cecb0c
created_at: 2026-11-05T06:45:14.107980050Z
event:
  event: finding
  recommendation: no_action
---

low: app/src/lib/ui/Input.svelte:43 the error prop and its alarm '!' line have no caller on this branch. It is DESIGN.md's stated Input error treatment and two lines, so keep it; noted so the next page with a field error uses it rather than a one-off.
