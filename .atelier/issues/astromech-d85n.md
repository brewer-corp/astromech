---
created_at: "2026-10-17T02:49:37.081149012+00:00"
id: "astromech-d85n"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Colonisation candidates: report truncation and test the caps"
updated_at: "2026-10-17T02:49:37.081149012+00:00"
---

## Goal

Make the colonisation candidates lookup say when it has truncated results, and test its caps.

## Done when

- When more than 100 unpopulated systems match, the page says farther systems were left out (`CANDIDATES` in crates/astromech-galaxy/src/colonies.rs; Spansh's page `count` is currently ignored).
- Tests exercise the 40-system body detail cap and the 3-page bodies limit, and hitting the limit is reported instead of silently dropping bodies.
- The unchecked count is taken after the age filter.
- From the review of astromech-20dy (R1–R3, low).
