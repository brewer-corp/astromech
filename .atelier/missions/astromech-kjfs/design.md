---
status: draft
last_changed: "2026-11-07: no crash restart (Sumeet, decision astromech-lkwj)"
---

# Shape

- **One process** (decision astromech-6i8s, superseding ARCHITECTURE §18's process model): the
  tray, the pipeline, voice, input and the window live in one Tauri app. The speech worker
  stays a separate supervised process. Closing the window
  destroys it, so WebView2 exits. Start with Windows never creates a window. Nothing restarts the
  app after a crash: a crash is a bug to fix, and the next start offers Save diagnostics
  (Sumeet, 7 Nov 2026; decision astromech-lkwj).
- **One command layer in a crate**: every way in (Tauri `invoke`, the TCP control API, and
  later WASM) calls the same handlers. Nothing in the window process bypasses it.
- **The core is headless-capable**: one binary runs without a window on Linux for CI, the dev
  bridge and the journey suite. Windows-only input and audio are behind traits with fakes.
- **Testing** (decision astromech-1zhg): test doubles only at true boundaries (game files,
  network, clock, input devices, audio, the clipboard). Fixtures are recorded raw inputs, never
  invented replies. Each case is tested once, at the lowest layer that fails. No tests that
  only mirror wiring, copy or static markup. UI behaviour tests run in real Chromium (Vitest
  browser mode) behind the one transport. A small journey suite runs end to end in CI.
  Algorithm tests stay in Rust. Visual changes are proven with before-and-after captures.
- **Determinism is enforced, not hoped for**: the core reads time through a clock the tests
  control. Tests wait on events, never sleep or poll, and are never retried. A flaky test is
  fixed or quarantined against an issue. Lint enforces what it can.
- When qualities conflict: copilot reliability while playing (kill switch, voice, input) wins
  over window features; then determinism of the tests over their coverage.

# Risks and Unknowns

- Tray and window in one Tauri event loop: the tray must keep working with no window and
  through window crashes. Spiked first in the single-process increment, before any migration.
- Whether destroying the window really frees WebView2's processes on Windows. Checked on
  Windows CI before the merge lands; memory is reported along the way.
- Core memory: the neutron star file and the scan caches move into the always-on process.
  They load on first use and are dropped when idle.
- A fault in the window side now shares a process with the kill switch. Commands run behind
  `catch_unwind` on worker threads; a native crash (SQLite, WebView2's host side) takes the
  process down; the next start offers its report, and the cause gets fixed. Speech stays isolated
  in its worker.
- Putting every window command on the control API widens what a local client can do; which
  commands pass `astromech-policy` is settled in the command layer increment.
- The headless core needs Linux stand-ins for sending keys, the keyboard hook and audio. Some
  exist for tests. Which are missing is found in the headless increment.
