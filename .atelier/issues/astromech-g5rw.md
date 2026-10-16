---
created_at: "2026-10-16T18:12:14.918064336+00:00"
id: "astromech-g5rw"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-snqt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Merge My controller into Activities"
updated_at: "2026-10-16T18:31:41.552776245+00:00"
---

## Goal

My controller and Activities now show nearly the same thing. Keep one page: the stick as its maker drew it, with an activity filter. "All" shows every bound action, and choosing an activity lights its actions and lists what is still unbound, with Bind this, as Activities does today.

## Done when

`#/controller` opens the merged page. Component tests show that "All" lists every binding on the device and that choosing an activity narrows the list to that activity's actions and its unbound gaps.

## Context

`app/src/modules/controller`, `app/src/modules/activities`.
