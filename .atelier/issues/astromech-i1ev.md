---
created_at: "2026-11-07T11:57:01.793960301+00:00"
id: "astromech-i1ev"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-d2nq"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-u8ge"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "World steps: game folder, game process and clock, settled by file generation and clock quiescence"
updated_at: "2026-11-07T16:02:54.681573508+00:00"
---

## Goal

A world's `world.jsonl` moves the game step by step (decision: world format): the journal folder, the companion files, whether Elite is running, and the clock. A test or driver runs it with `world_step` and gets control back only once the core has consumed each step. This is what lets yis2's "jump into the next stop" journey run with no sleeps.

## Done when

- [x] The header and these step kinds load and run:
  - `journal` appends a line verbatim to the current journal file, and a line with a new file name starts that file. A journal line stamped later than the clock first advances the clock to its timestamp and waits for quiescence, then is appended, because `line_is_fresh` rejects a line more than `AHEAD_SECS` (5 s) ahead of now (pipeline.rs:131, :167).
  - `file` writes a companion file (Status.json, NavRoute.json, Cargo.json, Market.json, the rest) atomically, or truncated with `partial: true`.
  - `game` sets the world's process list to running, launcher or exited.
  - `advance` moves the manual clock.
  - `mark` names a stop.
  A malformed line fails the load with its line number. The step kinds other children own (`focus`, `press`, `release`, `audio`, `say`, `net`) are reserved: loading one before its child lands fails with "not supported yet".
- [x] File settling is by generation. The world bumps a per-file counter after each write, and the pipeline reads the counter before handling that file and publishes it after handling, whether the file parsed or not. `world_step` waits until each written file's published generation reaches its write. A file the pipeline doesn't watch (the names it ignores, pipeline.rs:947-955) settles at once. The pipeline signals with a condvar, never a sleep.
- [x] `advance` steps settle by the manual clock's quiescence: every registered thread is parked in a clock wait again.
- [x] `game` steps set the process list, then advance the clock by `gamewatch::POLL` so the game watcher polls. Then:
  - `running` or `launcher` returns on the core's session-started-or-failed event. `activate` runs on a thread the watcher spawns (app.rs:1450-1457), which carries the watcher's registration; the event is published at the end of `activate`, whether the session started or failed.
  - `exited` returns once `deactivate` has dropped the session (pipeline and callout speaker stopped and joined) and the clock is quiescent. The push-to-talk loop is detached and parked in a clock wait, which already counts as quiescent. It isn't joined, because joining it would add up to 1 s to `deactivate` in the installed app.
  After any `game` step, `status` reports the new game state.
