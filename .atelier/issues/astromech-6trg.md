---
created_at: "2026-11-05T06:47:09.497179822+00:00"
id: "astromech-6trg"
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
    id: "astromech-pey5"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Suggestions popover: listbox roles and Tab behaviour"
updated_at: "2026-11-05T06:47:09.497179822+00:00"
---

## Goal

Autocomplete suggestion lists read correctly to screen readers and keep keyboard focus predictable.

## Done when

- In `app/src/lib/ui/Suggestions.svelte` the list items carry no implicit listitem role inside the listbox (e.g. `role="presentation"` on the `li`, or options rendered directly), and options are not separate Tab stops (`tabindex="-1"`, with arrow keys moving the active option via `aria-activedescendant` on the input).
- Tab from the input leaves the list closed and moves to the next control; a test covers it.
- `pnpm lint && pnpm check && pnpm test` pass.

## Context

Review finding astromech-80jd (reviewer-fable-pey5) on astromech-pey5: pre-existing in all four lists Suggestions replaced; deferred out of pey5.
