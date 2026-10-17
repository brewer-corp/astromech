---
schema: atelier.review
schema_version: 1
id: astromech-t3xw
title: 'R3 [low] app/src/modules/powerplay/Suggestions.svelte:17-20: screen readers get the unverified reason via the sr-only text (fine), but sighted users can only read the reason, and the meaning of the ethos and weak-spot tags, from a title tooltip on a non-focusable span, so keyboard-only users cannot reach it. Some screen readers may also read the reason twice (title plus sr-only). Suggest a focusable toggle/details element or a visible line for the reason, and dropping the title when the sr-only text is present.'
target: astromech-w62j
identity: review-agent (w62j review)
commit: 6ffb066bc9dc8ff1509916c30dc9fd799acbbb41
created_at: 2026-10-17T01:10:15.540253566Z
event:
  event: finding
  recommendation: defer
---

R3 [low] app/src/modules/powerplay/Suggestions.svelte:17-20: screen readers get the unverified reason via the sr-only text (fine), but sighted users can only read the reason, and the meaning of the ethos and weak-spot tags, from a title tooltip on a non-focusable span, so keyboard-only users cannot reach it. Some screen readers may also read the reason twice (title plus sr-only). Suggest a focusable toggle/details element or a visible line for the reason, and dropping the title when the sr-only text is present.
