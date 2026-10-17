---
created_at: "2026-10-17T20:12:38.600227989+00:00"
id: "astromech-36g1"
issue_type: "bug"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-36g1
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "The game's journal is never live when a journal name's stamp disagrees with real time"
updated_at: "2026-10-17T21:09:21.056381861+00:00"
---

## Goal

The game's own journal is live even when the folder holds a journal whose name stamp sorts later than it, or when the game's new session has an earlier stamp than the previous one. Examples: the clock went back for daylight saving; the player travelled west; another PC in a later timezone, or one with a skewed clock, syncs the same folder. This assumes the game names journals in local time, which still needs checking.

## Done when

- Pipeline tests cover both shapes and deliver the game's lines as live (EDDN, callouts):
  - a new session journal named earlier than the one followed, created after we started;
  - a foreign journal stamped later than the game's current one.
- A sync tool touching or restoring an older journal still stays a replay, and the astromech-0d7l tests still pass.
- When the followed journal isn't the folder's latest, the pipeline logs it once.

## Context

Found in review of astromech-0d7l (findings R2 and F1). Since 0d7l, Tailer::open treats only the folder's latest journal by name stamp (astromech_journal::latest_journal) as live. In these cases EDDN, EDSM and callouts go quiet with no log line until real time passes the stamp. One option is "latest by stamp, or created after we started".

## Notes

- 2026-10-17 [note] Check of the claim that the game names journals in local time: confirmed on real journals. All 22 files in ~/edjournal (Odyssey names, Journal.YYYY-MM-DDTHHMMSS.NN.log, October 2026) have a stamp exactly 4.0 h behind their Fileheader's UTC timestamp, on an America/New_York machine in EDT (-0400); for example a 13:35:02 stamp has a 17:34:54Z header. So the stamp is local start time and the US fall-back (1 Nov 2026) will name the next session up to an hour earlier than the previous one. Documentation does not say (the Frontier Journal Manual v30 and elite-journal.readthedocs only say the header timestamp is GMT; kayahr/ed-journal and ed-journals say nothing). One machine and one offset, so DST handling and other timezones are inferred, not observed. The names and offsets are recorded here; no journal content is committed. Rule chosen: a journal is the game's if it is the folder's latest by stamp, or its stamp is within 120 s of local wall-clock time (checked when we open it), or it shares the stamp of a session already judged the game's (later parts). A restored, copied or touched older journal has an old stamp so stays a replay; conflict copies have no stamp. Not covered: an app started after DST rolled back while the game's session is already running (that journal is not latest and its stamp is old; it stays quiet until a new session).
- 2026-10-17 [note] Limits found in review (follow-up astromech-t16r): the rule decides at open time from the name and the clock, so it does not cover the app starting while the game's journal (not the latest by name) is already running with a foreign later journal in the folder, nor a game journal first seen more than 120 s after its start and not the latest. A restored or copied journal stamped within 120 s of local now is taken for the game's (inherent to the window). A later part (.02) of a judged session stays live.