- [x] Race test: a world test with a gate in the game watcher's poll, a world-only hook rather than `#[cfg(test)]`, because the test spawns the binary. The test holds the gate over the control API (a world-only `local` command), and issues the `game` step on a second connection. The hook reports over the first connection that the watcher reached the gate. While the gate is held, the step's reply hasn't arrived; the test checks that without timing, since the gate is still closed when it reads. The step returns after the gate opens. The test fails if the step returns once the watcher is merely woken.
- [ ] A test over `worlds/jump`, spawning the binary (world-mode child's helper):
  1. Elite is running, and a route A→B→C is set over the control API.
  2. Step to `jumped`: an `FSDJump` to B is appended.
  3. `world_wait` sees the clipboard write of C, and `route_status` names C as the next stop.
  The same world with the jump stamped two hours before the clock copies nothing (catch-up is silent).
- [x] Stop test: a world goes from game running to game exited. The `exited` step returns, with no `advance` beyond its own, and `status` shows no session. This fails if stopping the pipeline waits for its timed wait to expire (clock child: stop wakes).
- [x] Partial writes: a partial Status.json write settles, `status` keeps the previous flags, and the next full write's flags show. A partial write that never gets a full one also settles.
- [x] The format is documented once, in the world module's doc comment. CONTRIBUTING.md's Tests section links to it, and docs/ARCHITECTURE.md §13's Ingest row describes worlds instead of `astromech-rec`.

## Scope

Script loading, the step runner, the pipeline's generation hook, `world_step` and the two worlds. Don't change how the pipeline reads files: the hook only publishes what it handled.

## Context

The pipeline loop is pipeline.rs:927-990. Status is read with `read_status_retry` (pipeline.rs:804-806), whose 1 ms retry stays real (clock child). Freshness is pipeline.rs:131. The watcher loop is app.rs:1440-1488. The route desk copies through the world's clip writer.

## Notes

- 2026-11-07 [note] From finding astromech-8t2s (teardown review): world mode's quit stops the history follower while the control API still accepts calls, so a set_folders arriving before the temp folder is removed would restart a follower and bring back 'Directory not empty'. Stop serving (or make following a no-op after stop) before stopping the follower, here where world steps first change folders.
- 2026-11-07 [decision] World mode's settle signals travel in one Boundaries field, settle: Option<Arc<world::Settle>> (file generations, session events, the game watch's gate); Session::start and pipeline::start_with_eddn take the file generations as an eighth parameter, with #[allow(clippy::too_many_arguments)]
  Reason: One world-only hook beside more_commands, None for the installed app, so each hook there is one Option check; an eighth parameter is smaller than an options struct for one optional field
  Rejected: putting the generations in session::Opts (Debug/Clone flags, not a hook)
  Rejected: carrying them on the StateHub (game state, not world wiring)
- 2026-11-07 [decision] The pipeline, history and binds watchers drop notify's read-only access events (Access Open, Close(Read)) before they reach their clock channels (pipeline::may_change)
  Reason: On Linux notify asks inotify for opens, so each watcher's own reads woke it again: the pipeline and history follower spun at 100% CPU in world mode, the clock was quiescent only between self-wakes, and stop-wakes couldn't be shown. Windows' backend emits no Access events, so the installed app is unchanged
  Rejected: leaving it, and settling on momentary quiescence
- 2026-11-07 [decision] world_gate holds the game watch at the top of its next poll and reports reached only once every other registered thread is parked (Clock::wait_others_parked); holding again while it is stopped lets that poll go and stops at the next
  Reason: The stop test must fail without the pipeline's stop-wake. The exited step's POLL advances fire the pipeline's 250 ms wait together with the watcher's, so without the quiet wait the pipeline was usually still running when its drop came and the mutant passed; at the second poll with the rest parked, its deadline is ahead and only the wake ends it
  Rejected: a stop test with no gate (passed under the no-wake mutant 12 of 12 runs)
