---
schema: atelier.review
schema_version: 1
id: astromech-ra07
title: approval on astromech-t34c @60dc9bf4
target: astromech-t34c
identity: review-agent-t34c-1
commit: 60dc9bf4bac92cb3eb296e3749f57bdc6cfc4761
created_at: 2026-11-04T23:39:50.563382321Z
event:
  event: approval
---

Reviewed a3009df6..60dc9bf4 (73w9, lpsk, rjhy) against each child's Done when and the replay-never-live property. Ran with a fresh CARGO_TARGET_DIR=/home/supadava/projects/astromech-target-review: cargo test -p astromech-core -p astromech-journal, 201 core unit tests and all integration suites ok, with the new tests listed by name (a_refused_touched_journals_header_is_read_once, after_a_clock_correction_and_a_restart_the_journal_being_written_is_followed, after_a_clock_correction_the_journal_being_written_takes_the_session, an_older_journal_appended_with_old_lines_is_not_taken_for_one_being_written, the_games_journal_beside_a_later_foreign_one_is_measured_by_handle). Clippy -D warnings and fmt are clean. lpsk being-written rule: a touched, restored or appended older journal keeps an old last line, so it can't qualify; foreign ones qualify only when actually being written now (multi-instance, out of scope per yc7h); flapping only after a clock correction with an idle game (medium, deferred). The six tests moved to 10-minute stamps still test the reopen split; they need the game idle because a touched copy is now, correctly, not followed while the game writes. 73w9 invalidation is sound except NTFS tunneling (low). rjhy: the 24 h window compares local-clock name stamps with the local clock read the same way, and it covers future stamps; startup cost is a few extra opens.
