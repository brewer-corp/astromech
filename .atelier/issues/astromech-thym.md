---
created_at: "2026-10-17T17:38:28.852514380+00:00"
id: "astromech-thym"
issue_type: "bug"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-thym
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:15:24.124794829+00:00"
status: "done"
title: "One bad value in astromech.toml resets every setting to default"
updated_at: "2026-10-17T19:15:24.124794829+00:00"
---

## Goal

A mistake in one config value (a wrong type, a misspelt key) costs only that value, not the whole file. Today `Config::load` (crates/astromech-core/src/config.rs, called from app.rs ~1062) fails on any parse error and the core runs with every setting at its default, including push-to-talk and the kill switch, so setup asks for them again and the user's binds look lost.

## Done when

A test loads a file with one bad value among good ones and keeps every good value, reports the bad key and line in the log and in Settings → Files, and uses the default for that key only. A file that isn't valid TOML at all still falls back to defaults with the existing message.

## Context

Found by the guide accuracy check (astromech-mf5e). Only callout cooldown_s and unknown [callouts.<id>] are already handled per value.

## Close Reason

Fixed in PR 127; test one_bad_value_costs_only_that_value
