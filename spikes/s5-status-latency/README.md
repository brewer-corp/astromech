# s5-status-latency (spike S5)

A small Windows command-line tool. It measures how long Elite Dangerous takes to
update `Status.json` after a keypress. It presses one of your own key bindings a
number of times and records how quickly the matching flag changes. MIT licensed.

## What it does
1. It finds your journal folder (`%USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous`)
   and your bindings folder (`%LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Bindings`).
2. It reads `StartPreset*.start` (using the Ship line) and loads the newest matching `.binds` file.
3. It looks up the keyboard binding for the action. The default is **ShipSpotLightToggle**
   (ship lights). `NightVisionToggle` and `LandingGearToggle` are also available.
4. It presses that key 20 times. Each press: key down, hold 60 ms, key up, then a random
   1.5–2.5 s gap. For each press it times how long the flag in `Status.json` takes to flip,
   measured two ways:
   - (a) reading the file every 2 ms
   - (b) a Windows file-change notification
5. It prints one line per press, then a summary, and writes a CSV.

## How to run
Unzip the files to a folder and open PowerShell in that folder.

```powershell
# 1. Check what it found. Nothing is pressed.
.\s5-status-latency.exe --dry-run

# 2. Real run. Sit in your ship's cockpit (docked or in normal space), with no panel open.
#    Start the tool, then click or alt-tab into the game within the 5-second countdown.
.\s5-status-latency.exe

# Optional: try holds of 20/40/60/100 ms (6 presses each)
.\s5-status-latency.exe --hold-sweep
```

Other options (see `--help`):
- `--action NightVisionToggle`, `--trials 40`, `--hold-ms 80`
- `--journal-dir`, `--bindings-dir`, `--binds-file`
- `--scancode 0x26 [--extended]` presses a key you choose, if your binding is HOTAS-only

## What's safe
- It only presses the one binding it shows on the `Will press:` line, plus that binding's
  modifiers. It never presses anything else.
- Before every press it checks that the foreground window is exactly
  `Elite - Dangerous (CLIENT)`, and that no panel or map is open (`GuiFocus` = 0).
  If either check fails, it waits instead of pressing.
- If the game is out of focus for more than 30 s, the run stops. Ctrl+C also stops it at any
  time. Keys are always released when it stops.
- At the end it presses once more if needed, so your lights or night vision are left as
  they were.
- **Landing gear:** only use it in normal space, away from surfaces. The tool refuses to
  press if it sees docked, landed or supercruise.
- Run it **normally, not as Administrator**. If the game and the tool run at different
  privilege levels, Windows silently drops the keypresses (UIPI), and the tool warns you
  about this.

## What the output means
- `poll` / `notify`: milliseconds from key-down until the flag flip was seen in `Status.json`.
  `MISS` means nothing changed within 3 s.
- The summary shows min, median, p95, max and misses for each method, plus the hold time used.
  With `--hold-sweep`, it also shows the shortest hold that had no misses.
- The game build is read from the newest journal's Fileheader.

## Sending the results back
A CSV file named `s5-status-latency-<action>-<date>-<time>.csv` is written next to the exe.
If that folder isn't writable, it goes in the current folder instead. Send that file back,
plus a copy of the console text if anything looked odd.

## Unsigned exe
This exe is not code-signed, so Windows SmartScreen may show "Windows protected your PC".
To run it anyway, click **More info → Run anyway**. Your antivirus may also flag it, because
it sends keypresses.
