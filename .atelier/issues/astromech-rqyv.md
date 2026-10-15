---
created_at: "2026-10-15T20:36:13.657456145+00:00"
id: "astromech-rqyv"
issue_type: "task"
labels:
- "needs-cmdr"
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-rh44"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Decide: sign v1.0 or ship it unsigned"
updated_at: "2026-10-15T20:36:13.657456145+00:00"
---

## Goal

Releases are unsigned today (V1-DESIGN §7.4 and §9 #7, the README, `release.yml`), while ARCHITECTURE §16 plans free OSS signing through the SignPath Foundation. Decide for v1.0.

## Done when

Sumeet's decision is recorded as a note here, and V1-DESIGN or ARCHITECTURE §16 is updated to match.

## Context

SignPath needs OSI licences for every component (GPL-3.0 binaries qualify) and may hesitate over software that injects input (ARCHITECTURE §16).
