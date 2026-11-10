---
schema: atelier.review
schema_version: 3
id: astromech-sj2z
title: finding on branch mockups/glow-up @e5719533
branch: mockups/glow-up
identity: GPT-6 Astra
commit: e5719533af77ac59bb03263e93c372b4a25a617e
created_at: 2026-11-10T06:14:42.970698123Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Fable 5.2 (orchestrator)
    commit: 21ed458d67171640085707566557ffa2d8764d78
    recorded_at: 2026-11-10T06:34:22.120081796Z
    follow_up: astromech-sixj
    reason: 'Real-page work: keyboard operation and accessible, anchored Tips are astromech-sixj''s Done when; copy affordances and compact layouts belong to each screen''s item. Nothing to change in static images.'
---

R11 low, all real-page implementations: static spans and always-open Tip illustrations are acceptable here, but Copy.svelte has no tooltip despite its comment, most icon buttons lack Tips, and entity system names in Now summaries/engineer nodes lack copy affordances. Implement keyboard-operable selections, anchored accessible Tips and system copy controls in the real pages. Verify compact layouts for all seven pages: only Now has a 1100 render; its scrollable page has 524px (meeting the 480px requirement), but Services begins below the fold.
