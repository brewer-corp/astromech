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
  relates:
  - kind: "issue"
    id: "astromech-i1ev"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Fake keys, focus, raw input, mic, speech-to-text and speech out in world mode on every OS"
updated_at: "2026-11-07T14:53:28.338450333+00:00"
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

- [ ] `Boundaries` (assembly child) gains:
  - `platform`, in place of `real_platform()` (session.rs:84-92, :288);
  - the control source: on the installed app, `bus::attach` exactly as now; in world mode, a `controls::Mapper` fed by the world's samples;
  - a mic trait over `astromech_voice::mic::Recorder`, used by the push-to-talk loop, calibration's `RecordFn` (calibration.rs:286, :308-325) and `mic_list`'s names (:346-351);
  - the speech-to-text maker (`WorkerStt`, session.rs:266);
  - the speaker maker for the session and Settings' voice preview (session.rs:231-264, app.rs:683-697);
  - the device inventory for the binds check (session.rs:149; `State::with_devices`).
  `Boundaries::real` passes today's.
- [ ] The push-to-talk loop (`start_voice`) compiles and runs on every OS. In world mode it is registered with the clock, inherited from the session's starter, and its speech-to-text thread is not registered.
  - Control source: it takes a small interface with one method, receive with a timeout. In the installed app that is today's bus `std::sync::mpsc::Receiver<Control>` with `recv_timeout(1s)` (session.rs:529), unchanged. In world mode it is the clock channel. The bus is not changed to make the types match; the interface adapts to it.
  - Its time reads go through the clock:
    - `down_at` (session.rs:508, :551) and `up_at` (:568), from which `held` is measured against `min_hold_ms` (default 300 ms; gate at :585, :616);
    - the voice turn's `ptt_down` and `job_queued` stamps (:548, :612);
    - `TESTER.armed` (:571).
  - The post-roll sleep (:572) is a clock wait.
  - The mic trait's `wait_after(t, max)` keeps its signature. The real `Recorder::wait_after` (crates/astromech-voice/src/mic.rs:237-247) stays on real time: it waits for the device's next callback, which is device timing. The world mic returns at once, because an `audio` step has already queued the clip, so it needs no clock. `controls::start` (controls.rs:228-290) and the `bus` module (session.rs:659-716) are untouched by the diff, apart from one `#[cfg(test)]` function in `bus` that forwards a `Control` to `current` exactly as the controls-bus thread does. The bus keeps the first session's button config by design (`OnceLock`, :672-702), and that must not change.
- [ ] Kill-switch hold time (R4): `Mapper`'s `kill_at`/`KillUp(t.elapsed())` (controls.rs:38, :188-191), compared with `rearm_hold_ms`, reads an injected `now`. `Mapper::new` keeps `Instant::now`, so `controls::start` and the installed app use exactly today's source. World mode builds its Mapper with the world clock. Each world sample's `t_ms`, and the Mapper's `tick`, come from the same world clock (ms since the world's start), so a hold is the `advance` between `press` and `release`.
- [ ] World fakes and step kinds:
  - the platform records keys and the foreground title (`focus` step), and applies the header's key reactions (mockgame's shape: a scancode toggles a Status flag, sets GuiFocus, or swaps vehicle) as a Status.json write that settles like a `file` step;
  - `press` and `release` carry listen::model `Sample`s, with `DeviceMeta` in the header;
  - `audio` (a recorded WAV) and `say` (the transcript);
  - the speaker's lines appear in `world_observed` as said (`speaks_aloud` true).
  Every key and speech effect appears in `world_observed`, and in world mode `Boundaries` holds no real platform, mic, worker or player.
- [ ] Tests spawning the binary, with no sleeps (decision astromech-1zhg):
  - `worlds/gear-down`: Elite is running and in focus. Push-to-talk down, the WAV with transcript "gear down", push-to-talk up. The world states its advances:
    - at least `min_hold_ms` between `press` and `release`;
    - after `release`, at least the 150 ms debounce (`debounce_ms`), since the world's Mapper sends `PttUp` only from a clock tick once the debounce window ends;
    - then at least the post-roll (`post_roll_ms`, 200 ms by default, less the debounce). `world_wait` sees `Key_L` sent, the gear flag set by the reaction, and the confirmation said.
  - `worlds/kill-switch`: the kill switch is pressed, "Kill switch on" is said, and a following voice command is denied and sends no key. A hold of at least `rearm_hold_ms` (1000 ms by default, by `advance`) re-arms it; a shorter hold says "Still killed". The voice command in this world uses the same advances as gear-down.
  - `worlds/paste`: in the galaxy map (GuiFocus) the paste button sends Ctrl+V and then Enter, and `world_wait` sees both under the manual clock with no `advance`. The modifier gap and paste settle are real pacing waits, and the paste runs on an unregistered thread (session.rs:627-632), so its keys are awaited, not assumed. Outside the map it sends nothing.
  - `worlds/jump` now also observes the next stop said.
- [ ] Windows stand-in for the in-game path: a Windows-only test in crates/astromech-core starts a session's `start_voice` with the real `bus::attach`, which starts the real Raw Input and XInput listener and the bus, with fake mic, speech-to-text and speaker. It feeds crates/astromech-input/tests/fixtures/replay-vkb.json through `controls::Mapper` into the bus's test forwarder. Push-to-talk gives one transcribed command, and the kill switch kills the policy. It runs in Windows CI's `cargo test --workspace` (not the `continue-on-error` mock-game step, ci.yml:112-120).
- [ ] After merge (PC, needs-human): push-to-talk, the kill switch and the paste button work in the game as before. Stand-in: the Windows test above.

## Scope

The seams, `start_voice`'s move out of `cfg(windows)`, the Mapper's injected `now`, the world fakes and the step kinds the script child reserved. Keep the Windows listeners, the bus, cpal, the voice worker and Kokoro as they are. The WAV is rendered by the real speech path (crates/astromech-core/examples/render-voices.rs) or recorded. Never commit personal recordings.

## Context

The kill switch shares this loop, and copilot reliability while playing wins over window features (strategy, Shape): review reads `start_voice`'s diff line by line. The existing fakes to build on are `RecordingSpeaker` and `FakeStt` (speech.rs:170, :190), `mock::MockGame` (crates/astromech-input/src/exec/platform.rs:224) and the replay of raw samples (listen/replay.rs:28).

## Notes

- 2026-11-07 [note] From the u8ge review (astromech-r2ar/p78l round): under the manual clock a kill switch pressed during a verify wait is seen only once an advance passes the hub's 25 ms slice (StateHub::wait_flag/wait_gui check ticket.is_cancelled between slices, and a slice is a clock deadline). The kill-switch world must advance at least 25 ms after the kill before expecting the verify wait to end as cancelled; it can't wait for that with no advance.
