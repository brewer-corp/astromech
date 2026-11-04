---
schema: atelier.review
schema_version: 1
id: astromech-kwic
title: approval on astromech-f2k1 @47967160
target: astromech-f2k1
identity: review-agent-f2k1-1
commit: 47967160ec0492c800103d37c0319669e6210fb2
created_at: 2026-11-04T22:40:41.069511271Z
event:
  event: approval
---

Read a74c66c3..47967160 against Done when. Checked these lifecycles: watchdog restart, crash or kill leftovers, the instance lock ordering (Windows mutex taken before publish, the note dropped before _guard, Drop removes only its own note), two cores racing, the window opened before, during and after a core's life, and an upgrade from the always-overwrite config.path writer. Read the shell's data_dir line against window::data_dir: it is equivalent to the old expression and now tested. Ran cargo test -p astromech-store -p astromech-core after touching lib.rs: store 19 and core 187 lib tests pass, including the 4 new and 1 revised tests. clippy -D warnings and fmt are clean. gh pr checks 194 is all green at 47967160, Windows MSVC and installer jobs included. No fix_now findings; R1 and R2 are deferred, R3 needs no action.
