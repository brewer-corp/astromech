---
created_at: "2026-11-07T06:33:38.677563981+00:00"
id: "astromech-unh9"
issue_type: "decision"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T06:33:51.739712362+00:00"
status: "done"
title: "Command layer: which commands a local control client and the window may call, and policy's role"
updated_at: "2026-11-07T06:33:51.739712362+00:00"
---

## Question

The command layer puts every window command on the core's local control API (astromech-bp23). Which commands may a local control-API client call, which may the window's page call, and which go through `astromech-policy`? Today the shell allow-lists the core commands its `core_call` forwards (app/src-tauri/src/window.rs:1205-1264), while the control API itself admits any command to a caller holding the token in `core.json` (crates/astromech-core/src/control.rs:98-109). The strategy leaves this to the command-layer increment (docs/strategies/one-tested-process.md, Risks).

## Options

- Every command passes `astromech-policy`, as ARCHITECTURE §18.2 planned ("Commands always pass through astromech-policy"). But `Policy` authorizes game key presses by tier, origin, rate and kill switch (crates/astromech-policy/src/lib.rs:1-12, `authorize` at :172). A keybind edit, a filter save or a Spansh lookup has no tier and presses nothing, so a policy check would be a tier table nothing reads.
- A separate allow-list per client. The control API has one client kind: same-user processes holding the token.
- One exposure per command in the registry: `window` (the page and local control clients) or `local` (local control clients only). Game key presses keep going through the policy inside the handler that presses, as now.

## Decision

The third option, decided by the orchestrator under the strategy's Risks line, which says which commands pass astromech-policy is settled in the command-layer increment.

- Each registered command is `window` or `local`. `local` covers what the window must not do today: `quit` and `open_window`, which stay with the tray and a second start. Every other command the window uses is `window`.
- The shell's proxy sets `origin: "app_window"` (`astromech_proto::Origin::AppWindow`) on every request it forwards, and the registry refuses a `local` command for that origin with "not allowed", as `core_call` does today. The page can't choose the origin. The proxy sets it. The registry's exposure replaces the shell's `ALLOWED` list, so there is one list.
- A request without an origin comes from a local control client. Today those are: a second start (crates/astromech-core/src/app.rs:1226), `--quit` from the installer (main.rs:131), the single-instance check off Windows (instance.rs:41), scripts/onboarding_smoke.py, and the Release workflow's PowerShell smoke step (.github/workflows/release.yml:191-200). Later the dev bridge joins them. The tray runs inside the core and calls `Core` directly, not the control API. A local control client may call every registered command. An origin is a restriction the honest proxy applies for the page, not authentication. So a missing origin means local: an explicit `Origin::Local` was considered and left out, because the token, not the origin, is the gate, and anyone holding it could send any origin. Its gate stays the per-run token in `core.json`, which only this user can read. The new commands it can reach (keybind file edits with backups, saved filters, lookups, opening a URL or folder that `open_external` allows) are things a process running as this user can already do to the same files and network. So the control API's boundary is unchanged in kind. It already admits `kill_switch`, `set_ptt`, `install_update` and `voice_test_live` today.
- `astromech-policy` stays the only way to the key executor. The one window command that presses keys, `voice_test_live`, already goes through it with `Origin::AppWindow` (crates/astromech-core/src/tester.rs:282). No other window command presses anything, so none gains a policy check.

Consequences: ARCHITECTURE §18.2's sentence about commands and policy is rewritten to say this. A future client with weaker trust than a same-user process (a LAN dashboard, cloud) needs its own exposure level and is out of this mission's scope. The control API stays JSON lines. A WebSocket API is for a browser client that connects directly, and nothing in this mission does that: the dev bridge is Vite middleware in Node.

## Close Reason

Each command is window or local (local: quit, open_window); the proxy stamps origin app_window; token-holding local clients may call every command; astromech-policy stays the only way to the key executor. Decided by the orchestrator under the strategy's Risks line ('which commands pass astromech-policy is settled in the command layer increment'); plan review astromech-bp23 found it within orchestrator authority.
