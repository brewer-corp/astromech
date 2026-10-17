---
schema: atelier.review
schema_version: 1
id: astromech-wekk
title: 'Upgrade visibility of user files (low): the default layout is byte-identical for both processes (LOCALAPPDATA, XDG_DATA_HOME, HOME, then exe_dir/astromech-data; portable = astromech.toml next to the exe; I diffed the old Config::app_dir/portable/default_path/data_dir against Paths line by line). But the shell now opens device-overrides.json (user-authored binding overrides, keybinds Api data_dir) in the data folder instead of the app dir, so a user with a data_dir override, or a portable install, who already saved overrides will see them vanish (the file is left in the old app dir, not deleted). Same for shell-made galaxy.db there (rebuildable). Not an issue for the default Windows layout. Suggest a one-time copy/fallback to the app dir, or a release note.'
target: astromech-4yds
identity: review-agent (sonnet, independent)
commit: 971e5efaa926c58970fd440f72b15847d1d6a6cb
created_at: 2026-10-17T06:35:09.379446925Z
event:
  event: finding
  recommendation: defer
---

Upgrade visibility of user files (low): the default layout is byte-identical for both processes (LOCALAPPDATA, XDG_DATA_HOME, HOME, then exe_dir/astromech-data; portable = astromech.toml next to the exe; I diffed the old Config::app_dir/portable/default_path/data_dir against Paths line by line). But the shell now opens device-overrides.json (user-authored binding overrides, keybinds Api data_dir) in the data folder instead of the app dir, so a user with a data_dir override, or a portable install, who already saved overrides will see them vanish (the file is left in the old app dir, not deleted). Same for shell-made galaxy.db there (rebuildable). Not an issue for the default Windows layout. Suggest a one-time copy/fallback to the app dir, or a release note.
