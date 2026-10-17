---
created_at: "2026-10-17T17:38:28.965938356+00:00"
id: "astromech-k20i"
issue_type: "bug"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-k20i
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:22:32.887513164+00:00"
status: "done"
title: "Opening the config writes a sample that silently sets Scroll Lock as the kill switch"
updated_at: "2026-10-17T19:22:32.887513164+00:00"
---

## Goal

Settings → Files → Open on a fresh install writes `astromech.sample.toml` as the config (`open_config` in crates/astromech-core/src/app.rs), and the sample contains `kill_switch = { device = "Keyboard", key = "Key_ScrollLock" }`. The user gets a kill switch they never chose and that the guide says is not set. Write a config without active binds (commented examples only), or one that reflects the current settings.

## Done when

A test calls open_config on an empty data folder and the written file changes no setting (loading it gives the same Config as no file), with the kill-switch example present only as a comment.

## Context

Found by the guide accuracy check (astromech-mf5e).

## Close Reason

Fixed in PR 128; test the_sample_config_written_by_open_config_changes_no_setting
