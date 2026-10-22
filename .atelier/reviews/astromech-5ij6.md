---
schema: atelier.review
schema_version: 1
id: astromech-5ij6
title: 'R3 [low, needs Sumeet] astromech-6a7c Done when was rewritten by the implementer (commit f3eb3fae): ''console runs publish or clear it, and a test covers each'' became ''console runs leave config.path alone''. The reasoning checks out against the code: run_console (crates/astromech-core/src/main.rs:221) writes no core.json and takes no instance lock, and app::run takes the lock (app.rs:1205) before choose_config publishes, so two tray cores cannot both publish. But it narrows the issue without a human sign-off and leaves the gap the record itself names: app/src-tauri/src/main.rs:1229 builds the Remote cache from data_dir() before the window''s own tray core publishes, so after an earlier --config run whose file still exists, the window''s remote-data cache sits in that profile''s data folder for the whole session. Sumeet should accept the revised Done when; file the startup gap as a backlog follow-up.'
target: astromech-0k88
identity: review-agent-0k88-1
commit: adf99099b228d9792fe2e97462a5f16a3beda99c
created_at: 2026-10-22T21:50:07.453265862Z
event:
  event: finding
  recommendation: defer
---

R3 [low, needs Sumeet] astromech-6a7c Done when was rewritten by the implementer (commit f3eb3fae): 'console runs publish or clear it, and a test covers each' became 'console runs leave config.path alone'. The reasoning checks out against the code: run_console (crates/astromech-core/src/main.rs:221) writes no core.json and takes no instance lock, and app::run takes the lock (app.rs:1205) before choose_config publishes, so two tray cores cannot both publish. But it narrows the issue without a human sign-off and leaves the gap the record itself names: app/src-tauri/src/main.rs:1229 builds the Remote cache from data_dir() before the window's own tray core publishes, so after an earlier --config run whose file still exists, the window's remote-data cache sits in that profile's data folder for the whole session. Sumeet should accept the revised Done when; file the startup gap as a backlog follow-up.
