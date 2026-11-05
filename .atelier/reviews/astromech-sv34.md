---
schema: atelier.review
schema_version: 1
id: astromech-sv34
title: approval on astromech-zirr @6b5f5330
target: astromech-zirr
identity: review-agent-zirr-1
commit: 6b5f5330b71aabb6a208d1b218c441b4d7fda5f9
created_at: 2026-11-04T23:34:14.366647850Z
event:
  event: approval
---

Read 9eb0c2e9..6b5f5330 against Done when. The note is now TOML with config, pid and exe. config_in_use ignores a note that fails to parse (so the old path-only format is ignored) and one whose pid is not running or runs another exe. On Windows that is OpenProcess, then GetExitCodeProcess == STILL_ACTIVE (which covers an exited process whose handle is still held), then QueryFullProcessImageNameW, with names compared case-insensitively. On Linux it reads /proc/pid/exe and trims ' (deleted)'. A reused pid is caught unless it is reused by another astromech-core, which is negligible. Window cost per call is one small file read, a TOML parse and one process query, well under a millisecond. The new test covers a killed pid, a reused pid under another exe name, an old-format note and a live note, all naming a deleted --config file. The ARCHITECTURE.md sentence now matches the code. windows-sys 0.61.2 is already in both lockfiles, and all six items used are in Win32_Foundation and Win32_System_Threading. Checks: cargo metadata --locked --offline passes for both the root and app/src-tauri workspaces, the lockfiles are unchanged, and cargo deny check passes all four categories. Fresh target-review build: cargo test -p astromech-store -p astromech-core passes (store 20 and core 190 lib tests, the new test listed), clippy -D warnings with --locked is clean, fmt is clean. gh pr checks 199 is all green at 6b5f5330; the Windows MSVC job ran clippy --locked and the new test passed there. No fix_now findings.
