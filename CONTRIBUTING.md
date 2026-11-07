# Contributing

Thanks for your interest! astromech is an alpha, so expect big changes. Players should start at
the [README](README.md) and the [user guide](docs/guide/index.md); this page is for people changing
the code.

## Where things are

- [docs/V1-DESIGN.md](docs/V1-DESIGN.md): what v1 is, and the copy rules for what astromech says (§6.5).
- [DESIGN.md](DESIGN.md): the window's design system, binding for UI changes.
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): architecture, crates, dependency rules and spike
  results. Open an issue before large changes.
- [docs/PLOTTING.md](docs/PLOTTING.md): routes: the next waypoint on the clipboard, "paste" in the galaxy map, and the plotting macro to come.
- [docs/KEYBINDS.md](docs/KEYBINDS.md): the keybinds module and the health check.
- [docs/REPORT.md](docs/REPORT.md) and [docs/COMPETITORS.md](docs/COMPETITORS.md): the research.
- [docs/diagrams/](docs/diagrams/): Mermaid sources and rendered PNGs. [spikes/](spikes/): the
  feasibility spikes.

```
crates/         the Rust crates (astromech-core is the tray process; the rest are libraries)
app/            Svelte 5 + TypeScript + Vite app shell; one folder per page in src/modules/*
app/src-tauri/  astromech-shell, the Tauri v2 window (a workspace member; a stub without its `window` feature)
data/           player-visible game data and the device sheets
docs/           design documents, and the user guide in docs/guide/
spikes/         spike tools (own code only)
worlds/         scripted worlds the headless core runs in (`astromech-core --world`)
scripts/        CI helpers (dependency-rule check); `scripts/ui` drives the Vite shell in headless Chrome
```

## Building

Requirements: Rust (pinned in `rust-toolchain.toml`), Node 22+ and pnpm 10 for the app.

```sh
cargo build --workspace          # all crates; Windows-only code compiles to stubs elsewhere
cargo test --workspace
python3 scripts/check-deps.py    # dependency rules
cargo deny check                 # licences, advisories, sources

cd app && pnpm install && pnpm build    # frontend
pnpm tauri build                        # app window (Windows; Linux needs webkit2gtk-4.1)
```

The window is in the workspace, but Tauri is behind its `window` feature, so `cargo build
--workspace` and `cargo test --workspace` build it as a stub and need no webkit2gtk.
`pnpm tauri build` and `pnpm tauri dev` turn the feature on (`build.features` in
`tauri.conf.json`); to build or lint it with cargo, add `-p astromech-shell --features window`.
Everything builds into the one `target/` folder.

