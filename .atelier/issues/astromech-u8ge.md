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
closed_at: "2026-11-07T14:40:56.987747331+00:00"
status: "done"
title: "One clock for the core's behaviour timers, with a manual clock whose advance waits for quiescence"
updated_at: "2026-11-07T14:40:56.987747331+00:00"
---

## Goal

The core's behaviour timers read and wait through one clock. In world mode that clock is a manual clock the script advances. `advance` returns only when the core's registered threads have finished what was due and are waiting on the clock again, so a world step after it sees a settled core. Today `astromech_proto::Clock` (crates/astromech-proto/src/lib.rs:66-78) gives only Unix seconds, and only the remote-data cache reads it (crates/astromech-data/src/lib.rs:212). Every timer reads `Instant::now` or the wall clock and sleeps or waits on its own.

## Done when

- [x] `Clock` gives:
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
- [x] `ManualClock::advance(d)` moves time, wakes every clock wait whose deadline passed (sleeps, channel waits by an internal wake message, condvars by `notify_all`), and returns only when every registered thread is parked in a clock wait, timed or untimed. An untimed clock wait counts as parked. Unregistered threads' timed clock waits also block until `advance`, but `advance` doesn't wait for them.
- [x] Rules, stated in the trait's doc comment:
  - A registered thread blocks only in clock waits. The implementer greps every registered thread's code for `recv(`, `recv_timeout`, `.wait(`, `wait_timeout`, `sleep` and `join`, converts or justifies each, and lists them in the close reason.
  - A thread is registered by its parent before it is spawned: `register()` is taken on the parent and moved into the child, so the child counts from its first instruction.
  - Whatever stops a clock-waiting thread wakes its wait. `SpeechOut::drop` does this already (stop, then `notify_all`, crates/astromech-core/src/callouts.rs:506-513). `Pipeline::drop` (pipeline.rs:224-230) gets the same: stop, then wake through the clock channel, then join. A loop that ends only when its sender disconnects needs its own stop and wake: the binds watch ends on `rx.recv()` returning an error (crates/astromech-core/src/health.rs:194).
  - The `notify` watchers' event handlers send into the clock channel.
- [x] The registered threads:
  - pipeline loop;
  - game watch, and the `activate` thread it spawns (app.rs:1450-1457);
  - idle reaper;
  - update checker;
  - callout speaker (`SpeechOut`);
  - history follower;
  - binds watch;
  - EDDN and EDSM workers;
  - later, the push-to-talk loop (devices child).
- [x] Unit tests of `ManualClock`:
  - race: a registered thread loops on `clock.sleep(1s)`, signals the test when woken, then waits on a gate the test holds. `advance(1s)` must not return while the gate is closed, which the test sees through a channel, not by timing. It returns after the gate opens. The test fails on a clock whose `advance` returns once sleepers are merely woken.
  - inherited registration: a child spawned with its parent's registration counts before its first wait.
  - stop wakes: dropping a pipeline whose thread is in a clock wait returns without any `advance`.
- [x] `Boundaries` gains `clock: Arc<dyn Clock>`, and `run` passes `SystemClock`. These sites wait or read through it:
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
- [x] Short pacing waits stay on real time, each with `#[allow(clippy::disallowed_methods)]` and the reason given here. This is the semantics for short inline waits under the manual clock: they finish on their own, and in a world they decide nothing that varies between runs.
  - policy modifier gap and key hold (lib.rs:298, :302-308, :311): they pace key events to the game;
  - paste settle (actions.rs:163): it gives the game time to take Ctrl+V before GuiFocus is re-read; in a world GuiFocus only changes between steps, so the re-read is the same every run;
  - `read_status_retry` (crates/astromech-status/src/status_file.rs:101): it retries a half-written file;
  - clipboard open retry (clipboard.rs:23) and SQLite busy retries (crates/astromech-data/src/cache.rs:25, crates/astromech-store/src/lib.rs:46): they retry a held resource;
  - voice worker I/O and its timeouts (crates/astromech-voice/src/client.rs:122, :134, :174): they talk to the speech worker process, which world mode doesn't start;
  - mic (mic.rs:246), XInput (xinput.rs:106) and `controls::start`'s loop (controls.rs:247-262): device timing, left untouched;
  - update apply (update.rs:402, :521) and the console-only main.rs paths, which world mode doesn't run.
