# S1: UI library spike (egui vs Slint vs iced): results

*Run 9 Oct 2026, 12:30–1:25 AM ET, on the shared Linux box (8 vCPU, no GPU; Mesa **lavapipe/llvmpipe** Vulkan software rasteriser; Xvfb for the windows). Versions were checked on crates.io on 9 Oct 2026.*

## Recommendation: **egui** (eframe 0.36.2 + egui-wgpu 0.36.2 on wgpu 30)

1. **The shared-device VR path uses only public API, and it's small.** `egui_wgpu::Renderer::new(&device, …)` renders into any `wgpu::Texture` we own. eframe either exposes its own device (`frame.wgpu_render_state()`) or accepts ours (`WgpuSetup::Existing`). The demo renders the 1024×768 VR panel at 30 Hz on the **same device as the desktop window** in about 130 lines (`egui-demo/src/offscreen.rs`). To do the same, Slint needed a custom `Platform` that wraps its *internal* `i-slint-backend-winit` crate (pinned `=1.18.1`, no semver promise) plus the `unstable-wgpu-30` feature. iced can't share a wgpu 30 device at all (it uses wgpu 27).
2. **It's the cheapest offscreen renderer in the test.** On llvmpipe, egui took a median **6.5 ms/frame** for UI build, tessellation, GPU and readback. Slint took 15.2 ms (FemtoVG/wgpu) or 13.4 ms (CPU software renderer), and iced 25.7 ms (wgpu, forced MSAA×4) or 11.9 ms (tiny-skia). egui's CPU-side work was only **about 0.25 ms**. The timings are software-rendered and only meaningful *relative to each other*.
3. **It reaches the target look.** The three screenshots are practically indistinguishable: chamfered panels, segmented bars, glows, custom fonts and an animated push-to-talk ring. Nothing in the HUD needed a workaround in egui. It's just more manual coordinate maths.
4. **Licence and wasm.** egui is MIT OR Apache-2.0, needs no `cargo-deny` exception and keeps the MIT-binary path open once sherpa-onnx 2.0 drops espeak-ng. The **same `hud.rs` compiles unchanged to wasm32** for the web dashboard.
5. **It's actively maintained.** egui has had 8 releases since 26 Mar 2026 (0.34.0 → 0.36.2), 31k stars, and was pushed to on 8 Oct 2026. The cost is 0.x API churn (see risks).

**Runner-up: Slint.** It has the nicest authoring model: declarative layouts, live-preview tooling, and `accessible-role`/`accessible-label` on any element. It would be the pick for a designer-led UI. It loses on three points: the hybrid window + VR panel setup relies on internal API, it's GPL-3.0 for us (OSI-compatible, but it would keep binaries GPL even after sherpa-onnx 2.0), and its offscreen frames were slower. **iced** is out: wgpu 27 can't share helm's device, it has no accessibility support, and it releases roughly once a year (0.14.0 was on 7 Dec 2025).

**Risks of choosing egui and how to handle them**
- *Manual layout:* build a small `helm-ui` widget kit once (Panel, Tile, SegBar, Meter, LogList) and keep views as pure `fn(&mut Ui, &State)`.
- *Accessibility:* painter-drawn content is invisible to AccessKit unless each item is allocated as a widget with `widget_info`. Budget for this in the status, settings and binds views.
- *API churn:* egui breaks its API on every minor release. For example, 0.36 changed `App::update` to `App::ui` and `textures_delta.set` became a SmallVec. Pin exact versions and plan an upgrade per release.

