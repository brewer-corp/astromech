---
created_at: "2026-10-17T16:54:23.553683589+00:00"
id: "astromech-5qh0"
issue_type: "task"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Agent walk of every app screen"
updated_at: "2026-10-17T16:54:23.553683589+00:00"
---

## Goal

Before Sumeet's walk (astromech-snqt), an agent screenshots every page and state against the example fixture and fixes what is visibly broken: overflow, empty states, loading and error states, narrow windows.

## Done when

A note lists every page with ok or the issue filed or fixed for it, with the screenshot set attached as evidence.

## Context

`scripts/app-shots.sh` and `scripts/ui` (headless Chrome via Puppeteer) run the app without Tauri.
