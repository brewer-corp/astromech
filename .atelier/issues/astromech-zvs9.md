---
created_at: "2026-10-17T22:50:33.370721486+00:00"
id: "astromech-zvs9"
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
status: "in_progress"
title: "Move the Clock trait into astromech-proto, as ARCHITECTURE 5.1 says"
updated_at: "2026-10-22T21:27:31.431075542+00:00"
---

## Goal

ARCHITECTURE section 5.1 puts the Clock trait in astromech-proto (no I/O), but the code has it in astromech-data. Since astromech-4irh, astromech-engineering (a journal-replay crate) depends on astromech-data, pulling in rusqlite and ureq for a one-method trait.

## Done when

Clock lives in astromech-proto, astromech-data re-exports or uses it, astromech-engineering no longer depends on astromech-data, and check-deps passes.
