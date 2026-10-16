---
created_at: "2026-10-16T18:43:21.359159310+00:00"
id: "astromech-9s05"
evidence_type: "validation"
commit: "6cfe4df3da000a4e128e9aea0dfdc26c1e63f012"
result: "pass"
captured_at: "2026-10-16T18:43:05.724645738+00:00"
command: "env LD_LIBRARY_PATH=/home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug /home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug/examples/render-voices --worker /home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug/astromech-voice-worker --models /tmp/kokoro-v1/x --out /tmp/kokoro-v1/render-british"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-a50f"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-a50f"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Real TTS path (TtsSpeaker -> astromech-voice-worker built on Linux with sherpa-onnx 1.13.8 shared libs -> Cockpit colour -> WAV cache) renders 'Landing gear deployed.' with all 8 British Kokoro v1.0 voices from the fp32 kokoro-multi-lang-v1_0 model, to /tmp/kokoro-v1/render-british/<voice>.wav (outside the repo)"
updated_at: "2026-10-16T18:43:21.359597457+00:00"
---

## Summary

Real TTS path (TtsSpeaker -> astromech-voice-worker built on Linux with sherpa-onnx 1.13.8 shared libs -> Cockpit colour -> WAV cache) renders 'Landing gear deployed.' with all 8 British Kokoro v1.0 voices from the fp32 kokoro-multi-lang-v1_0 model, to /tmp/kokoro-v1/render-british/<voice>.wav (outside the repo)

## Command

```console
env LD_LIBRARY_PATH=/home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug /home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug/examples/render-voices --worker /home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug/astromech-voice-worker --models /tmp/kokoro-v1/x --out /tmp/kokoro-v1/render-british
```

Exit status: 0

## Stdout

Bytes: 848
Truncated: no

```text
bf_alice (sid 20): /tmp/kokoro-v1/render-british/bf_alice.wav (93680 bytes, 1.95 s at 24000 Hz) in 2.4 s
bf_emma (sid 21): /tmp/kokoro-v1/render-british/bf_emma.wav (85842 bytes, 1.79 s at 24000 Hz) in 1.9 s
bf_isabella (sid 22): /tmp/kokoro-v1/render-british/bf_isabella.wav (89432 bytes, 1.86 s at 24000 Hz) in 1.8 s
bf_lily (sid 23): /tmp/kokoro-v1/render-british/bf_lily.wav (104204 bytes, 2.17 s at 24000 Hz) in 1.8 s
bm_daniel (sid 24): /tmp/kokoro-v1/render-british/bm_daniel.wav (97668 bytes, 2.03 s at 24000 Hz) in 1.9 s
bm_fable (sid 25): /tmp/kokoro-v1/render-british/bm_fable.wav (101448 bytes, 2.11 s at 24000 Hz) in 2.0 s
bm_george (sid 26): /tmp/kokoro-v1/render-british/bm_george.wav (98488 bytes, 2.05 s at 24000 Hz) in 1.9 s
bm_lewis (sid 27): /tmp/kokoro-v1/render-british/bm_lewis.wav (85334 bytes, 1.78 s at 24000 Hz) in 1.9 s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

