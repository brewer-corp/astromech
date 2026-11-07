---
created_at: "2026-11-07T11:57:35.950925980+00:00"
id: "astromech-7i2k"
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
    id: "astromech-wgtr"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "One Raw Input registration per process, shared by push-to-talk and the window's press listener"
updated_at: "2026-11-07T11:57:35.950925980+00:00"
---

## Goal

Within a process, astromech's input is one source that every listener subscribes to: push-to-talk and the kill switch (crates/astromech-core/src/controls.rs:238-250, started once through session.rs's `bus`, :656-706), and the window's press listener (app/src-tauri/src/window.rs:96-144).

Windows lets only one window per device class receive raw input in a process: the window passed to the last `RegisterRawInputDevices` call. Today the core and the window are separate processes and each registers its own (crates/astromech-input/src/listen/win/rawinput.rs:92-102). In one process, a second registration takes WM_INPUT away from the first. That includes tao's own keyboard and mouse registration, made when Tauri builds its event loop (tao-0.37.1 raw_input.rs:128-157, called at event_loop.rs:200; tauri-2.12.3 app.rs:2534).

This child lands the shared source while there are still two processes, so astromech-v3u4 only has to start it after `build`.

**In-game behaviour (input):** this changes how push-to-talk and the kill switch receive presses, and must leave what they do, and when they listen, unchanged.

## Done when

- [ ] `astromech-input::listen` has one per-process source of input. `subscribe()` returns a receiver of `Msg`, and every subscriber gets every Device and Sample message.
  - It starts Raw Input and XInput once, with one hidden window and one registration, when it is first subscribed.
  - Its lifetime is as today: once started it runs for the life of the process, as session.rs's `bus` keeps the controls listener now (a `OnceLock`; sessions attach and detach).
  - A subscriber that falls behind or is dropped never blocks the others.
  - It has a `restart()` that registers again. astromech-v3u4 calls it once after Tauri's `build`. On Windows it is called from tests only.
- [ ] `controls::start` (called by session.rs's `bus`), `--listen` (main.rs:423-461) and the shell's `start_input` subscribe instead of calling `rawinput::start` and `xinput::start` themselves. session.rs's `bus` keeps its attach and detach behaviour. The device list given to a new subscriber matches today's: `list_devices()` first, then the Device messages.
- [ ] Platform-neutral unit tests drive the fan-out through a fake source:
  - two subscribers each see the same samples in order;
  - dropping one leaves the other receiving;
  - the source starts once, on the first subscribe.
- [ ] A Windows-only `#[ignore]` test sends a key with SendInput and shows two subscribers both seeing it, before and after `restart()`. It runs with `cargo test -- --ignored` at a desktop session. Windows CI runs it best effort, like the gear-down end-to-end step, because SendInput needs a desktop session.
- [ ] **The kill switch, end to end.** scripts/onboarding_smoke.py, which must pass in the Release workflow, gains a kill-switch step. It runs after the session is active against the mock game, with the window open:
  - it taps the bound kill switch (F8) by SendInput, and `status` must show the kill switch on;
  - it holds F8 for 1.2 s, longer than `controls.rearm_hold_ms` (1000 ms by default; session.rs:641-648, config.rs:150), and `status` must show it re-armed;
  - then it quits the app with `--quit`, keeping the config, starts the mock game first, and starts the app again. The session's listener now starts at app start, before any window, as after a crash restart or an update relaunch while Elite runs. It opens the window by a second start with `--route settings` and repeats the tap and the hold.

  With two processes this proves the shared source carries the presses. astromech-v3u4 reruns it in one process.
- [ ] Checks: CONTRIBUTING's checks locally, and Windows CI's clippy, tests and shell build. The Release workflow's onboarding smoke passes on the PR, including the push-to-talk and kill-switch binding step, which is the window's listener seeing SendInput presses.

## Scope

crates/astromech-input (listen), crates/astromech-core (controls.rs, session.rs `bus`, main.rs `listen`), app/src-tauri/src/window.rs `start_input`, scripts/onboarding_smoke.py.

Nothing about what a press does changes: mapping, debounce, re-arm hold and tracker logic stay as they are. Both processes still exist after this child, each with its own single source. Don't change when the listener starts or stops: that would change input during play.

## Context

ARCHITECTURE §5.10a. The `Processor` and `Mapper` consumers stay as they are. Only where their `Msg` stream comes from changes. The spike (astromech-wgtr) records whether a listener registered before Tauri's build survives it (probe check 2).
