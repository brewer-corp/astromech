---
created_at: "2026-11-07T06:33:38.679868984+00:00"
id: "astromech-i8vr"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7rcn"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-se44"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T09:42:05.819845708+00:00"
status: "done"
title: "One shell proxy for every window call; forwarders, core_call and core_status gone"
updated_at: "2026-11-07T09:42:05.819845708+00:00"
---

## Goal

The window reaches every command through one shell proxy, and the shell's per-command forwarders, `core_call` and `core_status` are gone. The shell then holds only the window, startup and its link to the core: the proxy, the core start, the watchdog and the input listener, which the single-process increment (astromech-y6d6) removes.

## Done when

- [x] The shell registers one Tauri command, `call(cmd, args, progress)`. It forwards any command to the core with `origin: "app_window"` and no read timeout, resolves with `value`, rejects with `error`, and forwards progress lines to the channel. `generate_handler!` lists only it.
- [x] The shell's direct dependencies are tauri, tauri-plugin-single-instance, serde, serde_json, anyhow, astromech-api (the control client), astromech-store (`Paths`), astromech-log, astromech-keybinds (the log `Redactor`, window.rs:1320) and astromech-input (the input listener, window.rs:1145-1195). It drops astromech-materials, -journal, -engineering, -ship, -data, -galaxy, -market, -colonisation, -history, -odyssey, -station, -powerplay and -exploration.
- [x] `app/src/lib/transport.ts` (from astromech-xil2) sends every call through it. The `CORE_COMMANDS` split, the `text`-string special case if the wire dropped it, and the `CHANNELS` table go. A progress command's topic comes from the generated command map. `core_status` callers use `status`, with a failed call meaning the core isn't running, as `core_status` returning null means now. `powerplay_targets` sends `within_ly`.
- [ ] scripts/onboarding_smoke.py opens every top-level page of the installed app after the wizard (the modules in app/src/modules) and fails when a page shows "not allowed", "unknown command", an internal error or "the core is not running". It passes in the Release workflow on the PR. The core log it uploads shows the star map's open line (from the galaxy-move child) naming the installed `starmap.db` as the preload.
- [x] ARCHITECTURE.md §18.2 describes the registry in astromech-api, the exposure levels and policy's role (exposure decision), JSON lines with progress lines on the wire, and ts-rs types. The process-model text stays for astromech-y6d6.
- [ ] `cd app && pnpm -s check && pnpm -s lint && pnpm -s test` and the Rust checks pass, and so does Windows CI's shell build.

## Scope

window.rs, app/src/lib/transport.ts and the few lib modules that named shell-only commands, scripts/onboarding_smoke.py, ARCHITECTURE §18.2. The fake transport keeps its handlers, typed in the next child.

## Notes

- 2026-11-07 [note] From finding astromech-tabr (ndzx review), must be fixed here before bp23 merges: shell_info is now answered by the core, so Diagnostics (app/src/modules/diagnostics/View.svelte:29,49-51) can no longer show 'window X, background Y' after a half-applied update, and with the core down it shows '…'. Give the window its own version (built into the bundle, e.g. a Vite define from the workspace version, since it ships inside the shell exe), or drop the comparison only if y6d6 makes it moot — keep it until then.
- 2026-11-07 [decision] A command's progress is published on the topic named after the command (neutron_download), typed by Progress<C> over the generated map's progress; the Tauri transport gives every call a channel and publishes what arrives under the command's name.
  Reason: The generated map is types only, so a runtime topic table would be a second hand list; the command's name is the map's key, so the topic and its payload type come from the map with nothing to keep in step.
  Rejected: Keep the neutron_progress topic with a generated runtime table of topics
- 2026-11-07 [decision] The wire keeps text, so a string args is still the request's text; the mapping is in astromech_api::control::call_window, which the shell's proxy calls, and the transport sends args unchanged.
  Reason: One place, testable without Tauri (control::tests::the_windows_call_is_stamped_app_window_and_takes_text_as_a_string), and the generated map already types those commands' args as string; no core or wire change.
  Rejected: Teach the registry to read a string args as text
