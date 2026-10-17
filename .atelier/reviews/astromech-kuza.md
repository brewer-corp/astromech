---
schema: atelier.review
schema_version: 1
id: astromech-kuza
title: 'R1: Toggle::all() and Screen::all() are hand-written arrays not checked against the variant count, so the pressed_actions test can still miss a new variant.'
target: astromech-g1ys
identity: review-subagent-1
commit: c3094be9b64c767f83b59b3c10ca51e8158a6683
created_at: 2026-10-17T18:43:15.648961629Z
event:
  event: finding
  recommendation: defer
---

R1: Toggle::all() and Screen::all() are hand-written arrays not checked against the variant count, so the pressed_actions test can still miss a new variant.
