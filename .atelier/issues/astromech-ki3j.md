---
created_at: "2026-10-22T20:57:43.083817402+00:00"
id: "astromech-ki3j"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-ki3j
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
closed_at: "2026-11-04T20:51:11.302833652+00:00"
status: "done"
title: "At startup, follow the game's current journal, not the one a sync tool touched last"
updated_at: "2026-11-04T21:14:24.022803933+00:00"
---

## Goal

When a backup or sync tool touches an older journal, the core still shows the commander's current ship, system and credits from startup, not the touched journal's, until the game next writes.

## Done when

A pipeline test that starts with an older journal newest by modification time and the game's current journal later by name shows the current journal's state (ship, system, credits) right after the startup catch-up, and the pipeline does not switch back to the touched journal afterwards.

## Context

The other half of astromech-nrhv (see its 2026-10-17 note and the implementer's 2026-10-22 note): the replay-versus-live half is fixed on feature/astromech-g3qt; the pipeline still picks the most recently modified journal to follow, and switching to the latest-named one at startup alone is not enough because it switches straight back.

## Notes

- 2026-11-04 [decision] Order journals by their Fileheader's UTC timestamp. While running, switch to the newest-by-mtime journal only once it has a Fileheader and it did not begin before the followed one (ties switch, so a conflict copy is still followed both ways). At startup, follow the newest by mtime unless the latest by name began after it (only those two headers are read).
  Reason: Fileheader time is UTC, so a DST fall-back (astromech-782v) cannot reorder it, and touching or rewriting an old journal does not change it; name order breaks at fall-back and mtime is what sync tools change. Keeping mtime as the change detector and adding one comparison is the smallest rule that fixes both the startup state and the switch back. Waiting for a Fileheader keeps a tool's just-created empty file from being followed before we can tell when it began. Trade-off: a foreign journal whose header is later than the game's (two game instances writing one synced folder) would keep the session; multi-commander folders are out of scope (astromech-yc7h).
- 2026-11-04 [decision] Widened (astromech-xy2g): a Fileheader stamped more than 120 s in the future is not believed as a start time. Such a journal is never switched to, and a followed one (with no believable start, like one with no header) gives way to any journal with a believable header. Two journals are ordered only when their start times differ by more than 120 s; within that the newest by mtime wins as before.
  Reason: With strict header order, a followed journal with a future header (another PC's clock running ahead, or this PC's clock corrected back) pinned the pipeline: the game's journal was never read again. Distrusting future headers removes the pin for corrections and skew beyond 120 s; the 120 s slack covers smaller corrections. Trade-offs: an older journal that began within 120 s of the current one is followed when touched (a replay, nothing live; state wrong until the game writes); after a clock correction larger than 120 s, once real time passes the old journal's future header, a tool touching that old session's journal could again pin until a journal begins after it; and a foreign journal from a clock ahead by less than 120 s, or two game instances writing one synced folder, can still take the session (multi-commander folders are out of scope, astromech-yc7h). All fail closed: nothing is wrongly live.
- 2026-11-04 [resolution] Review fix (R1, astromech-xy2g) in 74fbb4ba: headers more than 120 s in the future are not trusted, and start times within 120 s are not ordered (newest by mtime wins), so a clock running ahead or corrected back can't pin the pipeline. Remaining limits, all fail closed (nothing becomes live): an older journal begun within 120 s of the current one is followed when touched; after a clock correction larger than 120 s an old future header becomes believable again once real time passes it; a foreign journal from a clock less than 120 s ahead can take the session. R2 deferred to astromech-73w9.

## Close Reason

Implementer report. Changed (crates/astromech-core/src/pipeline.rs): journals are ordered by their Fileheader's UTC time (new begins_before). follow_newest switches to the newest-by-mtime journal only once it has a Fileheader and it did not begin before the followed one (ties switch, so a conflict copy still goes both ways); catch_up follows the newest by mtime unless the latest by name began after it. New test the_catch_up_follows_the_games_journal_not_a_touched_older_one (ship, system, credits after catch-up; touched older journal not followed afterwards; both halves fail with the fix reverted). New test the_games_journal_followed_while_empty_at_startup_is_live_once_written. nrhv test unchanged except its startup assertion (now the current journal). Deviations: about 18 existing pipeline tests and gear_down's a_replayed_older_journal_leaves_the_route_and_clipboard_alone encoded following a touched/restored/foreign journal; they now assert it is not followed, the reopen tests (7uj3, left twice, bh0c) leave and return via a conflict copy with the same header, and journals in tests that lacked a Fileheader got one. Docs: module doc, ARCHITECTURE.md 5.2, newest_journal doc; no guide change (routes.md stays true). Surprises: a journal with no Fileheader is now never switched to (only followed at startup); real journals always start with one. Open items: (1) at startup only the newest-by-mtime and latest-by-name headers are read, so a touched journal that is also latest by name (a foreign one stamped later) still shows its state until the game writes (test the_game_running_before_we_start_is_live_beside_a_later_foreign_journal); (2) a foreign journal whose header is later than the game's session (two instances writing one synced folder) would keep the session; out of scope per astromech-yc7h; (3) independent review outstanding.
