---
created_at: "2026-11-07T06:53:57.601749599+00:00"
id: "astromech-96ci"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ndzx"
    type: "blocked_by"
  - kind: "review"
    id: "astromech-emsi"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "A panic the command layer catches leaves the core working and starts no crash report"
updated_at: "2026-11-07T06:53:57.601749599+00:00"
---

## Goal

A command that panics inside the core answers with an error, and the core then carries on as if nothing happened: its own locks recover, and the next start does not offer a crash report for a panic that never stopped the process.

## Done when

- [ ] The core's handlers and the session threads they share state with lock through `astromech_api::lock` (or the same poison recovery), so a panic under the `session` lock or any other core lock no longer makes later `status`, `voice_test`, exobiology and similar calls fail until restart. A test in crates/astromech-core panics a handler while it holds the session lock, then calls `status` over the control API and gets its normal reply.
- [ ] A panic that the registry catches writes no `crash.marker` (crates/astromech-core/src/crash.rs:16), while a panic outside dispatch still does; a test covers both.
- [ ] astromech-api's doc comment on panics says exactly what recovers.

## Context

Findings astromech-emsi and astromech-xrwd from the astromech-8s6k review. Both behaviours predate the registry (before it, the connection dropped). Do it after astromech-ndzx, so the state that moves into the layer's `State` is already covered there.
