---
schema: atelier.item
schema_version: 3
id: astromech-rj6i
title: Powerplay on one screen
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-irtr
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:42.585058701Z
---

## Goal

Powerplay shows standing as progress: rank with a meter to the next and merits to go as the main value, a cycle clock, this system's control as a reinforcement-against-undermining bar, what earns merits here as a compact list with its rules in Tips, and where to work as a short result list.

## Done when

- [ ] (provisional) The page renders the preview standing, system and targets with no rule paragraphs on the page.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockup: docs/design/glow-up-powerplay.png. Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

Whether the uncertain rules (astromech-c1sq, astromech-66pq) show a mark. The mockup's rank meter needs where the current rank starts, which the next-rank reply does not give (only the rank table in powerplay.json has it): the reply gains that threshold, or the meter goes.