- [x] One test per behaviour kind, under `ManualClock`, at the lowest layer that fails without the change, with no sleep and no wall-clock read:
  - a journal line stamped at the clock's now is fresh, and one stamped two hours earlier is not;
  - a verify wait returns at once when the flag is set, and times out only after `advance` passes `verify_timeout`;
  - a Spansh poll's next request goes out only after `advance` passes `Polling::every`;
  - a callout cooldown runs on the clock.
- [ ] `cargo test --workspace` passes on Linux and Windows CI. Review reads every changed in-game site (policy, hub, callouts, pipeline, health) against "behaves the same under `SystemClock`".

## Scope

The trait, `SystemClock`, `ManualClock`, its channel and condvar waits, registration, the stop-and-wake changes, the composition and the sites above. The lint and the remaining reads are the lint child. It can be built in parallel with the assembly child; whichever lands second rebases onto `Boundaries`.

## Context

The reaction engine already takes `now` (pipeline.rs:305), so its callers are the sites. Decision astromech-1zhg: determinism is enforced, not hoped for.

## Notes

- 2026-11-07 [decision] The condvar waits take the mutex as well as the condvar: dyn Clock's wait/wait_timeout(m: &Arc<Mutex<T>>, cv: &Arc<Condvar>, guard, d), and notify_all(m, cv).
  Reason: Under the manual clock, advance must wake a waiter that has registered its deadline but not yet entered cv.wait; the clock can close that window only by taking the waiter's mutex before notify_all. With the condvar alone the wake is lost and the wait ends only at a later advance. Under SystemClock the mutex is unused, so the calls are std's.
  Rejected: Arc<Condvar> alone (lost wakes under the manual clock)
  Rejected: Polling the condvar in short real-time slices under the manual clock (the strategy forbids polling)
