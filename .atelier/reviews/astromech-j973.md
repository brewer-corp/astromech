---
schema: atelier.review
schema_version: 3
id: astromech-j973
title: finding on branch mockups/glow-up @1e54c768
branch: mockups/glow-up
identity: GPT-6 Astra
commit: 1e54c76876691f849010741ceae168ff4f230f0b
created_at: 2026-11-10T06:15:17.467225119Z
event:
  event: finding
  recommendation: defer
---

R13 low, frame scaling: Frame.svelte uses max(1,min(1.8,height/1000,width/1100)), whereas revision 4 specifies height-based scaling only. The width cap and floor are sensible safeguards for the minimum window and 12px text, and do not compromise the supplied captures. Carry this exact formula into the foundation specification before implementation; validate tall narrow windows and Text size interaction rather than silently treating this as the agreed height-only rule.
