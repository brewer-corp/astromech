---
schema: atelier.item
schema_version: 3
id: astromech-v4bw
title: Explanations move from page prose into Tips
status: ready
mission: astromech-xajc
labels:
- size-s
priority: medium
blocked_by:
- astromech-sixj
implementers: []
ready:
  by: conversion
  at: 2026-11-10T06:04:05.980926316Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:02.350811962Z
---

## Goal

A page says what it shows and no more; how to read it is one hover away. Today Powerplay prints a paragraph of rules under each of 13 activities, and Exploration, Biology, Session, Mining, On foot, Station and Trade carry footnote paragraphs, one naming scripts/gen-odyssey.py.

## Done when

- [ ] Each explanatory paragraph or footnote on Powerplay, Exploration, Biology, Session, Mining, On foot, Station and Trade is either in a Tip on the thing it explains or deleted; none remains as page prose. Data-source and data-age facts a commander needs to trust a number ("Ardent Insight, 24 d ago") stay, as one Label line.
- [ ] No page names a script, a fixture or a 'sample' source outside the browser preview.
- [ ] docs/guide still explains what was removed from the pages; scripts/check-guide.py passes.
- [ ] Before-and-after captures of Powerplay and Exploration are in docs/design/; pnpm check, pnpm lint and pnpm test pass.

## Scope

Not the Route page: its on-page help goes with its redesign (astromech-irrv).

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Tip is astromech-sixj.
