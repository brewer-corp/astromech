---
created_at: "2026-11-07T11:57:35.952046418+00:00"
id: "astromech-lkwj"
issue_type: "decision"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "After a crash restart: do the kill switch and Pause voice come back as they were, and does End task leave astromech off?"
updated_at: "2026-11-07T11:57:35.952046418+00:00"
---

## Question

The one app comes back after a crash through Windows' restart registration (astromech-dl53). Today a crashed core comes back only while the window is open, through the shell's watchdog (app/src-tauri/src/window.rs:57-79). With the window closed, as in most play, it stays off. Two things about that restart change what astromech does while playing, so they are Sumeet's:

1. **State after a crash restart.** Whether the kill switch is on is held only in memory, in the session runner's policy. Re-arming takes holding the kill switch for `rearm_hold_ms`, 1000 ms by default (crates/astromech-core/src/session.rs:641-648, config.rs:150). Pause voice is the in-memory `VOICE_PAUSED` (session.rs:27). So a crash while either is on comes back armed, with voice listening.
2. **Ended from outside.** Restart registration never restarts a process ended from outside (End task, `taskkill /F`). Today the shell's watchdog restarts the core after that, when the window is open.

## Options

For 1, the state after a crash restart:
- **a. Restore within the same Elite run.**
  - If the kill switch or Pause voice was on, it comes back on after a crash restart, but only into a session for the same Elite run, the same game process.
  - The kill state is already per session (session.rs:303), and a new Elite run starts armed, as now. A session that ends clears the saved state.
  - A fresh start, a Quit or an update relaunch comes back armed and unpaused, as now.
  - Cost: a small state file in the app folder (kill switch, pause, the Elite run's id), a restore path in session start, and the game watcher recording the game's process id.
- **b. Come back armed, silently.** Every start comes back armed and unpaused, as a fresh start does now. If he had hit the kill switch, astromech comes back able to press keys, with voice listening for push-to-talk. The only sign is the tray's crash line, which he won't see in VR or full screen.
- **c. Come back armed, and say so.** As b, but when the restarted session starts it says "astromech restarted" out loud, so he knows the kill switch is off again. Cost: one spoken line on a `--restarted` start, and no state file.

For 2, ended from outside. Today, with the window closed, End task or `taskkill` already leaves astromech off, because the shell's watchdog only runs while the window is open. With the window open, the watchdog restarts the core.
- **a. It stays off in every case.** A user's End task means stop. This changes only the window-open case, and needs no code: restart registration never restarts a process ended from outside.
- **b. It comes back.** Restart registration can't do this. The strategy's Shape allows a minimal watchdog as a fallback, but success criterion one-process (one app process plus its speech worker) would then have to change, which is also Sumeet's call.

**Recommendation:** 1a and 2a. A crash shouldn't silently undo a kill switch the player pressed mid-game, and a deliberate End task should be respected. If 1a's state file seems too much, 1c is the cheaper safe choice. 1b is not recommended.
