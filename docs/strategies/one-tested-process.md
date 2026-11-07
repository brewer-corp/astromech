---
status: draft
last_changed: "2026-11-07: no memory bound in one-process (Sumeet, decision astromech-01ft)"
---

# Why

astromech is two processes that each do half the work. The core runs the live pipeline, voice,
input, the route follower and the tray. The Tauri shell, meant to be thin, registers 52
commands, 48 of them its own: journal scans, keybind edits, Spansh lookups and route plotting. So every
cache and the 184 MB neutron star file have two owners, and the "one API for every client"
rule (ARCHITECTURE §18) is broken by the shell itself. Testing matches the split. The window
is tested against hand-written fixtures behind an `inTauri` check in about 26 modules. The
fixtures drift from the real core, and the fake ships in the release bundle. No test drives a
user journey from the journal to the page. Every UI change waits on a Windows build or a hand
check at Sumeet's PC.

# Outcome

astromech is one Tauri process. A single command layer serves the window, the control API and
a headless mode. Agents and CI exercise the real core: from a scripted game world (game files,
recorded network, a controlled clock) through the pipeline to the page in Chrome, with faults
as ordinary inputs. The window reaches its backend through one transport. The fake transport
lives apart from the app, never ships, and serves only UI behaviour tests and the static
preview link. A small automated suite of user journeys runs on every push. Worth shipping on
its own: the transport seam and the command layer, even before the single-process merge.

# Success Criteria

- one-process: The installed app is one app process plus its supervised speech worker
  (`astromech-voice-worker`, which stays separate), and with the window closed no WebView2
  process is left. Shown on Windows CI. Memory is reported, not bounded (decision
  astromech-01ft): next to Elite and a browser it is small.
- one-command-list: Every command the window uses is served by the command layer and reachable
  over the control API and the headless mode. The shell holds only the window, the tray and
  startup. Shown by a test that lists the window's commands and calls each one through the
  control API of a headless core.
- journeys-in-ci: The journey suite (at least plot and follow, jump to the next stop and copy
  it, and a lookup failing with its error shown) runs in CI on every push against the real
  headless core and a scripted world, with no sleeps, no wall-clock reads and no retries.
  Shown by CI runs on the mission's last merged commit.
- no-fake-in-release: The release bundle contains no preview transport or fixtures. Shown by a
  build check that fails if a fixture string appears in `app/dist`.

# Shape

- **One process** (decision astromech-6i8s, superseding ARCHITECTURE §18's process model): the
  tray, the pipeline, voice, input and the window live in one Tauri app. The speech worker
  stays a separate supervised process. Closing the window
  destroys it, so WebView2 exits. Start with Windows never creates a window. Crash restart
  comes from Windows' restart registration, or a minimal watchdog if that can't cover it.
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

# Scope

In: the transport seam and the preview's move out of the app; the command layer; the
single-process merge, with tray, updater, installer and Start with Windows; the headless mode;
the world seam (scripted game folder, HTTP record and replay, clock, fault inputs) and a dev
bridge so the browser preview can run the real core; the journey suite in CI; a Windows smoke
test of the packaged app at release; the testing rules in AGENTS.md and CONTRIBUTING.md;
moving UI tests to browser mode; ARCHITECTURE.md updated to the new model.

Out: compiling the core to WASM for the static preview (its own backlog item once the command
layer exists); Electron or any other shell; the Route redesign (astromech-ok29), which
continues alongside and uses the transport seam; new user features.

Environment: Windows 10 and 11 with WebView2, next to Elite Dangerous and VR, for the shipped
app. Linux (this host and GitHub's ubuntu runners) for the headless core, the preview and CI.

# Authority

Sumeet set the outcome and the two decisions in conversation on 6 Nov 2026, and delegated plan
approval to an independent agent review. An orchestrator may change the increment breakdown,
sequencing, crate and module names, the record-and-replay format, and which journeys the
suite holds beyond the three named above. These must come back to Sumeet: changing the outcome
or success criteria; dropping the journey suite or the single-process merge; anything that
changes what the shipped app does while playing;.

# Risks and Unknowns

- Tray and window in one Tauri event loop: the tray must keep working with no window and
  through window crashes. Spiked first in the single-process increment, before any migration.
- Whether destroying the window really frees WebView2's processes on Windows. Checked on
  Windows CI before the merge lands; memory is reported along the way.
- Core memory: the neutron star file and the scan caches move into the always-on process.
  They load on first use and are dropped when idle.
- A fault in the window side now shares a process with the kill switch. Commands run behind
  `catch_unwind` on worker threads; a native crash (SQLite, WebView2's host side) takes the
  process down, which crash restart must cover. Speech stays isolated in its worker.
- Putting every window command on the control API widens what a local client can do; which
  commands pass `astromech-policy` is settled in the command layer increment.
- The headless core needs Linux stand-ins for sending keys, the keyboard hook and audio. Some
  exist for tests. Which are missing is found in the headless increment.
