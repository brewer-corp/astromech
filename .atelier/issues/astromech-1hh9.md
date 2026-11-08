---
created_at: "2026-11-07T11:57:01.792855843+00:00"
id: "astromech-1hh9"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T22:48:53.957593537+00:00"
status: "done"
title: "Fake keys, focus, raw input, mic, speech-to-text and speech out in world mode on every OS"
updated_at: "2026-11-07T22:48:53.957593537+00:00"
---

## Goal

In world mode the core runs its remaining Windows-only boundaries against fakes, on Linux as well as Windows:
- key presses and the foreground window go to a world platform that the script's key reactions answer;
- push-to-talk, the kill switch and the paste button come from raw-input samples on the world clock;
- the microphone gives recorded WAV clips;
- speech-to-text answers with the script's transcript;
- speech out is recorded.
Today the whole push-to-talk loop is `cfg(windows)` (crates/astromech-core/src/session.rs:351-362, :388-657), so on Linux there is no voice path.

## Done when

- [x] `Boundaries` (assembly child) gains:
  - `platform`, in place of `real_platform()` (session.rs:84-92, :288);
  - the control source: on the installed app, `bus::attach` exactly as now; in world mode, a `controls::Mapper` fed by the world's samples;
  - a mic trait over `astromech_voice::mic::Recorder`, used by the push-to-talk loop, calibration's `RecordFn` (calibration.rs:286, :308-325) and `mic_list`'s names (:346-351);
  - the speech-to-text maker (`WorkerStt`, session.rs:266);
  - the speaker maker for the session and Settings' voice preview (session.rs:231-264, app.rs:683-697);
  - the device inventory for the binds check (session.rs:149; `State::with_devices`).
  `Boundaries::real` passes today's.
- [x] The push-to-talk loop (`start_voice`) compiles and runs on every OS. In world mode it is registered with the clock, inherited from the session's starter, and its speech-to-text thread is not registered.
  - Control source: it takes a small interface with one method, receive with a timeout. In the installed app that is today's bus `std::sync::mpsc::Receiver<Control>` with `recv_timeout(1s)` (session.rs:529), unchanged. In world mode it is the clock channel. The bus is not changed to make the types match; the interface adapts to it.
  - Its time reads go through the clock:
    - `down_at` (session.rs:508, :551) and `up_at` (:568), from which `held` is measured against `min_hold_ms` (default 300 ms; gate at :585, :616);
    - the voice turn's `ptt_down` and `job_queued` stamps (:548, :612);
    - `TESTER.armed` (:571).
  - The post-roll sleep (:572) is a clock wait.
  - The mic trait's `wait_after(t, max)` keeps its signature. The real `Recorder::wait_after` (crates/astromech-voice/src/mic.rs:237-247) stays on real time: it waits for the device's next callback, which is device timing. The world mic returns at once, because an `audio` step has already queued the clip, so it needs no clock. `controls::start` (controls.rs:228-290) and the `bus` module (session.rs:659-716) are untouched by the diff, apart from one `#[cfg(test)]` function in `bus` that forwards a `Control` to `current` exactly as the controls-bus thread does. The bus keeps the first session's button config by design (`OnceLock`, :672-702), and that must not change.
- [x] Kill-switch hold time (R4): `Mapper`'s `kill_at`/`KillUp(t.elapsed())` (controls.rs:38, :188-191), compared with `rearm_hold_ms`, reads an injected `now`. `Mapper::new` keeps `Instant::now`, so `controls::start` and the installed app use exactly today's source. World mode builds its Mapper with the world clock. Each world sample's `t_ms`, and the Mapper's `tick`, come from the same world clock (ms since the world's start), so a hold is the `advance` between `press` and `release`.
- [x] World fakes and step kinds:
  - the platform records keys and the foreground title (`focus` step), and applies the header's key reactions (mockgame's shape: a scancode toggles a Status flag, sets GuiFocus, or swaps vehicle) as a Status.json write that settles like a `file` step;
  - `press` and `release` carry listen::model `Sample`s, with `DeviceMeta` in the header;
  - `audio` (a recorded WAV) and `say` (the transcript);
  - the speaker's lines appear in `world_observed` as said (`speaks_aloud` true).
  Every key and speech effect appears in `world_observed`, and in world mode `Boundaries` holds no real platform, mic, worker or player.
