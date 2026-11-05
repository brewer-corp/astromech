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
closed_at: "2026-11-05T06:49:13.207009238+00:00"
status: "done"
title: "Shared controls and icons per DESIGN.md: input, dropdown, choice, buttons, badge, icon set"
updated_at: "2026-11-05T06:49:13.207009238+00:00"
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

## Close Reason

Implementer report. Built the shared controls DESIGN.md defines in app/src/lib/ui (Input, Dropdown, Choice single/multiple, Button primary/secondary, IconButton, Badge, plus a Suggestions popover for autocompletes) from tokens.css only, and added add stop, set destination, open external, chevron and clear to Glyph, the whole set on a 1 px square-ended stroke. Every page uses them: the inverted Carriers/Odyssey and Held only toggles are choices filled when chosen, every dropdown has a chevron, the header box has no underline at rest, and the lookup GO cell is a set-destination icon. Seg.svelte, the mockups' Icon copy and the grey placeholder rule are gone; the guide (engineering, galaxy, routes, settings) is updated. Proof: in app/, pnpm lint, check (0 errors), build and test (66 files, 397 tests) pass; check-guide 0 problems; before/after shots in docs/design/controls-{trade,materials,settings,header-1100}-*.png from scripts/ui on the Vite dev server (layout only). Deviation: the 1100 px header check failed: the one-row header needs a 1513–1689 px window. Decision astromech-ghyo (9) puts the top tabs on a second 44 px row below about 1600 px; DESIGN.md now says so and astromech-2hbw implements the breakpoint. Review (reviewer-fable-pey5): 4 fix_now fixed in 117acc05 (onboarding Calibration had two fills; plot panel shadow/top border; clear glyph and the header width in DESIGN.md), 2 deferred as astromech-6trg and astromech-0wmn, 1 no action; approved astromech-e5w0. The reviewer settled 9 open design choices in decision astromech-ghyo. Surprise: a stray .atelier/issues/pey5.md record came from the orchestrator copying the start record into the worktree under the wrong name, not from atelier; fixed in 60dddc6d. Open: base-select dropdown styling and hover states unchecked in WebView2 (Windows CI builds it; the look on a real monitor stays with Sumeet, astromech-cxgq). Left to siblings: grey cells (dfvp), grey labels/text (yxtz), font (dzwy), tab-like rows (sllo), one Find or plot box and result rows (51ue children).
