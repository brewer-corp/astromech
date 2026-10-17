---
schema: atelier.review
schema_version: 1
id: astromech-cw9m
title: 'R2 medium: Shell logs cache problems with eprintln! (app/src-tauri/src/main.rs:1156) but release builds use windows_subsystem=windows, so stderr goes nowhere; the shell still effectively ignores a cache-open problem, the astromech-ay9o complaint. Give the shell a log file or surface the message.'
target: astromech-zs2q
identity: review-agent (sonnet, independent)
commit: 744a26a39adb67c24914d76aa9ebe47c1843c2e0
created_at: 2026-10-17T06:00:43.875765874Z
event:
  event: finding
  recommendation: defer
---

R2 medium: Shell logs cache problems with eprintln! (app/src-tauri/src/main.rs:1156) but release builds use windows_subsystem=windows, so stderr goes nowhere; the shell still effectively ignores a cache-open problem, the astromech-ay9o complaint. Give the shell a log file or surface the message.
