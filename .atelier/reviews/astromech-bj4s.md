---
schema: atelier.review
schema_version: 3
id: astromech-bj4s
title: finding on branch mockups/glow-up @797ea1d7
branch: mockups/glow-up
identity: GPT-6 Astra (round 2)
commit: 797ea1d7c847aff9d48f4bcbda9baf787d1af3f1
created_at: 2026-11-10T06:27:25.304400234Z
event:
  event: finding
  recommendation: fix_now
---

R2-1 medium, Engineers and Ship: Engineers.svelte:89-106 presents measured unlock progress (2 of 5 black markets, 9 of 25 terminals, 420k of 1M bonds, and meters); Ship.svelte repeats 2 of 5. The actual engineering_unlocks path (app/src/lib/unlocks.ts and crates/astromech-engineering/src/unlocks.rs::Node/tree) supplies state, rank and requirement text, not counted progress toward these tasks. EngineerLine also has no such counts. The brief promises existing replies. Remove numeric progress and its meters from these images, including the Dweller headline and Ship warning suffix; keep the known requirements and state. Invented sample values are fine for supported fields, but these promise unplanned data work.
