---
created_at: "2026-10-16T18:43:49.524648128+00:00"
id: "astromech-e68n"
evidence_type: "validation"
commit: "6cfe4df3da000a4e128e9aea0dfdc26c1e63f012"
result: "pass"
captured_at: "2026-10-16T18:43:30.513072494+00:00"
command: "sh -c 'for f in /tmp/kokoro-v1/render-british/*.wav; do echo \"{\\\"cmd\\\":\\\"load_stt\\\",\\\"encoder\\\":\\\"/tmp/kokoro-v1-stt/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/encoder.int8.onnx\\\",\\\"decoder\\\":\\\"/tmp/kokoro-v1-stt/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/decoder.int8.onnx\\\",\\\"joiner\\\":\\\"/tmp/kokoro-v1-stt/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/joiner.int8.onnx\\\",\\\"tokens\\\":\\\"/tmp/kokoro-v1-stt/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/tokens.txt\\\",\\\"provider\\\":\\\"cpu\\\",\\\"threads\\\":4}\" > /tmp/kokoro-v1/one.jsonl; echo \"{\\\"cmd\\\":\\\"stt\\\",\\\"wav\\\":\\\"$f\\\"}\" >> /tmp/kokoro-v1/one.jsonl; printf '\"'\"'%s: '\"'\"' $(basename $f); LD_LIBRARY_PATH=/home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug /home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug/astromech-voice-worker < /tmp/kokoro-v1/one.jsonl 2>/dev/null | grep -o '\"'\"'\"text\":\"[^\"]*\"'\"'\"'; done'"
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
title: "Parakeet (through astromech-voice-worker STT) transcribes each British render in /tmp/kokoro-v1/render-british as the intended line: proves the v1.0 renders, phonemised with espeak-ng 'en' for British voices, are intelligible. bm_fable comes back as 'Its landing gear deployed.' (the Cockpit pip read as a word)"
updated_at: "2026-10-16T18:43:49.524868419+00:00"
---

## Summary

Parakeet (through astromech-voice-worker STT) transcribes each British render in /tmp/kokoro-v1/render-british as the intended line: proves the v1.0 renders, phonemised with espeak-ng 'en' for British voices, are intelligible. bm_fable comes back as 'Its landing gear deployed.' (the Cockpit pip read as a word)

## Command

```console
sh -c 'for f in /tmp/kokoro-v1/render-british/*.wav; do echo "{\"cmd\":\"load_stt\",\"encoder\":\"/tmp/kokoro-v1-stt/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/encoder.int8.onnx\",\"decoder\":\"/tmp/kokoro-v1-stt/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/decoder.int8.onnx\",\"joiner\":\"/tmp/kokoro-v1-stt/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/joiner.int8.onnx\",\"tokens\":\"/tmp/kokoro-v1-stt/sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8/tokens.txt\",\"provider\":\"cpu\",\"threads\":4}" > /tmp/kokoro-v1/one.jsonl; echo "{\"cmd\":\"stt\",\"wav\":\"$f\"}" >> /tmp/kokoro-v1/one.jsonl; printf '"'"'%s: '"'"' $(basename $f); LD_LIBRARY_PATH=/home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug /home/supadava/projects/astromech-wt-task-astromech-a50f/target/debug/astromech-voice-worker < /tmp/kokoro-v1/one.jsonl 2>/dev/null | grep -o '"'"'"text":"[^"]*"'"'"'; done'
```

Exit status: 0

## Stdout

Bytes: 375
Truncated: no

```text
bf_alice.wav: "text":"Landing gear deployed."
bf_emma.wav: "text":"Landing gear deployed."
bf_isabella.wav: "text":"Landing gear deployed."
bf_lily.wav: "text":"Landing gear deployed."
bm_daniel.wav: "text":"Landing gear deployed."
bm_fable.wav: "text":"Its landing gear deployed."
bm_george.wav: "text":"Landing gear deployed."
bm_lewis.wav: "text":"Landing gear deployed."
```

## Stderr

Bytes: 0
Truncated: no

```text
```

