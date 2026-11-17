---
schema: atelier.item
schema_version: 3
id: astromech-7did
title: 'Engineers: search by module type answers where to engineer it'
status: building
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-3kf5
- astromech-5a01
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Ae656cbbe-5a7a-4232-82ca-3aa67f871c76
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.387957133Z
---

## Goal

"Where do I go to engineer my lasers?" is answered by typing "laser". Sumeet, 16 Nov 2026: it is the most common reason to open the Engineers page.

## Done when

- [ ] A search box at the top of Engineering > Engineers takes a module type in the commander's words: "laser", "fsd", "shield", "beam", "thrusters". A small committed table maps words to the catalog's module types; an unknown word says so and lists near matches.
- [ ] A new window command in the command layer returns, per engineer who offers a matched module: the best grade per module type, the commander's rank and unlock state with that engineer, the engineer's system, and its distance from the commander's current system (not Sol). It reads `data/catalog/engineering.json` and reuses the core's `engineers::offers`. It has a test over the recorded preview world and an `every_window_command` entry.
- [ ] The main answer is the best grade the commander can get today: among unlocked engineers, the highest grade their current rank allows, nearest first, with Set destination and Add stop beside it. Engineers who reach higher show the rank or the unlock step still needed. With none unlocked, the main answer is the nearest to unlock and its next step.
- [ ] The rest read as answers in three groups, as `tone5-engineers-laser-2560.png`: what they reach (orange hexagons and a line such as "Pulse 4 · burst 3 · beam 3"), the engineer and the commander's rank (white hexagons, or the invite step), and where (system with copy, distance).
- [ ] Rank and grade use the engineer hexagon (`app/public/engineers/access.svg`) through one shared component, also used wherever the page shows a rank.
- [ ] Suit engineers match by modification ("range", "magazine", "night vision"); the page says so when a weapon word finds none.
- [ ] Clearing the box returns the page as it was. Browser tests cover a match, no match, and rank below the best grade, on recorded replies. The Engineers guide page says how to search.

## Scope

The search, its command and the hexagon component. The unlock tree is astromech-m6gw, which follows this item.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). The mockup named the nearest engineer who reaches grade 5 while the commander was rank 3 with him; that is wrong and the third line above replaces it. `lib/engineerSpecialties.ts` lists only two module types per engineer and cannot drive this.
