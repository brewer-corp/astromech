---
schema: atelier.review
schema_version: 1
id: astromech-n1xc
title: finding on astromech-2hbw @90b989ff
target: astromech-2hbw
identity: reviewer-sonnet-2hbw
commit: 90b989ffe910d1aedca9a21dc413f7355a0138ea
created_at: 2026-11-05T15:32:59.223966478Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/route/PlotBar.svelte (add/destination) After keyboard Tab to a system row's Add stop or Set destination and Enter, open=false removes the focused button, so focus falls to body (verified: activeElement is BODY), and the result note under the box (FollowNote.svelte, a plain p with no role=status or aria-live) is not announced; the popover's 'Nothing matches' and 'Asking Spansh' lines are not live either. Fix: after an action return focus to the input, and give the note and the status lines role=status.