- 2026-11-07 [decision] Game state for tests is the local world_game command (the session hub's GameState, null with no session); status is unchanged
  Reason: status has no Status.json flags, and adding fields to the window's status reply would change the installed app for a test
- 2026-11-07 [decision] A game step polls (advance POLL) until the watch reports the game so, at most MISSES_TO_IDLE polls, then waits until every activate the watch started has ended (Sessions: started at activate's start, ended at its end, including the already-active early return)
  Reason: From running, exited takes two polls without the game; counting both ends of activate also settles launcher-then-running, where activate returns at once because a session is already there
- 2026-11-07 [decision] Step lines name their kind by key ({"journal": …}, {"file": …, "json"|"text"|"from"}, {"game": …}, {"advance": "1.5s"}, {"mark": …}); a journal object is appended exactly as written (serde_json raw_value), a string as the line itself
  Reason: Readable JSONL with no escaping for journal lines, verbatim bytes, and a malformed line still expressible
- 2026-11-07 [observation] Done when 5's two-hour variant: a line appended to the journal the game writes (Class::Game) is live whatever its stamp (pipeline.rs apply_class), so a jump stamped two hours before the clock and appended while the session runs DOES copy the next stop (observed). The test puts the old jump in while Elite is closed, so the next session's catch-up reads it: that is silent, which is what the line's '(catch-up is silent)' names. Whether a live append stamped long ago should be silent is a product question for the orchestrator, not changed here (scope: don't change how the pipeline reads files).

## Report

**Outcome.** `world.jsonl` loads and runs: a header (`start`) and the `journal`, `file`, `game`, `advance` and `mark` steps, with `focus`, `press`, `release`, `audio`, `say`, `net` and the header's `devices`, `reactions`, `foreground` reserved. `world_step {to?}` runs to a mark and returns once each step is consumed; `world_game` and `world_gate` are the other new `local`, world-only commands. Code commit 5197dd65 on `astromech-vucf` (after "Start astromech-i1ev" b89bc62c). Not pushed. Eight of nine Done when lines ticked; line 6 is not (below).

Where things are: crates/astromech-core/src/world.rs (the format and settle rules, in its doc comment), world/script.rs (loading), world/steps.rs (the runner), world/settle.rs (`FileGens`, `Sessions`, `Gate`); the pipeline's hook in pipeline.rs `spawn_loop` and `start_with_eddn`; `Boundaries::settle` in app.rs; `Clock::wait_others_parked` in astromech-proto; worlds/jump; crates/astromech-core/tests/world_steps.rs.

**Installed app.** `Boundaries::real` passes `settle: None`, `main.rs`'s console session passes no generations, so each settle hook is one `Option` check: per game-watch poll (the gate), twice per `activate` (the session events), per file notification the pipeline handles (`files.is_some()` before collecting names; the generations read and published only with `Some`; an empty `Vec` otherwise, which doesn't allocate). Equivalent in cost. The one change on every path is `pipeline::may_change`, which drops notify's read-only `Access` events; Windows' backend never emits those (notify 8.2.1 windows.rs has no `Access`), so on Windows it is a match per event that always passes.

