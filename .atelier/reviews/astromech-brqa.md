---
schema: atelier.review
schema_version: 1
id: astromech-brqa
title: 'Two processes, one folder (checked, no action): prune now matches only ''<name>.'' prefix, so the core never deletes astromech-shell.* and vice versa (test asserts it); roll-over renames only the process''s own live file. Rust std opens with FILE_SHARE_DELETE so rename of an open file works on Windows. The shell is single-instance (tauri-plugin-single-instance) and the core is too, so no two writers share one file. Diagnostics bundle already takes every *.log in the folder, so the docs'' ''both logs'' claim holds. check-deps OK (astromech-log added to LEAVES, odyssey -> log allowed); cargo test -p astromech-log -p astromech-core -p astromech-odyssey all pass; the named messages (cache-open in astromech-data''s caller, unknown-mod in odyssey, couldn''t start the core) no longer use eprintln.'
target: astromech-4kw5
identity: review-agent (sonnet, independent)
commit: 0d5a990db3565889bd0d823992046eff75dabc28
created_at: 2026-10-17T06:35:26.195507482Z
event:
  event: finding
  recommendation: no_action
---

Two processes, one folder (checked, no action): prune now matches only '<name>.' prefix, so the core never deletes astromech-shell.* and vice versa (test asserts it); roll-over renames only the process's own live file. Rust std opens with FILE_SHARE_DELETE so rename of an open file works on Windows. The shell is single-instance (tauri-plugin-single-instance) and the core is too, so no two writers share one file. Diagnostics bundle already takes every *.log in the folder, so the docs' 'both logs' claim holds. check-deps OK (astromech-log added to LEAVES, odyssey -> log allowed); cargo test -p astromech-log -p astromech-core -p astromech-odyssey all pass; the named messages (cache-open in astromech-data's caller, unknown-mod in odyssey, couldn't start the core) no longer use eprintln.
