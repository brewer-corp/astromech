---
created_at: "2026-10-17T07:00:35.383545345+00:00"
id: "astromech-arra"
issue_type: "task"
labels: []
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
status: "todo"
title: "astromech-odyssey: take an injected log callback instead of the global sink"
updated_at: "2026-10-17T07:00:35.383545345+00:00"
---

## Goal

astromech-odyssey, a pure library crate, logs through the process-global astromech_log::log sink since astromech-4kw5, so its messages depend on whichever process initialised logging (stdout before init). astromech-data takes an injected log callback; do the same here (finding astromech-4k88 on astromech-4kw5).

## Done when

astromech-odyssey has no dependency on the global log sink; a test captures an unknown-mod message through the injected callback.
