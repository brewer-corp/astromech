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
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-08T15:51:21.522469461+00:00"
status: "done"
title: "One Raw Input registration per process, shared by push-to-talk and the window's press listener"
updated_at: "2026-11-08T15:51:21.522469461+00:00"
---

## Goal

Within a process, astromech's input is one source that every listener subscribes to: push-to-talk and the kill switch (crates/astromech-core/src/controls.rs:238-250, started once through session.rs's `bus`, :656-706), and the window's press listener (app/src-tauri/src/window.rs:96-144).

Windows lets only one window per device class receive raw input in a process: the window passed to the last `RegisterRawInputDevices` call. Today the core and the window are separate processes and each registers its own (crates/astromech-input/src/listen/win/rawinput.rs:92-102). In one process, a second registration takes WM_INPUT away from the first. That includes tao's own keyboard and mouse registration, made when Tauri builds its event loop (tao-0.37.1 raw_input.rs:128-157, called at event_loop.rs:200; tauri-2.12.3 app.rs:2534).

This child lands the shared source while there are still two processes, so astromech-v3u4 only has to start it after `build`.

**In-game behaviour (input):** this changes how push-to-talk and the kill switch receive presses, and must leave what they do, and when they listen, unchanged.

## Done when

- [x] `astromech-input::listen` has one per-process source of input. `subscribe()` returns a receiver of `Msg`, and every subscriber gets every Device and Sample message.
  - It starts Raw Input and XInput once, with one hidden window and one registration, when it is first subscribed.
  - Its lifetime is as today: once started it runs for the life of the process, as session.rs's `bus` keeps the controls listener now (a `OnceLock`; sessions attach and detach).
  - A subscriber that falls behind or is dropped never blocks the others.
  - It has a `restart()` that registers again. astromech-v3u4 calls it once after Tauri's `build`. On Windows it is called from tests only.