**Proof** (Linux, after the last code change unless noted):
1. Header and step kinds: `world::script::tests` (4 tests: each kind loads, the journal object verbatim, the payload `from`, malformed lines name their line number, reserved kinds say "not supported yet (astromech-1hh9|xk0n)"); `world::tests::the_clock_starts_at_the_headers_time_and_a_bad_step_fails_the_load` (World::load: clock at 1794052800, "world.jsonl line 2: the step kind `focus` is not supported yet (astromech-1hh9)"). They run in the world tests below: a journal line to the current file and to a new file (`a_later_stamped_line_moves_the_clock_first`), whole and partial files, game, advance (inside game steps and stamped lines) and marks. Advance-then-append: `a_later_stamped_line_moves_the_clock_first` (a new journal stamped 57 s and 87 s ahead of the clock is followed and its jump copies the next stop). Mutation, append then advance: fails 3/3, `left: []` (the new journal is never followed).
2. File generations: `pipeline::tests::the_pipeline_publishes_the_generation_it_handled` (catch-up publishes; a write held inside its handling, in the callouts' delivery, isn't published yet; an unparsable write is published; an ignored name never is; no pipeline: settles at once). Mutation, publish before handling: fails 3/3, "published before it was handled". `settle::tests` (4) cover the counter, the lock that makes a read wait out a write in progress, sessions and the gate. World level: `a_partial_status_write_settles_and_keeps_the_last_flags`; mutation with the step's generation wait removed: fails 10/10 (flags still 4). Every wait is a condvar with a real-time guard (WAIT_GUARD), never a sleep.
3. `advance` settles by `ManualClock::advance`'s quiescence, guarded on a helper thread (a core that never settles fails the step at 30 s).
4. Game steps: `game_steps_start_and_stop_the_session` (launcher: `status` launcher, active; running: running, active, the session caught up to Sol; exited: not_running, inactive, `world_game` null). Exited takes two polls (MISSES_TO_IDLE). Session events at both ends of `activate`.
5. Race test `a_game_step_waits_for_the_watcher_it_woke`: gate held on connection 1 (progress "held"), the step on connection 2, "reached" back on 1; with the gate still held the step's reply channel is empty (a `try_recv`, no wait) and `status` still not_running; released, the step returns and the session runs. Mutation, the game step returns once it has woken the watcher (advance fired, not awaited): fails 10/10 and again 3/3 after the final changes, "the step returned while the watcher was held at the gate".
6. **Not ticked.** `a_jump_to_the_next_stop_copies_the_one_after` passes over `worlds/jump`: route Sol → Alpha Centauri → Barnard's Star set with `route_set`, step to `jumped`, `world_wait` sees the clipboard write of Barnard's Star, and `route` (there is no `route_status` command; `route`'s view) names it next. The two-hour variant as written is false about the core: appended while the session runs, a jump stamped two hours before the clock copies (observed: `left: ["Barnard's Star"]`), because every line appended to the game's own journal is live (Class::Game). `a_jump_stamped_hours_before_the_clock_is_caught_up_silently` writes the old jump while Elite is closed, so the next session's catch-up reads it: nothing copied, next stop unchanged, `world_game` system Alpha Centauri. That is the "(catch-up is silent)" property, but it isn't the line's wording; the orchestrator decides (observation note).
7. Stop test `the_game_exiting_stops_the_session_with_no_more_time`: held at the exited step's second poll until the rest is parked, released, the step returns with `ran: 1`, `status` not_running and inactive. Mutation, `Pipeline::drop` without its wake: fails 2/2 and 1/1 after the final changes, "the core didn't settle within 30 s of advancing the clock by 3s". (Without the gate the test passed under that mutation 12/12, decision note.)
8. Partial writes: as in 2 (a partial settles and keeps flags 4; the next whole write's 16777216 shows; a trailing partial and an ignored `notes.txt` settle).
9. Docs: world.rs's doc comment holds the format; CONTRIBUTING.md's "The core in a world" links to it; ARCHITECTURE §13's Ingest row describes worlds.

Also: finding astromech-8t2s: `history::Following::stop` is final, so a `set_folders` served after it starts no follower (`history::tests::nothing_is_followed_after_stop`).

Checks: `cargo test --workspace --no-fail-fast` exit 0 (136 binaries, 1351 passed, 0 failed); `cargo clippy --workspace --all-targets --locked -- -D warnings` clean; `cargo fmt --all --check`, `scripts/check-deps.py` ("dependency rules OK (32 workspace crates)"), `scripts/check-guide.py` and `cargo deny check` pass. Loop: world_steps (8 tests) and world_mode (3) together 100 times from the built binaries, 100 passed, 0 failed, no temp worlds left in /tmp. The real HOME has no astromech folders.

### Deviations
- `route_status` doesn't exist; the test reads `route`.
- Flags are read with the new `world_game`, not `status` (decision note).
- `game exited` from running polls twice; the step loops up to MISSES_TO_IDLE polls (decision note).
- The gate waits for the rest of the core to park before it reports, and can be moved to the next poll (decision note); `Clock::wait_others_parked` is new API on `dyn Clock`.
- Outside the pipeline hook: `may_change` in the pipeline, history and binds watchers (decision note). It is how files are noticed, not how they are read.
- One committed world (`worlds/jump`); the other worlds are variants the tests write from it (`support::world_with`). Scope said "the two worlds".
- `serde_json`'s `raw_value` feature on astromech-core.

### Surprises
- In world mode on Linux the pipeline and the history follower spun at 100% CPU on their own OPEN events, which made quiescence momentary and hid the stop-wake from any world test. Fixed by `may_change`. On a Linux desktop the same happened before this change.

### Open items
- Done when 6's two-hour wording (above).
- Windows: none of this was run there. On Windows `start_buttons` runs the real `start_voice` in world mode until astromech-1hh9, so the world tests' sessions may fail or touch real input there; Windows CI decides.
- The binds watcher had the same loop on Linux (its check reads the Bindings folder, which woke it again after its 500 ms debounce). It is filtered by `may_change` too, but no test covers it there.
- Independent review is outstanding.

