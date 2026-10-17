---
created_at: "2026-10-17T21:41:27.787064292+00:00"
id: "astromech-yywu"
evidence_type: "validation"
commit: "b87eecfd7f3311b7b5ff7fcfd19645d2580014bc"
result: "pass"
captured_at: "2026-10-17T21:41:27.786963604+00:00"
path: "docs/evidence/astromech-yil0/report.md"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-yil0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-yil0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "scripts/voice-eval.sh ran end to end on Linux (worker built with --features sherpa; Parakeet v2 int8 + Kokoro v1.0 fp32 fetched by the existing SHA-256-checked downloader into ~/.cache/astromech/models). 200 phrases x 4 voices (af_heart, am_michael, bf_emma, bm_george) x 4 conditions (clean, cockpit, noise 10 dB pink, cockpit+noise) = 3200 clips: execute-or-ask 3105/3136 (99%), wrong execute 0, miss 23, reject 64/64. Every family >= 95% (lowest: status 96.5%, flight 96.2%, confirm 96.9%). Commands below 95%: throttle.zero 81.2% (astromech-y296), confirm 93.8% (astromech-bztt), status.legal 87.5%, hyperspace_jump 90.6%, status.fuel/credits/shields/ship and galaxy_map.paste 93.8% (astromech-xi2i). Grammar-only (phrase text): 196/196, 4/4 rejects. Limits: synthetic voices, not real VR clips (astromech-cgrw)."
updated_at: "2026-10-17T21:41:27.787763544+00:00"
---

scripts/voice-eval.sh ran end to end on Linux (worker built with --features sherpa; Parakeet v2 int8 + Kokoro v1.0 fp32 fetched by the existing SHA-256-checked downloader into ~/.cache/astromech/models). 200 phrases x 4 voices (af_heart, am_michael, bf_emma, bm_george) x 4 conditions (clean, cockpit, noise 10 dB pink, cockpit+noise) = 3200 clips: execute-or-ask 3105/3136 (99%), wrong execute 0, miss 23, reject 64/64. Every family >= 95% (lowest: status 96.5%, flight 96.2%, confirm 96.9%). Commands below 95%: throttle.zero 81.2% (astromech-y296), confirm 93.8% (astromech-bztt), status.legal 87.5%, hyperspace_jump 90.6%, status.fuel/credits/shields/ship and galaxy_map.paste 93.8% (astromech-xi2i). Grammar-only (phrase text): 196/196, 4/4 rejects. Limits: synthetic voices, not real VR clips (astromech-cgrw).
