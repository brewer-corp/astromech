---
schema: atelier.review
schema_version: 2
id: astromech-72ox
title: finding on astromech-y6d6 @4fd16542
target: astromech-y6d6
identity: claude-reviewer-7i2k
commit: 4fd165420c02c19f3ef7e56ac925d164d100aa81
created_at: 2026-11-07T17:10:31.690168962Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/astromech-fmub.md exists only on astromech-y6d6 (git show main:.atelier/issues/astromech-fmub.md fails; filed in 579c3a8e on the branch). Its goal is to run before astromech-v3u4 removes the watchdog, and v3u4 is now in atelier work ready, but Sumeet's view of Needs a Human (atelier work human on main) does not list it, so as filed he can't find it. Second, its command passes --endpoint %LOCALAPPDATA%\astromech-spike\core.json, which only cmd expands; in PowerShell (Windows Terminal's default) the probe gets the literal string and waits on a path that never appears. The artifact itself checks out: astromech-spike-exe, id 13579199136, on run 42384570621, not expired. Fix: commit fmub to main as a planning record (AGENTS.md allows .atelier/issues straight to main), and say 'in cmd' or give the PowerShell form ($env:LOCALAPPDATA).
