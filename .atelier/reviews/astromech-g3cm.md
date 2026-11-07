---
schema: atelier.review
schema_version: 2
id: astromech-g3cm
title: approval on astromech-vucf @a4f2b03e
target: astromech-vucf
identity: claude-reviewer-7149
commit: a4f2b03eeefddbbf2a2ec862d58ed06300f0da90
created_at: 2026-11-07T12:59:09.563199509Z
event:
  event: approval
---

astromech-7149 (05ffa01e, bf21e8b3, a4f2b03e). Read run's diff against Boundaries::real line by line: same logging/crash/instance-lock/start-up order, same boundaries (gamewatch::process_names, explorer/xdg-open/notepad, clipboard::set_text, WindowsNotifier wrapped per session in Background as before, OsKeyring, Run key, remote_client on the configured data_dir), control API, retired-model check, history under config_lock, gamewatch, idle reaper, update thread, window, tray; _started keeps the server alive until quit. No Paths::current() left in the core after startup (only run's first line and main.rs's --quit/--apply-update/console paths). assemble_with is pub(crate), so the astromech-core binary can't reach it; assemble passes the identity. Fixture core-replies.json: two values change (app_dir under the test boundaries' LOCALAPPDATA, voice 'voice not loaded' as run sets it), no key added or removed, and the test compares whole Values; bindings test passes. Judged the four reported differences: autostart before config load is log order only (apply_default reads neither config); Paths once only affects a portable astromech.toml added mid-run, which before split the folders (remote cache already fixed); switches in start are set before the control API serves, as before; none changes play. Fresh on Linux: cargo test --workspace 134 binaries 1302 passed 0 failed; clippy --workspace --all-targets -D warnings, fmt, check-deps, check-guide clean. Live old (main) vs new binary, temp HOME, journal fixtures: same threads (control gamewatch history notify-rs preview-idle update + main), same status reply apart from pid, logs differ only in the autostart/config line order, second start asks the first to open the window and exits 0, --quit exits 0 and core exits 0. Not run: Windows CI (finding), cargo deny (no lockfile change), shell window clippy (shell untouched).
