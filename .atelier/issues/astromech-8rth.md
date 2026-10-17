---
created_at: "2026-10-17T16:54:24.249195787+00:00"
id: "astromech-8rth"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-5vjx"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Tag and publish v1.0.0-alpha.1"
updated_at: "2026-10-17T16:54:24.249195787+00:00"
---

## Goal

Bump the version, tag v1.0.0-alpha.1 and publish through the normal release workflow, so the in-app updater offers it.

## Done when

The release is on GitHub Releases as the latest release, and an installed v0.1.5 offers and installs the update (checked on the Windows runner or by Sumeet).
