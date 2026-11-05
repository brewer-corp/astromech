---
created_at: "2026-10-24T23:27:52.702017911+00:00"
id: "astromech-sllo"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-sllo
priority: "P2"
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
  - kind: "issue"
    id: "astromech-kwoi"
    type: "blocked_by"
  - kind: "review"
    id: "astromech-hehd"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T18:52:20.456076885+00:00"
status: "done"
title: "Two tab rows at most: fold third-level tabs into choices or sections"
updated_at: "2026-11-05T18:52:20.456076885+00:00"
---

## Goal

Apply DESIGN.md The Two Tab Rows Rule: navigation is top tabs plus one row of page tabs.

## Done when

- No page shows more than two tab rows; third levels (Raw / Manufactured / Encoded, On foot item tabs, Ship / Suit engineers, Trade modes) become Choice cells, sections or their own pages.
- The Engineering band is gone.
- URLs for the folded levels still work (query or hash state).
- Screenshots of Materials, On foot, Engineers and Ships before and after.

## Context

On foot and Materials stack up to four tab rows, each in a different caps style, and Engineering pages have a ~95 px dark band between rows.

## Notes

- 2026-11-05 [note] 2026-11-05 (orchestrator): resequenced: sllo now waits for astromech-kwoi instead of kwoi waiting for sllo. kwoi moves Sell/Buy/Sell my cargo out of Trade and links Materials' embedded lookups into Places, removing most of Trade's third tab level; doing sllo first on main would rework the same pages the 51ue branch rewrites. After 51ue merges, sllo folds the third levels that remain (Raw/Manufactured/Encoded, On foot item tabs, Ship/Suit engineers, whatever Trade keeps).
- 2026-11-05 [note] 2026-11-05: deferred finding astromech-hehd: when kwoi closes, re-read this Done when: Sell, Buy and Sell my cargo will have left Trade, so 'Trade modes' may only mean what Trade keeps (routes, Colonisation).

## Close Reason

Implementer report. Every page has at most two tab rows (DESIGN.md The Two Tab Rows Rule). Third levels became Choices: Materials kinds (#/materials?kind=; #/materials/<id> still picks its kind), On foot views (#/odyssey?view=) and item kinds, Ships Info/Engineering (#/engineering/<id>[/plan] unchanged), Ship/Suit engineers (#/unlocks?kind=suit), the controller's stick; tab-styled rows inside sections (Powerplay's Where to work, colonisation candidates' bodies of interest) too. Colonisation's construction sites became a Dropdown. The padded, ruled Engineering band is gone. The shell resolves the page from the hash part before the first / or ?, and lib/hashstate.ts keeps each page's choices, so links, Back and reload keep them; Find or plot and voice routes are unchanged. Trade had no third level left after kwoi. Guides and ARCHITECTURE.md updated. Proof: app/src/tabrows.test.ts counts tab rows on every page and finder and checks old and new links (15 of 43 fail on main); app lint, svelte-check 0/0, build, 508 tests; check-guide clean; screenshots docs/design/two-rows-*. Review: reviewer-fable-sllo; fix_now 1lib (Materials' Kind row rode on a duplicated .find rule) and jv5m (DESIGN.md still listed sllo as pending) fixed by the orchestrator in 5e5a8061; pkd1 (On foot Engineers duplicates the Engineers page) and w6gh (counter exclusions by markup) deferred to astromech-eq5k. Reviewer decisions (reversible, matching the branch): keep On foot's Engineers view for now; two stacked Choice rows on On foot inventory; construction sites as a Dropdown; leave page-body insets; fix the .find rule now.
