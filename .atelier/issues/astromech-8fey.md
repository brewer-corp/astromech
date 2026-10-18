---
created_at: "2026-10-17T21:44:36.987014145+00:00"
id: "astromech-8fey"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-8fey
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-18T01:40:39.842181117+00:00"
status: "done"
title: "Voice: single-word 'paste' scores 94% on the synthetic eval (galaxy_map.paste, T1)"
updated_at: "2026-10-18T01:40:39.842181117+00:00"
---

## Goal

Make "paste" (galaxy_map.paste, which presses keys as T1 `paste_search`) recognised reliably. On the synthetic eval (astromech-yil0, docs/evidence/astromech-yil0/report.md) it scores 45/48 (93.8%); its family, route, is 98.8%.

## Done when

galaxy_map.paste scores at least 95% on `scripts/voice-eval.sh` and no wrong executes appear.

## Context

The single word "paste" is heard as "Pays." (am_michael, noise), "Based on" (bm_george, cockpit+noise; asks srv_handbrake.on) and "Mays the" (bm_george, noise). "paste waypoint" and "paste system name" score 100%. Synthetic voices only: check against real clips (astromech-cgrw) before tuning.

## Notes

- 2026-10-17 [note] Handoff: 'pays' is MISHEARD for 'paste' (capped 0.80, asks). voice-eval paste 45/48 -> 46/48 (95.8%, one-clip margin), wrong execute 0, rejects 64/64. Review round 1: no FIX NOW. Reviewer found pre-existing 'Pace.' -> panel.close execute; filed astromech-8997. Approval outstanding (not recorded by the author by instruction).

## Close Reason

'Pays.' asks 'Did you mean paste?'; galaxy_map.paste 45/48 -> 46/48, 0 wrong executes. Landed through #162 with the astromech-1c66 learning guard. Reviewed.
