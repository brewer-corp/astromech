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
title: "Shared controls per DESIGN.md: one input, dropdown, choice, button and badge"
updated_at: "2026-10-24T23:27:52.142923521+00:00"
---

## Goal

Replace the five input styles, five button styles and four badge styles found in the 24 Oct audit with the single components DESIGN.md defines (Components: Input, Dropdown, Choice, Buttons, Badge), so a control looks the same on every page.

## Done when

- `app/src/lib/ui` has one Input, Dropdown (option cell, grey chevron, value text never orange), Choice (single and multiple; the on state is the brighter one), Button (primary: the one orange fill; secondary: orange caps text action) and Badge (outlined, mixed case), using only `tokens.css` tokens.
- Pages use them; the inverted Carriers/Odyssey toggles and chevron-less brown dropdowns are gone.
- The header "Plot to a system" box no longer has an orange underline at rest (focus is the 1 px orange outline).
- `pnpm lint && pnpm check && pnpm build && pnpm test` pass; screenshots of Trade, Materials and Settings before and after.
