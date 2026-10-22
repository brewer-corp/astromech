---
created_at: "2026-10-17T19:37:35.948104079+00:00"
id: "astromech-st8o"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "EDSM uploader has no timestamp-age guard on live events"
updated_at: "2026-10-22T20:59:22.909638300+00:00"
---

## Goal

EDDN drops a live journal line whose timestamp is more than 5 minutes old (feed.rs MAX_EVENT_AGE_SECS) in case a journal is re-read as live (backup tool, another client). The EDSM uploader (edsm/mod.rs Uploader::journal) trusts the live flag alone, so the same re-read would upload old lines. Decide whether EDSM should apply the same guard.

## Done when

Either the EDSM uploader applies the same age guard with a test, or the guide and a code comment record why it does not. Found while fixing astromech-koa6, whose guide wording now says EDSM has no timestamp check.

## Notes

- 2026-10-22 [note] 22 Oct 2026: decided against the guard, recorded why. EDDN's consumers read each message as what the game shows now; EDSM keeps the commander's own history, takes past events by design (it imports old journal files), and answers 101-103 for an event it already has, which the courier counts as sent; so a journal re-read as live costs requests, not wrong data. The tailer already reads an older journal touched later as a replay (core test an_older_journal_touched_later_is_read_as_a_replay). Comment on edsm::Uploader::journal and the module doc; guide (What both have in common) and ARCHITECTURE say why. Existing tests pin it: pipeline::tests::edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay and edsm::tests::the_startup_replay_is_not_sent (a months-old live line is queued). Sumeet can overrule: applying feed::MAX_EVENT_AGE_SECS in Uploader::journal is a few lines.
