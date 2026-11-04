---
created_at: "2026-10-22T23:48:36.936226438+00:00"
id: "astromech-j75v"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-j75v
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-e8ux"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Voice lookups: a named system ('near Sol') turns into a different commodity or module"
updated_at: "2026-11-04T21:06:32.601096501+00:00"
---

## Goal

Voice lookups search only from the current system. Saying a system as well is not understood and is misread as a slot value: on 22 Oct through the core Assistant, "where can I sell gold near Sol" asked "where can i sell Polymers" and "where is the nearest 5A frame shift drive near Sol" asked "where is the nearest Frame Shift Drive (SCO)". Either answer from the named system (the grammar already has a `{system}` slot for plot_route) or recognise the trailing "near <system>" and say lookups by voice are from where you are, instead of offering a wrong commodity or module.

## Done when

- Tests in astromech-intent/astromech-core: "where can I sell gold near Sol" and "nearest 5A frame shift drive near Sol" no longer resolve or ask about another commodity or module; they either search from Sol or say voice searches from the current system.

## Context

- Strategy preamble: "Every lookup works from the current system or a typed one, in the app and as a voice query" (docs/strategies/replace-the-toolbox.md). The app's search kit takes a typed origin; voice does not.
- Grammar: crates/astromech-intent/src/lib.rs; answers: crates/astromech-core/src/lookups.rs.
- Found by validation astromech-e8ux.

## Unknowns

- Whether the preamble means voice must search from a spoken system too (Sumeet). The validation read "a typed one" as the app's typed origin.

## Notes

- 2026-11-04 [decision] Voice lookups search from a named system: a trailing 'near <system>' or 'from <system>' is split off in astromech-intent (Grammar::best -> Match::origin) for the commands that take an origin (find.*, trade.*, sites.*, faction.*, powerplay.*); colonisation is excluded. 'near me', 'near here', 'from here', 'this system' etc. (HERE_WORDS) and naming the current system mean the current system. The split is used only when the head matches an origin-taking command at least as well as the whole transcript, so phrases with 'near me' of their own keep their match.
  Reason: Strategy preamble: every lookup works from the current system or a typed one, in the app and as a voice query. Galaxy::locate (star map, then Spansh exact name) already gives a position for any named system, so no lookup kind needed the 'voice searches from where you are' fallback. Splitting before slot matching stops the system words being absorbed into the commodity/module slot (the Polymers / FSD (SCO) misreads).
- 2026-11-04 [decision] The named system is free text (as said, or the grammar's known-system spelling) located at answer time: Galaxy::locate for faction/Powerplay/site lookups, the new astromech_galaxy::locate_remote (Spansh exact name, same code Galaxy::locate falls back to) for station and trade lookups, which do not need the star map. An unknown name answers 'Spansh does not know a system called <name>.' and nothing is searched; a failed request answers 'Could not look that up. <reason>'. Answers searched from a named system start with 'From <system>.' as a sentence of its own, and drop 'in this system' for a hit in that system.
  Reason: The grammar's {system} list is tiny (Sol) and Spansh knows every system, so restricting to known names would refuse most real systems. Copy guide 6.5: a line that may carry a long name says it as its own sentence, which keeps the existing answer sentences within budget. Saying the unknown name plainly meets the brief's rule that it never silently searches from the current system.
- 2026-11-04 [decision] Voice tester: a lookup with a named system appends 'Searches from <system>.' to what it would do, and the trade description no longer says 'near you' (it could be elsewhere).
  Reason: One place states the origin for every lookup kind instead of a variant in each describe function.