- 2026-11-07 [decision] The hub's verify waits keep their 25 ms slices; only the wait call and the time reads moved to the clock (clock.wait_timeout with min(end - now, 25 ms)).
  Reason: The slices exist so the kill switch's cancel (ticket.is_cancelled, checked between slices) ends a verify wait within 25 ms. Removing them would change kill-switch behaviour under SystemClock, which the strategy reserves for Sumeet. Under the manual clock a slice is a clock deadline, so a verify wait still times out exactly when advance passes verify_timeout.
  Rejected: One clock condvar wait to the deadline (the spec's wording): the kill switch would be seen only at the timeout
- 2026-11-07 [decision] Constructors keep their SystemClock defaults and gain a clock variant where a site needs one (StateHub::with_clock, Policy::with_clock, Pending::new, Following::new(timing, clock)); the pipeline runs on its hub's clock, the SpeechOut thread on its Pending's, and the command layer's idle times on the remote-data client's (Client::clock_in_use).
  Reason: Session::start, assemble and world mode pass the one clock to each; the tests and tools that build these pieces alone keep compiling and keep real time. One owner per clock: Boundaries::clock, with world mode building its remote-data client on the same ManualClock.
- 2026-11-07 [decision] World mode's boundaries now carry a ManualClock (World::clock), started at the real time and shared with its remote-data client; nothing advances it until astromech-i1ev's advance step.
  Reason: The Goal's world-mode clock is the manual one, and i1ev's steps need the handle; the d2nq world tests pass unchanged on it (30-iteration loop), and the world refuses every lookup, so no frozen limiter or Spansh wait is reached.

## Report

**Outcome.**

Built on branch `astromech-vucf`, commits 2dd4dd4d..eaa446ef (5 commits after "Start astromech-u8ge" 72f82dc0). Not pushed. Independent review is still to do.

- `astromech_proto::clock` holds the clock. `Clock` gives `now`, `utc`, `now_s`, `sleep` and `register`.
- `dyn Clock` adds `channel()` (a `ClockSender` and a `ClockReceiver` with `recv`, `recv_timeout` and `try_recv`), the condvar waits `wait` and `wait_timeout`, `notify_all` and `spawn`. `spawn` takes the registration on the parent and enters it first thing on the child.
- There are two clocks, `SystemClock` and `ManualClock`, and `ManualClock` has `advance`.
- `Boundaries::clock` and `Core::clock` carry the clock. `Boundaries::real` passes `SystemClock`. World mode passes the world's `ManualClock` (`World::clock`), and its remote-data client runs on the same one.

**Done when: proof.**

1. **The `Clock` API, with SystemClock exactly today's except at two teardowns.** The API is in crates/astromech-proto/src/clock.rs.
   - Under `SystemClock`, each wait is the `std` call with the caller's own timeout:
     - `recv_timeout` is `mpsc::Receiver::recv_timeout(d)`, and `recv` is `mpsc::Receiver::recv`;
     - `wait_timeout` is `Condvar::wait_timeout(g, d)`, and `wait` is `Condvar::wait`;
     - `notify_all` is `Condvar::notify_all`, and `sleep` is `thread::sleep`;
     - `now` is `Instant::now()`, `utc` is `SystemTime::now()`, and `now_s` gives the same value as the old `SystemClock`.
   - The channel's messages are `Item | Wake`. Under `SystemClock` only a stop sends a `Wake`, and that gives the two teardown exceptions:
     - `Pipeline::drop` now stops, wakes the clock channel, then joins. Before, the join waited for up to 250 ms (`IDLE`).
     - `HealthWatch::drop` now sets a stop flag and wakes the channel. Before, the thread ended when the notify sender disconnected. Because the thread checks that flag after the debounce, a change still being debounced when the session drops no longer gets one last `run_check`. That only happens at teardown.
   - Proof: `system_channel_is_mpsc`, and a live comparison of the installed app before and after this change, described below.
2. **`ManualClock::advance`.**
   - It fires the waits whose deadline has passed and marks their registered threads busy. It marks busy the condvar waiters that the notify will reach as well.
   - It calls each wake (a `Wake(id)` message, or taking the lock and then `notify_all`), then waits until every registration is parked or gone.
   - An untimed wait counts as parked. An unregistered thread's waits fire but are not waited for.
   - A send, a wake or a `notify_all` marks the receiving or waiting registered thread busy, under the clock's lock, before it delivers.
   - Tests: `channel_waits_end_by_deadline_item_or_wake` and `condvar_waits_end_by_deadline_or_notify`.
3. **The rules are in `Clock`'s doc comment.** It states the rules, plus one extra for a manual-clock receiver: it never sees its senders disconnect, so a loop that ends only on disconnect needs its own stop.
   Grep of each registered thread's code for `recv(`, `recv_timeout`, `.wait(`, `wait_timeout`, `sleep` and `join`:
   - **Pipeline loop** (pipeline.rs:937-1000):
     - `rx.recv_timeout(IDLE)` and `try_recv` are clock waits.
     - `on_status` calls `read_status_retry`, whose 1 ms sleep is real: a pacing wait.
     - The store's `enable_wal` busy retry is real: a pacing wait.
     - Delivering a callout is a lock and a notify. EDDN and EDSM `wake` is a send. Neither blocks.
   - **Game watch** (app.rs `watch_game`):
     - `clock.sleep(POLL)` is a clock wait.
     - `deactivate` joins the pipeline and the callout speaker. The join is justified: both are stopped and woken, so they end without clock time.
     - Dropping the session's voice workers waits on real time (voice client.rs:174). That is the speech worker's I/O, which world mode doesn't start.
   - **`activate` thread:** `Session::start` does file and SQLite I/O and starts the speech worker (real, worker I/O). It calls `deactivate` as above.
   - **Idle reaper:** only `clock.sleep(10 s)`.
   - **Update checker:** `clock.sleep` for `STARTUP_DELAY` and `CHECK_EVERY`. `check_update` does network I/O; it is installed-app only.
   - **Callout speaker:** `clock.wait_timeout`. `speaker.say` queues to the TTS thread; a `LazySpeaker` can start the worker (worker I/O).
   - **History follower:**
     - `recv_timeout(recheck)`, `clock.sleep(settle)` and `try_recv` are clock waits.
     - It first joins the previous follower. That join is justified: the previous follower was stopped and poked. See open item 3.
   - **Binds watch:** `recv()` and `recv_timeout(left)` are clock waits. `run_check` is file I/O.
   - **EDDN and EDSM workers:** `recv_timeout(wait)` and `try_recv`. The post is network I/O; world mode switches uploads off.
4. **Registered threads.** Every thread listed is spawned with `clock.spawn`. The game watch registers each `activate` thread before spawning it. The push-to-talk loop belongs to the devices child.
5. **Unit tests** (`cargo test -p astromech-proto`, 8 passed):
   - `advance_returns_only_once_the_woken_thread_is_parked_again`:
     - the test sees `advance` waiting for the thread while the gate is still closed, through a test-only channel that `advance` reports on;
     - `advance` returns only after the gate opens.
   - `a_child_counts_from_its_parents_registration`: the child is held before it enters its registration and before any clock wait, and `advance` still waits for it.
   - **The race test fails when "woken" is treated as "settled".** With `advance` patched to return right after the wakes, it panicked at clock.rs:675, "advance returned while the woken thread was held at the gate". The two other wait tests failed too.
   - **The inherited-registration test fails without the parent's registration counting.** With `register()` patched to insert as parked, it failed with "advance didn't wait for the child".
   - **Stop wakes:** `pipeline::tests::dropping_a_pipeline_wakes_its_clock_wait`. With `self.wake.wake()` removed, it failed at its 30 s guard with "the pipeline's drop waited for the clock".
6. **Sites converted, with their SystemClock equivalence.** Each one reads `Instant::now`, `SystemTime::now` or `Utc::now`, or makes the same `std` wait, with the same timeout:
   - **Policy:**
     - `wait_foreground`: `clock.now()` and `clock.sleep(50 ms)`.
     - `check`: `authorize` with `clock.now()`.
     - `Policy::new` keeps `SystemClock`; `Policy::with_clock` is new.
   - **Hub:**
     - `wait_flag` and `wait_gui`: `clock.now()` and `clock.wait_timeout(state, cv, g, min(end - now, 25 ms))`. That is `cv.wait_timeout(...)` plus `unwrap_or_else(into_inner)`, which is what `.recover()` did. The 25 ms slices are kept (decision note).
     - `update`: `clock.notify_all`. `state` and `cv` are now `Arc`s.
   - **Callouts:**
     - `SpeechOut`: `turn.active`, `q.pop` and `wait_timeout` with 250 ms or 50 ms, all on `Pending`'s clock.
     - The router's push and `SpeechOut::drop` use `notify_all`.
     - `Background`'s gap uses `clock.now()`.
   - **Pipeline:**
     - Freshness reads `now_s` in `line_is_fresh`, `start_time` and `retry_pending`'s `DateTime::from(clock.utc())`.
     - `note_startup_lengths` uses `clock.utc()` with `.with_timezone(&Local)`, which is how chrono's `Local::now()` is built.
     - The engine's `now` at the journal, tick, market, status and route call sites.
     - `last_read`, `last_scan` and `last_tick` use `saturating_duration_since`, which is `Instant::elapsed`'s own behaviour.
     - The watcher's handler is a closure that sends, the same as `notify`'s impl for `mpsc::Sender`.
     - The pipeline runs on its hub's clock.
   - **Health:** the binds watch, with teardown exception 2.
   - **History:** `recheck` and `settle`, via `Following::new(timing, clock)`.
   - **App:**
     - The game watch's `POLL`.
     - Its `activate` threads are spawned with `Builder::new().spawn(..).expect("failed to spawn thread")`, which is what `std::thread::spawn` does.
     - The idle reaper's 10 s and its `release_idle` calls.
     - The update thread, with the same thread names.
     - The install quit delay of 300 ms.
   - **Data:**
     - The limiter's `reserve(clock.now())` and `clock.sleep(wait)`.
     - Spansh polling: `saturating_duration_since` and `clock.sleep(every)`.
   - **EDDN and EDSM:** `Worker::spawn(.., clock, ..)` uses `clock.now_s()`, which equals `wall_now()`, and `clock.now()`. The courier's start time is `clock.now()`. The channel is a clock channel, and the `Stop` and `Wake` signals are its items.
   - **API idle:** the `GalaxyHolder` stamps and the `ForDir` stamps read the remote-data client's clock, which is `SystemClock` for the real client.
   - **Actions:** `authorize`'s `now` is `policy.clock().now()`.
7. **Pacing waits.** Each has an `#[allow(clippy::disallowed_methods)]` and a reason. The attribute goes on the statement or on the function:
   - policy's modifier gap and key hold, and the paste settle;
   - `read_status_retry`, the clipboard retry, and both `enable_wal` retries (data cache.rs and store lib.rs);
   - the voice client's `recv_timeout`, its 200 ms sleep and its exit wait;
   - `Recorder::wait_after`, the XInput `start`, and `controls::start`;
   - the update helper's `wait_pid` and `apply_cli`;
   - main.rs's `quit_running`, `run_console` and the Windows `listen`.
8. **One test per behaviour kind,** each with no sleep and no wall-clock read. The only real-time waits are 30 s hang guards, which decide nothing. Each test failed without its change:
   - `pipeline::tests::journal_freshness_is_on_the_clock` fails with `Utc::now()` in `line_is_fresh`.
   - `hub::tests::a_verify_wait_returns_on_the_flag_and_times_out_on_the_clock` fails with the old `Instant` and `cv.wait_timeout`.
   - recorded.rs `spansh_polls_on_the_clock` fails with `thread::sleep`: 3 requests where 2 were expected.
   - `pipeline::tests::a_callout_cooldown_runs_on_the_clock` fails with `Instant::now()` in `engine.journal`.
9. **`cargo test --workspace` on Linux and on Windows CI: Linux only so far.** This line stays unticked: Windows CI and the review are still to run.

**Checks run.**

All were run after the last code change unless noted.

- `cargo test --workspace --no-fail-fast` exited 0: 135 test binaries all ok, 1,323 tests passed (the baseline before this work was 1,312).
- `cargo clippy --workspace --all-targets --locked -- -D warnings` is clean, and `cargo fmt --all --check` passes.
- `python3 scripts/check-deps.py` reported "dependency rules OK (32 workspace crates)". `cargo deny check` passed advisories, bans, licences and sources.
- These ran 30 times in a loop, with the binaries built from the same code as now (the last commit only moved doc text):
  - the proto clock tests;
  - the core lib's `pipeline::`, `hub::`, `callouts::`, `history::`, `app::` and `world::` tests;
  - the data `spansh` tests and the eddn `worker` tests;
  - `world_mode`.

  Every iteration passed except one `world_mode` failure. That is the existing flake in open item 1: across 250 runs each, the old code and this code both failed it twice, with the same message.
- **Live run of the installed app's path.** Both the build at 72f82dc0 and this one ran a normal start under a temp HOME, in `env -i HOME=…`. Each run:
  - had a config naming the journal and Bindings folders, with update checks and EDDN off;
  - ran a fake `EDLaunch.exe` (a copy of bash), which started a session: binds check, pipeline, callout speaker, EDDN and EDSM workers;
  - killed the launcher (`going idle`), then ran `--quit`.

  After removing timestamps, pids and ports, the two logs are identical, and so are the files left in HOME. Both exited 0.

**Deviations.**

- The condvar waits take `&Arc<Mutex<T>>` as well as `&Arc<Condvar>`, and there is a `notify_all(m, cv)`. Decision note.
- The hub keeps its 25 ms slices. Decision note.
- Constructors keep their `SystemClock` defaults. The pipeline, the callout speaker and the command layer take their clock from the hub, the `Pending` queue and the remote-data client. Decision note.
- World mode already runs on a `ManualClock`. Decision note.
- The `notify` `Background` thread is unregistered. Its gap reads the clock, and its `std` `recv()` is unchanged.
- The verify thread's `t0.elapsed()` (the "confirmed in N ms" log line) and the other reads not listed here are left for astromech-zgoo.
- Finding astromech-90pi is fixed: ARCHITECTURE's Assembly line now lists the app window, and the clock too.

**Open items.**

1. **An existing flake, not caused by this change.** `world_mode`'s `WorldCore::quit` sometimes fails with "the core closed the connection before answering quit". The `quit` command sends `Ui::Quit`, and the process can exit before the control server writes the reply. Measured over 250 runs: 2 failures before this change and 2 after.

   Separately, `a_killed_test_process_ends_its_world_core` failed once in 330 runs of this code, with "couldn't remove /tmp/astromech-world-…: Directory not empty". In 330 runs of the old code it never failed. One failure isn't enough to tell whether it is new; it looks like a writer racing the temp folder's removal. Both are for the orchestrator to triage.
2. **Windows.** None of the Windows-only edits were compiled: the `#[allow]` attributes in xinput.rs, `controls::start`, clipboard.rs, update.rs's `wait_pid` and main.rs's `listen`. No changed signature is called from `cfg(windows)` code. The proof is the PR's Windows CI (test and clippy on windows-latest), which hasn't run.
3. **The history follower's settle sleep is not woken by a stop.** Under `SystemClock` that keeps today's 2 s behaviour. Under the manual clock, a follower re-pointed during its settle ends at the next `advance`. Meanwhile the new follower's `join` keeps it busy, so `advance` still settles and nothing hangs. Making a stop wake the settle would change `SystemClock` behaviour, which isn't mine to decide. For i1ev or Sumeet.
4. **Review.** The review should read every converted in-game site (policy, hub, callouts, pipeline, health) against the equivalences above, and the clock's mechanics against Done when 2 and 3.
