---
schema: atelier.review
schema_version: 2
id: astromech-swnq
title: approval on astromech-y6d6 @55668f02
target: astromech-y6d6
identity: claude-reviewer-xz7i
commit: 55668f027a4b4d97f55da867aad281bf05acb9c3
created_at: 2026-11-07T22:13:07.838683938Z
event:
  event: approval
---

Re-reviewed astromech-xz7i's fix 55668f02 for findings zocz, 9nze and m6f9. zocz: reopen_ok gates each cycle's reopen (exit 0 and a page load; a missing reopen after cycle 1 fails), quit requires ui_state window open and WebView2 present just before it, and cold start requires CYCLES samples where the closes run. Linux and --checks-only behave as before. 9nze: the spike's main.rs header and astromech-mqqa's line now point at webview_cost.py, and v3u4 has a note that --checks-only replaced the Release probe step. m6f9: rect then fallback fails, and only fallback at every reading is DEGRADED. Proof: python3 scripts/test_webview_cost.py, 28 OK. Four scratch mutations, each removing one fix (reopen_ok, the quit precondition, the sample count, the tray rule), each failed the suite. Residual: the quit step's reopen is judged by the window being open with WebView2 present, not by a page load, which is acceptable. The xz7i report's live --checks-only line (quit PASS on the headless build) predates the quit precondition, so quit would now fail on a build with no window. CI on 55668f02 was pending at review (Linux, Windows MSVC and atelier doctor listed). Done when lines 1, 3 and 8 still wait on Windows CI. No fix_now finding remains open.
