---
schema: atelier.review
schema_version: 1
id: astromech-ztlk
title: finding on astromech-dfvp @84b18f26
target: astromech-dfvp
identity: reviewer-fable-dfvp
commit: 84b18f26398d85ca5984acce5abcc1a316f7281d
created_at: 2026-11-05T17:20:02.752028845Z
event:
  event: finding
  recommendation: defer
---

low: scripts/recolour-sheets.py:12,25 The controller sheet recolouring still bakes the retired grey #2b2d2e (OPT) into the VKB sheet header images under app/public/devices/vkb; the branch only rewrote the docstring to say so. The shipped PNGs therefore keep a colour no token names. Follow-up: map the header to --surface-hi (or --surface) and regenerate the PNGs, checked in the real window; binary assets, so not in this CSS branch.
