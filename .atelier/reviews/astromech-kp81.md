---
schema: atelier.review
schema_version: 2
id: astromech-kp81
title: finding on astromech-bp23 @36531f8c
target: astromech-bp23
identity: claude-reviewer-ro7a
commit: 36531f8c99c5623824057f12726be11a1e8f58ce
created_at: 2026-11-07T11:16:05.856073101Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/ARCHITECTURE.md:1746 'With the typed call this is success criterion one-command-list' says the criterion is met by this in-process test plus call(). The criterion needs the headless core and the headless mode ('reachable over the control API and the headless mode ... control API of a headless core'), and ro7a's own Done when says it only advances the criterion, which astromech-vucf completes. An agent reading §18.2 would take one-command-list as done. Fix: 'With the typed call this advances one-command-list; astromech-vucf completes it by running the same table against the headless core and its scripted world.'
