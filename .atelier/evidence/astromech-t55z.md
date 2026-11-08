---
created_at: "2026-11-07T14:00:12.923674467+00:00"
id: "astromech-t55z"
evidence_type: "validation"
commit: "a2686ace45e2ae1025e85396e16c8d3945b3126a"
result: "pass"
captured_at: "2026-11-07T14:00:12.923539666+00:00"
uri: "https://github.com/brewer-corp/astromech/actions/runs/42374255964"
agent_identity: "astromech-wgtr implementer (Claude)"
target:
  kind: "issue"
  id: "astromech-wgtr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-wgtr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Windows CI run 42374255964 (draft PR #218, branch commit a54413ea; GitHub's merge commit 87ee21b8), job probe, windows-latest (Windows Server 2025, 4 CPUs): scripts/one_process_probe.py against spikes/o1-one-process passed checks 1-7.\n1 windowless: --background had a tray rect within 10 s, the window was closed, and no WebView2 process belonged to the app in 30 one-second samples.\n2 Raw Input with the listener started after Builder::build: an F13 sent by SendInput raised sink_keys at start, after open, after the first close and after tray_churn.\n3 tray_churn (50 menu rebuilds from a worker thread) answered in 96 ms and ui_state answered after it.\n4 a second start with --route keybinds exited 0 and the page loaded in 3.09 s; 6 WebView2 processes belonged to the app (by descent, --webview-exe-name and the EBWebView folder).\n5 WM_CLOSE by title over 5 cycles: window closed in about 0.24 s, no WebView2 left after about 2.3 s, app and tray still there.\n6 killing the renderer, then the browser process: the app lived, the tray and control port answered, and close plus reopen loaded the page.\n7 quit from a worker thread: exit 0 in 0.24 s with the window open, no WebView2 left.\nRecorded, not gated: with --sink-before-build the listener saw no key at any of the four points. Memory (private working set, app plus its WebView2): 3.3 MB windowless, 124.1 MB window open, 9.6 to 7.7 MB after closes 1-5, plain tray-icon baseline 1.9 MB; idle CPU 0.0% windowless, 0.25% of the machine with the window open. Renderer killed: WebView2's 'This page is having a problem' page (Error code 15) stayed until close; browser killed: a blank white window and every WebView2 process gone; ProcessFailed fired with kind 1 and kind 0. The job then failed after the checks on a cp1252 console print (fixed in 388ab5a7).\nArtifact one-process-probe: report.md, report.json, spike.log, dead-window-renderer.png, dead-window-browser.png.\nLimits: a CI runner, not a gaming PC; the tray icon was never clicked; the probe uses SendInput (injected keys), not a physical keyboard; Elite was not running."
updated_at: "2026-11-07T14:00:13.191115058+00:00"
---

Windows CI run 42374255964 (draft PR #218, branch commit a54413ea; GitHub's merge commit 87ee21b8), job probe, windows-latest (Windows Server 2025, 4 CPUs): scripts/one_process_probe.py against spikes/o1-one-process passed checks 1-7.
1 windowless: --background had a tray rect within 10 s, the window was closed, and no WebView2 process belonged to the app in 30 one-second samples.
2 Raw Input with the listener started after Builder::build: an F13 sent by SendInput raised sink_keys at start, after open, after the first close and after tray_churn.
3 tray_churn (50 menu rebuilds from a worker thread) answered in 96 ms and ui_state answered after it.
4 a second start with --route keybinds exited 0 and the page loaded in 3.09 s; 6 WebView2 processes belonged to the app (by descent, --webview-exe-name and the EBWebView folder).
5 WM_CLOSE by title over 5 cycles: window closed in about 0.24 s, no WebView2 left after about 2.3 s, app and tray still there.
6 killing the renderer, then the browser process: the app lived, the tray and control port answered, and close plus reopen loaded the page.
7 quit from a worker thread: exit 0 in 0.24 s with the window open, no WebView2 left.
Recorded, not gated: with --sink-before-build the listener saw no key at any of the four points. Memory (private working set, app plus its WebView2): 3.3 MB windowless, 124.1 MB window open, 9.6 to 7.7 MB after closes 1-5, plain tray-icon baseline 1.9 MB; idle CPU 0.0% windowless, 0.25% of the machine with the window open. Renderer killed: WebView2's 'This page is having a problem' page (Error code 15) stayed until close; browser killed: a blank white window and every WebView2 process gone; ProcessFailed fired with kind 1 and kind 0. The job then failed after the checks on a cp1252 console print (fixed in 388ab5a7).
Artifact one-process-probe: report.md, report.json, spike.log, dead-window-renderer.png, dead-window-browser.png.
Limits: a CI runner, not a gaming PC; the tray icon was never clicked; the probe uses SendInput (injected keys), not a physical keyboard; Elite was not running.
