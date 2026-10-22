---
created_at: "2026-10-17T19:45:31.519135676+00:00"
id: "astromech-otsm"
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
status: "todo"
title: "Tighten the data-sharing limits test to whole sentences"
updated_at: "2026-10-22T20:41:37.879230731+00:00"
---

## Goal

crates/astromech-eddn/tests/documented_limits.rs matches a bare "30 minutes" and the EDDN-only sentence but not the EDSM sentences, and the guide hard-codes 8 tries and 2,000.

## Done when

The test asserts the full sentences for both limits in the page and guide and builds 8 and 2,000 from MAX_ATTEMPTS and MAX_QUEUED.
