---
created_at: "2026-11-07T02:59:31.569919498+00:00"
id: "astromech-l5c6"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-l5c6
priority: "P1"
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
status: "in_progress"
title: "Publish the browser preview as an artifact"
updated_at: "2026-11-07T03:00:18.082733892+00:00"
---

## Goal

Design work happens in the real app on its work branch, not in hand-drawn mockups. Sumeet sees it through a link: `scripts/preview-publish` builds the app's browser preview (fakecore, no Tauri) into static files that publish as one claude.ai artifact.

## Done when

- [ ] `scripts/preview-publish [--core ready] [--page route] [--out dir]` builds the preview with relative asset paths into a directory ready to publish, its page with no document wrapper of its own.
- [ ] Without `?core=` the build answers as the chosen scenario; the dev server and Tauri builds are unchanged.
- [ ] The published build opens on the chosen page and its navigation works in the artifact viewer (checked once, by publishing it).
- [ ] CONTRIBUTING.md's UI check says how to share a preview.

## Context

The `#/mockup/ds-*` mockups re-implement components in `ds.css` and drift from the app. The browser preview already runs the real pages against `lib/fakecore.ts`. An artifact link carries neither `?core=` nor a `#/path`, so the scenario and start page are set at build time.
