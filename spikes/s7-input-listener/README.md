# s7-input-listener

Spike S7, simplified. It listens to **every controller, keyboard and mouse input in the background** for a set time (60 s by default) while you play, then reports what it saw. The headline answer is: **does Windows still deliver controller input to a background app while Elite has focus?**

It's also the seed of the Helm keybinds **key tester** and the input crate. MIT licence.

## Read-only

- It **only listens.** Raw Input is registered with `RIDEV_INPUTSINK` on a hidden window. It doesn't use `RIDEV_NOLEGACY` or `RIDEV_CAPTUREMOUSE`, so nothing is taken from the game. XInput pads are read with `XInputGetState` only (no rumble).
- It **never sends input** and never writes game files. With `--with-binds` it *reads* your active preset.
- Its only output is two report files next to the exe. Your Windows username is redacted from paths.
- **Privacy:** keyboard keys are logged by name, so the report shows what you typed while it ran. Use `--no-key-names` to record only that a key was pressed.
- No installer, no network, and only standard Windows DLLs (kernel32, user32, advapi32, hid, ntdll, bcryptprimitives; XInput is loaded at runtime if present).

## Run it

1. Unzip anywhere (e.g. `Downloads\s7-input-listener\`).
2. Start Elite and get into the cockpit.
3. Run it from PowerShell in that folder. `--with-binds` adds the bound actions to each press:
   ```
   .\s7-input-listener.exe --with-binds
   ```
   You can also double-click it (60 s, no binds). A window opens and waits for Enter at the end.
4. During the **5 s countdown**, switch to Elite (Alt+Tab, or click the game).
5. For the next **60 s**, in Elite:
   - Press buttons on both sticks, including the push-to-talk button and the hats in all directions.
   - Move every axis (throttle, twist, ministick, wheel).
   - Press a few keys and mouse buttons.
   - Briefly Alt+Tab out, press something, then go back. That gives a "not focused" comparison.
6. The console shows each event live, which is handy on a second monitor. At the end it prints the summary and verdict.
7. Send back the `s7-input-YYYYMMDD-HHMMSS.json` written next to the exe. The `.html` is the readable version.

### Options

| Option | Default | Use |
|---|---|---|
| `--seconds N` | 60 | Listening time |
| `--countdown N` | 5 | Delay before listening, to switch to Elite |
| `--with-binds` | off | Load the active ship preset (StartPreset → .binds, read-only) and show bound actions for each button, hat and axis |
| `--bindings-dir P` | `%LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Bindings` | Bindings folder |
| `--game-dir P` | auto | Game install (built-in presets, DeviceMappings.xml) |
| `--elite-title T` | `Elite - Dangerous (CLIENT)` | Window title that counts as "Elite focused" |
| `--no-key-names` | off | Don't record which keys were pressed |
| `--replay F` | — | Feed recorded/synthetic samples instead of listening (testing; see `tests/fixtures/replay-vkb.json`) |
| `--out-dir P` | exe folder | Where the report goes |
| `--no-pause` | — | Don't wait for Enter |

## What it logs

Each event records:
- the time
- the device: product, VID:PID, and Elite ID such as `231D3201`
- the control, with its Elite name:
  - buttons: down/up with the HID button number → `Joy_N`
  - hats: Elite POV naming, `Joy_POV1Up/Right/Down/Left`. Diagonals press two directions.
  - axes: summarised per movement (start/min/max/end % and duration) as `Joy_XAxis`, `Joy_RZAxis`, and so on. A movement starts when an axis moves 5% of its range from rest and ends after 400 ms without a change of 1% or more.
  - keys: `Key_A` and so on
  - mouse buttons: `Mouse_N`
  - XInput buttons and axes
- the foreground window title, and whether Elite had focus
- with `--with-binds`, the bound Elite actions, including modifiers

## Summary and verdict

- **Per device:** buttons, hats and axes seen; press counts with and without Elite focus; declared button/axis/hat counts.
- **Buttons above 32**, with the warning that Elite reportedly reads only `Joy_1..Joy_32` per device.
- **Possible dropped input:** downs without ups (still held at the end), key ups without downs, controls already held at start.
- **Verdict** per class (controllers, keyboard, mouse), plus a headline for controllers: **`Background input capture while Elite focused: WORKS / NOT OBSERVED / FAILED`**.
  - WORKS: real device events arrived while Elite was focused.
  - NOT OBSERVED: Elite never had focus, or nothing was pressed while it had focus.
  - FAILED: Raw Input registration failed, or events arrived only while Elite *wasn't* focused despite at least 10 s with Elite focused (suspected loss).

## Known limits

- Slider/dial axes are named `Joy_UAxis`/`Joy_VAxis` in DirectInput slider order. That hasn't been checked against the game.
- XInput controls aren't mapped to Elite's gamepad key names.
- Elite key names assume the US-layout naming used in `.binds` files.
- Ending early (Ctrl+C or closing the window) writes no report.
- If Elite runs **as administrator** and this tool doesn't, behaviour is untested (UIPI). Run both the same way.

## Build

```
cargo test
cargo xwin build --release --target x86_64-pc-windows-msvc        # static CRT via .cargo/config.toml
cargo xwin build --release --target x86_64-pc-windows-msvc --examples   # mockfocus.exe test harness (not shipped)
```
