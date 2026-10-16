---
created_at: "2026-10-15T20:36:13.657328768+00:00"
id: "astromech-rh44"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "v1.0 release"
updated_at: "2026-10-15T20:36:13.657328768+00:00"
---

## Goal

Polish from testing, README, code signing (SignPath) and a GitHub Release.

## Done when

The v1.0 installer on GitHub Releases installs on a clean Windows account and, if the signing decision says to sign, passes `signtool verify /pa`.
