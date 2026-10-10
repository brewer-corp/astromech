# s7-s9-input-voice: Elite voice assistant spikes S7 + S9

This is one small Windows tool that answers two questions on **your** PC:

* **S7: controller input with Elite focused.** Do your push-to-talk (PTT) and kill-switch buttons reach a background program while Elite is the active window? Does Elite still get them too? How fast do they arrive?
* **S9: local voice speed.** How long does it take from releasing PTT to getting a transcript back (Parakeet speech-to-text)? How long from text to the first audio (Kokoro text-to-speech)? Everything runs on the CPU; an optional test also times the two big models on your GPU through DirectML (works with AMD, Intel and NVIDIA).

Licences: the source is MIT (`LICENSE`). `s9-voice-worker.exe` has sherpa-onnx, which contains GPL-3.0 espeak-ng, linked in, so that exe is GPL-3.0 as a whole (`COPYING`, details in `NOTICE.md`). The downloaded models keep their own licences.

**What's in the zip:** `s7-s9-input-voice.exe` (run this one), plus two helpers it starts itself: `s9-voice-worker.exe` (the speech engine) and `s9-gpu-probe.exe` (the optional GPU test). Don't run the helpers directly.

## What's safe (and what it does)

* **It never presses keys or buttons.** It doesn't use SendInput, doesn't rumble pads (it never calls XInputSetState), and doesn't hook the keyboard. It only *listens*:
  * **Raw Input** with `RIDEV_INPUTSINK` on an invisible window. This adds the tool as an extra listener, so nothing is taken away from Elite. It doesn't use `RIDEV_NOLEGACY` or mouse capture.
  * **XInput** polling, to compare with Raw Input on Xbox pads.
