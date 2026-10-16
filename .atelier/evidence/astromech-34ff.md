---
created_at: "2026-10-16T18:55:35.547654448+00:00"
id: "astromech-34ff"
evidence_type: "validation"
commit: "d8b2724a885aa7afec1f60f49a3538928cde901e"
result: "pass"
captured_at: "2026-10-16T18:54:55.830560604+00:00"
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
title: "After review fixes: real TTS path still renders all 8 British voices from a completed (marker-present) fp32 v1.0 download in /tmp/kokoro-v1/x to /tmp/kokoro-v1/render-british"
updated_at: "2026-10-16T18:55:35.547895027+00:00"
---

## Summary

After review fixes: real TTS path still renders all 8 British voices from a completed (marker-present) fp32 v1.0 download in /tmp/kokoro-v1/x to /tmp/kokoro-v1/render-british

## Command

```console
env LD_LIBRARY_PATH=/home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug /home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug/examples/render-voices --worker /home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug/astromech-voice-worker --models /tmp/kokoro-v1/x --out /tmp/kokoro-v1/render-british
```

Exit status: 0

## Stdout

Bytes: 849
Truncated: no

```text
bf_alice (sid 20): /tmp/kokoro-v1/render-british/bf_alice.wav (93600 bytes, 1.95 s at 24000 Hz) in 12.0 s
bf_emma (sid 21): /tmp/kokoro-v1/render-british/bf_emma.wav (85824 bytes, 1.79 s at 24000 Hz) in 5.2 s
bf_isabella (sid 22): /tmp/kokoro-v1/render-british/bf_isabella.wav (89432 bytes, 1.86 s at 24000 Hz) in 4.1 s
bf_lily (sid 23): /tmp/kokoro-v1/render-british/bf_lily.wav (104118 bytes, 2.17 s at 24000 Hz) in 4.7 s
bm_daniel (sid 24): /tmp/kokoro-v1/render-british/bm_daniel.wav (97088 bytes, 2.02 s at 24000 Hz) in 3.2 s
bm_fable (sid 25): /tmp/kokoro-v1/render-british/bm_fable.wav (101510 bytes, 2.11 s at 24000 Hz) in 3.8 s
bm_george (sid 26): /tmp/kokoro-v1/render-british/bm_george.wav (98482 bytes, 2.05 s at 24000 Hz) in 4.2 s
bm_lewis (sid 27): /tmp/kokoro-v1/render-british/bm_lewis.wav (85300 bytes, 1.78 s at 24000 Hz) in 2.4 s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

