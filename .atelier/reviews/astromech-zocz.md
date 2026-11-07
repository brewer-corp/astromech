---
schema: atelier.review
schema_version: 2
id: astromech-zocz
title: finding on astromech-y6d6 @9da11270
target: astromech-y6d6
identity: claude-reviewer-xz7i
commit: 9da11270e69ff3ee7e11679b161a4b132e39ae26
created_at: 2026-11-07T22:10:52.355710306Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/webview_cost.py:154-158 (judge's close gate) and :595-601, :621-623 (measure). xz7i (1759598f..9da11270). The reopens are measured but never gated: each cycle's c['reopen'] (cycles 2-5) and the quit step's reopen after the browser crash are ignored by judge. A reopen that exits 'timeout' or never loads a page still passes every gate, since close_window finds the window by title whether or not its page loaded, and the quit gate then passes with no window open, so 'quit with the window open' can pass vacuously. Shown in scratch on windows_run(): checks-only with closes[2].reopen = {exit_code: 'timeout', page_load_s: None} -> failed() False; in a full run, cold_start passes on 1 of 5 windowless samples. The old probe gated both (check 5: a reopen that didn't load failed its cycle; check 6: r['reopened'] after each crash), and Release now runs --checks-only, where cold start is skipped, so Release loses its proof that the window reopens by a second start, including after a browser-process crash. Fix: gate each reopen (exit 0 and a page load within the timeout) in the close gate, record and gate the quit step's reopen (or require WebView2 present just before quit), require len(windowless_s) == CYCLES on Windows for cold start, and add the tests.
