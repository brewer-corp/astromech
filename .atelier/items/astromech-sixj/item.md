---
schema: atelier.item
schema_version: 3
id: astromech-sixj
title: 'Tip: an instant tooltip in place of the native title'
status: changes_requested
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3Ab5fcac56-311f-4ab2-8d10-52e880842435
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

- [x] IconButton and every other title= tooltip in app/src use Tip, except where the full text is already visible; a test fails on a new title= on an interactive element.
- [x] Keyboard focus shows it; screen readers get the same text by aria-describedby.
- [x] DESIGN.md has Tip as a component.

## Design

One shared Tip: shown at once on hover or keyboard focus, content swapped in place when the pointer moves to the next target, gone when it leaves, on scroll or Escape. Hull Raised with an Orange Mid border and a pointer at the target; text at the page's own sizes; above the target, below near the top edge, kept inside the window. Plain (one line) or rich (a title, a fact line, a note). The Route mockup (astromech-ok29) has it.

## Report

Hovering or tabbing to an icon, a value or a link now shows its explanation at once, in the window's own look, and screen readers read the same words.

- **Tip** is `app/src/lib/ui/tip.ts`: a Svelte attachment, `{@attach tip("…")}` or `tip({ title, fact, note })`, with one shared popover in `document.body` styled in `app/src/design/tokens.css`. It shows on pointerenter and on `:focus-visible` focus, swaps in place on the next target, and hides on pointerleave, blur, scroll and Escape. It sits above the target, below near the top edge, clamped inside the window, with the pointer aimed at the target's middle.
- **Screen readers:** each target gets `aria-describedby` to a hidden node holding the words, except when the words equal its `aria-label` (an IconButton without a separate `tip`), so the name isn't read twice. A target that can't take focus gets `tabindex=0`; `stop: false` skips that for a mark repeated down a column whose words are its accessible name (StationKind, the unlocks access mark).
- **Converted:** IconButton, Choice (option field `title` renamed `tip`), Sheet cells, the sub-tabs, Help and Finish setup in `App.svelte`, the Spansh/Inara/Coriolis/EDSY/Resume/Copy buttons, Keybinds and Presets cells, Trade links, the route strip's destination link, the Specs/Stats jump values, handling stat, row heads and warnings, the trade route estimate line, the powerplay band bar, StationKind, and the two DESIGN.md mockups.
- **Left as native `title`:** non-interactive text whose title repeats its own visible text, a name cut short by an ellipsis (SearchKit cells, the Home tile's context, the route strip's system and warnings, inline Failure, the ship's location). ShipIcon's fallback title is gone: the type name is printed beside it in both places, so its `name` prop is removed too.
- **Guard:** `app/src/lib/ui/tip.test.ts` parses every `.svelte` in `app/src` with `svelte/compiler` and fails on `title=` on `a`, `button`, `input`, `select`, `textarea`, `summary`, an element with `tabindex`, `onclick` or an interactive role, or on `Button`/`IconButton`.
- **Behaviour test:** `app/src/lib/ui/tip.browser.test.ts` covers hover, swap in place, leave, Escape, scroll, keyboard focus via real Tab presses with the accessible description, and the tab-stop rule. Tests that found buttons by title now find them by accessible description.
- **DESIGN.md:** a new Tip component section. Icon button, Icon Or Word, Elevation and Shapes now mention it, since its Orange Mid border is a new border use.
- **Captures:** `docs/evidence/astromech-sixj/tip-above-jump-value.png` (Engineering, above a jump value) and `tip-below-help.png` (Help, below near the top edge, clamped at the right). There is no before capture because headless Chrome doesn't draw native titles.
- **Checks:** `pnpm lint`, `pnpm check`, `pnpm build`, `scripts/check-dist.py` and `scripts/check-guide.py` pass. Vitest node: 131 passed. Vitest browser: 439 passed. Port 63315 was busy from another thread, so the browser run used `--api.port`.
- **For review:** with no delay, every keybinds and presets cell shows a Tip as the pointer crosses the grid. That follows the item's design, but it may be busy in use. DESIGN.md is also being revised by astromech-b2k3, so the Tip section may need a rebase there.

### Responses

- 1-x9f4.1: fixed in e62cc7fb: `.tip.rich:not([hidden])`; the browser test loads tokens.css and checks drawn visibility, with a rich case.
- 1-x9f4.2: fixed in e62cc7fb: the Pad options use `tip`, and `ChoiceOpt` now refuses `title`.
- 1-x9f4.3: fixed in e62cc7fb: a recreated Tip reshows on `:focus-visible` too, with a test for a focused content change.
- 1-x9f4.4: fixed in e62cc7fb: `docs/guide/using-the-app.md` says how Tips appear, go and reach the keyboard.

