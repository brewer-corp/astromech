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
| `q1-linked-places/` | Can "place A within N ly of place B" queries (HGE system near a trader, hotspot ring near a buyer) run on Spansh and Ardent? Yes: one search per side and a local join. Findings are in the close reason of Atelier issue astromech-yn6y; `replay.sh` reruns them from the gitignored `cache/` | not yet (galaxy rework, astromech-51ue) |
| `o1-one-process/` | Can one Tauri process hold the tray with no window, survive the window crashing, free WebView2 when the window closes and keep astromech's Raw Input listener receiving keys? Yes to all four on Windows CI: the tray works windowless, the app outlives its renderer and browser process being killed, a close frees every WebView2 process in about 2.3 s, and the listener keeps receiving keys if it starts after Tauri's `build`. Windowless it cost 3.3 to 3.5 MB. Whether Windows' restart registration restarts a crash is unproven: WER never engaged on the runner (astromech-fmub, astromech-dl53). The answers are in Atelier issue astromech-wgtr. Its probe's checks now run against the one app in `scripts/webview_cost.py`, and its CI workflow is gone (astromech-xz7i); `scripts/one_process_probe.py --restart` is what is left, for the restart test. Build it with `cargo build --release` in the folder | the one app (astromech-v3u4) |

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
