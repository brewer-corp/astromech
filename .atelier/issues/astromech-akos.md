---
created_at: "2026-11-07T06:58:53.027925325+00:00"
id: "astromech-akos"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "astromech-95ql"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Move the Linux CI job back to ubuntu-latest once Playwright supports Ubuntu 26.04"
updated_at: "2026-11-07T06:58:53.027925325+00:00"
---

## Goal

The Linux CI job runs on `ubuntu-latest` again, so it does not sit on an image GitHub will retire.

## Done when

- [ ] `.github/workflows/ci.yml` Linux job uses `ubuntu-latest`, with a Playwright version that installs `chromium-headless-shell` with `--with-deps` on Ubuntu 26.04, and the job passes.

## Context

Pinned to `ubuntu-24.04` on PR #215 (astromech-7rcn) because Playwright 1.60 refuses to install Chromium on 26.04. Also drop CONTRIBUTING.md's local `PLAYWRIGHT_HOST_PLATFORM_OVERRIDE` note once it is not needed.
