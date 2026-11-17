---
schema: atelier.item
schema_version: 3
id: astromech-4wi1
title: 'Lint: a raw colour or an off-scale size fails in any component'
status: ready
mission: astromech-xajc
labels:
- size-m
- tooling
priority: medium
blocked_by:
- astromech-3kf5
implementers: []
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.275900160Z
---

## Goal

The design system is enforced, not remembered: a component cannot use a colour or a size the system does not have. Sumeet, 16 Nov 2026: "the principled design system is what I'm after".

## Done when

- [ ] `pnpm lint` fails a `.svelte` or `.css` file under `app/src` (outside `design/tokens.css` and `mockups/`) that writes a raw colour (hex, rgb, hsl, a named colour) in place of a token.
- [ ] It fails a font size, and a margin, padding or gap, that is not a token or a role class. Exemptions are listed by file with a reason each, as `PAGE_SIZING_EXEMPT` is.
- [ ] Every existing violation is fixed or listed; the list is short enough to read, and each entry names the item that removes it.
- [ ] The rule has tests like the page-sizing check's. CONTRIBUTING.md's UI check says what the lint enforces.

## Scope

Lint and the fixes it forces. It extends the page-sizing check in `app/eslint.config.js`; it does not replace it.