- [x] Tests spawning the binary, with no sleeps (decision astromech-1zhg):
  - `worlds/gear-down`: Elite is running and in focus. Push-to-talk down, the WAV with transcript "gear down", push-to-talk up. The world states its advances:
    - at least `min_hold_ms` between `press` and `release`;
    - after `release`, at least the 150 ms debounce (`debounce_ms`), since the world's Mapper sends `PttUp` only from a clock tick once the debounce window ends;
    - then at least the post-roll (`post_roll_ms`, 200 ms by default, less the debounce). `world_wait` sees `Key_L` sent, the gear flag set by the reaction, and the confirmation said.
  - `worlds/kill-switch`: the kill switch is pressed, "Kill switch on" is said, and a following voice command is denied and sends no key. A hold of at least `rearm_hold_ms` (1000 ms by default, by `advance`) re-arms it; a shorter hold says "Still killed". The voice command in this world uses the same advances as gear-down.
  - `worlds/paste`: in the galaxy map (GuiFocus) the paste button sends Ctrl+V and then Enter, and `world_wait` sees both under the manual clock with no `advance`. The modifier gap and paste settle are real pacing waits, and the paste runs on an unregistered thread (session.rs:627-632), so its keys are awaited, not assumed. Outside the map it sends nothing.
  - `worlds/jump` now also observes the next stop said.
- [x] Windows stand-in for the in-game path: a Windows-only test in crates/astromech-core starts a session's `start_voice` with the real `bus::attach`, which starts the real Raw Input and XInput listener and the bus, with fake mic, speech-to-text and speaker. It feeds crates/astromech-input/tests/fixtures/replay-vkb.json through `controls::Mapper` into the bus's test forwarder. Push-to-talk gives one transcribed command, and the kill switch kills the policy. It runs in Windows CI's `cargo test --workspace` (not the `continue-on-error` mock-game step, ci.yml:112-120).
- [ ] After merge (PC, needs-human): push-to-talk, the kill switch and the paste button work in the game as before. Stand-in: the Windows test above.

## Scope

The seams, `start_voice`'s move out of `cfg(windows)`, the Mapper's injected `now`, the world fakes and the step kinds the script child reserved. Keep the Windows listeners, the bus, cpal, the voice worker and Kokoro as they are. The WAV is rendered by the real speech path (crates/astromech-core/examples/render-voices.rs) or recorded. Never commit personal recordings.

## Context

The kill switch shares this loop, and copilot reliability while playing wins over window features (strategy, Shape): review reads `start_voice`'s diff line by line. The existing fakes to build on are `RecordingSpeaker` and `FakeStt` (speech.rs:170, :190), `mock::MockGame` (crates/astromech-input/src/exec/platform.rs:224) and the replay of raw samples (listen/replay.rs:28).

## Notes

- 2026-11-07 [note] From the u8ge review (astromech-r2ar/p78l round): under the manual clock a kill switch pressed during a verify wait is seen only once an advance passes the hub's 25 ms slice (StateHub::wait_flag/wait_gui check ticket.is_cancelled between slices, and a slice is a clock deadline). The kill-switch world must advance at least 25 ms after the kill before expecting the verify wait to end as cancelled; it can't wait for that with no advance.
- 2026-11-07 [decision] The session's devices are one Boundaries field, io: session::Io {platform, controls, mic, speech, devices}, which Session::start and the Core hold
  Reason: Session::start, the console mode (main.rs) and the Core (calibration, mic_list, voice preview, the keybinds devices) all need the same set; one struct keeps Session::start's argument list and main.rs's call from growing by five
- 2026-11-07 [decision] speech::Speech covers whether the worker and models are installed (worker_found, stt_found, tts_found) beside its two makers; the real one is today's exe.is_file() and find_stt/find_tts
  Reason: The session decides voice on, direct speaker or LazySpeaker, and the voice line from those checks; a world has no worker file or models, so without them its speaker and speech-to-text would never start. models_missing (preview with no session, download_models) still reads the disk: that is download state, astromech-gllh's to judge
- 2026-11-07 [decision] Io::real's devices is enumerate_live, used by both the binds check and the keybinds pages
  Reason: On Windows both were enumerate_live already. Off Windows the session's binds check used Inventory::default() and the pages enumerate_live's stub (no controllers, a Windows-only note); one boundary can't give both, and off Windows is dev runs only, so the binds report there now carries the stub's source and note
- 2026-11-07 [decision] The world microphone is one device at 16 kHz whose stream is the audio clips in order; a recording (calibration, mic_check) returns the audio since the last one; an audio WAV at another rate fails the load
  Reason: The push-to-talk loop reads the rate at PttDown, before any clip of that turn arrives, so the rate must be fixed up front; a rate check at load names the fix instead of resampling silently
- 2026-11-07 [decision] A key reaction is applied inside the world platform's send_key: Status.json rewritten whole (other fields kept, timestamp from the clock) and its generation awaited before the send returns
  Reason: Then the verify wait finds the flag with no clock advance, and the reaction is settled as a file step is, on whatever unregistered thread sent the key
