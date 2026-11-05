---
created_at: "2026-11-05T17:21:38.613712758+00:00"
id: "astromech-0132"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-dfvp"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Count-first material tiles, filled selected rows, EDSM keyring error, controller sheet greys"
updated_at: "2026-11-05T17:21:38.613712758+00:00"
---

## Goal

Finish DESIGN.md's remaining surface and state rules on the pages dfvp touched.

## Done when

- Material tiles lead with the count in Value XL, with the cap in the column head, per DESIGN.md and docs/design/design-system-materials-after.png (finding astromech-tuy0).
- A row a side panel explains (My controller row, Shopping list row, selected binding) is a solid orange fill with On Orange text, designed against the Listening fill on Keybinds (finding astromech-qqws).
- `Edsm.svelte`'s keyring failure is a hard error: red "!", words and a recovery action (finding astromech-uprs).
- `scripts/recolour-sheets.py` stops baking the old grey `#2b2d2e` into the controller sheet headers and the sheets are regenerated, with a before/after look (finding astromech-ztlk).
- `pnpm lint && pnpm check && pnpm build && pnpm test` pass; `scripts/ui` screenshots of Materials, My controller, Ships shopping list and Data sharing.

## Context

Deferred findings from the dfvp review (reviewer-fable-dfvp). The filter-fixed columns question from decision astromech-5p9c stays with the Sell and Mining finders.
