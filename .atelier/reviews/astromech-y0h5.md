---
schema: atelier.review
schema_version: 2
id: astromech-y0h5
title: review on astromech-bp23 @e0fac0ea
target: astromech-bp23
identity: claude-reviewer-i8vr
commit: e0fac0ea40744ef51d4ea921bc10933300547cd8
created_at: 2026-11-07T09:47:50.707129347Z
event:
  event: review
---

Reviewed astromech-i8vr (1aa6880b..e0fac0ea: 9a60032d start, e1e28513 code, e0fac0ea close) against its Done when and the tabr note, bp23, unh9, 1zhg, the one-tested-process Authority section and ARCHITECTURE 18.2. Not approved: two fix_now findings. klw0: the unbounded status poll, and the watchdog can't rescue a hung core on Windows. 1k9v: the smoke's galaxy_systems timeout escapes 'except ValueError'. One defer (34i3: stale 'shell' wording). Checked: proxy origin, text, progress, error and value mapping (call_window and window.rs); transport.ts, core.ts and coreCall's {ok:false,error}; every lib caller's arg names (only within_ly was multi-word); the neutron_download topic against commands.ts; no removed command referenced anywhere; WINDOW_VERSION via the Vite define, stamped by set-version.py before tauri build, shown with the core down; smoke routes against the 24 manifests, error strings against astromech-api's; the release.yml paths. Reran: cargo test --workspace 1300/0; workspace clippy -D warnings, fmt, check-deps, check-guide and deny all clean; pnpm check, lint and test 532/532; shell --features window clippy -D warnings clean (fake .pc files, scratch export). The author's screenshots: only the Diagnostics PNGs differ. Not run: Windows CI, the Release onboarding smoke, a live core via call_window.
