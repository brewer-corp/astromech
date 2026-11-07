---
created_at: "2026-11-07T11:57:35.951761517+00:00"
id: "astromech-dl53"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-lkwj"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-v3u4"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Crash restart for the one app through Windows restart registration"
updated_at: "2026-11-07T11:57:35.951761517+00:00"
---

## Goal

When the one app crashes, it comes back tray-only, with no shell and no watchdog process. It uses Windows' restart registration (strategy Shape). Today a crashed core is restarted only while the window is open, by the shell's watchdog (app/src-tauri/src/window.rs:57-79). With the window closed, as in most play, a crash leaves astromech off.

**In-game behaviour:** after a crash in play, astromech now comes back by itself: voice reloads because Elite is running, and the tray offers Save diagnostics. What it restores, and what an End task does, are Sumeet's (decision astromech-lkwj, which blocks this child).

## Done when

- [ ] Once its tray is up, the installed app calls `RegisterApplicationRestart("--background --restarted", RESTART_NO_PATCH | RESTART_NO_REBOOT)`. Console mode, portable mode and the `--apply-update` helper don't register. A Windows CI unit test reads the registration back with `GetApplicationRestartSettings` for the current process.
- [ ] **Uncaught panics on the main thread** go through Windows Error Reporting (WER), if the spike showed that a `main_thread_panic` exits with code 101 and isn't restarted. The panic hook (crates/astromech-core/src/crash.rs:16-37) calls `std::process::abort()` after writing the marker, and only for a panic on the main thread, which runs Tauri's event loop. Panics on other threads are unchanged. The `catch_unwind` recoveries keep working:
  - the command layer (`panic_is_caught`, no marker);
  - controls.rs:254 (the input thread);
  - callouts.rs:643;
  - astromech-eddn outbox.rs:554.

  A test panics inside one of them and shows the process still runs.
- [ ] A start with `--restarted` and no crash marker writes one ("astromech stopped unexpectedly and was restarted"). The tray's existing crash line (tray.rs:91, 105-107) then offers diagnostics after a native crash too.
- [ ] The kill switch and Pause voice after a restart follow decision astromech-lkwj. Under its recommended option, 1a:
  - The kill state lives in each session's `Policy` (session.rs:303), and `Core::kill_switch` does nothing without a session (app.rs:342-353). So a fresh Elite run already starts armed.
  - A small state file holds the kill switch, `VOICE_PAUSED` (session.rs:27) and the Elite run it belongs to: the game process's id. The game watcher reads only names today (crates/astromech-core/src/gamewatch.rs:87-120, `process_names`), so it also records `th32ProcessID` for `EliteDangerous64.exe`.
  - The file is written when either changes and cleared when a session ends.
  - On a `--restarted` start, it is restored only into the first session, and only if that session is for the same Elite run.

  Tests cover each case:
  - a `--restarted` start in the same Elite run restores the state;
  - a different Elite run comes back armed;
  - a session that ended clears the file;
  - a plain start comes back armed and unpaused.

  Under 1c, the restarted session says "astromech restarted" when it starts, and the state file isn't needed.
- [ ] The test hook. A `local` command `crash` (kinds `abort` and `access_violation`) is registered only when `ASTROMECH_CRASH_TEST=1` is set at start. A test shows it is an unknown command otherwise. The variable isn't assumed to survive a WER restart, so each crash kind is a fresh process started with it.
- [ ] A crash-restart job on Windows CI. It reuses the spike's `restart` job, now run against the installed app: a step in the Release workflow after the install smoke, or its own job that needs the installer artifact.
  - **Checks:** for each crash kind, a fresh process is started with `ASTROMECH_CRASH_TEST=1 astromech-core.exe --background`. After 65 s of uptime it is sent `crash abort` or `crash access_violation`. Within 90 s one new `astromech-core.exe` with `--restarted` must appear. It answers `status` with no window (`ui_state`), and exactly one app process is running. That process is ended with `--quit` before the next kind.
  - **The restored state:** one run keeps the mock game (crates/astromech-mockgame, copied as `EliteDangerous64.exe` as scripts/onboarding_smoke.py does) running across the crash.
    1. The run waits for the session (`status.active`).
    2. It turns the kill switch on with the `kill_switch` command and checks `status.killed`.
    3. It crashes the app, waits for the restarted process, and waits for its new session (`status.active`) before it reads `status.killed`.

    It must show the state the decision chose.
  - **Stays off:** after TerminateProcess (`taskkill /F`) the app stays off for 90 s, per the decision.
  - **Records, not asserted:** that a crash before 60 s isn't restarted, which is Windows' own rule.
  - **If the spike found that WER on the runner restarts nothing:** the job asserts only the registration, and this line moves to an after-merge check. "After merge (needs-human: Sumeet's PC): `python scripts/one_process_probe.py --restart` against his install shows both crash kinds restarted. Stand-in: the registration test."
- [ ] V1-DESIGN §7.7's crash restart paragraph and ARCHITECTURE §18.1 describe this.

## Scope

crates/astromech-core (crash.rs, app.rs startup, session.rs for the saved state, a Windows-only restart module), scripts/one_process_probe.py's restart mode, the workflow step.

**Stop rule:** if restart registration misses `abort` or access-violation crashes while WER is on, don't build a watchdog. Stop and ask Sumeet. The Shape allows a watchdog, but success criterion one-process forbids a third process.

## Context

- The spike (astromech-wgtr)'s `restart` job results.
- Windows only restarts a process that ran for at least 60 s and failed through WER. A process ended from outside isn't restarted.
- The speech worker exits when the app dies, because its stdin closes (crates/astromech-voice/src/bin/astromech-voice-worker.rs:196-199). So a restart leaves no orphan worker.
