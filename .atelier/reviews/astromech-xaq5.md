---
schema: atelier.review
schema_version: 2
id: astromech-xaq5
title: review on astromech-y6d6 @26afef19
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 26afef19d845b864358994d291ec64c98ed410ca
created_at: 2026-11-07T17:57:43.993059201Z
event:
  event: review
---

Reviewed v3u4 (7b88ee7b, 26afef19) against main for in-game behaviour: tray items, texts, conditional lines, left-click and dark menus (tray.rs vs main's); menu and tray events arrive on the main thread through Tauri's proxy (tauri-2.12.3 app.rs:2520-2531); kill switch, PTT and the input source after Builder::build (tao registers Raw Input only in build and set_device_event_filter, which Tauri calls inside build; nothing subscribes before boot); voice; --background builds no window; close prevents ExitRequested{code:None}; second start and --route via begin before Tauri; quit through request_exit's proxy, shutdown then core.json then mutex in RunEvent::Exit (same order as main); main-thread abort in crash.rs; ProcessFailed handler; in-process call (origin app_window, local refused, spawn_blocking). Looked for deadlocks: no Tauri call under a Core lock, no blocking main-thread round trip from a lock holder, window builds off the main thread. Judged the 7 decision notes and reported deviations acceptable (tray Install/Diagnostics threads keep each item's effect and install_update/apply_on_quit are guarded by UpStatus::Applying). Ran: cargo test -p astromech-core --lib tray:: (5/5), cargo test -p astromech-api --lib control:: (7/7), cargo clippy -p astromech-app --all-targets -D warnings (headless) clean, a live headless run (temp HOME: --background, ui_state, ui_state refused as app_window, open_window error, second start --route exits 0, --quit exits 0 and removes core.json). Limits: the window feature and Windows code read only (not built); Tauri's NSIS template order (hook before its running-app check) not verified locally; no CI has run on 26afef19 yet (gh pr checks 218: none; latest runs are on b418f604), so the Release smoke, kill-switch runs, probe and App window cost are unseen. No approval: the item still has 7 unticked Done when lines (atelier review request refuses) and two fix_now findings are open.
