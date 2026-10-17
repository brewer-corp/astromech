---
created_at: "2026-10-17T08:19:28.809346663+00:00"
id: "astromech-mzkq"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Voice lookups polish: short forms, visit states, one no-position line, colonisation journal cache"
updated_at: "2026-10-17T08:19:28.809346663+00:00"
---

## Goal

Deferred findings from the reviews of astromech-6sf7, astromech-8joi and astromech-3ez9:
- astromech-37bf: 'nearest federation' / 'nearest empire' alone don't reach the faction lookups (only the '... system' and '... faction' forms do).
- astromech-9mvu: the app records the commander's own visits before a faction search (main.rs:677-680); the voice path doesn't, so voice can report Spansh's older state where the app says 'Seen on your visit'.
- astromech-ps6t: the 'position not known' line (and the 'star map could not be opened' line) is defined once per lookup module; keep one of each.
- astromech-vnsb: the colonisation voice journal cache re-implements astromech_materials::journal_version, with no test that it re-reads on journal or Cargo.json change; during play nearly every progress question replays the whole history, and those questions don't say 'Looking that up.' first.
- astromech-uwwg: 'where can I buy steel for the station' now runs the colonisation lookup; 'what is left' asks 'Did you mean what is left to deliver?'. Decide whether that's wanted.

## Done when

Each item fixed or explicitly kept, with a test per fix.