## What was built (`/workspace/elite-spikes/s1-ui/`, MIT)
- `hud-model/`: a deterministic fake ship state, `snapshot(t)`, shared by all three, so the same `t` produces the same frame. It covers the header, 6 toggles with a "VERIFYING" state that reflects S5's ~1 s Status.json cycle, the FSD state machine, fuel/reservoir/shields/hull/heat bars, push-to-talk state with mic level, and a growing comms log.
- `egui-demo/`, `slint-demo/`, `iced-demo/`: the same panel, each with
  - a desktop window (egui and Slint also render the VR panel at 30 Hz on the window's wgpu device);
  - `--offscreen N [--png f] [--software]`: a headless 1024×768 render into an RGBA8 buffer, which is what `openvr` `set_raw_data` takes.
- Fonts: Rajdhani, Barlow and Share Tech Mono (OFL), identical in all three.
- `wasm/egui-web`, `wasm/slint-web`: the same UI code compiled for `wasm32-unknown-unknown`.

## Screenshots (`shots/`)
| | Desktop window (Xvfb, lavapipe) | Offscreen 1024×768 RGBA frame |
|---|---|---|
| egui | `shots/egui-desktop.png` | `shots/egui-offscreen.png` |
| Slint | `shots/slint-desktop.png` | `shots/slint-offscreen-wgpu.png` (FemtoVG/wgpu), `shots/slint-offscreen-software.png` (CPU) |
| iced | `shots/iced-desktop.png` | `shots/iced-offscreen-wgpu.png`, `shots/iced-offscreen-tinyskia.png` |
| Small text, 3× zoom | `shots/smalltext-compare-3x.png` (11–13 px labels from every offscreen path) | |

## Offscreen render to RGBA buffer, 1024×768 (300 frames, llvmpipe; relative only). Raw output: `results/offscreen-*.txt`
| Path | Median | p95 | First frame | Breakdown (median) |
|---|---|---|---|---|
| **egui**, egui-wgpu → texture → readback | **6.5 ms** | 7.5 | 83 ms | UI 0.10 · tessellate 0.13 · GPU+readback ~6 |
| Slint FemtoVG on wgpu 30 → texture → readback | 15.2 ms | 17.8 | 805 ms | apply 0.03 · record 2.1 · GPU+readback ~13 |
| Slint software renderer → pixel buffer | 13.4 ms | 13.8 | 21 ms | render 7.3 · premultiplied→RGBA conversion 5.9 (naive loop; could be roughly halved) |
| iced Headless wgpu 27 (own device, MSAA×4 built in) | 25.7 ms | 28.5 | 392 ms | build 0.2 · draw 0.7 · render+readback ~25 |
| iced Headless tiny-skia | 11.9 ms | 12.8 | 30 ms | mostly rasterisation |

**In-process, shared device, next to a live desktop window** (bottom-right label in the window screenshots, EMA): egui ≈ 10 ms per VR frame, Slint (hybrid platform) ≈ 23 ms.

**Desktop CPU** (10 s, animating at about 30 fps, *with software GPU rasterisation counted as CPU*): egui+VR 242 %, Slint+VR 178 %, iced (no VR, `window::frames()` redraws every vsync) 452 %. **This doesn't reflect real-GPU or idle behaviour.** Idle CPU wasn't measured, since every demo animates continuously.

## Offscreen support and zero-copy OpenVR outlook
- **egui:** first-class. The renderer works on any device and target, and we own the texture (`RENDER_ATTACHMENT | COPY_SRC | TEXTURE_BINDING`). For a later zero-copy path, `wgpu::Texture::as_hal::<Dx12|Vulkan>()` can give the raw `ID3D12Resource`/`VkImage` for `SetOverlayTexture` (DX12 or Vulkan texture types). ⚠ Untested; S2's job.
- **Slint:** public `FemtoVGWGPURenderer::render_to_texture` (behind `unstable-wgpu-30`, which may change in any minor release) plus a custom `WindowAdapter`/`Platform`. Offscreen-only use is fine. **Desktop window + VR panel in one process** needed a `HybridPlatform` that delegates to the internal `i_slint_backend_winit::Backend` and forwards `InternalToken`. It works here, but it's fragile. The software renderer is a neat CPU path with no GPU at all, but it has **no drop-shadows** (a `TODO` in the 1.18.1 source) and draws paths only with `software-renderer-path`.
- **iced:** `Headless::screenshot()` returns `Vec<u8>` from **its own** wgpu 27 device, and the app runtime doesn't expose its device. Sharing with helm's wgpu 30 device isn't possible until iced moves to wgpu 30.

## Lines of code (non-blank, non-comment) and styling effort
| | UI/styling | Glue and offscreen | Notes |
|---|---|---|---|
| egui | `hud.rs` 292 | `offscreen.rs` 131, `main.rs` 139 | Immediate mode: `draw(ui, &snapshot)` with no binding layer. Chamfers via `Shape::convex_polygon` (1 call); gradients need a 4-vertex mesh; `ScrollArea::stick_to_bottom` is built in. All layout is manual `Rect` maths. 2 compile fixes, both version drift |
| Slint | `hud.slint` 303 | `main.rs` 442 (about 70 property binding, about 105 offscreen adapter/target, about 60 two platforms; both desktop modes and both offscreen renderers) | Layouts, gradients and borders are concise. A chamfer is a ~20-line `Path` with a size-bound viewbox. Pitfalls hit: a property named `col` silently collides with GridLayout's built-in `col`; `viewport-*` is deprecated in favour of `content-*`; a percentage width inside a layout gives a "binding loop (deprecated, may panic)" warning; stick-to-bottom has to be done by hand (`changed content-height`). Every value has to be pushed from Rust |
| iced | `hud.rs` 404 | `main.rs` 125 | Widgets plus one `canvas::Program` per custom shape (~15 lines each). Container shadows give real glows. `scrollable().anchor_bottom()` didn't apply in headless frames |

## Text at small sizes (VR readability)
In `smalltext-compare-3x.png`, all paths render Barlow and Rajdhani cleanly at 11–15 px. egui's glyphs come out slightly heavier and brighter, which helps on a resampled VR overlay. Slint FemtoVG and Slint software look identical, and iced (cosmic-text) is similar. Recommendation for VR panels: ≥ 14 px body text on a 1024-wide overlay. ⚠ Not checked in a headset (S2).

## Accessibility, IME and DPI
| | Accessibility | IME | DPI |
|---|---|---|---|
| egui | AccessKit through eframe (UIA on Windows; `uiautomationcore.dll` is imported). Built-in widgets are exposed; **custom-painted HUD content isn't** unless allocated with `widget_info` | Yes (egui-winit, for TextEdit) | Per-monitor scale → `pixels_per_point`; fonts rasterised at physical DPI |
| Slint | AccessKit through winit; `accessible-role`/`accessible-label` on any element (used in the demo) | Yes (TextInput) | Logical px, automatic scale factor |
| iced 0.14 | **None** (no AccessKit dependency) | Yes (`input_method`, new in 0.14) | Yes |

## Web dashboard (wasm)
Both build for `wasm32-unknown-unknown` with **the UI code unchanged**:
- egui `hud.rs` via `#[path]` with eframe `WebRunner`: 10.7 MB raw, 3.2 MB gzip (about 46 s clean build);
- Slint `hud.slint`, winit-web + FemtoVG/WebGL: 5.9 MB raw, 2.0 MB gzip (about 49 s).

These sizes are before wasm-bindgen and wasm-opt, and include 5 embedded fonts. ⚠ Neither was run in a browser (no wasm-bindgen CLI on the box). iced also supports the web, but it wasn't tested.

## Binary size
| | Windows x64 MSVC, static CRT (cargo-xwin 0.23.1) | Linux x64 release, stripped |
|---|---|---|
| egui | **12.7 MB** | 17.2 MB |
| Slint (FemtoVG-wgpu + software renderer + internal winit backend) | 19.5 MB | 25.4 MB |
| iced (wgpu + tiny-skia) | 10.0 MB | 13.1 MB |

All three Windows exes import no `VCRUNTIME140`/`api-ms-win-crt-*` DLLs (checked with llvm-readobj).

## Compile times (Linux, 8 vCPU, `-j8`, separate target dir per demo, deps pre-downloaded; `results/compile-times.txt`)
| | Clean dev | Incremental dev (touch UI file) | Clean release | Incremental release |
|---|---|---|---|---|
| egui | 49 s | **1.7 s** | 67 s | 19 s |
| Slint | 55 s | 2.8 s (re-runs slint-build) | 93 s | 36 s |
| iced | 44 s | 1.4 s | 58 s | 18 s |

All three Windows exes cross-compiled with cargo-xwin in 2 min 11 s together (warm). The first run spent about 8 minutes downloading the MSVC CRT and SDK into `~/.cache/cargo-xwin`. **The MSVC link works on the box** (closes S8's "MSVC link not run" caveat for UI crates; the sherpa-onnx link is still untested).

## Licence
- egui, eframe and egui-wgpu: MIT OR Apache-2.0. Default fonts OFL/UFL are already allowlisted (§16.1).
- Slint 1.18.1: `GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0`. For us that means GPL-3.0 plus a `cargo-deny` exception. Today's binaries are GPL-3.0 anyway because of sherpa-onnx 1.x, but Slint would keep them GPL permanently.
- iced: MIT.
- Demo fonts: OFL-1.1.

## Maintenance (crates.io and GitHub API, 9 Oct 2026; `results/maintenance.txt`)
| | Latest | Cadence | GitHub |
|---|---|---|---|
| egui | 0.36.2 (8 Sep 2026) | 8 releases since 26 Mar 2026; breaking minors | 31.1k★, 1,033 open issues, pushed 8 Oct |
| Slint | 1.18.1 (21 Sep 2026) | Minor about every 2–3 months, semver-stable 1.x; company-backed (SixtyFPS GmbH) | 24.1k★, 830 open, pushed 8 Oct |
| iced | 0.14.0 (7 Dec 2025) | About yearly (0.13 was Sep 2024) | 31.7k★, 509 open, pushed 7 Oct |

## Demo package
`/workspace/elite-spikes/s1-ui-demos.zip` (19 MB) contains the three static-CRT exes, README.txt, SHA256SUMS.txt, the OFL licences and the desktop screenshots. The demos are standalone: fake data, no game access, no key presses, no network, and no files written unless `--png` is passed. They use console-subsystem exes so the `--offscreen` benchmarks print output. README asks Sumeet to run the four benchmark commands on his real GPU.

## ⚠ Unverified
- **The Windows exes haven't been launched on Windows** (no Wine on the box). They're linked and import-checked only.
- All timings are from software rendering (llvmpipe). Real-GPU numbers should come from Sumeet's run of the README benchmarks.
- Idle CPU wasn't measured. The wasm builds weren't run in a browser. Headset readability and zero-copy `SetOverlayTexture` are S2's job.
- The original S1 exit criteria also asked for a live 1,000-row action log and a binds-doctor diff view. Only the status panel (with a growing log) was built.
