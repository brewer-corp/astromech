---
schema: atelier.review
schema_version: 2
id: astromech-9h8f
title: finding on astromech-bp23 @3259fb8b
target: astromech-bp23
identity: claude-reviewer-6siv
commit: 3259fb8b166e5940156c0b00ef28b266818c256a
created_at: 2026-11-07T07:10:41.289124377Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: a4115e14a3f8564ad4b979a2602861795520cc1c
    recorded_at: 2026-11-07T07:11:07.966122408Z
    follow_up: astromech-ndzx
    reason: One config owner is the end state; ndzx removes the shell's readers (noted on ndzx)
---

low: crates/astromech-core/src/app.rs:129-143 (api_state/api_folders) The folder source reads Core::cfg(), the in-memory config. The core re-reads a good astromech.toml only at start (refresh_config re-reads only a file that had problems), so a hand edit of journal_dir/bindings_dir/game_dir reaches these commands only after a restart. The shell re-read the file on every call, and 6siv's Done when says it keeps that behaviour. Until astromech-ndzx, the shell's remaining lookups (window.rs config()/journal_dir()) still read the file, so after a hand edit the two halves of the window can read different journal folders. One owner of the config is the intended end state. Say so in the Close Reason, and check the split is gone in ndzx.
