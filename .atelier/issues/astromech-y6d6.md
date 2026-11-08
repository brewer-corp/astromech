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
updated_at: "2026-11-08T03:44:36.209899724+00:00"
---

## Goal

astromech ships as one Tauri process holding the tray, pipeline, voice, input and the window (decision astromech-6i8s), with the second executable and the start-the-core plumbing gone. The speech worker stays a separate supervised process.

## Done when

- [x] A spike runs first, on Windows CI. It shows the tray working with no window, through a window crash (renderer and browser process) and after the window closes. It shows that closing the window ends WebView2's processes, matched by command line as well as by descent. It shows that astromech's Raw Input listener still receives keys after Tauri builds its event loop and after the window opens and closes. It also reports the windowless idle cost and which crashes Windows' restart registration covers. If one of decision astromech-6i8s's premises fails, the work stops for Sumeet.
- [x] Start with Windows creates no window. Closing the window destroys it. A second start focuses the window, or opens it at `--route`.
- [x] Push-to-talk and the kill switch keep working with the window open and after it closes, including when Elite was already running when the app started. This is shown by the onboarding smoke tapping F8 (kill) and holding it 1.2 s (re-arm), in a run with the mock game started after the app and one with it started before. (New: one Raw Input registration per process, started after Tauri's `build`.)
- [x] No automatic crash restart (decision astromech-lkwj, Sumeet, 7 Nov 2026): the shell's watchdog is gone with the shell, nothing registers for restart, and the next start after a crash offers Save diagnostics through the crash marker; an uncaught panic on the main thread aborts so Windows records it (crates/astromech-core/src/crash.rs). The probe script and crash-restart child (astromech-dl53) are removed.
- [x] The installer, the updater and Start with Windows move to the one executable, `astromech-core.exe`. The release build and its smoke run pass, and the smoke covers launch, window up, connected, quit (decision astromech-1zhg). Each is shown by a step already in release.yml: launch by the `--background` start and the onboarding smoke's start of the installed app; window up by the onboarding smoke driving the page over CDP; connected by `status` over the control API and every page's calls reaching their commands; quit by `--quit` with the process gone. (Moved from astromech-yis2 on 7 Nov 2026.)
- [x] An existing two-process install upgrades cleanly, by both routes: `setup.exe /S` over the running install, and the last release's own update helper overlaying this build's zip. This covers the old Start with Windows entry, a tray core already running, the single-instance mutex, old Start-menu and pinned shortcuts, and the log files. It is shown on Windows CI from an install of the newest release.
- [x] `scripts/webview_cost.py` and `.github/workflows/webview-cost.yml` run against the one-process app. They report its memory windowless, open and closed again, and fail if any WebView2 process is left with the window closed (decision astromech-01ft: reported, not bounded).
- [x] These describe one process and link astromech-6i8s: ARCHITECTURE.md (§4, §5.10, §18.1, §18.6), docs/V1-DESIGN.md (§7.1, §7.2, §7.3, §7.5, §7.7, §7.10), CONTRIBUTING.md's layout and build lines, docs/guide/getting-started.md and the 08-app-processes diagram. `scripts/stage-sidecars.sh`, `externalBin` and the smoke step in .github/workflows/release.yml fit the new layout.
- [ ] After merge (needs-human: Sumeet's PC next to Elite): the S11 play check of astromech-v3u4; and after the next release his own install updating through the in-app updater (astromech-9lh3). Stand-in: the onboarding smoke with the kill-switch runs, the webview-cost job, and the upgrade job on Windows CI.
- [x] Success criterion one-process (one app process plus its speech worker, and no WebView2 process left with the window closed) is shown by the App window cost workflow on Windows, linked from an evidence receipt here.

## Report

astromech is one Tauri app: `astromech-core.exe` holds the tray, the pipeline, voice, input and the window, and the speech worker stays its own supervised process. The second executable is gone except for the tiny `astromech-shell.exe` forwarder that old shortcuts start (decision astromech-nade).

- Spike (astromech-wgtr) passed on Windows CI before migration: the tray works windowless, through renderer and browser kills and after close; closing the window ends WebView2; Raw Input still arrives after Tauri's `build`. Restart registration restarted no crash on the runner.
- One Raw Input registration per process (astromech-7i2k), started after `build`; the onboarding smoke's kill-switch runs (F8 tap and 1.2 s hold, mock game started before and after the app) pass in the Release build.
- The one app (astromech-v3u4): Start with Windows makes no window, closing destroys it, a second start focuses it or opens `--route`; `open_window` replies before the build so a cold WebView2 start can't time out a caller.
- Installer, updater, Start with Windows and the upgrade (astromech-9lh3): the Release build's installer smoke and the new upgrade job (from v1.0.0-alpha.4, by `setup.exe /S` and by alpha.4's own update helper) pass at 916c97c3.
- webview_cost.py measures the one app (astromech-xz7i). At 916c97c3: 7.2 MB windowless, 0.27% idle CPU, no WebView2 process left after five closes or after either kill (receipt astromech-osyy, success criterion one-process).
- No automatic crash restart (decision astromech-lkwj, Sumeet): astromech-dl53 and the probe script are removed; a crash leaves a marker and the next start offers Save diagnostics.
- Merged main's world mode (astromech-vucf) and yis2 in 617410f7, with fixes in 916c97c3. A fresh reviewer approved it (astromech-gput). All 9 PR #218 checks passed at 916c97c3.

Left for Sumeet at his PC: the S11 play check next to Elite (astromech-v3u4) and his own install updating after the next release (astromech-9lh3). Follow-ups: astromech-ttq7, astromech-vzwk, astromech-y2m1.

## Background

What the code settles, and what the spike (astromech-wgtr) settles:
- **tao and the tray with no window.** From the code: Tauri's tray uses the same `tray-icon` and `muda` crates. tao registers keyboard and mouse Raw Input when it builds the loop, so the input source starts after `build`. The spike settles, on Windows, that the tray works windowless, through window crashes and after a close, and whether an app-owned Raw Input listener survives.
- **Idle cost.** The spike measures it against a plain-tray baseline: reported, not gated (astromech-01ft).
- **`core.json`.** It stays as the control API's endpoint, and the window stops using it. This is settled by decision astromech-nade; the spike is not needed for it.
- **Restart registration.** The spike tried it; no crash was restarted on the runner, and crash restart was then dropped altogether (decision astromech-lkwj).

## Notes

- 2026-11-07 [decision] Removed the blocked_by links between y6d6's children (7i2k on wgtr; v3u4 on 7i2k and wgtr; dl53, 9lh3 and xz7i on v3u4); external ones stay (7149, nade, lkwj); the order is kept by the orchestrator
  Reason: Under Atelier's new lifecycle (astromech-zk7z) a sibling blocker on one branch can never clear before the owner merges
- 2026-11-07 [decision] No automatic crash restart; astromech-dl53 and astromech-fmub superseded, decision astromech-lkwj decided
  Reason: Sumeet, 7 Nov 2026: a crash is a bug to fix, not to paper over; restart is either unused or a band-aid
