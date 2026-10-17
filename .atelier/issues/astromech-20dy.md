---
created_at: "2026-10-15T20:36:13.650065526+00:00"
id: "astromech-20dy"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-20dy
priority: "P1"
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
closed_at: "2026-10-17T02:51:28.466493251+00:00"
status: "done"
title: "Colonisation candidates near me"
updated_at: "2026-10-17T02:51:28.466493251+00:00"
---

## Goal

Unclaimed systems within 15 ly of a colonised system, near me, with bodies of interest (rings, ELW, landables, star types).

## Done when

A recorded Spansh response returns only unclaimed systems within 15 ly of a colonised one.

## Context

Spansh system and body search. Unverified: that the colonisation fields are exposed.

## Notes

- 2026-10-17 [note] Built: Trade > Colonisation lists unclaimed systems within 15 ly of a populated one (astromech_galaxy::colonies, shell colony_candidates). Spansh exposes the claim flags but its nearest_populated_distance lags new colonies, so only <=15 is trusted and the rest are measured against populated systems within radius + 15 ly; more than 500 populated around means some systems are counted as unchecked, never listed unproven. Data is Spansh's, so a claim made since its last update may still show as open.

## Close Reason

Colonisation candidates listed from Spansh with the 15 ly rule verified by recorded-response tests; reviewed and approved; follow-up astromech-d85n.
