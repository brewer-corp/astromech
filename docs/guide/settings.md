# Settings

Settings has four pages: **General**, **Data sharing**, **Diagnostics** and **Voice tester**.
This page covers General and Diagnostics. See [Data sharing](data-sharing.md) and
[Voice tester](voice-tester.md) for the others.

Each row is a label on the left and its value on the right, with the action beside it. Changes
apply at once unless the row says otherwise. If the window cannot reach astromech's background
process it says so, and nothing can be shown; it starts with the window, so open Diagnostics
if that stays.

## General

### Startup

- **Start with Windows.** On by default after install. astromech starts quietly in the tray and
  waits for Elite, so voice is ready when you launch the game. **Turn off** stops it.
- **Setup.** "Done" or "Not finished". **Run again** reopens the [first-run
  setup](getting-started.md#first-run).

### Voice

- **Push-to-talk** and **Kill switch.** The buttons you chose. **Change** opens that setup
  step. If push-to-talk is not set, voice cannot listen and this row is highlighted.
- **Speech models.** "Installed", the download progress, or "Not downloaded (about 800 MB)"
  with a **Download** button.
- **Voice.** "Listening while Elite runs" or "Paused". **Pause** and **Resume** control whether
  push-to-talk is heard. [Callouts](callouts.md) keep going while paused.

### Voice responses

- **Speaker.** British or American voices. **Preview** speaks a sample line.
- **Colour.** Natural, Cockpit (the default) or Harsh. See [Voice](voice.md#spoken-replies).
- **Speed.** 0.7 to 1.3 in steps of 0.1. "Normal" is the colour's own pace.

The next reply uses the change. Preview needs the speech models.

### Microphone

- **Device.** "Windows default" or the microphone you chose. **Change** opens that setup step.
  If a chosen microphone is unplugged, the row says so and shows "(not connected)".
- **Calibration.** "Ignores clips quieter than -45 dBFS" for example. **Run again** (or
  **Calibrate**) opens a dialog: a moment of silence, then three phrases. It measures the room
  and your voice on this mic, so quiet speech is not dropped and background noise is not heard
  as words.

### Prospector callouts

One box per ore, in percent. When a prospector limpet reports a rock with at least this much of
an ore, astromech reads it out ("Platinum 34%."). An empty box turns that ore off, and **Reset**
returns to the default. A motherlode is always read out. The defaults are 20% for Platinum, 25%
for Low Temperature Diamonds, Painite and Osmium, 30% for Palladium, Bromellite and Tritium. Ores
without a default are only called out as a motherlode. See [Callouts](callouts.md#prospector).

### Updates

- **Version.** The installed version. **Release notes** opens the GitHub releases page.
- **Updates.** "Up to date", "<version> is available", "is ready to install" or an error.
  **Check now**, or **Install and restart** when one is ready. See
  [Getting started](getting-started.md#updates).

### Files

- **Config file.** The path of `astromech.toml`, with **Open**. If there is no config file yet, **Open** writes the sample one, which sets the kill
  switch to Scroll Lock (known bug: astromech-k20i); change it in setup if that is wrong. Keep the name exactly
  `astromech.toml`: if Notepad saves it as `astromech.toml.txt`, astromech reports the file it
  found instead.
- **Logs.** The log folder, with **Open folder**.

## More in the config file

Some settings have no row yet and live in `astromech.toml`. Edit it with astromech closed, or
restart afterwards.

| Setting | What it does | Default |
|---|---|---|
| `[callouts.<id>]` `enabled`, `cooldown_s` | Turn one [callout](callouts.md) off, turn the Powerplay arrival one on, or change how long a callout stays quiet after it speaks | On (Powerplay arrival: off) |
| `[controls]` `rearm_hold_ms` | How long to hold the kill switch to re-arm | 1000 |
| `[voice]` `min_hold_ms` | Push-to-talk holds shorter than this are dropped | 300 |
| `[voice]` `min_level_dbfs` | Clips quieter than this are dropped (calibration sets it) | -45 |
| `[voice]` `tts` | `false` prints replies in the log instead of speaking them | true |
| `[voice]` `save_clips` | Keep your push-to-talk recordings in `data\clips` for debugging | false |
| `[actions]` `dry_run` | Log key presses instead of sending them | false |
| `[update]` `check`, `apply`, `feed` | Update behaviour | true, `ask` |
| `[game]` `journal_dir`, `bindings_dir`, `game_dir` | Where the game's files are, if not the usual places | empty |
| `data_dir` | Where history, caches and models are kept | `%LOCALAPPDATA%\astromech\data` |

A callout with an unknown id or a `cooldown_s` that is not a number of seconds is reported in the log
and ignored on its own. Any other error in the file (a typo, a bad value) is different: the **whole file
is discarded** and astromech runs with every default, including no push-to-talk and no kill switch,
until you fix it. Setup says "config file has an error (see logs)"; Settings (under Files) and
Diagnostics say "config error (see logs)". Once you save a fixed file, astromech reads it within a
few seconds and the message goes away; you do not need to restart. (Known bug: astromech-thym.)

## Diagnostics

What astromech sees right now, and a way to collect a bug report.

- **Version, Elite, State, Voice, Kill switch, Setup, Process.** "Elite" is Not running,
  Launcher open or Running. "State" is Idle until Elite starts, then Active. "Setup" lists what
  is still needed, or Done.
- **Save diagnostics.** Writes a zip with the logs, the last crash, the keybinds health report,
  your `astromech.toml` and a short system summary. Your Windows user name and profile paths are
  replaced in the text. It does not include your journals, speech recordings or history, but the logs record what you said, as text. The
  page shows where the zip was saved (a `diagnostics` folder in the data folder); attach it to a
  GitHub issue.
- **Open log folder.**
- If astromech crashed last time, a note says so, with the suggestion to save diagnostics.
- **EDDN.** Whether sharing is on and how many messages are waiting, the gateway's answers and
  the messages that were not sent, each with the reason. See [Data sharing](data-sharing.md).

The key tester is in [Controls](controls.md#key-tester).
