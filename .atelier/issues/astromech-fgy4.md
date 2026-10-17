---
created_at: "2026-10-17T08:14:09.018496342+00:00"
id: "astromech-fgy4"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-fgy4
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
status: "in_progress"
title: "Exobiology: no known-biology line on a relog into a scanned system; nearest star for differently named stars"
updated_at: "2026-10-17T08:26:24.256053447+00:00"
---

## Goal

Follow-ups from the astromech-h5gs and astromech-ys47 reviews: a login can say 'Known biology on N bodies.' when the system has already been scanned (astromech started after the jump, the Biology page already fetched the answer, then a relog there); say nothing on a login when the system's own results exist (finding astromech-evgz). The nearest-star rule in scripts/gen-exobiology.py drops a known star in systems whose stars have other catalogue names (e.g. 'LFT 601' with 'Gl 319 B'), about 0.5% of bodies; fall back to the body's immediate parent star (finding astromech-0nz3).

## Done when

A replay test of the relog case says nothing; the regenerated catalogue assigns a star to those bodies, with held-out recall at or above 0.98.
