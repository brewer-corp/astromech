# Agent guide

astromech is a voice-first copilot for Elite Dangerous on Windows: a Rust core plus a Tauri v2
and Svelte 5 app window. Start with [README.md](README.md).

## Work tracking

The backlog, research and plans live in [Atelier](https://github.com/spadaval/atelier) records
under `.atelier/`, committed with the code they describe. There is no separate backlog file.

- Run `atelier prime` at the start of a session and `atelier land` before handing off (the
  Claude Code hooks in `.claude/settings.json` do this).
- `atelier work missions`, `atelier work ready` and `atelier issue show <id>` orient you.
- Missions: "Replace the daily toolbox" ([strategy](docs/strategies/replace-the-toolbox.md))
  and "Ship v1.0".
- `atelier start <id>` creates the work branch (`<issue_type>/<id>`) from `main`. Integrate
  with `atelier merge <id>` after independent review, never a plain `git merge` into `main`.
  Open a GitHub PR from the work branch for CI. `atelier merge` fast-forwards `main` to the
  reviewed branch; pushing `main` then marks the PR merged.
- Someday work, and the children of a draft epic, sit in the `backlog` status so
  `atelier work ready` shows only near-term work. New issues start in `todo`;
  `atelier issue transition <id> defer` moves one to `backlog`, and `plan` moves it back.
- Records labelled `needs-cmdr` need Sumeet at the PC or in the game. Prepare everything
  else, then hand off.

## Where things are

- Product and v1 scope: [docs/V1-DESIGN.md](docs/V1-DESIGN.md)
- Architecture, crates and dependency rules: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- Data sources and the tool survey: [docs/REPORT.md](docs/REPORT.md),
  [docs/COMPETITORS.md](docs/COMPETITORS.md)
- Checks to run before a PR, the UI check and the game-data rules:
  [CONTRIBUTING.md](CONTRIBUTING.md)

## Constraints

- Never commit Frontier assets, game files, datamined data, models, or personal logs
  (CONTRIBUTING.md). Wiki-visible game data is fine in `data/`.
- Voice callouts only for what the game HUD doesn't already show prominently (strategy, Shape).
