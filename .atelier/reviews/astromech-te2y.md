---
schema: atelier.review
schema_version: 1
id: astromech-te2y
title: 'Reviewed git diff main...HEAD (63470471) against vett, ikwr, yrfv, xvov, gwh0, hloh (Done when unchanged on the branch; notes only). Reran: cargo test --workspace (all pass), fmt --check, clippy -D warnings on the 7 touched crates, real_journals harness on a copy of ~/edjournal (0 problems, 0 known). Extra checks: built a v7 history.sqlite from main''s code over the real journals, opened it with the branch (v8 migration + rebuild): ships rows identical to a fresh v8 import, reopen idempotent, the two id reuses (5 python->python_nx, 12 type9->panthermkii) now two rows each. Compiled the shell''s PowerplayCache/powerplay_journal verbatim in a scratch crate against astromech-powerplay (compiles; callers use Journal by value). Fed the real journals to powerplay::Follower in 7919-byte chunks with a refresh after each (1965 refreshes): result equals from_journal_dir. Checked real journals: Powerplay precedes Missions at every pledged first login; relogs write LoadGame+Missions only; no taxi Loadout with a ShipID; all 38 Modifier labels map. Reasoned each new test fails on main (busy-timeout test and rate test excepted). Not run: the shell build, Windows, the logbook search timing on the 400 MB synthetic history.'
target: astromech-fdh3
identity: review-agent-fdh3-1
commit: 63470471264d143f5c0e854f4641ee757634d5a8
created_at: 2026-10-22T21:45:43.923339140Z
event:
  event: review
---

Reviewed git diff main...HEAD (63470471) against vett, ikwr, yrfv, xvov, gwh0, hloh (Done when unchanged on the branch; notes only). Reran: cargo test --workspace (all pass), fmt --check, clippy -D warnings on the 7 touched crates, real_journals harness on a copy of ~/edjournal (0 problems, 0 known). Extra checks: built a v7 history.sqlite from main's code over the real journals, opened it with the branch (v8 migration + rebuild): ships rows identical to a fresh v8 import, reopen idempotent, the two id reuses (5 python->python_nx, 12 type9->panthermkii) now two rows each. Compiled the shell's PowerplayCache/powerplay_journal verbatim in a scratch crate against astromech-powerplay (compiles; callers use Journal by value). Fed the real journals to powerplay::Follower in 7919-byte chunks with a refresh after each (1965 refreshes): result equals from_journal_dir. Checked real journals: Powerplay precedes Missions at every pledged first login; relogs write LoadGame+Missions only; no taxi Loadout with a ShipID; all 38 Modifier labels map. Reasoned each new test fails on main (busy-timeout test and rate test excepted). Not run: the shell build, Windows, the logbook search timing on the 400 MB synthetic history.
