---
created_at: "2026-10-22T20:55:34.472353984+00:00"
id: "astromech-84vh"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-84vh
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T20:56:03.142390693+00:00"
status: "done"
title: "Bump rustls-webpki and webpki-roots for RUSTSEC-2026-0381 and 0382"
updated_at: "2026-10-22T20:56:03.142390693+00:00"
---

## Goal

`cargo deny check` passes again: RUSTSEC-2026-0382 (rustls-webpki 0.103.15) and RUSTSEC-2026-0381 (webpki-roots 1.0.9), both via ureq, fail CI on every PR since 22 Oct 2026.

## Done when

Cargo.lock (and app/src-tauri/Cargo.lock if it has them) resolve rustls-webpki >= 0.103.16 and webpki-roots >= 1.0.10, and `cargo deny check` and the workspace tests pass.

## Context

Seen on PR #170's Linux job (run 39852452826).

## Close Reason

Both lockfiles resolve the fixed versions; cargo deny advisories passes.