The repository's version is the placeholder `0.0.0-dev`, which sorts below every release, so a
build from a checkout always accepts a published update. Don't bump it by hand: see
[Releasing](#releasing).

The local voice worker is behind a feature, because it downloads prebuilt sherpa-onnx libraries
and makes the binary GPL-3.0 (see NOTICE.md):
`cargo build -p astromech-voice --features sherpa --release`.

`scripts/voice-eval.sh` scores voice recognition on a synthetic 200-phrase set (Kokoro renders it,
Parakeet transcribes it; it downloads the models, about 830 MB, on first run). The report is
`results/voice-eval/report.md`; never commit the audio or the models.

Windows binaries can be cross-built from Linux with `cargo xwin build --target x86_64-pc-windows-msvc`.
They use a static CRT (`.cargo/config.toml`), so no DLLs are needed.

`astromech-core` also runs without the tray for debugging: `--console` (type commands), `--no-voice`,
`--dry-run` (log key presses instead of sending them), `--health` (the binds check), `--listen 20`
(print button names) and `--debug-input`. `astromech-core --help` lists them all.

## Before a PR

- Run `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `python3 scripts/check-deps.py`, `python3 scripts/check-guide.py` (guide links), `cargo deny check`, and in `app/`:
  `pnpm lint && pnpm check && pnpm build && pnpm test`, then `python3 scripts/check-dist.py` (the build carries no browser preview).
  `pnpm test` runs the UI tests in headless Chromium: install it once with
  `pnpm exec playwright install --only-shell chromium` (see [Tests](#tests)).
- Tracker records: the **Atelier** workflow (`.github/workflows/atelier.yml`) runs
  `atelier doctor` on every PR and every push to `main`, record-only commits included. It
  checks every done claim and the whole review history as they will stand on `main`, with
  Atelier built at the commit the workflow pins (`ATELIER_REV`). On a push to `main` it also
  runs `atelier land` on `main` over the pushed commits (from the commit `main` was at before the
  push), so the job fails when a pushed commit changes code without recorded review
  (`land_base_unreviewed`) or marks an issue done outside review (`land_base_done_claim`), the
  same refusals `atelier land` on `main` gives before a push. That check comes after the push:
  the commits are on `main` by then, and the fix is a revert. Run `atelier land` on `main` and
  `atelier doctor` locally before pushing; the Stop hook's `atelier land` checks only what the
  session changed.
- **A change a player can see updates its guide page in the same PR.** That covers a new or
  changed page, setting, voice command, lookup or callout, and anything removed. The page for
  each app page is in [docs/guide/](docs/guide/index.md); the app's Help links point at them, and
  `scripts/check-guide.py` fails if a link target (or a help link's page or heading) is missing. Say what the feature does,
  how to use it, what it needs (network, settings, game data) and its limits, in plain short
  sentences like the app's own copy (docs/V1-DESIGN.md §6.5). Describe what the code does, not
  what is planned. A page you add goes in [docs/guide/index.md](docs/guide/index.md) too, and every
  link must resolve. A change with no visible effect needs no guide edit.
- UI changes follow [DESIGN.md](DESIGN.md), the window's design system: use its tokens
  (`app/src/design/tokens.css`, mapped in its Token map) and check the page against its rules.
  Run the Vite app (`cd app && pnpm dev`) and check it with headless Chrome:
  `scripts/ui --url http://127.0.0.1:5173/#/materials --wait Iron --screenshot /tmp/materials.png --text`.
  Needs Google Chrome (or `CHROME` pointing at Chromium). `scripts/app-shots.sh` and
  `scripts/mockup-shots.sh` use the same binary; `scripts/mockup-shots.sh ds-sell ds-places`
  re-renders only the named mockups (the DESIGN.md ones are `ds-*`).
- Outside Tauri (`pnpm dev`, the tests, a preview build) the window's one transport,
  `app/src/lib/transport.ts`, is answered by the browser preview's fake in `app/src/preview/`
  (`?core=ready`, `?route=exact`, `?fail=`, `?slow=`). App code reaches the backend only through
  `call`, `subscribe` and `previewOption` (lint enforces it); a new command or topic needs a fake
  handler (`app/src/preview/coverage.test.ts`), and a release build must carry none of the fake
  (`scripts/check-dist.py app/dist` after `pnpm build`, run in CI).
- Every command's args, reply and progress types are exported to TypeScript from the Rust
  command layer (ts-rs): `app/src/lib/api/types/` (generated, so lint skips it) and
  `app/src/lib/api/commands.ts`, which maps each command name to `{ args, reply, exposure,
  progress? }`. After changing a command or a type it returns, update them with
  `ASTROMECH_UPDATE_BINDINGS=1 cargo test -p astromech-core --lib bindings` and commit the result.
  `cargo test -p astromech-core` fails while the checked-in files differ, and
  `cargo test -p astromech-api` while the command layer's own part does. A `serde_json::Value`
  (ts-rs's `JsonValue`) in a command's args or reply needs an entry with a reason in
  `JSON_VALUES` (crates/astromech-core/src/app.rs). Two types with one name need a
  `#[ts(rename = "…")]`; the test names them. The window's `call` and the browser preview's
  handlers are typed by the map, and each preview fixture is read through `recorded<ReplyType>()`
  (`app/src/preview/recorded.ts`), so `pnpm check` fails on a fixture missing a field the core
  sends. Take the field from the fixture's recorded input (or the example that dumps it).
- A new `window` command needs an entry in the table in
  `crates/astromech-core/src/app/every_window_command.rs`: example args and what the call gives
  (ok, or the handler's own error). The test calls each one through a core's control API, as the
  shell sends it, and fails for a `window` command with no entry. It has no network and starts no
  program: a command whose effect has no seam yet is called with state that stops it first, or not
  called, and is named in `NOT_RUN_FOR_REAL` with the missing seam.
- To show a UI change to someone off this machine, work on the real page on its branch against the
  browser preview (add a fixture under `app/src/preview/fixtures/` when the change needs data the
  preview lacks), then `scripts/preview-publish --page route --query route=exact`. It builds the
  preview into `app/dist-preview/` with the scenario and start page baked in, since an artifact link
  carries neither, and lists the files to publish as one claude.ai artifact beside `index.html`. Republish the same `index.html` to update the link.
- Respect the dependency rules: actions go through `astromech-policy`; voice and tools never use
  the input executor directly.
- Player-visible game data (anything on the Elite Dangerous wiki) is fine in `data/`; credit its
  source in `NOTICE.md`. Never commit Frontier assets, copies of game files (default `.binds`),
  datamined data, models, or personal logs and results (journals, device paths, window titles).
  Use synthetic fixtures.
- The star map preload is not committed and not rebuilt per release. The manual **Star map
  preload** workflow (`.github/workflows/starmap.yml`) builds it from Spansh's systems dump and
  publishes `starmap.db.gz` on the `starmap` pre-release; tagged release builds download that.
  Refresh it only when it's worth it. Local shell builds bundle an empty file (the app fills in
  from EDSM as you travel); for the real one, `gh release download starmap` and gunzip it into
  `app/src-tauri/resources/starmap.db`.
- The neutron star file for the local neutron plotter is built the same way: the manual
  **Neutron star file** workflow (`.github/workflows/neutron.yml`) runs `scripts/gen-neutron.py`
  on Spansh's neutron dump and publishes `neutron.bin.gz` and its SHA-256 on the `neutron`
  pre-release. The app downloads it only when the user asks. The format is in the script's
  docstring; `python3 scripts/test_gen_neutron.py` checks the script against the reader's fixture.
- EDDN outfitting needs every module a station lists in `data/catalog/eddn-outfitting.json`: a
  module in neither of its lists makes astromech send nothing for that station
  (`crates/astromech-eddn/src/companion.rs`). The **EDDN module list** workflow
  (`.github/workflows/eddn-outfitting.yml`) runs `scripts/gen-eddn-outfitting.py --check` against
  EDCD/FDevIDs' latest commit every Monday and by hand, and fails naming each new, moved or gone
  symbol. When it fails, regenerate the file (`scripts/gen-eddn-outfitting.py <FDevIDs checkout>`),
  commit it and ship it in the next release. `python3 scripts/test_gen_eddn_outfitting.py` tests the
  check offline. The real-journal harness (`crates/astromech-core/tests/real_journals.rs`) also
  reports any module in your `Outfitting.json` the file doesn't know as a problem.
- The **Release build** also runs `scripts/onboarding_smoke.py` after the installer smoke test: a
  fresh silent install, the onboarding wizard driven in the real window, then "gear down" run live
  from the voice tester against the mock game, with each step's time. It opens the window with
  `astromech-shell --webview-debug-port <port>`, which turns on WebView2's DevTools port on
  127.0.0.1 so the page can be driven; nothing else uses that flag.
- The game-free MVP criteria (docs/ARCHITECTURE.md §14 Phase 1) are measured on Windows by
  `scripts/mvp_criteria.py`: the release core against the mock game window
  (`crates/astromech-mockgame`), with a report against each threshold. The
  **MVP criteria** workflow runs a 5-minute soak on PRs that change the core; the full 4 h run is
  manual (Actions → MVP criteria → Run workflow). Its analysis tests run anywhere:
  `python3 scripts/test_mvp_criteria.py`.
- New dependencies must pass `cargo deny` (permissive licences only).
- Contributions are accepted under the MIT licence.

## Tests

These rules come from decision astromech-1zhg. They apply to new tests and to any test you
change; [AGENTS.md](AGENTS.md) has the short form.

**What to fake.** Test doubles go only at true boundaries: the game's files, the network, the
clock, input devices, audio and the clipboard. The core's behaviour always runs for real.
Fixtures are recorded raw inputs (journals, Spansh and Ardent responses), dumped or recorded
from the real thing, never invented replies. A reply written in a test is fine only to set up a
failure, an empty or absent state, or a fault the fixtures don't hold (`answer("status", () =>
null)`, a rejected call); data the page shows comes from the recorded fixtures.

**Where a case goes.** Test each case once, at the lowest layer that fails without the fix.
Algorithms (parsers, jump range and fuel, plotters, engineering maths) are tested in Rust,
table-driven; add a row to an existing table before writing a new test. Don't repeat a case in a
wrapper, at each call site or in the page that shows it.

**What not to test.** Copy text, static markup, callback wiring, what the types already
guarantee, mock-call mirrors (a test that only checks a function called the function it calls)
and smoke renders. A behaviour test asserts what the commander sees or what reached the
backend after something happened.

**Bugs and visual changes.** A logic, data, state or protocol bug gets a failing test first: the
fewest cases that fail on `main` for that reason, in an existing case where one fits, then the
fix. A purely visual change (spacing, colour, alignment, a layout shift) is proven with
before-and-after captures (`scripts/ui ... --screenshot`, `scripts/app-shots.sh`), not a test;
don't add a UI test for it.

**Determinism.** Tests wait on events, never on time. No sleeps (`setTimeout`), no polling
loops, no wall-clock reads (`Date.now()`, `performance.now()`) and no retries. Wait for what the
page shows (`findBy…`, `waitFor`, `vi.waitFor`) or for the calls to answer (`idle()`, below).
Code that runs on a timer is driven with `vi.useFakeTimers` (fake only what it uses, such as
`toFake: ["setInterval", "clearInterval"]`, so the waits keep working) and `vi.setSystemTime` for
dates. A flaky test is fixed, not retried. Skipping one (`it.skip`, `describe.skip`) is a
quarantine: it needs an Atelier issue, named on the line above (`// quarantine: astromech-xxxx
<why>`). Lint rejects `setTimeout` and wall-clock reads in test files
(`app/eslint-rules/tests.js`, tested by `tests.test.js` beside it).

**The frontend's two test projects** (`app/vite.config.ts`):

- `node`: pure logic in `app/src/**/*.test.ts`, run in Node.
- `browser`: UI behaviour in `app/src/**/*.browser.test.ts`, run in real headless Chromium
  (Vitest browser mode with Playwright) at the app window's size. Name a test file
  `.browser.test.ts` when it needs a DOM, `window` or `location`.

`pnpm test` runs both. Run one file with `pnpm vitest run src/modules/route/Plot.browser.test.ts`,
or one project with `--project browser`. The browser project needs Playwright's Chromium:
`pnpm exec playwright install --only-shell chromium` (CI installs it with `--with-deps`). On a
Linux release Playwright doesn't know yet, it refuses to install; set
`PLAYWRIGHT_HOST_PLATFORM_OVERRIDE=ubuntu24.04-x64` for the install.

**UI tests fake only the transport.** A UI test renders the real page and lets the browser
preview's fake (`app/src/preview/`) answer its calls, as `pnpm dev` does. To change an answer,
use `app/src/test/transport.ts`, not `vi.mock` of an app module:

- `answer(cmd, (args) => reply)` answers a command (a rejected promise or a throw is a failed
  call). At a file's top level it holds for the whole file; in a test or `beforeEach`, for that
  test.
- `watch(cmd)` is a mock called with each call's args, for asserting what reached the backend.
- `previewAnswer(cmd, args)` is the fake's own answer: what the fake core holds now, or a reply
  to adjust inside an `answer`.
- `startPreview("/?core=ready&fail=route")` runs the preview under a query from a fresh fake core
  (the query switches are listed in "Before a PR"); before each test the helper empties the
  query and starts the fake core and the neutron file over.
- `idle()` resolves once every call has answered and the page has drawn the answers, for
  asserting that nothing more happens. `sent` records every call, for a call that never answers.

The fake world's own events come from `app/src/preview/fakecore.ts` (`fakeJump`,
`fakePlotInGame`, `fakeClipboard`, `fakePreviews`); lint allows only those from it in tests.
The preview's own tests (`app/src/preview/`) and the two tests of the app's side of Tauri
(`staleTauri`, `tauriFailure`, which fake `invoke`) are exempt. A few older tests still
`vi.mock` an app module (the voice tester, colonisation, Powerplay targets, the route plot, the
stale-data and callout announcement tests, and `lib/mining.test.ts`); move one behind `answer`
when you change it. `vi.mock` of a module the preview imports deadlocks in the
browser, so never add one.

**The core in a world.** A Rust test that needs the whole core starts it headless in a world:
`astromech-core --world <dir>` copies the world directory to a temp folder, runs the real core
there with fakes for the processes, opener, clipboard, notifications, Start with Windows and
secrets, and prints its control port and token on stdout. The format and the `world_observed`
and `world_wait` commands are in `crates/astromech-core/src/world.rs`; the worlds are in
`worlds/`. `crates/astromech-core/tests/support/` starts one (`WorldCore`) and calls it over the
control API. Each world core is its own process, because the core keeps process-wide state. The
helper points every home variable (`HOME`, the `XDG_*` folders, `LOCALAPPDATA`, `APPDATA`,
`USERPROFILE`) at an empty folder that `quit()` checks is still empty, and holds the core's
stdin: world mode quits when its stdin closes, so a killed test leaves no core running. Started
by hand, keep its stdin open: `sleep infinity | astromech-core --world worlds/minimal`.

## Releasing

1. Write `docs/releases/vX.Y.Z[-pre].md` (the GitHub release body, as published) and point
   README's release-notes link at it. `python3 scripts/check-release-notes.py <file>` checks
   that every open `needs-human` issue is a test step once and that guide links resolve.
2. Get it reviewed and merged like any change (`atelier merge`). It touches only Markdown, so
   no CI runs for it.
3. Tag `main` (`git tag -a vX.Y.Z[-pre]`) and push the tag. The **Release build** stamps the
   tag's version into the version files (`scripts/set-version.py`), builds and smoke-tests the
   installer, and publishes the release with the signed update. It fails before publishing if
   the notes file is missing or any step fails.
