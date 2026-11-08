---
schema: atelier.review
schema_version: 2
id: astromech-gput
title: approval on astromech-y6d6 @916c97c3
target: astromech-y6d6
identity: claude-reviewer-y6d6-merge
commit: 916c97c3d378b97645c35f5e0085e265eb6510ee
created_at: 2026-11-08T15:37:27.522697087Z
event:
  event: approval
---

Reviewed 571fddcd, ee3fa65c, 617410f7 (with git show --remerge-diff) and 916c97c3 at head 916c97c3; no finding medium or worse, two low deferred (astromech-3rf9, astromech-4r9s). Crash restart: nothing registers for restart; no RegisterApplicationRestart, --restarted, one_process_probe or watchdog left in code, scripts or .github/workflows; V1-DESIGN 7.7 and 7.10, ARCHITECTURE 18.1, the one-tested-process strategy and spikes/README agree that nothing restarts after a crash and the next start offers Save diagnostics; crash.rs's comment matches its code (logs, writes the marker, aborts only on the main thread, caught command panics leave no marker); the speech worker's supervision stays. ee3fa65c: the release-smoke line is on y6d6 and yis2 notes the move. Merge: every conflict hunk keeps both sides. on_status keeps main's status_reads count, EDDN BodyName and clock-read callouts inside this branch's whole-file read; quit keeps main's after_reply with ui.quit(); install_update keeps main's clock.sleep; control.rs keeps the public window_request and main's call_window_to; window.rs's only main-side change was clippy allows, now carried in host.rs. Post-merge: WorldUi records window {route} as main's closure did and sends refresh and quit to the same tray() loop; --world is dispatched in cli::main before the app, and --webview-debug-port now conflicts with it; the six binary tests and support moved to app/src-tauri/tests with ../../ paths that resolve to the same root; nothing in .github, scripts, app/package.json, app/dev or the docs still names the old paths or -p astromech-core --test; Linux and Windows CI's cargo test --workspace runs them, and having integration tests is what makes cargo test build target/debug/astromech-core for bridge-check and the journeys. Not run: builds or tests (the caller reports cargo test, clippy for Linux and Windows, pnpm check, lint, test and the journeys passing); nothing on Windows. The merge commit alone doesn't build (fixed in 916c97c3, as its message says).
