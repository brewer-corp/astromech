---
schema: atelier.review
schema_version: 1
id: astromech-w52v
title: finding on astromech-1d92 @5afebb00
target: astromech-1d92
identity: reviewer-fable-1d92
commit: 5afebb00886ec2d015edb250a6fab6d5e189e093
created_at: 2026-11-05T07:56:19.107111194Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/route/Strip.svelte:85 The strip's hard-error state (red '!' when the first read or an action fails) shows the words but no recovery action; DESIGN.md States gives this exact case as '! Could not read the route. TRY AGAIN'. The 2 s poll retries a read on its own and the copy icon retries a copy, so nothing is lost, but the binding example isn't met. Fix later: a TRY AGAIN beside the read failure, or amend DESIGN.md's States row to name a silent retry as the recovery for polled reads.
