---
schema: atelier.review
schema_version: 2
id: astromech-k6f1
title: finding on astromech-y6d6 @e9dea942
target: astromech-y6d6
identity: claude-reviewer-wgtr
commit: e9dea94204159f14c934bcc2c0de5b6d7240d3d4
created_at: 2026-11-07T14:05:58.559352959Z
event:
  event: finding
  recommendation: defer
---

low: scripts/one_process_probe.py:519 and :282. Check 6's pass ignores r['close'] (closing the dead window and WebView2 exiting), so a crashed window whose WebView2 lingers after close would still pass check 6. tray_ok's fallback (icon built plus Ok(None)) would pass check 1 with no tray rectangle. Neither weakened these runs: every close passed, and every tray reading was 'rect' (art2 report.json). But xz7i is to move check 6 into webview_cost.py's gates. Fix there: require the close in the crash case to pass, and treat the tray fallback as a reported degradation, not a pass.
