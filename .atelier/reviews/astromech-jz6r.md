---
schema: atelier.review
schema_version: 2
id: astromech-jz6r
title: approval on astromech-y6d6 @f19adab5
target: astromech-y6d6
identity: claude-reviewer-7i2k
commit: f19adab5f35eae79f75396ae144710964cc6d453
created_at: 2026-11-07T17:15:14.324760045Z
event:
  event: approval
---

Re-checked the fixes at f19adab5f35eae79f75396ae144710964cc6d453. 72ox: fmub is on main (424b8a18) with the PowerShell form ($env:LOCALAPPDATA) and the cmd form noted, related to v3u4; the branch copy has the same command. 6wwr: 9lh3, xz7i and dl53 carry related links to v3u4. yzyz is deferred with a note on v3u4 naming source.rs:99-104; cw1y is no_action. Earlier review, at 4fd16542: 7i2k's shared input source (source.rs, rawinput.rs restart via WM_APP+1, controls.rs, main.rs --listen, window.rs start_input) leaves session.rs's bus, Mapper timing, debounce and rearm_hold unchanged, and uses one epoch for t_ms and elapsed. A slow subscriber can't block (unbounded sends). Device replay and disconnect are race-free. Proof read: CI run 42384570629 (source_sendinput ok, Windows clippy, tests and shell build) and Release run 42384570600 (onboarding smoke PASS with both kill-switch runs). Locally: astromech-input tests, clippy -D warnings, fmt. New finding: an add/add conflict on fmub at the next main merge (deferred). The y6d6 Done when lines remain open by design.
