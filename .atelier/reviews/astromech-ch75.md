---
schema: atelier.review
schema_version: 2
id: astromech-ch75
title: review on astromech-y6d6 @9da11270
target: astromech-y6d6
identity: claude-reviewer-xz7i
commit: 9da11270e69ff3ee7e11679b161a4b132e39ae26
created_at: 2026-11-07T22:10:53.716383540Z
event:
  event: review
---

Reviewed astromech-xz7i, commits 1759598f and 9da11270 (head of astromech-y6d6, PR #218). atelier review request refused (Done when 1, 3 and 8 are unticked and wait on Windows CI), so this review was done from 'atelier issue show --full' and the diff. Read: webview_cost.py in full, the reduced one_process_probe.py (diffed against 1759598f per function with ast: only restart mode is left, and its interface is unchanged), test_webview_cost.py, webview-cost.yml, release.yml, the deleted one-process.yml, CONTRIBUTING, ARCHITECTURE §14 and §18.1, spikes/README, host.rs UiState and its log lines, instance.rs and paths.rs (the scratch profile follows LOCALAPPDATA; the mutex is session-wide), and the 08-app-processes PNG, which renders. Ran: python3 scripts/test_webview_cost.py, 24 OK, plus scratch judge probes under /tmp/claude-1000/xz7i-review. Holds: ownership by descent, --webview-exe-name or EBWebView (both the scratch and the real folder); no vacuous pass on missing data for windowless, close, app_alive, window_crash and quit (the open gate requires WebView2 seen, so a blind matcher fails); the k6f1 crashed-window close is gated; v3u4's --expect-recovery is now unconditional in window_crash and runs under --checks-only in Release; Linux shows not-run rows and checks-only skips as the decision notes say; memory has no pass mark (01ft). Not run: anything on Windows, and the harness against a window build. CI on PR #218 was pending at review time (all jobs except atelier doctor). Findings: one fix_now (the reopens aren't gated) and two defer. Not approved.
