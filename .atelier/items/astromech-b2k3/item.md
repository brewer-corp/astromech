---
schema: atelier.item
schema_version: 3
id: astromech-b2k3
title: 'DESIGN.md revision 4: the Glow-up amendments'
status: ready
mission: astromech-xajc
labels:
- size-s
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-10T05:54:56.753081776Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:54:56.753081776Z
---

## Goal

DESIGN.md says the rules the Glow-up mission builds to, before any page is built to them, so every later item has one rulebook. Docs first: a bad rule is cheapest to fix on paper.

## Done when

- [ ] DESIGN.md has the amendments listed under 'DESIGN.md amendments (revision 4)' in .atelier/missions/astromech-xajc/ that no other issue owns: the four page layouts (Board, List with detail, Workbench, Form) with when to use each and the rule 'an action sits next to what it acts on' in place of the content-width caps; white or Options Grey for entity names outside Controls, with the test for when; brief state feedback (what is allowed, how long, and that decorative motion stays banned); the star, body and station icon sets; the collapsed filter summary; the NOW tab and default page in the Frame.
- [ ] Its Do's and Don'ts, Deviations table and Token map agree with the new text, and nothing in the document still states a superseded rule (the 1200 px list cap, 'no motion' without the exception).
- [ ] The scale rule, the new orange values, Tip and the status bar are left to astromech-k6pl, astromech-sixj and astromech-q79q, and DESIGN.md says so nowhere twice.
- [ ] scripts/check-guide.py and the repo's docs checks pass.

## Scope

DESIGN.md only. No tokens, components or pages.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. The amendments were approved by Sumeet on 10 Nov 2026 (decision astromech-ci0o). DESIGN.md is the binding design system (AGENTS.md).
