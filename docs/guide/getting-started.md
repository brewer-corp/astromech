# Getting started

This page takes you from download to "gear down" by voice, and says how updates work.

## What you need

- Windows, with Elite Dangerous played on the same PC. astromech reads the game's journal
  and its files, so it cannot see a game running on another machine.
- About 800 MB of free space for the speech models, and a microphone. A headset mic is fine.
- A network connection for the first model download, for lookups (Spansh, Ardent Insight,
  EDSM) and for updates. Voice commands, callouts, keybinds and your history work without one.
- A keyboard key in Elite for each action you want to say by voice. astromech presses keys,
  not stick buttons. Setup checks this for you.

## Install

1. Download `astromech_<version>_x64-setup.exe` from the
   [latest release](https://github.com/spadaval/astromech/releases/latest).
2. Run it. It installs for your account only, with no administrator prompt, into
   `%LOCALAPPDATA%\astromech`.
3. The installer is not signed. Windows SmartScreen will say "Windows protected your PC".
   Choose **More info**, then **Run anyway**. Each release lists a SHA-256 checksum if you want
   to check the file first.

astromech then starts in the tray (the icons by the clock) and opens its window.

Run Elite and astromech normally. If one of them runs as administrator, Windows blocks the
key presses and astromech tells you so.

## First run

The window opens in **Setup**. It has six steps; Back and Next move between them, and **Skip
setup** leaves the rest for later. Until voice can work, the header shows **Finish setup**.

1. **Find Elite.** astromech looks for Elite's keybindings (Options\\Bindings, under Frontier
   Developments) and journals (Saved Games). Both appear after Elite has run once on this
   Windows account. If they are somewhere else, paste your Bindings folder. The game install
   folder is optional; without it Elite's built-in presets are not listed, but your own are.
2. **Controllers.** Lists the sticks, throttles and pads Elite can see. Press a button to check
   it arrives. Elite reads only buttons 1 to 32 on each device.
3. **Push-to-talk.** Click **Set**, then press the button, hat direction or key you will hold to
   speak. Do the same for the **kill switch**, a button Elite does not use. The kill switch is
   optional but recommended: it stops everything astromech is doing (see
   [Voice](voice.md#the-kill-switch)). astromech only listens to the press. It sends nothing to
   the game. If the button is already bound in Elite, setup says what it does there.
4. **Microphone.** Pick the mic you speak into while flying. This matters in VR, where
   Windows may switch its default to the headset. **Check level** records two seconds and tells you
   whether the level is good, quiet or silent.
5. **Calibration.** A moment of silence and three phrases, so quiet speech is heard and room
   noise is not. It sets the level below which a clip is ignored. Run it again in
   [Settings](settings.md#microphone) after changing mic or headset.
6. **Health check.** Lists what stands between you and "gear down": Elite's folders,
   push-to-talk, speech models, calibration and your keybindings. Each row has a fix
   button. A voice action with no keyboard key is an error with **Bind a key**: press the key
   you want and astromech writes it into your preset (see [Controls](controls.md#the-bind-dialog)).
   When everything required is fine, it says "Ready".

The speech models (about 800 MB) download from the health check or from Settings. They are
checked against a SHA-256 hash and kept in astromech's data folder (see below).

When setup is done, start Elite, get into your ship in normal space, hold push-to-talk and say
"gear down". Try it somewhere an unexpected gear change cannot hurt.

Push-to-talk and the kill switch you set take effect the next time Elite starts, if Elite is
already running; setup says so.

## The tray icon

astromech's main process lives in the tray and uses little memory until Elite starts. It loads
the speech models only while Elite (or its launcher) is running, and unloads them after Elite
has gone.

- Left-click the icon to open the window.
- Right-click for the menu. The first line says what astromech is doing: "Waiting for Elite",
  "Elite running", "Starting voice", and so on.
  - **Open astromech** opens the window.
  - **Pause voice** stops it listening to push-to-talk. Spoken [callouts](callouts.md) carry on.
  - **Kill switch (stop all actions)** appears while Elite is running. When the kill switch is
    on it reads **Re-arm**.
  - **Finish setup** appears while something is missing.
  - **Restart to update** appears when an update is ready.
  - **Quit astromech** stops the whole thing.

By default astromech starts with Windows, quietly in the tray. Turn that off in
[Settings](settings.md#startup).

## Updates

astromech checks GitHub for a new version about once a day, including while you play, and
downloads it when found. Updates are signed; astromech checks the signature before using one.

- **Settings, Updates** shows the version and whether an update is available. **Check now**
  asks straight away. When an update is ready, **Install and restart** closes astromech for a few
  seconds and reopens it. Elite can keep running.
- The tray menu shows **Restart to update** when one is ready.
- The first install is the setup.exe. Later versions are zips that replace astromech's
  files only. Your settings, history, logs and speech models are never replaced.
- In `astromech.toml` the `[update]` table has `check` (true or false), and `apply`: `ask`
  (the default, wait for you), `on_quit` (install when astromech quits) or `never`. Open the
  file from Settings, Files, **Open**. With no file yet, that writes the sample, which sets the kill
  switch to Scroll Lock.

If an update does not install, Settings says why. Run the setup.exe from the release page
instead; it keeps your data.

## Where things are kept

Everything astromech writes is in `%LOCALAPPDATA%\astromech\data`: the settings file
`astromech.toml`, the models, the journal history, the star map, caches and logs. It never
writes to the game's folders. The one exception is your keybindings, and only when you ask
(see [Controls](controls.md)), with a backup first.

Uninstall from Windows Settings. The uninstaller keeps your data unless you tick the box to
delete it.

## If something is wrong

- **Finish setup** in the header lists what is missing.
- **Settings** shows push-to-talk, the models, the microphone and calibration at a glance.
- **Diagnostics** shows whether astromech sees Elite, the voice state and the kill switch.
  **Save diagnostics** makes a zip of the logs for a bug report.
- If the window says astromech's background process is not running, it starts with the
  window. If that message stays, open Diagnostics, or look in `data\logs`.
- Hold push-to-talk, say "gear down", and nothing happens: [Voice](voice.md#when-nothing-happens)
  has a checklist.

Next: [Voice](voice.md).
