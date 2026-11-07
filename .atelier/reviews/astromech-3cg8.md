---
schema: atelier.review
schema_version: 2
id: astromech-3cg8
title: finding on astromech-bp23 @3259fb8b
target: astromech-bp23
identity: claude-reviewer-6siv
commit: 3259fb8b166e5940156c0b00ef28b266818c256a
created_at: 2026-11-07T07:10:40.900813314Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/astromech-api/src/state.rs:205-219 (State::binds) and app/src/lib/store.svelte.ts:34 The keybinds Api, which holds the device inventory (enumerate_live) and the game installs found at discover, now lives for the core's whole life (often since Windows login, via autostart). Before, it lived in the shell, so every window open re-enumerated. Only onboarding calls binds_refresh_devices, so a controller plugged in or powered on after the core started shows as not connected on the controller page (modules/controller/View.svelte:141, inventory.controllers) and is missing from preset matching until the core restarts. The child's goal was no change the window sees. Fix: refresh devices once when the binds store first loads in a window (call binds_refresh_devices in place of binds_state on start), or rebuild the inventory on binds_state; add a test if done core-side.
