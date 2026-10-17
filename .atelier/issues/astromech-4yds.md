---
created_at: "2026-10-17T05:06:49.258564988+00:00"
id: "astromech-4yds"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Shell opens the core's files through the core's own path rules"
updated_at: "2026-10-17T05:06:49.258564988+00:00"
---

## Goal

The shell re-implements portable detection, the config path and app_dir (`core_data_dir` in app/src-tauri/src/main.rs; its `app_dir()` already differs from `Config::app_dir()`: no XDG_DATA_HOME, different fallback), and still opens galaxy.db, history.sqlite, the outfitting book and astromech.toml from its own `app_dir()`. In portable mode or with a `data_dir` override these aren't the core's files. Move the whole path rule (config path, portable detection, app dir, data dir) into one function both use, and open every shared file through it.

## Done when

A test of the shared rule covers portable mode, the override and the default; the shell has no path logic of its own; ARCHITECTURE's "the core, which opens the same galaxy.db" holds in every layout.

## Context

Review findings astromech-0ohq and astromech-9gjv on astromech-n1ij. Related: astromech-zs2q.
