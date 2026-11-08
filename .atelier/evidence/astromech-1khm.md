---
created_at: "2026-11-08T16:29:27.535839511+00:00"
id: "astromech-1khm"
evidence_type: "validation"
commit: "9402d4b94bb62f3016357090b1e881513ac5d2af"
result: "pass"
captured_at: "2026-11-08T16:29:27.535705961+00:00"
uri: "https://github.com/brewer-corp/astromech/actions/runs/42496995856"
agent_identity: "claude-orchestrator-kjfs"
target:
  kind: "issue"
  id: "astromech-kjfs"
  role: "validates"
criteria:
- slug: "one-command-list"
  result: "pass"
criteria_mission: "astromech-kjfs"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kjfs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "ci.yml on main at 64ee930a: app/src-tauri/tests/every_window_command.rs (every_window_command_runs_against_the_world) lists every command the window calls and calls each through the control API of a headless world core; it passed on Linux and Windows. The shell is gone: the one app's window calls the same command layer in process (astromech-y6d6)."
updated_at: "2026-11-08T16:29:27.808672584+00:00"
---

ci.yml on main at 64ee930a: app/src-tauri/tests/every_window_command.rs (every_window_command_runs_against_the_world) lists every command the window calls and calls each through the control API of a headless world core; it passed on Linux and Windows. The shell is gone: the one app's window calls the same command layer in process (astromech-y6d6).
