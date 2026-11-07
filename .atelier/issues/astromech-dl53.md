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
    id: "astromech-fmub"
    type: "related"
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
updated_at: "2026-11-07T14:07:01.291052660+00:00"
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
- [ ] A crash-restart job on Windows CI asserts what the runner can show. It reuses the spike's `restart` job, now run against the installed app: a step in the Release workflow after the install smoke, or its own job that needs the installer artifact. The spike showed that WER never engages on the runner, so CI can't prove a restart (evidence astromech-fqw9: no restart, WER report or WerFault in 15 crashes across three WER configurations).
  - **Registration:** the installed app's own process reads back `--background --restarted` and the flags with `GetApplicationRestartSettings` (the unit test above, run against the installed build).
  - **The restarted start, faked:** the job starts `astromech-core.exe --background --restarted` itself, as WER would. Without a marker it writes one, the tray's crash line is there, it answers `status` with no window, and exactly one app process runs.
  - **The restored state, faked the same way:** with the mock game (crates/astromech-mockgame, copied as `EliteDangerous64.exe` as scripts/onboarding_smoke.py does) running, the job waits for the session (`status.active`), turns the kill switch on (`kill_switch`, `status.killed`), ends the app with TerminateProcess, starts it with `--restarted` in the same Elite run, waits for the new session and reads `status.killed`. It must show the state decision astromech-lkwj chose.
  - **Records, not asserted:** each crash kind (`abort`, `access_violation`) after 65 s, and whether a `--restarted` process appeared within 90 s, so a runner where WER works shows up.
- [ ] After merge (needs-human: Sumeet's PC): on his install, each crash kind (`abort`, `access_violation`) after 65 s of uptime comes back within 90 s as one `astromech-core.exe --background --restarted`, tray-only; a TerminateProcess (`taskkill /F`) stays off or comes back as decision astromech-lkwj says; and a crash before 60 s isn't restarted (recorded). Run as scripts/one_process_probe.py's `--restart` mode does, with `ASTROMECH_CRASH_TEST=1`. Stand-in: the CI job above (registration read back, and the `--restarted` start handled). If Windows restarts neither kind there with WER on, the stop rule applies: the watchdog question goes to Sumeet.
- [ ] V1-DESIGN §7.7's crash restart paragraph and ARCHITECTURE §18.1 describe this.

## Scope

crates/astromech-core (crash.rs, app.rs startup, session.rs for the saved state, a Windows-only restart module), scripts/one_process_probe.py's restart mode, the workflow step.

**Stop rule:** if restart registration misses `abort` or access-violation crashes while WER is on, don't build a watchdog. Stop and ask Sumeet. The Shape allows a watchdog, but success criterion one-process forbids a third process.

## Context

- The spike (astromech-wgtr)'s `restart` job results.
- Windows only restarts a process that ran for at least 60 s and failed through WER. A process ended from outside isn't restarted.
- The speech worker exits when the app dies, because its stdin closes (crates/astromech-voice/src/bin/astromech-voice-worker.rs:196-199). So a restart leaves no orphan worker.

## Notes

- 2026-11-07 [note] From spike astromech-wgtr (evidence astromech-fqw9): restart registration couldn't be shown on Windows CI. RegisterApplicationRestart returned 0, but no crash was restarted in 15 cases (abort, access_violation and main_thread_panic after 65 s, TerminateProcess, abort after 10 s) across three configurations: the runner's WER (WerSvc stopped, DontShowUI=1), WER on, and WER on with the app launched with the default error mode. The runner hands its children error mode 0x3 (SEM_NOGPFAULTERRORBOX), and it refuses CREATE_BREAKAWAY_FROM_JOB. Even with error mode 0, no WER report, WerFault or dialog appeared, so WER never engaged there. Its CI job therefore can't prove a restart: Done when now has the job assert registration and a faked --restarted start, and the real restart moves to an after-merge needs-human check at Sumeet's PC. If Windows restarts neither abort nor access_violation there with WER on, the watchdog question goes to Sumeet (stop rule). Also, main_thread_panic exits 0x65 with no WER, so the main-thread abort line applies (its condition is met).
- 2026-11-07 [note] From finding astromech-1vi3 (wgtr review): word the runner result as 'no restart, WER report or WerFault was observed' (WerSvc went from stopped to running during the runner-WER configuration, so 'WER never engages' overstates it), and have the records step log AeDebug, LocalDumps and the WerSvc state before and after each crash.
