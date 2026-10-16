---
created_at: "2026-10-16T18:44:01.763647904+00:00"
id: "astromech-lbhx"
evidence_type: "validation"
commit: "6cfe4df3da000a4e128e9aea0dfdc26c1e63f012"
result: "pass"
captured_at: "2026-10-16T18:44:01.763505739+00:00"
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
title: "Catalog facts checked 2026-10-16: GitHub release digests for tts-models kokoro-multi-lang-v1_0.tar.bz2 (349,906,910 B, sha256 c5f7e2d2...3298) and kokoro-int8-multi-lang-v1_0.tar.bz2 (132,303,094 B, sha256 4c3052ab...e4e) match sha256sum of the downloaded files (/tmp/kokoro-v1/). Extracted: fp32 401 MB, int8 190 MB; voices.bin identical in both (28,200,960 B = 54 voices). Speaker list read from the speaker_names/id2speaker metadata of both model files (0 af_alloy ... 20-27 British bf_alice bf_emma bf_isabella bf_lily bm_daniel bm_fable bm_george bm_lewis ... 53 em_santa); v0.19 order read from kokoro-int8-en-v0_19 metadata (af, af_bella, af_nicole, af_sarah, af_sky, am_adam, am_michael, bf_emma, bf_isabella, bm_george, bm_lewis). Before the change the worker exits loading v1.0 ('please pass --kokoro-lexicon or --kokoro-lang'); after it, the int8 model also loads and renders bf_emma, bm_george and af_heart through TtsSpeaker (/tmp/kokoro-v1/render-int8/, transcribed correctly by Parakeet). espeak-ng has no voice named 'en-gb' (Failed to set eSpeak-ng voice); 'en' is English (Great Britain). The Chinese lexicon/dict in the package is unused: with lang set, sherpa-onnx 1.13.8 phonemises non-Chinese text with espeak-ng (offline-tts-kokoro-impl.h, kokoro-multi-lang-lexicon.cc at v1.13.8)."
updated_at: "2026-10-16T18:44:01.764281265+00:00"
---

Catalog facts checked 2026-10-16: GitHub release digests for tts-models kokoro-multi-lang-v1_0.tar.bz2 (349,906,910 B, sha256 c5f7e2d2...3298) and kokoro-int8-multi-lang-v1_0.tar.bz2 (132,303,094 B, sha256 4c3052ab...e4e) match sha256sum of the downloaded files (/tmp/kokoro-v1/). Extracted: fp32 401 MB, int8 190 MB; voices.bin identical in both (28,200,960 B = 54 voices). Speaker list read from the speaker_names/id2speaker metadata of both model files (0 af_alloy ... 20-27 British bf_alice bf_emma bf_isabella bf_lily bm_daniel bm_fable bm_george bm_lewis ... 53 em_santa); v0.19 order read from kokoro-int8-en-v0_19 metadata (af, af_bella, af_nicole, af_sarah, af_sky, am_adam, am_michael, bf_emma, bf_isabella, bm_george, bm_lewis). Before the change the worker exits loading v1.0 ('please pass --kokoro-lexicon or --kokoro-lang'); after it, the int8 model also loads and renders bf_emma, bm_george and af_heart through TtsSpeaker (/tmp/kokoro-v1/render-int8/, transcribed correctly by Parakeet). espeak-ng has no voice named 'en-gb' (Failed to set eSpeak-ng voice); 'en' is English (Great Britain). The Chinese lexicon/dict in the package is unused: with lang set, sherpa-onnx 1.13.8 phonemises non-Chinese text with espeak-ng (offline-tts-kokoro-impl.h, kokoro-multi-lang-lexicon.cc at v1.13.8).
