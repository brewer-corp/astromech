---
created_at: "2026-10-17T16:54:23.553683589+00:00"
id: "astromech-5qh0"
issue_type: "task"
labels:
- "alpha"
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-5qh0
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Agent walk of every app screen"
updated_at: "2026-10-17T20:55:47.918112423+00:00"
---

## Goal

Before Sumeet's walk (astromech-snqt), an agent screenshots every page and state against the example fixture and fixes what is visibly broken: overflow, empty states, loading and error states, narrow windows.

## Done when

A note lists every page with ok or the issue filed or fixed for it, with the screenshot set attached as evidence.

## Context

`scripts/app-shots.sh` and `scripts/ui` (headless Chrome via Puppeteer) run the app without Tauri.
