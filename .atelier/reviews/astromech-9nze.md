---
schema: atelier.review
schema_version: 2
id: astromech-9nze
title: finding on astromech-y6d6 @9da11270
target: astromech-y6d6
identity: claude-reviewer-xz7i
commit: 9da11270e69ff3ee7e11679b161a4b132e39ae26
created_at: 2026-11-07T22:10:52.960315820Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: dd4c882c2b6b7c4a4baac3f732a634baa56abd1a
    recorded_at: 2026-11-08T15:38:49.775199212Z
    reason: 'Fixed: the spike''s main.rs and mqqa no longer point at the probe or one-process.yml; v3u4 notes xz7i replaced the Release probe step.'
---

low: stale pointers to what xz7i removed. spikes/o1-one-process/src/main.rs:3-4 still says the probe drives the spike on Windows CI (.github/workflows/one-process.yml), which is deleted. astromech-mqqa's Done when matches WebView2 'by command line, as scripts/one_process_probe.py does', but that rule now lives only in scripts/webview_cost.py (webview_owner). astromech-v3u4's ticked Done when line 73 says one_process_probe.py runs in the Release installer job, and that's no longer true once this merges with it. fmub and dl53 are not orphaned: restart mode keeps its interface (--restart --app --endpoint --out --config --clean-launch --log --crash-hook-arg), and fmub's exe artifact (run 42384570621) outlives the workflow. Fix: one-line edits, plus a note on v3u4 that xz7i replaced the Release probe step.
