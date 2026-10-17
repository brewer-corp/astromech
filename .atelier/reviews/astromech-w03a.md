---
schema: atelier.review
schema_version: 1
id: astromech-w03a
title: 'Approved at HEAD. Contract: pass. Paths is the single rule; Config and the shell use it, the shell''s app_dir/core_data_dir are gone, and galaxy.db, history.sqlite, outfitting.sqlite, astromech.sqlite, the remote cache, astromech.toml and device-overrides.json all open through Paths::data_dir/config_file. Default layout and portable detection match the old core rules exactly (env order, is_file check, exe_dir/astromech-data); the shell''s old rule (no XDG, ./ fallback) is the one that changed, only off Windows. Tests cover default, fallbacks, portable and override; cargo test -p astromech-store -p astromech-core passes; check-deps OK. open_external widened to app, log and data folders only. Two DEFER findings (device-overrides.json orphaned for override/portable users; shell wiring untested, Linux cannot compile it). Not run: the shell build; Linux CI job passed, both Windows jobs were pending, so merge should wait for them.'
target: astromech-4yds
identity: review-agent (sonnet, independent)
commit: 971e5efaa926c58970fd440f72b15847d1d6a6cb
created_at: 2026-10-17T06:35:24.516321080Z
event:
  event: approval
---

Approved at HEAD. Contract: pass. Paths is the single rule; Config and the shell use it, the shell's app_dir/core_data_dir are gone, and galaxy.db, history.sqlite, outfitting.sqlite, astromech.sqlite, the remote cache, astromech.toml and device-overrides.json all open through Paths::data_dir/config_file. Default layout and portable detection match the old core rules exactly (env order, is_file check, exe_dir/astromech-data); the shell's old rule (no XDG, ./ fallback) is the one that changed, only off Windows. Tests cover default, fallbacks, portable and override; cargo test -p astromech-store -p astromech-core passes; check-deps OK. open_external widened to app, log and data folders only. Two DEFER findings (device-overrides.json orphaned for override/portable users; shell wiring untested, Linux cannot compile it). Not run: the shell build; Linux CI job passed, both Windows jobs were pending, so merge should wait for them.
