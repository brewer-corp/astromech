---
created_at: "2026-10-24T23:27:52.142923521+00:00"
id: "astromech-pey5"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-pey5
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
status: "in_progress"
title: "Shared controls and icons per DESIGN.md: input, dropdown, choice, buttons, badge, icon set"
updated_at: "2026-11-05T06:13:00.869514383+00:00"
---

## Goal

Replace the five input, five button and four badge styles found in the 24 Oct audit with the single components DESIGN.md defines, and add its icon set, so a control looks the same on every page.

## Done when

- `app/src/lib/ui` has one Input and Dropdown (ember cell, orange value, orange chevron), Choice (ember cells; the chosen option a solid orange fill with dark text, single and multiple), Button (primary: the one orange fill per view; secondary: orange caps text action), Icon button (28 px, Orange Mid at rest, Orange Lit on hover, tooltip and accessible label always) and Badge (outlined, mixed case), using only `tokens.css` tokens.
- `Glyph.svelte` gains the DESIGN.md icons: add stop, set destination, open external and chevron (drawings in `app/src/mockups/ds/Icon.svelte`); all own-drawn, 1 px stroke.
- Pages use them; the inverted Carriers/Odyssey toggles, chevron-less dropdowns and the orange underline on the header box at rest are gone; repeated row actions are icon buttons.
- `pnpm lint && pnpm check && pnpm build && pnpm test` pass; screenshots of Trade, Materials and Settings before and after.

## Notes

- 2026-11-04 [note] Deferred from decision astromech-5p9c (Fable R8): DESIGN.md says the minimum window width is 1100 px and nothing in the frame wraps above it, but no mockup or check proves the header (wordmark, eight top tabs, 320 px Find or plot, status, Help) fits at 1100 px. Check it at 1100 px when the shared Input lands and shrink or drop parts until it does, then fix the number in DESIGN.md if it has to change. The screenshot half is done on c4i0: scripts/mockup-shots.sh now renders the ds-* mockups (scripts/mockup-shots.sh ds-sell ...); add a narrow-window shot here.
