---
schema: atelier.review
schema_version: 1
id: astromech-pex1
title: 'crates/astromech-core/src/app.rs:1065-1088 (a_journal_folder_set_after_startup_is_imported) depends on the machine: the config has no game.journal_dir, so Config::journal_dir (config.rs:220) falls back to %USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous. On a PC with Elite Dangerous journals there (Sumeet''s), the startup follower imports the real journals into the temp history.sqlite, re-pointing keeps them, events never equal 52, and wait_for times out after 20 s. It also reads personal logs during cargo test. Reproduced on Linux with USERPROFILE pointing at a fake home holding one other journal: FAILED, timed out waiting for the journals set in onboarding. Fix: start the config with game.journal_dir set to an empty (or missing) temp folder, which is the real Windows first-run case, and keep the set_folders and 52-event assertions.'
target: astromech-5t34
identity: review-agent (5t34 review)
commit: cdaba6007cd054cf1d165f344050e991005f5ea5
created_at: 2026-10-17T00:09:00.651536898Z
event:
  event: finding
  recommendation: fix_now
---

crates/astromech-core/src/app.rs:1065-1088 (a_journal_folder_set_after_startup_is_imported) depends on the machine: the config has no game.journal_dir, so Config::journal_dir (config.rs:220) falls back to %USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous. On a PC with Elite Dangerous journals there (Sumeet's), the startup follower imports the real journals into the temp history.sqlite, re-pointing keeps them, events never equal 52, and wait_for times out after 20 s. It also reads personal logs during cargo test. Reproduced on Linux with USERPROFILE pointing at a fake home holding one other journal: FAILED, timed out waiting for the journals set in onboarding. Fix: start the config with game.journal_dir set to an empty (or missing) temp folder, which is the real Windows first-run case, and keep the set_folders and 52-event assertions.
