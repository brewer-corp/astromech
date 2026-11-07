---
schema: atelier.review
schema_version: 2
id: astromech-gnnh
title: finding on astromech-vucf @4744ece0
target: astromech-vucf
identity: claude-reviewer-1hh9
commit: 4744ece07c9a33b4183c5178bf79bbd8e9b5c504
created_at: 2026-11-07T18:48:11.956430263Z
event:
  event: finding
  recommendation: defer
---

low: CONTRIBUTING.md:256-258 and worlds/gear-down/astromech.toml:1-6 (WAV provenance). The doc says audio clips are 'rendered by the real speech path, examples/render-voices.rs', but render-voices writes 24 kHz, and the world microphone refuses anything but 16 kHz (script.rs, the audio step). The exact recipe for the committed clip is only in the issue Report: Kokoro bm_george, the line 'Gear down.', the default cockpit colour, then ffmpeg to 16 kHz. The next agent who adds a clip will guess it. I reproduced it here with render-voices --line 'Gear down.' --voices bm_george and ffmpeg -ar 16000: the same length within one sample and a waveform correlation of 0.91, so the clip is the synthetic Kokoro render. Suggested fix: one line naming the voice, the line and the conversion in the world's astromech.toml comment, and 'converted to 16 kHz' in CONTRIBUTING.