- [x] `controls::start` (called by session.rs's `bus`), `--listen` (main.rs:423-461) and the shell's `start_input` subscribe instead of calling `rawinput::start` and `xinput::start` themselves. session.rs's `bus` keeps its attach and detach behaviour. The device list given to a new subscriber matches today's: `list_devices()` first, then the Device messages.
- [x] Platform-neutral unit tests drive the fan-out through a fake source:
  - two subscribers each see the same samples in order;
  - dropping one leaves the other receiving;
  - the source starts once, on the first subscribe.
- [x] A Windows-only `#[ignore]` test sends a key with SendInput and shows two subscribers both seeing it, before and after `restart()`. It runs with `cargo test -- --ignored` at a desktop session. Windows CI runs it best effort, like the gear-down end-to-end step, because SendInput needs a desktop session.
- [x] **The kill switch, end to end.** scripts/onboarding_smoke.py, which must pass in the Release workflow, gains a kill-switch step. It runs after the session is active against the mock game, with the window open:
  - it taps the bound kill switch (F8) by SendInput, and `status` must show the kill switch on;
  - it holds F8 for 1.2 s, longer than `controls.rearm_hold_ms` (1000 ms by default; session.rs:641-648, config.rs:150), and `status` must show it re-armed;
  - then it quits the app with `--quit`, keeping the config, starts the mock game first, and starts the app again. The session's listener now starts at app start, before any window, as after a crash restart or an update relaunch while Elite runs. It opens the window by a second start with `--route settings` and repeats the tap and the hold.

  With two processes this proves the shared source carries the presses. astromech-v3u4 reruns it in one process.
- [x] Checks: CONTRIBUTING's checks locally, and Windows CI's clippy, tests and shell build. The Release workflow's onboarding smoke passes on the PR, including the push-to-talk and kill-switch binding step, which is the window's listener seeing SendInput presses.

## Scope

crates/astromech-input (listen), crates/astromech-core (controls.rs, session.rs `bus`, main.rs `listen`), app/src-tauri/src/window.rs `start_input`, scripts/onboarding_smoke.py.

Nothing about what a press does changes: mapping, debounce, re-arm hold and tracker logic stay as they are. Both processes still exist after this child, each with its own single source. Don't change when the listener starts or stops: that would change input during play.

## Context

ARCHITECTURE §5.10a. The `Processor` and `Mapper` consumers stay as they are. Only where their `Msg` stream comes from changes. The spike (astromech-wgtr) records whether a listener registered before Tauri's build survives it (probe check 2).

## Notes

- 2026-11-07 [note] From spike astromech-wgtr (evidence astromech-t55z, astromech-mb4f): start the shared Raw Input source after Builder::build, never before. With astromech's listener started before build, it saw none of the keys sent at start, after the window opened, after it closed and after tray churn: tao registers keyboard and mouse to its own hidden window when it builds the loop, and the process keeps only the last registration per device class. Started after build, it saw every key at 8 points, including after each of 5 window closes and after 50 tray-menu rebuilds. Nothing re-registers when a window opens or closes; only Tauri's set_device_event_filter (run once inside build) would, so the app must not call it later.
- 2026-11-07 [decision] subscribe() returns a Subscription { rx: Receiver<Msg>, epoch, status }: the receiver plus the source's time base and its status line
  Reason: Every sample's t_ms counts from the source's start, and Mapper and Processor compare it with epoch.elapsed() (debounce, tick), so each subscriber must use the source's Instant, not its own; controls::start still returns the Raw Input/XInput status line, which only the first start knows
  Rejected: A bare Receiver<Msg> with separate epoch()/status() calls: they would start the source too, or depend on call order
- 2026-11-07 [decision] Unbounded per-subscriber channels, and the source remembers each device it has announced and replays a Device message for each to a new subscriber before live messages
  Reason: An unbounded send never waits, so a slow subscriber holds nobody up and none loses a key-up (a bounded channel would drop presses from a slow one, e.g. a kill-switch release). The replay keeps what a late subscriber knows the same as with its own listener: a stick's Device message names its Elite ID, and the window joining after the session's listener (astromech-v3u4) would otherwise never see it again
  Rejected: Bounded channels with try_send (drops input)
- 2026-11-07 [decision] rawinput::start keeps its signature and stays public; the source uses a new rawinput::start_sink that also returns the hidden window (Sink) for restart. restart() sends the window WM_APP+1 so it registers again on its own thread
  Reason: spikes/o1-one-process (excluded from the workspace, built by one-process.yml) calls rawinput::start to probe registrations directly; SendMessageW returns the registration's result synchronously

## Report

The process's one input source is `astromech_input::listen::source` (crates/astromech-input/src/listen/source.rs): a platform-neutral `Hub` over a `Backend` (Raw Input plus XInput on Windows, a fake in tests), with a fan-out thread that sends every Device and Sample message to each subscriber's own channel. On Windows the free functions `subscribe()` and `restart()` use one `OnceLock` hub. `controls::start` (through session.rs's `bus`, unchanged), `--listen` and the shell's `start_input` subscribe; each still calls `list_devices()` first. The kill-switch step is in scripts/onboarding_smoke.py (`kill_switch`, `restart_with_game_running`), and the Windows-only `#[ignore]` test is crates/astromech-input/tests/source_sendinput.rs, run best effort by a new step in ci.yml's Windows job. On PR #218 at b418f604 every check passed (evidence astromech-aypb): the ignored test passed in the Windows job's best-effort step, and the Release workflow's onboarding smoke passed both kill-switch runs (session started after the app; game running before the app) and the binding step.

### Deviations

- `subscribe()` returns a `Subscription` whose `rx` is the receiver, with the source's `epoch` and `status` beside it (decision note).
- The `#[ignore]` test also takes the keyboard away with a second registration in the test process (as tao's does) and asserts the key is lost before `restart()`, so the after-restart check shows `restart()` re-registering, not a registration that never moved.
- In the smoke's second run the script closes the first window before `--quit`, so the window's watchdog can't start the core again, and waits up to 30 s for core.json to go rather than trusting `--quit`'s 5 s exit code. It checks the window opened by the second start as the astromech-shell.exe process running.

### Decisions

See the three decision notes of 2026-11-07: the `Subscription` shape and shared time base; unbounded channels with device replay for late subscribers; `rawinput::start` kept for the o1 spike, with `start_sink` and a WM_APP message for `restart()`.

### Surprises

Each listener used its own `Instant` as the samples' time base, which Mapper's debounce and tick compare with; shared samples need the source's one epoch, so subscribers take it from the subscription.

### Open items

- Two processes still: astromech-v3u4 calls `restart()` after Tauri's `build` and reruns the kill-switch smoke in one process. The smoke's second run checks the window as the astromech-shell.exe process running, which v3u4 has to change.
- The fan-out adds one channel hop between the Raw Input thread and the controls thread (microseconds, not measured). The XInput poller no longer stops when its only listener's thread ends (the hub keeps its channel open); it never did in practice, since the controls thread runs for the process.
