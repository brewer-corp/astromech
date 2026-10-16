---
schema: atelier.review
schema_version: 1
id: astromech-t8cf
title: 'app/src/modules/colonisation/View.svelte:16-30: polls are not sequenced; a slow earlier loadProjects() resolving after a later one overwrites projects with older data until the next 2 s tick. Also err is hidden once projects exist (line 61), so a persistent poll failure goes unnoticed. app/src-tauri/src/main.rs:600: the cache key uses None when Cargo.json mtime is unreadable, so edits would not invalidate (rare; Windows normally supports mtime). The shell cache key itself has no test; metadata --locked --offline passes but main.rs was only read, not compiled (key tuple type, Option closure and filter(|(s,_)| *s == version) look type-correct, JournalVersion needs PartialEq as before).'
target: astromech-k3wp
identity: review-agent (k3wp fix review, sonnet)
commit: 59f832959323c130ff06ac678b9f1b3a7bb80deb
created_at: 2026-10-16T23:18:11.758464262Z
event:
  event: finding
  recommendation: defer
---

app/src/modules/colonisation/View.svelte:16-30: polls are not sequenced; a slow earlier loadProjects() resolving after a later one overwrites projects with older data until the next 2 s tick. Also err is hidden once projects exist (line 61), so a persistent poll failure goes unnoticed. app/src-tauri/src/main.rs:600: the cache key uses None when Cargo.json mtime is unreadable, so edits would not invalidate (rare; Windows normally supports mtime). The shell cache key itself has no test; metadata --locked --offline passes but main.rs was only read, not compiled (key tuple type, Option closure and filter(|(s,_)| *s == version) look type-correct, JournalVersion needs PartialEq as before).
