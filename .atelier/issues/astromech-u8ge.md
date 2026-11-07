---
created_at: "2026-11-07T11:57:01.792573617+00:00"
id: "astromech-u8ge"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-e1a5"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "One clock for the core's behaviour timers, with a manual clock whose advance waits for quiescence"
updated_at: "2026-11-07T11:57:01.792573617+00:00"
---

## Goal

The core's behaviour timers read and wait through one clock. In world mode that clock is a manual clock the script advances. `advance` returns only when the core's registered threads have finished what was due and are waiting on the clock again, so a world step after it sees a settled core. Today `astromech_proto::Clock` (crates/astromech-proto/src/lib.rs:66-78) gives only Unix seconds, and only the remote-data cache reads it (crates/astromech-data/src/lib.rs:212). Every timer reads `Instant::now` or the wall clock and sleeps or waits on its own.

## Done when

- [ ] `Clock` gives:
  - monotonic now and UTC now;
  - `sleep(d)`;
  - waits the clock can wake:
    - a clock-owned channel (`clock.channel()`), whose receiver has `recv_timeout` and an untimed `recv`;
    - a condvar wait, timed and untimed, on an `Arc<Condvar>` handed to the clock, so it can `notify_all`;
  - `register()`, returning a guard.
  Under `SystemClock` these are exactly today's `std::sync::mpsc` and `Condvar` calls with today's timeouts, so the installed app doesn't change, with two teardown exceptions. Both happen only when Elite exits or the app quits:
  - pipeline stop wakes the pipeline thread at once, instead of after up to 250 ms (`IDLE`);
  - the binds watch ends on an explicit stop instead of on its notify channel disconnecting.
  Review checks each converted site against this.
- [ ] `ManualClock::advance(d)` moves time, wakes every clock wait whose deadline passed (sleeps, channel waits by an internal wake message, condvars by `notify_all`), and returns only when every registered thread is parked in a clock wait, timed or untimed. An untimed clock wait counts as parked. Unregistered threads' timed clock waits also block until `advance`, but `advance` doesn't wait for them.
- [ ] Rules, stated in the trait's doc comment:
  - A registered thread blocks only in clock waits. The implementer greps every registered thread's code for `recv(`, `recv_timeout`, `.wait(`, `wait_timeout`, `sleep` and `join`, converts or justifies each, and lists them in the close reason.
  - A thread is registered by its parent before it is spawned: `register()` is taken on the parent and moved into the child, so the child counts from its first instruction.
  - Whatever stops a clock-waiting thread wakes its wait. `SpeechOut::drop` does this already (stop, then `notify_all`, crates/astromech-core/src/callouts.rs:506-513). `Pipeline::drop` (pipeline.rs:224-230) gets the same: stop, then wake through the clock channel, then join. A loop that ends only when its sender disconnects needs its own stop and wake: the binds watch ends on `rx.recv()` returning an error (crates/astromech-core/src/health.rs:194).
  - The `notify` watchers' event handlers send into the clock channel.
- [ ] The registered threads:
  - pipeline loop;
  - game watch, and the `activate` thread it spawns (app.rs:1450-1457);
  - idle reaper;
  - update checker;
  - callout speaker (`SpeechOut`);
  - history follower;
  - binds watch;
  - EDDN and EDSM workers;
  - later, the push-to-talk loop (devices child).
- [ ] Unit tests of `ManualClock`:
  - race: a registered thread loops on `clock.sleep(1s)`, signals the test when woken, then waits on a gate the test holds. `advance(1s)` must not return while the gate is closed, which the test sees through a channel, not by timing. It returns after the gate opens. The test fails on a clock whose `advance` returns once sleepers are merely woken.
  - inherited registration: a child spawned with its parent's registration counts before its first wait.
  - stop wakes: dropping a pipeline whose thread is in a clock wait returns without any `advance`.
- [ ] `Boundaries` gains `clock: Arc<dyn Clock>`, and `run` passes `SystemClock`. These sites wait or read through it:
  - behaviour waits:
    - policy focus wait (crates/astromech-policy/src/lib.rs:216-231);
    - hub `wait_flag` and `wait_gui` (crates/astromech-core/src/hub.rs:99-133), its 25 ms slices replaced by a clock condvar wait;
    - callout queue (callouts.rs:491-497);
    - history follower (history.rs:210-213);
    - binds watch (health.rs:194-203);
    - pipeline loop (pipeline.rs:935-944);
    - game watch (app.rs:1485);
    - idle reaper (app.rs:1641);
    - update thread (app.rs:1654-1657);
    - remote-data limiter (crates/astromech-data/src/lib.rs:250-252);
    - Spansh polling (spansh.rs:1052-1061);
    - EDDN/EDSM worker (crates/astromech-eddn/src/worker.rs:145-146);
    - install quit delay (app.rs:539);
  - behaviour reads: journal freshness (pipeline.rs:131, :430, :562, :872), the reaction engine's `now` arguments (pipeline.rs:305, :340, :389, :814, :909), policy `authorize` callers (actions.rs:152, :227), cooldowns and the voice turn (callouts.rs:331, :523), api idle (crates/astromech-api/src/state.rs:104-134, :266) and the remote-data cache.
- [ ] Short pacing waits stay on real time, each with `#[allow(clippy::disallowed_methods)]` and the reason given here. This is the semantics for short inline waits under the manual clock: they finish on their own, and in a world they decide nothing that varies between runs.
  - policy modifier gap and key hold (lib.rs:298, :302-308, :311): they pace key events to the game;
  - paste settle (actions.rs:163): it gives the game time to take Ctrl+V before GuiFocus is re-read; in a world GuiFocus only changes between steps, so the re-read is the same every run;
  - `read_status_retry` (crates/astromech-status/src/status_file.rs:101): it retries a half-written file;
  - clipboard open retry (clipboard.rs:23) and SQLite busy retries (crates/astromech-data/src/cache.rs:25, crates/astromech-store/src/lib.rs:46): they retry a held resource;
  - voice worker I/O and its timeouts (crates/astromech-voice/src/client.rs:122, :134, :174): they talk to the speech worker process, which world mode doesn't start;
  - mic (mic.rs:246), XInput (xinput.rs:106) and `controls::start`'s loop (controls.rs:247-262): device timing, left untouched;
  - update apply (update.rs:402, :521) and the console-only main.rs paths, which world mode doesn't run.
- [ ] One test per behaviour kind, under `ManualClock`, at the lowest layer that fails without the change, with no sleep and no wall-clock read:
  - a journal line stamped at the clock's now is fresh, and one stamped two hours earlier is not;
  - a verify wait returns at once when the flag is set, and times out only after `advance` passes `verify_timeout`;
  - a Spansh poll's next request goes out only after `advance` passes `Polling::every`;
  - a callout cooldown runs on the clock.
- [ ] `cargo test --workspace` passes on Linux and Windows CI. Review reads every changed in-game site (policy, hub, callouts, pipeline, health) against "behaves the same under `SystemClock`".

## Scope

The trait, `SystemClock`, `ManualClock`, its channel and condvar waits, registration, the stop-and-wake changes, the composition and the sites above. The lint and the remaining reads are the lint child. It can be built in parallel with the assembly child; whichever lands second rebases onto `Boundaries`.

## Context

The reaction engine already takes `now` (pipeline.rs:305), so its callers are the sites. Decision astromech-1zhg: determinism is enforced, not hoped for.
