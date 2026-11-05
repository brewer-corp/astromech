---
created_at: "2026-11-05T00:37:53.641659317+00:00"
id: "astromech-5p9c"
issue_type: "decision"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T00:38:04.271180564+00:00"
status: "done"
title: "DESIGN.md direction (astromech-c4i0): approved by an independent panel, with changes"
updated_at: "2026-11-05T00:38:04.271180564+00:00"
---

## Question

Is the DESIGN.md direction on task/astromech-c4i0 (draft PR #185, revision 2) approved, so the branch can be finished and merged and the UI work blocked on it (astromech-51ue and the yxtz, dfvp, pey5, sllo, dzwy follow-ups) can start?

## Options

- Approve as is.
- Approve with changes: fix the contradictions the panel found, then merge after review.
- Rework the direction.

## Decision

Approve with changes. On 4 Nov 2026 Sumeet directed that this not wait on his verdict about quality or design, and that an independent panel of at least two fresh reviewers, one on a different model, settle it against the repo's design goals (astromech-bi94, the c4i0 goal and Done when, the sparse-UI ethos).

Panel, 4 Nov 2026:
- Fable 5.1 reviewer: Contract pass, Fitness pass with changes. Verdict APPROVE WITH CHANGES.
- Opus 5.5 reviewer: Contract pass, Fitness pass with changes. Verdict APPROVE WITH CHANGES.

Both found the after-mockups clearly more scannable than the befores, and the orange ramp a real, reviewable intensity system. Before merge, these must be done (union of both panels' FIX NOW findings):
1. One placeholder spec (Orange at reduced opacity, as the mockups draw it) across DESIGN.md, design.json, ds.css and tokens.css. The Ctrl+K hint is no longer Orange Mid on a cell.
2. A role-to-token table mapping DESIGN.md's names (void, hull, ember-cell, orange, row-hover…, spacing, widths) to tokens.css. Retire or fix the old grey `--row-hover`, and make `--form-w` and the text agree.
3. The type table's colours are defaults that components may override.
4. A Places home tile has one main value: the answer is Lit, and the finder name is Name in Orange.
5. Focus on a filled element is an On Orange outline, inset.
6. A list has a selected row only when something shows the selection (a side panel or keyboard focus). There is no default selected row.
7. Reword "'On' is always the filled one" to "the chosen option is always filled".
8. Settings statuses in cells: the rule and the mockup must agree.
9. One sentence reconciling bi94's "orange only where it means something" with orange as the base: that rule is now carried by brightness and fill.
10. Orange Dim is state, not content, and knowingly below AA. It must never carry text the commander needs to act on.
11. Red is for hard errors only. A stale row keeps its name at Orange, dims its facts to Orange Mid, and shows its age in Orange. PRODUCT.md's constraint becomes "stock orange, plus red for hard errors".
12. AGENTS.md and CONTRIBUTING.md (the UI check) point at DESIGN.md.
13. Rewrite the PR description to match revision 2.

Item 11 is where the panel split. Fable recommended red for hard errors only, so each hue has one meaning and the system's own brightness rule carries staleness. Opus raised no objection to red ages but wanted stale facts readable. The orchestrator took the narrower option: it changes PRODUCT.md's binding constraint least, satisfies both reviewers, and is reversible in one token.

The four questions left for Sumeet, as the panel answered them unanimously: keep the filled top tab; the page-tab contrast is enough (an inactive page tab hovers to Orange, not Lit); retiring monospace is fine (logs as columns in tabular figures); Controls and Settings keep grey-white labels, with the boundary written down as options-style pages.

Deferred, recorded in the follow-ups: the minimum window width and the screenshot script covering the ds mockups (Fable R8); cell-hover contrast (Fable R9, Opus R8); a definition of "current section" (Opus R7); fact list, multi-value stat and meter patterns (Opus R9); filter-fixed columns (Opus R10); placeholder and scrollbar greys in tokens.css (Opus R12).

## Close Reason

Independent panel (Fable 5.1, Opus 5.5) approved the DESIGN.md direction with 13 changes; per Sumeet's 4 Nov direction