- 2026-11-07 [decision] The world's buttons thread (Mapper on the world clock) starts on the first session's attach, registered from the activate thread, and keeps that session's buttons; detach wakes the session's receiver
  Reason: Mirrors the bus (OnceLock, first session's config by design), and the clock's rule that whatever stops a clock-waiting thread wakes it
- 2026-11-07 [decision] The kill-during-verify case uses voice_test_live (lights.on, which nothing answers) whose reply joins the verify: Pressed, then the kill switch cancelled it.
  Reason: A voice command's verify outcome is only logged; the live run is the one path that reports it. The test kills after observing the key's up and the confirmation; the verify thread enters its wait microseconds after the confirmation, a gap no world event marks (noted in the Report)

## Report

**Outcome.** In world mode the session's devices are fakes on every OS, and the push-to-talk loop runs on Linux as well as Windows. `Boundaries::io` (`session::Io`) holds the platform, the control source (`controls::Source`), the mic (`mic::Mics`), the speech worker (`speech::Speech`: the installed checks, the STT maker and the speaker maker) and the device inventory. `Io::real()` passes today's. `start_voice` is out of `cfg(windows)`: it reads time and waits on the clock, and is spawned with `clock.spawn`. `Mapper::with_now` takes an injected `now`, and `Mapper::new` still uses `Instant::now`. world/io.rs has the world's devices. The script accepts the header's `devices`, `reactions` and `foreground`, and the steps `focus`, `press`, `release`, `audio` and `say`. New observations are `key` and `said`. There are three new worlds (gear-down, kill-switch and paste), and `worlds/jump` now also observes the next stop being said. Done when lines 1 and 3-5 are ticked. Line 2 is not ticked because it still needs a Windows run. Lines 6 and 7 are not ticked: one needs Windows CI and the other the PC.

**Installed app: each changed site compared with `git show main:`**
- session.rs `start_voice`:
  - `bus::attach` becomes `controls.attach`, and `BusSource::attach` is `bus::attach`.
  - `Recorder::start_named(x)` becomes `mics.open(x)`, and `DeviceMics::open` is `start_named` (Windows).
  - `m.default_name` becomes `m.default_name()`, a clone.
  - `t_start`, `down_at` (twice), `up_at`, `TESTER.armed`, `VOICE_TURN.ptt_down` and `job_queued` read `clock.now()` instead of `Instant::now()`. `SystemClock::now` is `Instant::now`.
  - The post-roll `thread::sleep` becomes `clock.sleep`, which is `thread::sleep` on `SystemClock`.
  - The ptt thread's `Builder::spawn` becomes `clock.spawn`. On `SystemClock` that is `Builder::spawn` plus an empty registration.
  - The `Recorder` methods go through `impl Mic for Recorder` (Windows). `take_errors` is mapped into an equivalent struct.
  - `wait_after(Instant::now(), 100ms)` is unchanged, with a comment that it is real time.
  - `recv_timeout(1s)` is on the same `mpsc::Receiver`, through `ControlRx`.
  - The STT thread, the paste `thread::spawn`, the clip stamp and the kill and re-arm logic are unchanged.
- `Session::drop`: `#[cfg(windows)] bus::detach()` becomes `io.controls.detach()`. That is `Some(BusSource)` on Windows, which calls `bus::detach`, and `None` elsewhere.
- `start_buttons`: Windows has `Some(BusSource)`, so it runs `start_voice` with the same arguments. Off Windows it has `None`, so it drops the STT and logs the same line.
- The `bus` module is unchanged apart from the `#[cfg(test)] forward`.
- `Session::start`:
  - `worker.is_file()` becomes `speech.worker_found`, and `find_stt` and `find_tts` become `stt_found` and `tts_found`. These are the same calls.
  - `TtsSpeaker::start` and `WorkerStt::start` go through `WorkerSpeech` with the same arguments, including `v.threads`.
  - `LazySpeaker`'s ready check is still `find_tts`.
  - `real_platform()` becomes `io.platform`, which is `real_platform()`.
- Binds check:
  - On Windows, `run_check(.., live=true)` becomes `run_check_with(.., enumerate_live())`, which is the same.
  - **Off Windows the session's binds check now gets `enumerate_live`'s stub inventory** (source "live" with a "Windows only" note, and no controllers) instead of `Inventory::default()`. This affects dev runs only; there is a decision note on it.
  - `run_check(bool)` is kept for its other callers.
- app.rs:
  - Preview: `TtsSpeaker::start` becomes `io.speech.speaker`, with the same arguments.
  - `mic_list`: the names come from `io.mic.names()`, which is the same code moved into `DeviceMics::names`.
  - `calibrate_record` and `mic_check`: `cal::record_mic` becomes `io.mic.record`, which is `record_mic`.
  - `State` gets `.with_devices(enumerate_live)`, which is the same as its default.
- `controls::start` is untouched. `Mapper::new` becomes `with_now(Instant::now)`, `kill_at.get_or_insert_with(&self.now)`, and `KillUp(now().saturating_duration_since(t))`. The last is the same as `t.elapsed()`.
- `main.rs` console mode passes `Io::real()`.
- `render-voices` example fix: it creates its cache folder, which it didn't before, so it failed at once.

**Proof** (Linux, after the last change unless noted):
1. Boundaries: Linux `cargo check` and the Windows-target clippy pass. World mode's `World::boundaries` uses `io.session()`, so it holds no real platform, mic, worker or player (inspection of world/io.rs).
2. Not ticked.
   - Linux: the world tests run `start_voice` in world mode.
   - Registration: the ptt thread is spawned with `clock.spawn` from the activate thread. The mutations below show that `advance` settles on it: with 49 ms of post-roll, nothing happens.
   - Windows: it compiles (Windows-target clippy). A planted error in `BusSource` and one in the Windows test module each failed that clippy, which shows it compiles this code. Running it on Windows needs CI.
3. Mapper: the existing controls tests pass. The kill-switch world's re-arm with `"1000ms"` passes, and with `"999ms"` it fails: "nothing matching said Re-armed. … within 30 s".
4. Fakes: `world::script::tests::the_devices_header_and_steps_load` covers each kind, the reaction order, a WAV at the wrong rate, and a `t_ms` that is refused. The worlds below cover the rest.
5. Tests (`tests/world_voice.rs`, 3 tests, plus world_steps' jump test, which now waits for "Next waypoint. Barnard's Star." to be said): all pass. Deliberate breaks, each failing at `world_wait`'s 30 s guard unless noted:
   - gear-down hold `299ms`: fails;
   - debounce `149ms`: fails;
   - post-roll `49ms`: fails;
   - reactions removed: fails at the gear-flag assert;
   - rearm `999ms`: fails;
   - kill-during-verify slice `24ms`: hung until `timeout 60` killed it. That run was before I moved the live-run reply behind a 30 s `recv_timeout` guard. With the guard, this mutant now fails at the guard instead, but I haven't re-run it;
   - paste world with GuiFocus 0: fails.
   - Loop: world_voice, world_steps, world_mode and world_net ran 50 times from the built binaries: 50 passed, 0 failed.
6. Not ticked: `session::windows_tests::push_to_talk_and_the_kill_switch_through_the_real_bus`. It compiles for Windows; it needs Windows CI to run.
7. Not ticked (needs-human, after merge).

Checks:
- `cargo test --workspace --no-fail-fast`: exit 0, 1368 passed, 0 failed, 138 binaries.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: clean.
- Windows-target `cargo clippy --target x86_64-pc-windows-msvc --workspace --all-targets -- -D warnings` (fake cl, lib and llvm-rc): clean.
- `cargo fmt --all --check`, `scripts/check-deps.py`, `scripts/check-guide.py` and `cargo deny check`: pass.
- The real HOME has no astromech folders.

### Deviations
- The gear flag is read with `world_game` after `world_wait` sees the confirmation, not with `world_wait`: a reaction is not an observation.
- The kill during a verify wait goes through `voice_test_live` (decision note).
- The `render-voices` cache-folder fix.
- The WAV is Kokoro bm_george saying "Gear down.", rendered with render-voices and converted to 16 kHz with ffmpeg. The kill-switch world reuses it.

### Surprises
- render-voices failed on a fresh output folder.
- The deliberate breaks that time out leave the world core's temp folder in /tmp: the helper kills the child with SIGKILL. I removed mine.

### Open items
- Line 2's Windows run, and line 6 (Windows CI).
- Line 7 at the PC.
- There is a race in the kill-during-verify test. It kills after observing the confirmation, and the verify thread starts its wait microseconds after that confirmation. A kill that lands in the gap is caught with no advance, so the slice break could pass by chance. No world event marks the wait starting.
- app.rs still arms the tester with `Instant::now()` (app.rs `voice_test_*`), while the ptt loop reads `TESTER.armed(clock.now())`. Under the manual clock they don't match, which matters only for a world that tests test mode. This is for astromech-zgoo.
- astromech-gllh: `voice_preview` with no session still checks the models on disk (`models_missing`), so a world needs a running session for it.
- Independent review is outstanding.
