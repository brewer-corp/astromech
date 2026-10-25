---
created_at: "2026-10-24T23:27:52.142923521+00:00"
id: "astromech-pey5"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Shared controls and icons per DESIGN.md: input, dropdown, choice, buttons, badge, icon set"
updated_at: "2026-10-25T19:02:01.384978423+00:00"
---

## Goal

Replace the five input, five button and four badge styles found in the 24 Oct audit with the single components DESIGN.md defines, and add its icon set, so a control looks the same on every page.

## Done when

- `app/src/lib/ui` has one Input and Dropdown (ember cell, orange value, orange chevron), Choice (ember cells; the chosen option a solid orange fill with dark text, single and multiple), Button (primary: the one orange fill per view; secondary: orange caps text action), Icon button (28 px, Orange Mid at rest, Orange Lit on hover, tooltip and accessible label always) and Badge (outlined, mixed case), using only `tokens.css` tokens.
- `Glyph.svelte` gains the DESIGN.md icons: add stop, set destination, open external and chevron (drawings in `app/src/mockups/ds/Icon.svelte`); all own-drawn, 1 px stroke.
- Pages use them; the inverted Carriers/Odyssey toggles, chevron-less dropdowns and the orange underline on the header box at rest are gone; repeated row actions are icon buttons.
- `pnpm lint && pnpm check && pnpm build && pnpm test` pass; screenshots of Trade, Materials and Settings before and after.
