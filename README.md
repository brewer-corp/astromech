# astromech

A voice-first copilot for **Elite Dangerous** on Windows. Hold a button, say "gear down", and the
landing gear goes down. Ask where to sell your cargo and it answers out loud. While you play it
speaks up about what the HUD does not show, and a window next to the game holds your keybinds,
ships, materials, trade and logbook.

> **Alpha.** astromech works day to day but is still changing, and it is not signed. It is an
> unofficial fan project and is not affiliated with Frontier Developments.

## What it does

- **Voice commands.** Push-to-talk ship controls (gear, lights, hardpoints, cargo scoop, flight
  assist, panels and more), checked against what the game reports, and spoken answers about fuel,
  cargo, location and the like. Speech recognition and the voice both run on your PC.
- **Lookups by voice and on screen.** The nearest Interstellar Factors, a module, a ship, the
  best place to sell your cargo, a raw material site, a Powerplay target, what your colonisation
  build still needs.
- **Callouts.** Short spoken lines for what the HUD leaves out: an arrival briefing, docking
  pad position, fuel along your route, prospector finds, scans, High Grade Emissions, a full
  material, mission reminders and a session summary.
- **Keybinds.** Your controller as its maker drew it, every Elite binding searchable, health checks
  and press-to-bind, presets with backups.
- **Ships and engineering.** Your fleet with its stats, a build planner with the materials,
  trades and engineer trips it needs, the engineer unlock tree, Odyssey blueprints.
- **Galaxy, trade, exploration.** Where things are and how many jumps away, commodity prices,
  trade routes, the bodies and biology of the system you are in, Powerplay and colonisation.
- **Local first.** Voice and your history stay on your PC. The network is used for lookups,
  updates, the first speech model download and, only if you turn it on, sharing data with EDDN
  and EDSM. See [Data sharing](docs/guide/data-sharing.md).

## Install

You need Windows and Elite Dangerous on the same PC.

1. Download `astromech_<version>_x64-setup.exe` from the
   [latest release](https://github.com/spadaval/astromech/releases/latest).
2. Run it. It installs for your account, with no administrator prompt. The build is unsigned, so
   Windows SmartScreen says "Windows protected your PC": choose **More info**, then **Run
   anyway**.
3. astromech starts in the tray, with a window for setup. The first run walks you through
   finding Elite, picking push-to-talk and a microphone, and downloading the speech models
   (about 800 MB).
4. Start Elite, hold push-to-talk and say "gear down".

astromech then runs in the tray, starts with Windows (you can turn that off in Settings) and
waits until Elite starts. Later versions arrive as updates from inside the app.

## Read the guide

The [guide](docs/guide/index.md) covers every page and feature:

- Set up: [Getting started](docs/guide/getting-started.md), [Using the app](docs/guide/using-the-app.md), [Settings](docs/guide/settings.md)
- Voice: [Voice](docs/guide/voice.md), [Voice commands](docs/guide/voice-commands.md), [Voice lookups](docs/guide/lookups.md), [Callouts](docs/guide/callouts.md), [Voice tester](docs/guide/voice-tester.md)
- Controls: [Controls](docs/guide/controls.md)
- Ships and engineering: [Ships](docs/guide/ships.md), [Materials](docs/guide/materials.md), [Engineers and on foot](docs/guide/engineering.md)
- Galaxy and trade: [Galaxy](docs/guide/galaxy.md), [Routes](docs/guide/routes.md), [Trade](docs/guide/trade.md), [Mining](docs/guide/mining.md), [Colonisation](docs/guide/colonisation.md), [Powerplay](docs/guide/powerplay.md)
- Exploring and history: [Exploration](docs/guide/exploration.md), [Logbook](docs/guide/logbook.md), [Data sharing](docs/guide/data-sharing.md) (exactly what is sent where)

## Help and feedback

Something wrong? Open Settings, then Diagnostics, and **Save diagnostics**, then
[open an issue](https://github.com/spadaval/astromech/issues) and attach the zip. Do not post
journals or logs you would not want public.

## Contributing

Building from source, the crates, the checks to run and the design documents are in
[CONTRIBUTING.md](CONTRIBUTING.md).

## Licence

MIT, Copyright (c) 2026 Sumeet Padavala. See [LICENSE](LICENSE). The installer's voice worker
links GPL-3.0 libraries, so the installed binaries are GPL-3.0 as a whole; see
[NOTICE.md](NOTICE.md) and [THIRD_PARTY.md](THIRD_PARTY.md).
