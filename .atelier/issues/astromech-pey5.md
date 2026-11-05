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
updated_at: "2026-11-05T06:34:13.569222160+00:00"
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
- 2026-11-05 [note] 1100 px header check (pey5, 5 Nov): DESIGN.md's one-row header does not fit at 1100 px. Measured in the ds mockup frame (Saira Variable 87.5 %, Nav caps, 12 px tab padding) with Find or plot at DESIGN's 320 px: wordmark 122 + eight top tabs 808 + Find 320 + four 24 px gaps 96 + Help 24 + status 63-239 px = 1433-1609 px of content, so a 1513-1689 px window with 40 px margins; at 1100 px (1020 px inside the margins) it is 413-589 px over. The tabs, wordmark and Find alone need 1322 px. Shot: docs/design/design-system-header-1100.png (scripts/mockup-shots.sh ds-settings). The live app keeps today's two-row header (tabs under the wordmark), which fits at 1100 px (docs/design/controls-header-1100-after.png). Options for a decision: (a) accept two header rows below a breakpoint and change the Frame table (64 px becomes 64 + 44 under it); (b) shrink the row: Find or plot collapses to a Ctrl+K button that opens it as a popover, the status becomes one word with the rest in a tooltip, tab padding 12 to 8 px (roughly 400 px saved, fits only with a short status); (c) move status and Help out of the header (status to the route strip's right end) and collapse Find to a button: about 1038 px, still about 20 px over at 1100 px unless tab padding also drops to 8 px (then about 975 px, which fits); (d) raise the minimum window width to about 1520-1700 px; (e) fewer top tabs (merge groups). DESIGN.md's number is unchanged pending that decision.
