---
schema: atelier.review
schema_version: 1
id: astromech-nhfz
title: 'R2 [low] crates/astromech-store/src/paths.rs:92 config_in_use fallback: a tray core started with --config pointing at a file that does not exist yet runs on Config::default() (crates/astromech-core/src/app.rs:1238) and publishes that path; the window used to read defaults too (matching), and now reads the default astromech.toml instead, so its data_dir and Bindings folder can differ from the core''s until the core first writes its --config file. The original Done when asked for this fallback, so it meets the requirement; the mismatch is a narrow edge (fresh --config profile). Fix later if it matters: fall back to the default file only when the core is not running, or have the core publish whether its file existed.'
target: astromech-0k88
identity: review-agent-0k88-1
commit: adf99099b228d9792fe2e97462a5f16a3beda99c
created_at: 2026-10-22T21:50:07.343535365Z
event:
  event: finding
  recommendation: defer
---

R2 [low] crates/astromech-store/src/paths.rs:92 config_in_use fallback: a tray core started with --config pointing at a file that does not exist yet runs on Config::default() (crates/astromech-core/src/app.rs:1238) and publishes that path; the window used to read defaults too (matching), and now reads the default astromech.toml instead, so its data_dir and Bindings folder can differ from the core's until the core first writes its --config file. The original Done when asked for this fallback, so it meets the requirement; the mismatch is a narrow edge (fresh --config profile). Fix later if it matters: fall back to the default file only when the core is not running, or have the core publish whether its file existed.
