---
created_at: "2026-10-16T18:20:01.765922192+00:00"
id: "astromech-6230"
issue_type: "task"
labels:
- "size-m"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Sign Windows releases"
updated_at: "2026-10-16T18:20:01.765922192+00:00"
---

## Goal

Sign the installer, the core and the voice worker in `release.yml`, most likely through Azure Artifact Signing (see the decision astromech-rqyv). Deferred until there is evidence people use astromech.

## Done when

A release built in CI passes `signtool verify /pa` on each signed binary, and the README and V1-DESIGN §7.4 no longer describe the SmartScreen click-through.
