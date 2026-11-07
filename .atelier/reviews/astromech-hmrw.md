---
schema: atelier.review
schema_version: 2
id: astromech-hmrw
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:25.559051582Z
event:
  event: finding
  recommendation: fix_now
---

high: astromech-6i8s Decision and strategy one-process criterion The decision says 'the core already hosts the riskiest native code (sherpa-onnx, SQLite)'. It doesn't: sherpa-onnx runs in a separate astromech-voice-worker process (crates/astromech-voice bin, externalBin in app/src-tauri/tauri.conf.json, scripts/stage-sidecars.sh; ARCHITECTURE 5.10 'one crash-isolated helper keeps the main exe free of native ML code'; V1-DESIGN 7.7 'supervised child'). So the fault-isolation rationale overstates the current risk profile, and the criterion 'The installed app is one process' is false as worded if the worker stays. Fix: correct the rationale; state in 6i8s, the strategy Shape and one-process that the voice worker remains a supervised helper process (or decide otherwise with Sumeet).
