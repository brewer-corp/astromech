---
created_at: "2026-10-17T00:53:10.964530289+00:00"
id: "astromech-c1sq"
issue_type: "task"
labels:
- "needs-human"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Check the uncertain Powerplay activity rules in game"
updated_at: "2026-10-17T00:53:10.964530289+00:00"
---

## Goal

Confirm or correct, in game, the Powerplay activity rules marked `uncertain` in `data/catalog/powerplay-activities.json` (and the journal fields the suggestions rely on), so "What earns merits here" on the Powerplay page only shows verified rules.

To check (each is a rule or field in the file):
- `conflict_progress` 0.3: in an acquisition conflict, does the journal's `PowerplayConflictProgress` cross 0.3 for both powers (36,000 of 120,000 control points)? Does `PowerplayState` read `Unoccupied` or something else (e.g. `Contested`) in a conflict?
- acquire / sell-mined: where must the goods be mined (a supporting system, or anywhere)?
- acquire / reboot-missions: do restore and reactivate missions earn merits in acquisition systems?
- conflict / flood-markets: must the cheap goods be bought in a supporting system?
- reinforce / exobiology: hand in at the Power Contact or by selling at Vista Genomics?
- undermine / salvage: where is it handed in for merits?
- undermine / stronghold-carrier: which strongholds have a carrier, and can we tell from the journal (e.g. an FSS signal)?
- Escape pods: still suspended?
- Capitals (home systems): what `PowerplayState` does the journal write, given they can't be reinforced or undermined?

## Done when

Each item above is checked in game and recorded: the rule's `uncertain` note is removed or the rule corrected (bump `version` and `fetched`), with the fixture test in `crates/astromech-powerplay/tests/activities.rs` updated to match.

## Context

Filed from astromech-w62j. Sources were the Elite Dangerous wiki and Ian Doncaster's reference card; Inara's guide was unreachable (HTTP 503) on 2026-10-16 and is worth a read when checking.