* **What it saves from your input:** only the PTT and kill-switch buttons you choose, plus press counts and timings. It doesn't log any other key you type. If Elite wasn't the active window when a cue played, it also saves the title of the window that was active instead.
* **Game files:** it only *reads* `Status.json` in `%USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous\`. It never writes any game file.
* **Downloads:** the models come only from the official sherpa-onnx GitHub releases (`github.com/k2-fsa/sherpa-onnx/releases`). The optional GPU test downloads two DLLs from Microsoft's official NuGet packages (`api.nuget.org`). Each file is checked against a SHA-256 hash built into the tool before it's used.
* **Nothing leaves your PC.** Speech recognition and synthesis run locally. Your recordings are saved in `s9-audio\` next to the exe so you can listen to them or delete them.
* **Ctrl+C or closing the window** stops it at any point. Nothing is held down, because nothing is ever pressed.

## Before you start

1. **Extract the zip** to a normal folder, e.g. `C:\EliteSpikes\s7-s9\`. Don't run it from inside the zip: the models are stored next to the exe.
   * Keep all three exes together in that folder. The speech engine is now built into `s9-voice-worker.exe`, so there are no DLLs to download or keep track of.
2. **SmartScreen:** the exe isn't code-signed, so Windows may show "Windows protected your PC". Click **More info**, then **Run anyway**.
   * Simplest: before extracting, right-click the **zip**, choose **Properties**, tick **Unblock**, and click OK. Then all three exes are unblocked.
3. Have a **headset or microphone** set as the Windows *default recording device*. Also have **speakers or headphones**, because S7 uses short beeps as cues while you're in the game.
4. For S7, **Elite should be running, and you should be in your ship** (any safe place: docked, or in open space). Run Elite and the tool the normal way. Don't use "Run as administrator" for either.

## Run it

Double-click `s7-s9-input-voice.exe`. It asks before each part. You can also open a terminal in the folder and use one of these:

```
s7-s9-input-voice.exe --dry-run     # shows devices, mic, downloads and paths; changes nothing
s7-s9-input-voice.exe               # S7, then S9
s7-s9-input-voice.exe s7            # only the input test
s7-s9-input-voice.exe --s9-only     # only the voice test (uses the PTT learned in S7, or Space); same as "s9"
s7-s9-input-voice.exe --s9-only --gpu   # also the DirectML GPU test without asking (see below)
s7-s9-input-voice.exe --help        # all options
```

### S7: about 5 minutes per device

1. **Learn:** press the button you want as **PTT**, then the one for the **kill switch**. Use any keyboard key, mouse button 3/4/5, joystick, HOTAS or gamepad button, or hat direction.
   * The tool shows what it saw: device name, VID/PID, and the button number.
   * Esc skips. Mouse left/right and Enter are ignored, because you use them in the console.
2. **Test:** press Enter, then switch to Elite within 10 s. After that, just listen:
   * rising 3 notes: started
   * one **high beep**: press and release **PTT** (20 times)
   * two **low beeps**: press the **kill switch** (5 times)
   * long buzz: Elite isn't the active window; click back into it
   * two-note chime: press **a button on the same device that does something visible in Elite** 3 times, about 3 s apart (lights, landing gear, cargo scoop, and so on). This checks that the game still gets your input.
   * falling 3 notes: done; go back to the console
3. Answer the y/n questions in the console. Then the tool asks "Test another device?".

### S9: about 10–15 minutes

Before downloading anything, the tool checks that its speech engine starts (it prints "Voice worker OK: ..."). The first S9 run then downloads the models. The tool shows the size and asks first.

| What | Download |
|---|---|
| Parakeet TDT 0.6B v2, English, int8 (speech-to-text) | 482.5 MB |
| Kokoro English v0.19 (text-to-speech) | 319.6 MB |
| **Total** | **802 MB** (about 1.0 GB on disk afterwards; ~1.8 GB free needed while unpacking) |
| *optional* `--small-tts` (int8 Kokoro instead) | 103.2 MB, so 586 MB in total |
| *optional* GPU test: ONNX Runtime (DirectML build) + DirectML.dll, from Microsoft's NuGet | +15.4 MB |

The tool shows download progress. Each file is hash-checked and then unpacked into `models\` (and `runtimes\directml\` for the GPU test). Later runs reuse them.

**Already ran S7 (or ran the earlier version of this tool)?** Just run `s7-s9-input-voice.exe --s9-only`. It reuses the PTT button saved in `s7-bindings.json` and any models already in `models\`. If you extracted this version into a new folder, copy `models\` and `s7-bindings.json` over from the old folder to skip the 800 MB download. The old `runtimes\cpu\` folder is no longer used and can be deleted.

If a download keeps failing, get the same file in a browser from the URL that `--dry-run` prints. Put it in a `downloads\` folder next to the exe; the tool checks the hash and uses it.

Then:

* **Test 0** (automatic): Kokoro says each of the 15 commands, and Parakeet transcribes them. No mic is involved; it's a sanity check.
* **Test 1** (you): for each of up to 15 commands shown on screen ("landing gear down", "plot route to Sol", ...), **hold PTT, say it, release**. The tool times PTT release → transcript and compares the result with the expected words. Esc stops early.
* **Test 2**: Kokoro speaks 10 short confirmations, and you should hear each one. The tool measures time to first audio and the real-time factor.
* **CPU thread sweep** (automatic, about a minute): the same audio is decoded with 1/2/4/8 threads. Skip it with `--no-sweep`.
* **GPU test (optional, about a minute):**
  * Offered when Windows lists a GPU (answer y), or run it directly with `--gpu`. Skip it with `--no-gpu`.
  * It uses **DirectML**, which works with any DirectX 12 GPU (AMD Radeon, Intel, NVIDIA). Nothing gets installed: it downloads `onnxruntime.dll` (DirectML build) and `DirectML.dll` from Microsoft's NuGet packages into `runtimes\directml\` and loads them from there.
  * It times the two heavy models (the Parakeet encoder and Kokoro) on the CPU and then on the GPU, with the same inputs, and prints the speed-up.
  * It needs the **Microsoft Visual C++ 2015-2022 Redistributable (x64)**, which most games install. If it's missing, the test says so and is skipped. Nothing else is affected.
  * If DirectML fails (old driver, or no DirectX 12), the error is recorded. That's a useful answer too.

## Afterwards

When it finishes, the tool prints a summary. Please send back the file it names:

```
s7-s9-results-YYYYMMDD-HHMMSS.json   (next to the exe)
```

* The JSON contains your CPU, GPU, RAM and Windows version, the device names and VID/PIDs, the timings, the transcripts, and your y/n answers. It doesn't contain any audio.
* `s7-bindings.json` holds the buttons you learned.
* `logs\worker-cpu.log` and `logs\gpu-probe.log` are only useful if something failed.

**To remove everything,** delete the folder. The tool installs nothing else and changes no settings.

## If something goes wrong

* **"sherpa-onnx-c-api.dll was not found"**: that was the previous version (its helper needed DLLs). This version has no such DLLs. Make sure you're running the new `s7-s9-input-voice.exe`.
* **"s9-voice-worker.exe is missing"**: extract the whole zip again, keeping the three exes together. Check that your antivirus didn't quarantine it.
* **"the voice worker ... does not start: ..."**: the message says why (for example a blocked or damaged file). Run `s7-s9-input-voice.exe --dry-run`, which runs the same check, and send the output.

## Building (for developers)

* It's a Rust workspace with four crates:
  * `crates/main`: the exe
  * `crates/worker`: `s9-voice-worker`. On Windows it links sherpa-onnx + onnxruntime **statically**, using the `static-MT` prebuilt (`static` feature), so it needs no DLLs. On Linux (reference runs) it uses the `shared` feature and the downloaded runtime.
  * `crates/gpu-probe`: `s9-gpu-probe`. It uses the `ort` crate with `load-dynamic`, so nothing is linked at build time. It loads Microsoft's `onnxruntime.dll` + `DirectML.dll` by full path at run time.
  * `crates/common`: protocol, downloader (tar.bz2, plus single zip members via HTTP range requests), scoring
* The worker runs as a separate process from the exe's own folder. The main exe runs `s9-voice-worker --version` as a startup self-check.

```
# Windows target from Linux (cargo-xwin, MSVC ABI because the sherpa-onnx prebuilts are MSVC):
SHERPA_ONNX_LIB_DIR=/path/to/sherpa-onnx-v1.13.8-win-x64-static-MT-Release-lib/lib \
  cargo xwin build --release --target x86_64-pc-windows-msvc
# (cross builds need a PathCch.lib -> pathcch.lib symlink in the xwin SDK lib folder)
cargo test          # unit tests (Linux or Windows)
```
