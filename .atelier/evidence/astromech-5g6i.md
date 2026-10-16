---
created_at: "2026-10-16T20:49:48.251579013+00:00"
id: "astromech-5g6i"
evidence_type: "validation"
commit: "b7f8ebf10ba90a43749644253f10789ef81d8c5f"
result: "pass"
captured_at: "2026-10-16T20:49:48.251530883+00:00"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-sd56"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-sd56"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "UI check (scripts/ui, headless Chrome, Vite preview at #/trade, My cargo): the hold line lists 300 t LTDs, 200 t Painite, 100 t Platinum and what is left out (stolen gold, limpets, mission gold); whole-load stations best total first with 'takes all 600 t' or 'takes 500 of 600 t'; 'Best for each on its own' lists each commodity's station, price, total and demand. Screenshots read: trade-cargo.png, trade-cargo-each.png. Limits: preview answers from the recorded fixture plan; the Tauri command trade_cargo builds only on Windows CI; real Cargo.json only in game."
updated_at: "2026-10-16T20:49:48.251914760+00:00"
---

UI check (scripts/ui, headless Chrome, Vite preview at #/trade, My cargo): the hold line lists 300 t LTDs, 200 t Painite, 100 t Platinum and what is left out (stolen gold, limpets, mission gold); whole-load stations best total first with 'takes all 600 t' or 'takes 500 of 600 t'; 'Best for each on its own' lists each commodity's station, price, total and demand. Screenshots read: trade-cargo.png, trade-cargo-each.png. Limits: preview answers from the recorded fixture plan; the Tauri command trade_cargo builds only on Windows CI; real Cargo.json only in game.
