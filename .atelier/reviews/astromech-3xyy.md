---
schema: atelier.review
schema_version: 2
id: astromech-3xyy
title: finding on astromech-y6d6 @e9dea942
target: astromech-y6d6
identity: claude-reviewer-wgtr
commit: e9dea94204159f14c934bcc2c0de5b6d7240d3d4
created_at: 2026-11-07T14:05:58.185774276Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: 579c3a8ebda8dbeb856438298aa50b1fb3b7ac3d
    recorded_at: 2026-11-07T14:07:21.689024501Z
    follow_up: astromech-v3u4
    reason: Noted on v3u4
---

low: scripts/one_process_probe.py:446-447, 868-869 and astromech-v3u4's probe line. v3u4's Done when runs this probe against the installed app 'with --no-sink' and 'no extra test hook'. But the probe has no --no-sink option, check 2 reads ui_state.sink_keys, check 3 calls tray_churn (a test hook), and main() fails unless all of checks 1-7 are present. As written, the probe can't pass against the real app. Its default --log/--user-data (spike.log and EBWebView next to core.json) also don't fit the real app's layout, though they can be overridden. Fix in v3u4 or xz7i: add a note on v3u4 that the probe needs --no-sink, a way to skip check 3 (or a tray_churn command), and explicit --log/--user-data/--title for the installed app.
