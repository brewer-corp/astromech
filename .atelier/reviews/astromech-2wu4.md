---
schema: atelier.review
schema_version: 2
id: astromech-2wu4
title: finding on astromech-y6d6 @26afef19
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: 26afef19d845b864358994d291ec64c98ed410ca
created_at: 2026-11-07T17:57:42.432537241Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src-tauri/src/host.rs:204-257 and scripts/one_process_probe.py:521-530. The WebView2 ProcessFailed recovery (reload after a renderer exit, rebuild after a browser exit) has no automated proof, and the Release probe step can't give one: check 6 kills the renderer/browser, waits 6 s, then closes the window and reopens it by a second start, and passes on app alive + tray + control + reopened, so it passes identically with no handler at all. The report labels it 'by inspection only', and Sumeet's after-merge list doesn't cover a window crash, so v3u4's window line ('Whatever the spike found necessary is in ... ProcessFailed') would be ticked on CI evidence that never exercises it. Fix: for the app (e.g. under --no-sink/--no-churn or a new --recovers flag), before the close in window_crash, require the log line 'window: reloading the page' (renderer) or 'window: rebuilt' plus a new 'window: page loaded' line (browser) and ui_state window=open. Related to deferred finding astromech-k6f1 (check 6 ignores r['close']), which doesn't cover this.
