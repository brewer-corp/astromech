# Licences of what is in this zip

- **Source code of this tool:** MIT (see `LICENSE`).
- **`s9-voice-worker.exe`** has sherpa-onnx 1.13.8 and ONNX Runtime linked in statically (the
  official `sherpa-onnx-v1.13.8-win-x64-static-MT-Release-lib` libraries from
  https://github.com/k2-fsa/sherpa-onnx/releases/tag/v1.13.8). Those libraries include
  **espeak-ng and ucd-tools (GPL-3.0)**, so this exe as a whole is distributed under the
  **GPL-3.0** (text in `COPYING`). Its complete corresponding source is this tool's source
  plus the sherpa-onnx v1.13.8 source (https://github.com/k2-fsa/sherpa-onnx/tree/v1.13.8),
  which contains the exact espeak-ng and onnxruntime versions it builds.
  Components inside it:
  - sherpa-onnx, kaldi-native-fbank, kaldi-decoder: Apache-2.0
  - ONNX Runtime: MIT (Copyright (c) Microsoft Corporation)
  - espeak-ng, ucd-tools: GPL-3.0
  - piper-phonemize: MIT
  - also linked: OpenFst (`sherpa-onnx-fst`), kissfft, ssentencepiece. Their licences are
    permissive as far as we know (Apache-2.0 / BSD-3-Clause) but were not re-verified for this spike.
- **`s7-s9-input-voice.exe`** and **`s9-gpu-probe.exe`** contain no GPL code (MIT plus
  permissively licensed Rust crates).

Downloaded at first run (not in this zip), each under its own licence:
- Parakeet TDT 0.6B v2 (NVIDIA): CC-BY-4.0
- Kokoro-82M v0.19: Apache-2.0; its `espeak-ng-data` folder: GPL-3.0
- Optional GPU test only: ONNX Runtime 1.24.4 DirectML build (MIT, Microsoft) and
  DirectML 1.15.4 (Microsoft DirectML licence), straight from nuget.org.
