# Spikes

Source of the feasibility spikes (all MIT, our own code). Results are summarised in
[docs/ARCHITECTURE.md §17](../docs/ARCHITECTURE.md) and [docs/KEYBINDS.md](../docs/KEYBINDS.md).
They are standalone cargo projects, not part of the workspace or CI. Their code was copied into
the crates as seeds.

| Spike | What it answered | Seeded |
|---|---|---|
| `s1-ui/` | egui vs iced vs Slint for the VR overlay renderer | `astromech-overlay` |
| `s5-status-latency/` | Press → Status.json latency; SendInput with scancodes; UIPI check | `astromech-input::exec`, `astromech-status` |
| `s6-journal/harness/` | Is `ed-journals` usable as a runtime dependency? (No; our own parser) | `astromech-journal` |
| `s7-input-listener/` | Background Raw Input while Elite has focus (works for sticks, keyboard, mouse) | `astromech-input::listen` |
| `s7-s9-input-voice/` | Local STT/TTS latency (Parakeet TDT + Kokoro via sherpa-onnx) | `astromech-voice` |
| `s8-licence-audit/` | Licence audit of the planned dependency tree; GPL-3.0 in sherpa-onnx 1.x | `deny.toml`, `NOTICE.md` |
| `k0-binds-doctor/` | Why a .binds file doesn't show up in game; device ID checks | `astromech-keybinds` |

## Deliberately left out

- **Personal results** (JSON/CSV from Sumeet's PC): they contain device instance paths and window
  titles. Summaries are in the docs.
- **Third-party code and data:** the cloned `ed-journals` and `ed-journal` repositories and their
  journal corpus (S6), samples harvested from other tools, the downloaded sherpa-onnx crate source
  and prebuilt libraries (S8), and the OFL fonts used by the S1 demos (get Rajdhani, Barlow and
  Share Tech Mono from Google Fonts into `s1-ui/assets/fonts/` to build them).
- **Frontier assets:** S5's tests used a copy of a default keyboard `.binds` file and a keymap
  JSON (both via COVAS:NEXT). They are not included, so two tests in
  `s5-status-latency/tests/binds_and_keys.rs` need local copies of those fixtures to run.
- Models, WAV recordings, build outputs and zips.
