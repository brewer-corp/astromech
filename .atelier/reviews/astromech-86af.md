---
schema: atelier.review
schema_version: 2
id: astromech-86af
title: finding on astromech-vucf @168fc814
target: astromech-vucf
identity: claude-reviewer-zgoo
commit: 168fc8140dcef80ec71579e18900b09ec4ba7d2c
created_at: 2026-11-07T21:46:39.944501987Z
event:
  event: finding
  recommendation: defer
---

crates/astromech-api/src/state.rs:227 and :252 (JournalState::build and ::get): the fleet's transfer arrivals are judged on a hard-coded astromech_data::SystemClock (from_journal_dir_at and Settled::current), not on State::clock(), so in a world a ship transfer lands on real time while the rest of the command layer runs on the manual clock. The lint can't see it (it is a clock call, not a std read). Pre-existing (astromech-6siv), not on zgoo's Done when list, and the installed app is unaffected (both are the system clock). Fix later: pass State::clock() into JournalState::build/get, with a world test that an advance past a transfer's arrival changes the fleet. File a follow-up (--discovered-from this finding), alongside astromech-2ebj.
