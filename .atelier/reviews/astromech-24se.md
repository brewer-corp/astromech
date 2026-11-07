---
schema: atelier.review
schema_version: 2
id: astromech-24se
title: review on astromech-y6d6 @7441499d
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 7441499d59915f8285815090520428ce34af0146
created_at: 2026-11-07T20:42:17.971835899Z
event:
  event: review
---

Reviewed a7edc42e and 7441499d (and records 8fa5f13a..0609f29e, not re-reviewed beyond what's named). host.rs: TauriUi::open_window now always spawns a 'window-open' thread and answers at once. Host::open_window holds the 'opening' mutex across the build; it is entered only from window-open threads and rebuild_window's thread, never the main thread, so a build blocked on the event loop can't deadlock. Concurrent opens serialize on 'opening', and the second finds the window, focuses it and emits its route (no double build). A rebuild racing a tray or second-start open still builds one window, and the later caller focuses it. Opens racing a WM_CLOSE may focus a dying window and lose that route; that's as on main, where a second shell met a closing one. Focus: no AllowSetForegroundWindow on main or here; on main a child shell of the core built the window, so foreground rights are equivalent. Behaviour vs main: main also answered open_window immediately (it spawned the shell); the in-process blocking build was this branch's own; failures are now only logged in the app ('couldn't open the app window'), not returned to a second start, which logged and exited 0 either way. The log line reads 'opening the app window'. Waits: the probe's second_start polls until the page load and fails only past --page-load-s (it reports up to max(20, 2x)); the onboarding smoke (Page(CDP_PORT, 60), 30 s ui_state loops) and webview_cost.py's wait_load are existing poll-until bounds, not sleeps: consistent with 1zhg. 30 s for the Release probe is reasonable: it is a liveness bound with 13 s observed worst case and 0.65 s typical (ahj1); the 2 s budget is gated by App window cost. one-process.yml now on spikes/** only: nothing else depends on it running for probe changes except restart mode (finding above). Probe parses. Not run: host.rs is window-feature Windows code, read only; CI on 7441499d59915f8285815090520428ce34af0146 not seen yet.
