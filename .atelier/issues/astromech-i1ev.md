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
status: "todo"
title: "World steps: game folder, game process and clock, settled by file generation and clock quiescence"
updated_at: "2026-11-07T15:21:22.604471470+00:00"
---

## Goal

A world's `world.jsonl` moves the game step by step (decision: world format): the journal folder, the companion files, whether Elite is running, and the clock. A test or driver runs it with `world_step` and gets control back only once the core has consumed each step. This is what lets yis2's "jump into the next stop" journey run with no sleeps.

## Done when

- [ ] The header and these step kinds load and run:
  - `journal` appends a line verbatim to the current journal file, and a line with a new file name starts that file. A journal line stamped later than the clock first advances the clock to its timestamp and waits for quiescence, then is appended, because `line_is_fresh` rejects a line more than `AHEAD_SECS` (5 s) ahead of now (pipeline.rs:131, :167).
  - `file` writes a companion file (Status.json, NavRoute.json, Cargo.json, Market.json, the rest) atomically, or truncated with `partial: true`.
  - `game` sets the world's process list to running, launcher or exited.
  - `advance` moves the manual clock.
  - `mark` names a stop.
  A malformed line fails the load with its line number. The step kinds other children own (`focus`, `press`, `release`, `audio`, `say`, `net`) are reserved: loading one before its child lands fails with "not supported yet".
- [ ] File settling is by generation. The world bumps a per-file counter after each write, and the pipeline reads the counter before handling that file and publishes it after handling, whether the file parsed or not. `world_step` waits until each written file's published generation reaches its write. A file the pipeline doesn't watch (the names it ignores, pipeline.rs:947-955) settles at once. The pipeline signals with a condvar, never a sleep.
- [ ] `advance` steps settle by the manual clock's quiescence: every registered thread is parked in a clock wait again.
- [ ] `game` steps set the process list, then advance the clock by `gamewatch::POLL` so the game watcher polls. Then:
  - `running` or `launcher` returns on the core's session-started-or-failed event. `activate` runs on a thread the watcher spawns (app.rs:1450-1457), which carries the watcher's registration; the event is published at the end of `activate`, whether the session started or failed.
  - `exited` returns once `deactivate` has dropped the session (pipeline and callout speaker stopped and joined) and the clock is quiescent. The push-to-talk loop is detached and parked in a clock wait, which already counts as quiescent. It isn't joined, because joining it would add up to 1 s to `deactivate` in the installed app.
  After any `game` step, `status` reports the new game state.
- [ ] Race test: a world test with a gate in the game watcher's poll, a world-only hook rather than `#[cfg(test)]`, because the test spawns the binary. The test holds the gate over the control API (a world-only `local` command), and issues the `game` step on a second connection. The hook reports over the first connection that the watcher reached the gate. While the gate is held, the step's reply hasn't arrived; the test checks that without timing, since the gate is still closed when it reads. The step returns after the gate opens. The test fails if the step returns once the watcher is merely woken.
- [ ] A test over `worlds/jump`, spawning the binary (world-mode child's helper):
  1. Elite is running, and a route A→B→C is set over the control API.
  2. Step to `jumped`: an `FSDJump` to B is appended.
  3. `world_wait` sees the clipboard write of C, and `route_status` names C as the next stop.
  The same world with the jump stamped two hours before the clock copies nothing (catch-up is silent).
- [ ] Stop test: a world goes from game running to game exited. The `exited` step returns, with no `advance` beyond its own, and `status` shows no session. This fails if stopping the pipeline waits for its timed wait to expire (clock child: stop wakes).
- [ ] Partial writes: a partial Status.json write settles, `status` keeps the previous flags, and the next full write's flags show. A partial write that never gets a full one also settles.
- [ ] The format is documented once, in the world module's doc comment. CONTRIBUTING.md's Tests section links to it, and docs/ARCHITECTURE.md §13's Ingest row describes worlds instead of `astromech-rec`.

## Scope

Script loading, the step runner, the pipeline's generation hook, `world_step` and the two worlds. Don't change how the pipeline reads files: the hook only publishes what it handled.

## Context

The pipeline loop is pipeline.rs:927-990. Status is read with `read_status_retry` (pipeline.rs:804-806), whose 1 ms retry stays real (clock child). Freshness is pipeline.rs:131. The watcher loop is app.rs:1440-1488. The route desk copies through the world's clip writer.

## Notes

- 2026-11-07 [note] From finding astromech-8t2s (teardown review): world mode's quit stops the history follower while the control API still accepts calls, so a set_folders arriving before the temp folder is removed would restart a follower and bring back 'Directory not empty'. Stop serving (or make following a no-op after stop) before stopping the follower, here where world steps first change folders.
