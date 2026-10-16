---
created_at: "2026-10-16T19:44:54.109968208+00:00"
id: "astromech-3tlm"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Keep the journal history current from the core"
updated_at: "2026-10-16T20:01:28.288015583+00:00"
---

## Goal

The core runs the history import (astromech-history, `history.sqlite` in the data folder) on a background thread at startup and again whenever the journal folder changes, so the history follows the game without a manual import. Progress (files and lines done) is visible to the app.

## Done when

A core test with a temp journal folder: startup imports the existing files; appending lines to the newest journal gets them into `events` and the ledgers; the game-state pipeline is never blocked by a running backfill.

## Context

Split out of astromech-aonb, which built the crate, the resumable import and the ledgers but left wiring for later: nothing consumes the history yet (the logbook astromech-fxp0 is first), and the real-logs timing (astromech-tqdh) should say whether a first-run backfill needs throttling. `History::import` is already incremental, so following the files is calling it again on a journal-folder notification (coalesced), not a second tailer.
