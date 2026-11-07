---
created_at: "2026-11-07T02:59:43.905221193+00:00"
id: "astromech-ok29"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-irrv"
  - kind: "issue"
    id: "astromech-qwvs"
  - kind: "issue"
    id: "astromech-woqv"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route page redesign: the title is the plotter, the route a timeline"
updated_at: "2026-11-07T15:10:12.818348629+00:00"
---

## Goal

The Galaxy > Route page shows where you are and what's next at a glance, and is where you plot. Agreed with Sumeet on 6 Nov 2026 from the artifact mockup https://claude.ai/artifact/JduR92Bxotyi7hd6Jg8nN7 (version 9); built in the real page against fakecore, shared with scripts/preview-publish.

## Done when

- [ ] Every child is done and the page matches the design in the preview at 1440p.
- [ ] The page holds up in the preview at 1100 px and 2560 px, and the route tests pass.

## Design

- The title is the plotter: `Khepri to [destination]`, the destination editable in place; Plotter (Neutron, Exact) and Worked out (On this PC, By Spansh) dropdowns, the neutron star file's status, download and check in the second; one Replot from here; a line when the ship's range differs from the range it was plotted at. No side panel, no import box: Import, Open on Spansh and Clear in a ⋯ menu; the galaxy-map help behind a ?.
- The route is a timeline whose nodes are the stars' type icons (scoopable, neutron, white dwarf, brown dwarf). The leg you are on opens out one row a jump, your row highlighted with a map pin. Other legs show only the stars passed, a jump count past five jumps, and a warning when there is nowhere to scoop. No distance columns.
- The next stop is the one raised row: larger name, its copy icon beside it, "N jumps away". Other stops: copy icon and Make next on hover. Skip and Back buttons go (voice keeps them).
- Inhabited systems get a second line: allegiance and government, security, population, controlling faction and its state.
- No "On your clipboard" line: the copy icon turns to a check; a failed copy is the red error.
