---
created_at: "2026-11-07T04:51:23.056349964+00:00"
id: "astromech-y6d6"
issue_type: "feature"
labels:
- "size-l"
workflow_branch:
  branch_base: main
  work_branch: astromech-y6d6
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-7i2k"
  - kind: "issue"
    id: "astromech-9lh3"
  - kind: "issue"
    id: "astromech-dl53"
  - kind: "issue"
    id: "astromech-v3u4"
  - kind: "issue"
    id: "astromech-wgtr"
  - kind: "issue"
    id: "astromech-xz7i"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-01ft"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-bp23"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "One process: the core and the window in one Tauri app"
updated_at: "2026-11-07T12:31:30.396301948+00:00"
---

## Goal

astromech ships as one Tauri process holding the tray, pipeline, voice, input and the window (decision astromech-6i8s), with the second executable and the start-the-core plumbing gone. The speech worker stays a separate supervised process.

## Done when

- [ ] A spike runs first, on Windows CI. It shows the tray working with no window, through a window crash (renderer and browser process) and after the window closes. It shows that closing the window ends WebView2's processes, matched by command line as well as by descent. It shows that astromech's Raw Input listener still receives keys after Tauri builds its event loop and after the window opens and closes. It also reports the windowless idle cost and which crashes Windows' restart registration covers. If one of decision astromech-6i8s's premises fails, the work stops for Sumeet.
- [ ] Start with Windows creates no window. Closing the window destroys it. A second start focuses the window, or opens it at `--route`.
- [ ] Push-to-talk and the kill switch keep working with the window open and after it closes, including when Elite was already running when the app started. This is shown by the onboarding smoke tapping F8 (kill) and holding it 1.2 s (re-arm), in a run with the mock game started after the app and one with it started before. (New: one Raw Input registration per process, started after Tauri's `build`.)
- [ ] Crash restart works without the shell, through Windows restart registration. Windows CI shows the registration read back and a faked `--restarted` start coming back tray-only, with the kill switch and Pause voice and the End task behaviour as decision astromech-lkwj says; it records real crash restarts without gating on them, since the spike (astromech-wgtr, evidence astromech-fqw9) could not show a restart on the runner. (Reworded from "kills": restart registration never covers an outside kill. Decision astromech-lkwj is Sumeet's and blocks astromech-dl53.)
- [ ] The installer, the updater and Start with Windows move to the one executable, `astromech-core.exe`. The release build and its smoke run pass.
- [ ] An existing two-process install upgrades cleanly, by both routes: `setup.exe /S` over the running install, and the last release's own update helper overlaying this build's zip. This covers the old Start with Windows entry, a tray core already running, the single-instance mutex, old Start-menu and pinned shortcuts, and the log files. It is shown on Windows CI from an install of the newest release.
- [ ] `scripts/webview_cost.py` and `.github/workflows/webview-cost.yml` run against the one-process app. They report its memory windowless, open and closed again, and fail if any WebView2 process is left with the window closed (decision astromech-01ft: reported, not bounded).
- [ ] These describe one process and link astromech-6i8s: ARCHITECTURE.md (§4, §5.10, §18.1, §18.6), docs/V1-DESIGN.md (§7.1, §7.2, §7.3, §7.5, §7.7, §7.10), CONTRIBUTING.md's layout and build lines, docs/guide/getting-started.md and the 08-app-processes diagram. `scripts/stage-sidecars.sh`, `externalBin` and the smoke step in .github/workflows/release.yml fit the new layout.
- [ ] After merge (needs-human: Sumeet's PC next to Elite): the S11 play check of astromech-v3u4; the real crash restart on his install (astromech-dl53); and after the next release his own install updating through the in-app updater (astromech-9lh3). Stand-in: the onboarding smoke with the kill-switch runs, the webview-cost job, dl53's registration and faked-restart job, and the upgrade job on Windows CI.
- [ ] Success criterion one-process (one app process plus its speech worker, and no WebView2 process left with the window closed) is shown by the App window cost workflow on Windows, linked from an evidence receipt here.

## Background

What the code settles, and what the spike (astromech-wgtr) settles:
- **tao and the tray with no window.** From the code: Tauri's tray uses the same `tray-icon` and `muda` crates. tao registers keyboard and mouse Raw Input when it builds the loop, so the input source starts after `build`. The spike settles, on Windows, that the tray works windowless, through window crashes and after a close, and whether an app-owned Raw Input listener survives.
- **Idle cost.** The spike measures it against a plain-tray baseline: reported, not gated (astromech-01ft).
- **`core.json`.** It stays as the control API's endpoint, and the window stops using it. This is settled by decision astromech-nade; the spike is not needed for it.
- **Restart registration.** The spike settles which crash kinds Windows restarts on the runner.
