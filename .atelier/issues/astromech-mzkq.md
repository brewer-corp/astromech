---
created_at: "2026-10-17T08:19:28.809346663+00:00"
id: "astromech-mzkq"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-mzkq
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
closed_at: "2026-10-17T08:38:39.322865007+00:00"
status: "done"
title: "Voice lookups polish: short forms, visit states, one no-position line, colonisation journal cache"
updated_at: "2026-10-17T08:38:39.322865007+00:00"
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

## Close Reason

37bf fixed: 'nearest federation/federal/empire/imperial' reach the faction lookups. 9mvu fixed: the voice faction lookup records the journal visits first via Galaxy::record_journal_visits (the app uses the same RECENT_JOURNALS constant). ps6t fixed: one NO_POSITION and NO_STAR_MAP in astromech_tools::lookup, used by station, site, faction, Powerplay and colonisation voice. vnsb fixed: JournalVersion/journal_version now live once in astromech-journal (materials re-exports, core history and the colonisation cache use it), a test shows replay on journal growth, a new file and Cargo.json change, and every colonisation question says 'Looking that up.' first. uwwg KEPT, test pins it: 'for the station' in a buy question is the construction depot (nothing else says it), and a bare 'what is left' only asks 'Did you mean...', never runs. Not compiled here: the one-line app/src-tauri/src/main.rs constant use (Tauri shell does not build on Linux).
