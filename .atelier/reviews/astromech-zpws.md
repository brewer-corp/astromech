---
schema: atelier.review
schema_version: 2
id: astromech-zpws
title: finding on astromech-bp23 @36531f8c
target: astromech-bp23
identity: claude-reviewer-ro7a
commit: 36531f8c99c5623824057f12726be11a1e8f58ce
created_at: 2026-11-07T11:16:05.293058551Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 3e64e50c9ac1f658844cdc523a1d0585f6a819f8
    recorded_at: 2026-11-07T11:18:18.143369590Z
    resolution_commit: 3e64e50c9ac1f658844cdc523a1d0585f6a819f8
    reason: Fixed in 3e64e50c
---

medium: .atelier/issues/astromech-vucf.md Done when 2 (edited in 5ffa4f1d) The edit adds 'or is not called' to vucf's END state ('A command that still has no seam is called with args or state that make it refuse before the effect, or is not called'). The old line required every no-seam command to be called, and strategy one-command-list says the test 'calls each one through the control API of a headless core'. So vucf could close the criterion with autostart_on/off, open_config and mic_check never called. On the Linux headless core they are callable now: autostart::set is a non-Windows stub that errors (autostart.rs:130), record_mic bails 'recording needs Windows' (calibration.rs:323), and open_config only needs the opener seam vucf adds. Fix: drop 'or is not called' from vucf's end state (keep it as ro7a's interim allowance only), or say why a command may stay uncalled at the end and get Sumeet's OK, since it changes a success criterion's proof.
