# NOTICE

astromech's source code is MIT-licensed (see `LICENSE`), Copyright (c) 2026 Sumeet Padavala.

## Binary caveat: GPL-3.0 via sherpa-onnx / espeak-ng

The local voice worker (`astromech-voice-worker`, cargo feature `sherpa` of `astromech-voice`,
**off by default**) links the official prebuilt **sherpa-onnx 1.x** libraries. Those libraries
include **espeak-ng and ucd-tools, which are GPL-3.0** (confirmed in spike S8,
docs/ARCHITECTURE.md §17). Consequences:

- The **source code** in this repository stays MIT.
- A **binary built with the `sherpa` feature** is distributed under the **GPL-3.0** as a whole,
  together with the corresponding source (this repository plus the matching sherpa-onnx
  release source). Release builds must ship the GPL-3.0 text and a source offer.
- Builds without the feature (the default, and everything CI builds) contain no GPL code.
- This caveat is expected to go away with sherpa-onnx 2.0 or a lexicon-only TTS build.

## Not in this repository

- No game data, Frontier assets or copies of Frontier's default `.binds` files. Elite
  Dangerous is a trademark of Frontier Developments plc; astromech is an unofficial fan project
  and is not affiliated with or endorsed by Frontier.
- No speech models. They are downloaded at first run, each under its own licence:
  Parakeet TDT 0.6B (NVIDIA, CC-BY-4.0), Kokoro-82M (Apache-2.0; its `espeak-ng-data` is GPL-3.0).

Third-party Rust and npm dependency licences: see `THIRD_PARTY.md`.
