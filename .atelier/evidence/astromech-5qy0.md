---
created_at: "2026-10-17T23:35:08.248992082+00:00"
id: "astromech-5qy0"
evidence_type: "validation"
commit: "123164dc47f6c6be09f4a0546cc46d3a7ffa3fed"
result: "pass"
captured_at: "2026-10-17T23:35:08.248943542+00:00"
uri: "https://github.com/spadaval/astromech/actions/runs/39062129341"
agent_identity: "Claude (author, agent-factory implement)"
target:
  kind: "issue"
  id: "astromech-08w8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-08w8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Full-size MVP criteria run vs the mock game (scripts/mvp_criteria.py at ffe94d3, release core, windows-latest 4 CPUs, run 39062129341 on a throwaway branch with full-size defaults; manual dispatch needs the workflow on main): (1) verified success 200/200 for each of 11 T1 tools (gear, lights, hardpoints, cargo scoop, night vision, flight assist, silent running, HUD mode, open screen x6 + close, SRV handbrake, SRV drive assist) vs >= 99%: PASS; (2) 0 unintended key-downs and 0 keys left held in a 240.4 min soak with 7204 journal lines replayed and 3532 key-downs vs zero: PASS; (3) kill switch: 50/50 kills, max 1.2 ms from SendInput of the bound key to the last held key's key-up (key plus modifier), all 50 verifications cancelled, 0 presses while killed vs <= 50 ms: PASS; (4) action start estimate 396.4 ms p95 vs <= 400 ms (measured PTT release handled p95 218.5 ms, 733 presses; command to key-down p95 13.9 ms, 2200 presses; plus post-roll tail 50, mic 10, STT 104 from S9): PASS by 3.6 ms, estimate only, no margin (finding astromech-e0zz). Limits: keyboard only (no mouse 4/5, HOTAS, gamepad), typed commands not speech, mock not the game, no pips presets tool, recognition accuracy not covered."
updated_at: "2026-10-17T23:35:08.249320385+00:00"
---

Full-size MVP criteria run vs the mock game (scripts/mvp_criteria.py at ffe94d3, release core, windows-latest 4 CPUs, run 39062129341 on a throwaway branch with full-size defaults; manual dispatch needs the workflow on main): (1) verified success 200/200 for each of 11 T1 tools (gear, lights, hardpoints, cargo scoop, night vision, flight assist, silent running, HUD mode, open screen x6 + close, SRV handbrake, SRV drive assist) vs >= 99%: PASS; (2) 0 unintended key-downs and 0 keys left held in a 240.4 min soak with 7204 journal lines replayed and 3532 key-downs vs zero: PASS; (3) kill switch: 50/50 kills, max 1.2 ms from SendInput of the bound key to the last held key's key-up (key plus modifier), all 50 verifications cancelled, 0 presses while killed vs <= 50 ms: PASS; (4) action start estimate 396.4 ms p95 vs <= 400 ms (measured PTT release handled p95 218.5 ms, 733 presses; command to key-down p95 13.9 ms, 2200 presses; plus post-roll tail 50, mic 10, STT 104 from S9): PASS by 3.6 ms, estimate only, no margin (finding astromech-e0zz). Limits: keyboard only (no mouse 4/5, HOTAS, gamepad), typed commands not speech, mock not the game, no pips presets tool, recognition accuracy not covered.
