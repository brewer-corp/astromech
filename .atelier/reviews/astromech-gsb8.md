---
schema: atelier.review
schema_version: 1
id: astromech-gsb8
title: 'Shell log folder (low): log_dir() in main.rs is app_dir()/logs with the shell''s own app_dir(), so in portable mode (or without LOCALAPPDATA/XDG) it differs from the core''s folder, and the diagnostics bundle would miss the shell log there. Does not block: the core never opens the window in portable mode, so the shell is not normally run there, and installed Windows is identical to the core''s. It will conflict textually with astromech-4yds (which deletes app_dir()); whichever merges second should replace log_dir() with Paths::current().log_dir(). Recommend DEFER, resolved by that rebase.'
target: astromech-4kw5
identity: review-agent (sonnet, independent)
commit: 0d5a990db3565889bd0d823992046eff75dabc28
created_at: 2026-10-17T06:35:26.310083242Z
event:
  event: finding
  recommendation: defer
---

Shell log folder (low): log_dir() in main.rs is app_dir()/logs with the shell's own app_dir(), so in portable mode (or without LOCALAPPDATA/XDG) it differs from the core's folder, and the diagnostics bundle would miss the shell log there. Does not block: the core never opens the window in portable mode, so the shell is not normally run there, and installed Windows is identical to the core's. It will conflict textually with astromech-4yds (which deletes app_dir()); whichever merges second should replace log_dir() with Paths::current().log_dir(). Recommend DEFER, resolved by that rebase.
