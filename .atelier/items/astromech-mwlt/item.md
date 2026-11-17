---
schema: atelier.item
schema_version: 3
id: astromech-mwlt
title: 'Token lint: accept exponent numbers, root-relative lengths and decimal whole factors'
status: ready
mission: null
labels:
- tooling
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T09:00:09.190860084Z
trivial: null
caused_by: null
created_at: 2026-11-17T08:59:59.935957085Z
---

## Goal

Taken from deferred finding astromech-4wi1/1-dvey.5, in .atelier/items/astromech-4wi1/reviews/1-dvey.md:

> Finding 5: [P3] Recognise valid CSS exponent numbers and root-relative lengths
> Recommendation: defer
>
> app/eslint-rules/tokens.js:64-66 recognises only plain decimal numbers and omits root-relative font units such as rcap. `padding: 1e1px`, `font-size: 1e1px` and `padding: 2rcap` all pass lint in .css and Svelte. Chromium accepts them; the first two compute to 10px. These are literal lengths, so the rule's stated coverage is broader than its implementation. This is rare syntax with no observed production use and can follow the ordinary-value fixes above. Extend the dimension handling and tests so all supported CSS numeric forms and fixed/font-relative units are checked.

## Done when

- [ ] The token lint checks exponent numbers (`1e1px`) and root-relative font units (`rcap`, `rlh` and the like) as the literal lengths they are, with a test each.
- [ ] A whole-number factor written with a decimal point (`calc(var(--sp-2) * 2.0)`) passes as `* 2` does (finding astromech-4wi1/2-h0lb.5).

## Scope
