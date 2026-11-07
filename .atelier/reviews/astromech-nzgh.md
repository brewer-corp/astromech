---
schema: atelier.review
schema_version: 2
id: astromech-nzgh
title: finding on astromech-vucf @50e59010
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 50e59010c8f51ecae55b994a55df7e4d1680b986
created_at: 2026-11-07T13:50:05.600772313Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/app.rs:689-699 open_window is still a real spawn in world mode: with ASTROMECH_SHELL set (as in a tauri dev shell) the local open_window command starts the real window, which uses Paths::current() (app/src-tauri/src/window.rs:25, :168), so it logs into the real app dir and talks to the user's real core. Not reachable in d2nq's tests and outside Done when 6's opener/autostart/notifier list. Fix: route open_window through a boundary that world mode records, or refuse it in world mode. Defer to astromech-gllh (its no-spawn proof).
