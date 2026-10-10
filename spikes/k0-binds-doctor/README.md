# k0-binds-doctor

Read-only checker for Elite Dangerous keybinding files. It looks at every `.binds` file in your Bindings folder and the controllers plugged in right now, then tells you why a preset probably isn't showing up in the game's Controls menu.

Spike K0 for the Helm keybinds module, and the start of its future "Doctor". MIT licence.

## Safety

- **Read-only.** It never writes, renames or moves anything in the Bindings folder, the game install or the logs. It opens HID devices in query-only mode (access 0) to read their names and capabilities.
- Its only output is two report files written **next to the exe** (or to `--out-dir`). It refuses to run if that folder is inside a game folder.
- Your Windows username is redacted from paths in the reports (`%LOCALAPPDATA%`, `%USERPROFILE%`, `\Users\<user>`).
- No network access. No installer. No DLLs beyond standard Windows ones (kernel32, advapi32, hid, setupapi; XInput is loaded optionally at runtime).

## Run it

1. Unzip `k0-binds-doctor.exe` somewhere like `Downloads\k0-binds-doctor\`. Don't put it in the game or Bindings folder.
2. Plug in all your controllers. Running it while the game is closed is best, but not required.
3. Double-click `k0-binds-doctor.exe`. A console window shows the summary and waits for Enter.
   You can also run it from a terminal: `.\k0-binds-doctor.exe`
4. Open `binds-doctor-YYYYMMDD-HHMMSS.html` (next to the exe) in a browser. The `.json` file holds the same data for machines.

Windows SmartScreen may warn that the exe is unsigned: choose *More info → Run anyway*.

### Options

| Option | Use |
|---|---|
| `--bindings-dir <path>` | Bindings folder (default `%LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Bindings`) |
| `--game-dir <path>` | Game install, if auto-detect fails, e.g. `"D:\SteamLibrary\steamapps\common\Elite Dangerous"`. Any folder above `Products\<product>\ControlSchemes` works. |
| `--logs-dir <path>` | Extra folder of game logs to scan |
| `--out-dir <path>` | Write the report somewhere other than next to the exe |
| `--fake-devices <json>` | Use a device list from a file instead of live USB enumeration (testing) |
| `--no-pause` | Don't wait for Enter |

The game install is found via Steam (registry `SteamPath`, `libraryfolders.vdf`), Epic (`%PROGRAMDATA%\Epic\EpicGamesLauncher\Data\Manifests\*.item`), and Frontier launcher default paths (`C:\Program Files (x86)\Frontier`, `%LOCALAPPDATA%\Frontier_Developments`).

## What it checks

**Devices.** Every HID device on usage page 1 with usage 4, 5 or 8 (joystick, gamepad, multi-axis), plus a count of keyboards and mice. For each one: VID, PID, product, manufacturer, buttons, axes and hats from the HID capabilities, and Elite's device id (`VVVVPPPP`, e.g. `231D0200`). It also lists connected XInput pads (slots 0–3). It flags:
- two controllers with the same VID:PID, because Elite can't tell them apart
- devices reporting more than 32 buttons (Elite reportedly reads only 32)
- devices that `DeviceMappings.xml` gives a name, which Elite writes instead of the raw id

**Game data.** `ControlSchemes\DeviceMappings.xml`, which maps names like `ThrustMasterHOTAS4` to VID/PID, and the built-in preset names.

**Each `.binds` file**, including files in subfolders, which are flagged:
- PresetName, MajorVersion/MinorVersion, the file-name suffix (`.4.2.binds`) compared with the version the folder currently uses
- XML well-formedness, with the line and column of any error (truncated files point to the end)
- encoding: UTF-8/UTF-16 BOM, NUL bytes, invalid UTF-8, a declared encoding that doesn't match the file
- file name vs PresetName, duplicate PresetNames at the same version, PresetNames that clash with a built-in preset
- each device referenced, with bind counts, how it's referenced (raw VID+PID, DeviceMappings name, keyboard/mouse, unknown name) and whether it's connected, missing or unknown
- keyboard-only presets, buttons above 32, buttons/axes/hats the connected device doesn't have, repeated actions

**Near misses.** Files like `X.4.2.binds.txt` or `X.4.2.binds (1)`, and `.xml`/`.txt` files that contain `<Root PresetName`.

**Active presets.** Every line of every `StartPreset*.start` file, labelled by its likely context (General, Ship, SRV, On-foot), and what each name resolves to: your file, a built-in preset, a file with a different version only, or nothing.

**Logs.** `BindingLoadingErrors.log`, quoted in full (first 300 lines). It also scans the newest three `*.log` files in `Products\<product>\Logs` and `%LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Logs` for lines mentioning binds, presets, devices or input.

**"Likely reasons it won't show up"**, per file, ranked by confidence: high, medium or low.

| Code | Confidence | Meaning |
|---|---|---|
| WRONG_FOLDER | high | File is in a subfolder |
| MALFORMED_XML / WRONG_ROOT / NO_PRESET_NAME | high | File can't be used |
| MISSING_DEVICES | high | It references a device that isn't connected |
| OLD_VERSION | high | Older major version suffix, e.g. Horizons `.3.0` |
| UNKNOWN_DEVICES, ENCODING, NO_VERSION_SUFFIX, ODD_SUFFIX, OLDER_MINOR, VERSION_ATTR_MISMATCH, DUPLICATE_PRESET_NAME, BUILTIN_NAME | medium | Plausible causes |
| UTF8_BOM, NAME_MISMATCH, ODD_NAME, NO_VERSION_ATTRS, NEWER_VERSION, CASE_DUPLICATE | low | Probably harmless |

**Frontier doesn't document how the game decides which presets to list.** Every reason here is a hypothesis drawn from community reports (Frontier forums, Steam guides, EDRefCard/EDCD notes). Each run tests those hypotheses against your files. Run it, check which presets the game actually lists, and tell us where it was wrong.

## Known limits

- Axis names: HID X/Y/Z/Rx/Ry/Rz map to Elite's `Joy_X…RZAxis`. Sliders and dials (0x36/0x37) are assumed to be `Joy_UAxis` then `Joy_VAxis` (DirectInput slider order). This hasn't been checked against the game.
- Button counts come from the HID descriptor. Some devices (VKB among them) declare more buttons than are physically wired.
- When there are several `StartPreset` files, the highest-numbered one is assumed to be the one in use. "Current version" is inferred from that number and the newest suffix in the folder.
- Where the game writes its client logs, and what it says about bindings, is unconfirmed, so the log scan is a best-effort keyword filter.

## Fake devices (testing)

```json
{"controllers":[{"vid":"231D","pid":"3201","product":"VKB-Sim Omni Throttle EVO L","usage":4,"buttons":32,"axes":["X","Y","Z","RX","RY","U"],"hats":2}],
 "keyboards":[{"vid":"046D","pid":"C33F","product":"Keyboard"}],"mice":[],"xinput_slots":[0]}
```
See `tests/fixtures/` in the source tree.

## Build

```
cargo test
cargo xwin build --release --target x86_64-pc-windows-msvc   # static CRT via .cargo/config.toml
```
