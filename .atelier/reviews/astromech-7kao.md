---
schema: atelier.review
schema_version: 2
id: astromech-7kao
title: approval on astromech-bp23 @d708b7b0
target: astromech-bp23
identity: claude-reviewer-batch1
commit: d708b7b038a51d1cccefbc8b7d9beaf2e4bdea72
created_at: 2026-11-07T08:16:57.361591941Z
event:
  event: approval
---

Re-review of fixes cd743073..d708b7b0 for u6ak (1bbaa850), p3tv (37a4ac1a), 5bvd (af09b190) and their dispositions. u6ak: GalaxyHolder slot NotTried/Failed/Open with a separate opening lock, so peek never waits; Session::start warms it on a star-map thread; Open keeps the old path, Failed says NO_STAR_MAP alone in the old check order (targets NOT_PLEDGED > NO_STAR_MAP > NO_POSITION; faction/hotspot star map before position; sites position first) and retries off the speech thread. The remaining difference (a lookup during the first open that then fails says 'Looking that up.' first, and position wins) is acceptable without Sumeet: on main the session opened galaxy.db synchronously before the microphone started, so no voice lookup could arrive during that open, and states the shipped app reaches speak as before. p3tv: the four condvar waits recover; hub test poisons state, then wait_flag sees a flag set while it waits (event-driven, no sleep). 5bvd: panic_is_caught marks the thread Unwinding on the first caught panic, so an abort's second panic writes the marker; dispatch restores the outer state (nested Call::run ok); no handler uses its own catch_unwind. The child-process test re-runs only itself (--exact), writes the marker in a tempdir, logs to stdout (log not initialised); ulimit -c 0 and apport here, no stray files in the worktree after 6 runs, passes with --test-threads=1. Fresh: cargo test -p astromech-api 33 passed; -p astromech-core all pass (215 lib, site_lookup 8, crash_marker 2); crash_marker+site_lookup 5x stable; clippy -D warnings api/core all-targets clean; fmt clean.
