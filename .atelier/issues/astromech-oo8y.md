---
created_at: "2026-10-16T20:16:16.798744343+00:00"
id: "astromech-oo8y"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-oo8y
priority: "P2"
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
title: "Session summary by voice and at logout"
updated_at: "2026-10-17T05:01:04.027175363+00:00"
---

## Goal

Ask for the session summary by voice ("how much did I make this session?") and hear a short one when the session ends (logout or quitting to the menu): credits earned and per hour, jumps and distance, bounties awarded. It reads `History::session` from `history.sqlite`.

## Done when

A replay with the fixture history: the voice query answers with the session's totals, and a `Shutdown` (or a return to the main menu) speaks the summary once.

## Context

Split out of astromech-bg0x, which built `History::session` (the totals, tested on the fixture sessions) and the app's Session tab. Speaking needs a read connection to the history in the core's session and a tool in astromech-tools. Only say what the game doesn't already show (AGENTS.md).
