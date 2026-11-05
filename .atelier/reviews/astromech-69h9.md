---
schema: atelier.review
schema_version: 1
id: astromech-69h9
title: finding on astromech-f2k1 @47967160
target: astromech-f2k1
identity: review-agent-f2k1-1
commit: 47967160ec0492c800103d37c0319669e6210fb2
created_at: 2026-11-04T22:40:34.934064395Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-store/src/paths.rs:108 config_in_use now trusts any config.path, including one left by a core that never ran Published's Drop (crash, kill/TerminateProcess, or an upgrade from a pre-f2k1 core, which never removed it). With the is_file fallback gone, a leftover naming a deleted --config file now gives the window defaults while the core it starts (watchdog or startup, no --config) reads astromech.toml; before this change that case fell back to the default. The leftover-whose-file-exists case is unchanged from main. Impact is narrow (needs --config, an unclean exit, and a window opened before a new core publishes; the Remote cache at app/src-tauri/src/main.rs:1301 is fixed for the window's life). Suggested follow-up: tie the note to a live core (write the pid into config.path and ignore it when that process is gone), which covers crash, kill, watchdog restart and upgrade leftovers together.
