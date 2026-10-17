---
created_at: "2026-10-15T20:36:13.658661252+00:00"
id: "astromech-lg9r"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-505u"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T01:49:37.311907485+00:00"
status: "done"
title: "Docking: pad position and denial reason"
updated_at: "2026-10-17T01:49:37.311907485+00:00"
---

## Goal

On `DockingGranted`, say where the pad is in the station (the number is on screen; the position isn't), from a pad-layout table like EDMC LandingPad. On `DockingDenied`, say the reason.

## Done when

Replay tests give the right position for a Coriolis pad number and the right reason line for each denial reason.

## Close Reason

DockingGranted at a starport says the pad's clock position and depth; DockingDenied says the reason; replay tests per pad group and per reason (evidence astromech-arti). The layout follows the LandingPad plugin and is checked in game by astromech-505u.
