---
created_at: "2026-11-08T15:38:38.989060439+00:00"
id: "astromech-vzwk"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "astromech-81t0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Updater: a 45 s timeout no longer kills the apply helper and leaves astromech off"
updated_at: "2026-11-08T15:38:38.989060439+00:00"
---

## Goal

When the update helper times out waiting for the old core to exit, it stops the core without ending itself, so the update still copies and astromech relaunches.

## Done when

- [ ] `stop_locked_exes` in `crates/astromech-core/src/update.rs` ends `astromech-core.exe` without `taskkill /T`, so the helper (the core's child, `astromech-apply-<pid>.exe`) survives. The WebView2 and speech-worker images are already named and still end.
- [ ] The `LOCKED_EXES` doc comment says why the helper survives (no tree kill), not only that its image name differs.

## Context

Review finding astromech-81t0. Same on main and v1.0.0-alpha.4, so it predates the one app.
