---
created_at: "2026-11-07T03:00:10.645958844+00:00"
id: "astromech-sixj"
issue_type: "feature"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Tip: an instant tooltip in place of the native title"
updated_at: "2026-11-07T03:00:10.645958844+00:00"
---

## Goal

Hovering an icon or a star explains it at once. The native title tooltip takes about a second to appear, is small and hard to read, and most people ignore it (Sumeet, 6 Nov 2026).

## Done when

- [ ] IconButton and every other title= tooltip in app/src use Tip, except where the full text is already visible; a test fails on a new title= on an interactive element.
- [ ] Keyboard focus shows it; screen readers get the same text by aria-describedby.
- [ ] DESIGN.md has Tip as a component.

## Design

One shared Tip: shown at once on hover or keyboard focus, content swapped in place when the pointer moves to the next target, gone when it leaves, on scroll or Escape. Hull Raised with an Orange Mid border and a pointer at the target; text at the page's own sizes; above the target, below near the top edge, kept inside the window. Plain (one line) or rich (a title, a fact line, a note). The Route mockup (astromech-ok29) has it.
