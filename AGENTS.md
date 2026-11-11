# Agent guide

astromech is a voice-first copilot for Elite Dangerous on Windows: a Rust core plus a Tauri v2
and Svelte 5 app window. Start with [README.md](README.md).

## Work Tracking

Work is tracked with Atelier, in `.atelier/`: missions in
`missions/<id>/mission.md`, items in `items/<id>/item.md` with their
reviews beside them, and the rules in `config.toml`.

- Run `atelier status` first; Claude Code and Codex run it at session start
  through `.claude/settings.json` and `.codex/hooks.json`. It says what you
  hold and the next command, and `atelier <command> --help` says what each
  does. Agent Factory, loaded as `/agent-factory`, says how to decide.
- An item, in order: `atelier start <id>`, implement, tick every Done when
  line and write the `## Report`, `atelier submit`. A reviewer, in a thread
  that did not write the change, runs `atelier review <id>` and `atelier
  verdict`. Once approved, the implementer runs `atelier merge`, which
  marks the item done on `main`.
- Planning happens on `main` with `atelier new`, `set` and `ready`;
  `ready`, `drop`, `close` and `lead` make their own commit. A mission is
  read with `atelier show <mission>`.
- This repository's config names no forge, so nothing pushes for you.
  After `atelier merge`, or after planning on `main`, push it with
  `git push origin main`. The pre-push hook checks every commit the push
  brings to `main`: code arrives only through an Atelier merge. A rule
  can be passed on purpose with `ATELIER_SKIP="<reason>"` on the command;
  use it only when you mean to.
- `atelier setup` installs the Git hooks in a new clone.

## Where things are

- Product and v1 scope: [docs/V1-DESIGN.md](docs/V1-DESIGN.md)
- The window's design system (colours, type, layout, components, states), binding for any
  UI change: [DESIGN.md](DESIGN.md), with its tokens in `app/src/design/tokens.css`
- Architecture, crates and dependency rules: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- Data sources and the tool survey: [docs/REPORT.md](docs/REPORT.md),
  [docs/COMPETITORS.md](docs/COMPETITORS.md)
- Checks to run before a PR, the UI check and the game-data rules:
  [CONTRIBUTING.md](CONTRIBUTING.md)

## Tests

The rules are in [CONTRIBUTING.md](CONTRIBUTING.md#tests) (decision astromech-1zhg); in short:

- Doubles only at true boundaries (game files, network, clock, input, audio, clipboard); the
  core runs for real. Fixtures are recorded raw inputs, never invented replies; a reply written
  in a test only sets up a failure, an empty state or a fault the fixtures lack.
- One case, at the lowest layer that fails. Don't test copy text, static markup, callback wiring
  or what the types guarantee.
- A logic bug gets a failing test first; a visual change gets before-and-after captures, not a test.
- No sleeps, polling, wall-clock reads or retries: wait on events, fake timers for timer code.
  A skipped test is a quarantine and names an Atelier issue. Lint enforces what it can.
- UI tests are `*.browser.test.ts` in headless Chromium and fake only the transport, through
  `app/src/test/transport.ts`.

## Constraints

- Never commit Frontier assets, game files, datamined data, models, or personal logs
  (CONTRIBUTING.md). Wiki-visible game data is fine in `data/`.
- Voice callouts only for what the game HUD doesn't already show prominently (strategy, Shape).
