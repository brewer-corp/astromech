---
schema: atelier.item
schema_version: 3
id: astromech-sixj
title: 'Tip: an instant tooltip in place of the native title'
status: ready
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-07T03:00:10.645958844Z
trivial: null
caused_by: null
created_at: 2026-11-07T03:00:10.645958844Z
---

## Goal

Hovering an icon or a star explains it at once. The native title tooltip takes about a second to appear, is small and hard to read, and most people ignore it (Sumeet, 6 Nov 2026).

## Done when

- [ ] IconButton and every other title= tooltip in app/src use Tip, except where the full text is already visible; a test fails on a new title= on an interactive element.
- [ ] Keyboard focus shows it; screen readers get the same text by aria-describedby.
- [ ] DESIGN.md has Tip as a component.

## Design

One shared Tip: shown at once on hover or keyboard focus, content swapped in place when the pointer moves to the next target, gone when it leaves, on scroll or Escape. Hull Raised with an Orange Mid border and a pointer at the target; text at the page's own sizes; above the target, below near the top edge, kept inside the window. Plain (one line) or rich (a title, a fact line, a note). The Route mockup (astromech-ok29) has it.
