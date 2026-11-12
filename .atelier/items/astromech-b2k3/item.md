---
schema: atelier.item
schema_version: 3
id: astromech-b2k3
title: 'DESIGN.md revision 4: the Glow-up amendments'
status: changes_requested
mission: astromech-xajc
labels:
- size-s
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3Aba9586df-07e7-40e8-a9c9-e2fc47303d34
ready:
  by: conversion
  at: 2026-11-10T05:54:56.753081776Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:54:56.753081776Z
---

## Goal

DESIGN.md says the rules the Glow-up mission builds to, before any page is built to them, so every later item has one rulebook. Docs first: a bad rule is cheapest to fix on paper.

## Done when

- [x] DESIGN.md has the amendments listed under 'DESIGN.md amendments (revision 4)' in .atelier/missions/astromech-xajc/ that no other issue owns: the four page layouts (Board, List with detail, Workbench, Form) with when to use each and the rule 'an action sits next to what it acts on' in place of the content-width caps; white or Options Grey for entity names outside Controls, with the test for when; brief state feedback (what is allowed, how long, and that decorative motion stays banned); the star, body and station icon sets; the collapsed filter summary; the NOW tab and default page in the Frame.
- [x] Its Do's and Don'ts, Deviations table and Token map agree with the new text, and nothing in the document still states a superseded rule (the 1200 px list cap, 'no motion' without the exception).
- [x] The scale rule, the new orange values, Tip and the status bar are left to astromech-k6pl, astromech-sixj and astromech-q79q, and DESIGN.md says so nowhere twice.
- [x] scripts/check-guide.py and the repo's docs checks pass.

## Scope

DESIGN.md only. No tokens, components or pages.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. The amendments were approved by Sumeet on 10 Nov 2026 (decision astromech-ci0o). DESIGN.md is the binding design system (AGENTS.md).

## Report

DESIGN.md revision 4 is in (commit e0978d9f), DESIGN.md only.

- **Layouts.** Layout → "Content widths and rows" is replaced by "Page layouts": a table of
  Board, List with detail, Workbench and Form with what each holds, when to use it and example
  pages, the panel-below-content fallback at 1100 x 700, and the rows and grid sizes that
  survive. The new Named Rule "The Next To It Rule" carries "an action sits next to what it
  acts on" with the one-column test from the mission's success criterion.
- **Lifted names.** "The Lifted Name Rule" in Colors → Named Rules, with the test: lift when
  the element is about the named thing; leave Orange when a value ranks the element or when
  every name would be lifted; White for a few, Options Grey for a long column. White, Options
  Grey, Key Characteristics and the Hierarchy table's Name row point at it.
- **Brief feedback.** "The Brief Feedback Rule" under Elevation & Depth: copy to the `copied`
  check for 1.5 s, instant both ways, nothing fades or spins, no page transitions. A "Done"
  row joins the States table.
- **Icons.** Shapes → Icons lists the star, body and station sets as the mockups'
  `Icon.svelte` draws them, how a type icon is placed (a fact, the name's colour, 16 or 24 px,
  the type as its accessible name), and that the three starport icons replace the old single
  starport glyph.
- **Collapsed filters.** A paragraph in Lookup result row, matching astromech-drnv's Done when
  lines (summary plus CHANGE, live edits, closes only on DONE, Escape or CHANGE, non-default
  filters always named). The example keeps "large pad" because `app/src/lib/search.test.ts`
  asserts that exact string as DESIGN.md's example.
- **NOW.** A paragraph under Frame: NOW is the first top tab and the window opens on it.
- **Consistency.** Do's and Don'ts, Deviations (motion row amended, a lifted-names row added)
  and the Token map follow. The Token map drops the Result list width row; a note says
  `--list-w` stays in tokens.css only until its last page moves onto a layout (it is still
  used by SearchKit, FinderPage and Colonisation). No "no motion" without the exception is left.
- **Left to others.** The height scale, the orange values, Tip and the status bar are named
  once, in the revision comment at the top, as belonging to astromech-k6pl, astromech-sixj
  and astromech-q79q. I treated amendment 2's "body weight 500; labels used less" as part of
  k6pl's new values, and left the header's status text (amendment 3) to q79q. Amendment 5's
  "explanations live in Tips" is left to sixj.
- **Checks.** `python3 scripts/check-guide.py`: 0 problems. No other check reads DESIGN.md.

For the reviewer:

- The mockups and `docs/design/glow-up-*.png` are only on the `mockups/glow-up` branch, not on
  `main`. The revision comment says so. The item's Context assumes they are on main.
- `SystemChip.svelte` reverts its copy check after 1200 ms, not 1.5 s. Code is out of this
  item's scope; whichever item next touches the copy button should match the rule.

