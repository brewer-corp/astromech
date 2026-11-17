---
schema: atelier.item
schema_version: 3
id: astromech-rhn3
title: Finder rows that go compact stay compact after the window widens
status: building
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by:
- astromech-srab
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A17040ddb-fde9-4b03-9dbd-722b1d3ea977
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T09:35:15.494625804Z
trivial: null
caused_by: astromech-3ca6
created_at: 2026-11-17T09:35:15.407940249Z
---

## Goal

A finder's rows return to their two-line form when there is room again. Found by astromech-7obs's implementer on 18 Nov 2026: at 160 percent Text size with the sidebar folded, Sell's rows switch to their one-line compact form at a page column of 1,823 px and stay compact after the window is widened to 2,130 px. `SearchKit.svelte` measures `.out`, whose width in `Split`'s side layout shrinks to fit the rows, so once the rows are compact the measured width never grows back.

## Done when

- [ ] A failing browser test first: the rows go compact when the page column narrows, and return to two lines when it widens again, at 160 percent with the sidebar folded.
- [ ] The fix is at the cause: SearchKit measures the space its rows are given, not an element that shrinks to fit them.
- [ ] The dock-width test still passes.

## Scope

The shared result row's compact switch.
