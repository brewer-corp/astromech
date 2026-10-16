---
created_at: "2026-10-16T18:40:52.563876796+00:00"
id: "astromech-emzr"
evidence_type: "validation"
commit: "097183b0982db06aa358f950abdadc75306b2935"
result: "pass"
captured_at: "2026-10-16T18:40:52.563819579+00:00"
target:
  kind: "issue"
  id: "astromech-txzq"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-txzq"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Timed check on Linux: the shell's materials_inventory code, copied verbatim from app/src-tauri/src/main.rs into a scratch crate (the shell needs webkit2gtk, which isn't installed), polled every 1000 ms like the Materials page, while a writer kept a temp journal open and appended synthetic events at offsets that don't line up with the poll. Append-to-visible latency: MaterialCollected 712 ms, MaterialDiscarded 334, MaterialTrade 966, EngineerCraft 591, Synthesis 220, TechnologyBroker 849, MissionCompleted 482, full Materials 108; worst 966 ms against a 2000 ms budget. Release-build recompute for an 8 MB session journal is 157 ms; a cache hit is about 0.1 ms. Not covered: a real WebView, and Windows NTFS behaviour with the game as the writer (astromech-rggm)."
updated_at: "2026-10-16T18:40:52.564319251+00:00"
---

Timed check on Linux: the shell's materials_inventory code, copied verbatim from app/src-tauri/src/main.rs into a scratch crate (the shell needs webkit2gtk, which isn't installed), polled every 1000 ms like the Materials page, while a writer kept a temp journal open and appended synthetic events at offsets that don't line up with the poll. Append-to-visible latency: MaterialCollected 712 ms, MaterialDiscarded 334, MaterialTrade 966, EngineerCraft 591, Synthesis 220, TechnologyBroker 849, MissionCompleted 482, full Materials 108; worst 966 ms against a 2000 ms budget. Release-build recompute for an 8 MB session journal is 157 ms; a cache hit is about 0.1 ms. Not covered: a real WebView, and Windows NTFS behaviour with the game as the writer (astromech-rggm).