- 2026-11-07 [decision] CORE_COMMANDS and CoreCmd move from lib/transport.ts to lib/core.ts as the core's own commands: coreCall's argument type and the browser preview's routing to its fake core. The transport has no split.
  Reason: The preview's handlers stay as they are until astromech-96dv types call() and the fakes against the generated map; that child can delete the list.
  Rejected: Derive CoreCmd from the generated map now (96dv's scope)
- 2026-11-07 [decision] The window's version is app/package.json's, built in by a Vite define (__APP_VERSION__, lib/core.ts WINDOW_VERSION); Diagnostics compares it with status.version again. The window no longer calls shell_info, which stays registered (bp23 Done when).
  Reason: scripts/set-version.py stamps app/package.json with the release's version alongside the workspace's, and the bundle ships inside the shell's exe, so it is the window's own version with the core down or from another release.
- 2026-11-07 [decision] The Release workflow's PR paths gain app/src/lib/transport.ts and app/src/lib/core.ts, and the smoke also fails on 'bad args for' and 'invalid args'.
  Reason: The smoke now drives every page through the window's half of the proxy, which nothing else exercises against the installed app; a wrongly named arg (the within_ly class) or a Tauri refusal of the proxy's own arguments is the failure this change could introduce.

## Report

Implementer report (commit e1e28513 on feature/astromech-bp23). Outcome: the shell registers one Tauri command, call(cmd, args: Option<Value>, progress: Channel<Value>) -> Result<Value, String>, on spawn_blocking, over astromech_api::control::call_window (new: origin app_window, no read timeout, a string args becomes the request's text, progress lines to the callback); generate_handler![call] only. The 52 forwarders, core_call, core_status, call_with and the blocking/forward helpers are deleted (window.rs 399 lines removed); the watchdog's bounded check is control::call_short(status). The shell's direct deps are tauri, tauri-plugin-single-instance, serde_json, astromech-api, -store, -log, -keybinds, -input (astromech-proto dropped; serde and anyhow are allowed by the Done when line but unused, so not added). transport.ts: every call is invoke('call', {cmd, args: args ?? null, progress: new Channel}); the CORE_COMMANDS split, the text special case and CHANNELS are gone from it; subscribe registers locally (progress, topic = the command's name) and listens for Tauri events. lib/core.ts: coreStatus calls status, null on a failed call; coreCall gives a failed call as {ok: false, error} as core_call did; CORE_COMMANDS/CoreCmd moved here (decision note); WINDOW_VERSION from app/package.json via a Vite define, and Diagnostics compares it with the core's (finding astromech-tabr). neutron.ts: NeutronProgress = Progress<'neutron_download'>, topic neutron_download. powerplay_targets sends within_ly. Preview: status replaces core_status, the topic is neutron_download, powerplay_targets reads within_ly. Tests renamed core_status to status; staleTauri mocks invoke('call', {cmd, args, progress}) with core_down as a rejection 'the core is not running'. New app/src/lib/proxy.browser.test.ts (exempted in eslint.config.js like the other two Tauri-faking tests). onboarding_smoke.py: after the verified gear-down, opens #/<id> for every module id read from app/src/modules/*/manifest.ts (24), waits until the text is still for 1.5 s, fails on 'not allowed', 'unknown command', 'internal error', 'the core is not running', 'bad args for', 'invalid args'; notes Diagnostics' version line; then calls galaxy_systems as a local client and requires a core log line 'star map: ... preload <install dir>\starmap.db' (with %LOCALAPPDATA% redacted as the log writes it); the onboarding total is now the gear-down step's time, so the page tour isn't counted. release.yml PR paths gain transport.ts and core.ts. Docs: ARCHITECTURE §18.2 (registry, exposure and policy per unh9, wire, the window's path, ts-rs types; process-model text untouched), crate tree's shell line, three 'shell command'/'in the shell' slips; PLOTTING, V1-DESIGN and CONTRIBUTING name status.
Behaviour differences the user can see: (1) Diagnostics shows the window's own version again ('window X, background Y' after a half-applied update; 'X (background process not running)' with the core down), where since ndzx both came from the core; in the dev preview the window's version reads 0.0.0-dev, was 'dev'. (2) The status poll has no read timeout now: a core that accepts the connection but never answers leaves the header's poll waiting instead of reading null after 10 s (the watchdog keeps its 10 s check and restarts such a core). Nothing else: the args, replies and errors the pages see are the same.
Proof: cargo test -p astromech-api control 6/6, new the_windows_call_is_stamped_app_window_and_takes_text_as_a_string (seen failing with the origin stamp removed). app/src/lib/proxy.browser.test.ts 4/4: within_ly sent, a string sent as is and none as null, a rejected call gives {ok:false,error} and a null status, progress on the command's topic; all three failing with withinLy, publish('neutron_progress') and coreCall rethrowing. Shell window feature on Linux: cargo clippy -p astromech-shell --features window --all-targets -D warnings clean on an exported copy with fake .pc files, placeholder externalBin and app/dist under /tmp/claude-1000/i8vr-work (removed); a planted &cmd -> cmd type error failed at window.rs:86; cargo tree --depth 1 lists the deps above. Live: real astromech-core --background (debug) under temp XDG/LOCALAPPDATA with an empty starmap.db beside the exe, a scratch example calling control::call_window: status ok, voice_test 'gear down' (text) ok, prospector_set JSON string ok, powerplay_targets within_ly reaches the handler ('journal folder not found'), withinLy gives 'bad args for powerplay_targets: missing field within_ly', quit and open_window 'not allowed', nope 'unknown command', binds_state, galaxy_systems, neutron_check, find_elite ok; log 'star map: opened %LOCALAPPDATA%/astromech/data/galaxy.db, preload .../live/bin/starmap.db'; --quit stopped it. The smoke's new functions run on Linux against that core with a fake Page: 24 ids, a planted 'not allowed' line on powerplay raises with the page named, the preload line found and a wrong install dir raises; py_compile ok. Production build: dist has const ...="0.0.0-dev" and invoke('call',{cmd,args,progress}), no core_call/core_status/neutron_progress. Screenshots (scripts/ui, dev preview ?core=ready, 1200x800, all 24 pages plus Diagnostics with no core) before and after: 23 pages byte-identical; Diagnostics differs only in 'window dev' -> 'window 0.0.0-dev' (/tmp/claude-1000/i8vr-work/shots/{before,after}). cargo test --workspace 1300 passed 0 failed; cargo clippy --workspace --all-targets -D warnings clean; cargo fmt --all --check clean; check-deps OK (32 crates); check-guide OK; cargo deny ok. cd app && pnpm -s check (0 errors), pnpm -s lint clean, pnpm -s test 74 files 532 tests pass.
Open items: (1) Windows CI and the Release workflow on the PR only: the shell's window build and clippy on Windows; the onboarding smoke's page tour and preload line against the installed app (Done when lines 4 and 6 stay unticked until they pass); stand-ins above. (2) shell_info stays registered for bp23's Done when but the window no longer calls it; astromech-ro7a's test or y6d6 decides whether it goes. (3) CORE_COMMANDS now lives in lib/core.ts for coreCall's type and the preview's routing; astromech-96dv can replace it with the generated map. (4) A page's change outside transport.ts/core.ts still doesn't start the Release workflow on a PR, so the page tour runs only when packaging, the proxy or the wizard changes. (5) The smoke's pattern list is substrings of the page text; a page that someday shows one of them legitimately would need it narrowed. Review outstanding.
