---
created_at: "2026-11-07T06:33:38.680831587+00:00"
id: "astromech-96dv"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-i8vr"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T10:35:15.431461811+00:00"
status: "done"
title: "Type the window's call() and the fake's handlers against the generated commands"
updated_at: "2026-11-07T10:46:11.304497464+00:00"
---

## Goal

The window's `call()` and the fake's handlers are typed against the generated command map. A misspelt command, wrong args or a fixture that doesn't match the core's reply type then fails `pnpm check`.

## Done when

- [x] `call` is typed as `call<K extends WindowCommand>(cmd: K, args: Args<K>): Promise<Reply<K>>`, from `app/src/lib/api/commands.ts`. `WindowCommand` is the `window`-exposure names, so a `local` command doesn't typecheck from app code.
- [x] The preview transport's handlers are typed `{ [K in WindowCommand]: (args: Args<K>) => Reply<K> | Promise<Reply<K>> }`, and their fixtures are checked against the reply types. Each drift the typecheck finds is fixed in the fixture, or, when the core is wrong, in a filed bug. List them in the Close Reason. `app/src/preview/coverage.test.ts` takes its command list from the map.
- [x] Hand-written reply interfaces in `app/src/lib/*.ts` that duplicate a generated type are replaced by the generated one, or re-exported from it.
- [x] `cd app && pnpm -s check && pnpm -s lint && pnpm -s test` pass; `scripts/ui` screenshots of each top-level page in the preview look as before.

## Scope

app/src only. No Rust change unless a generated type is wrong. If so, fix it in the Rust type and regenerate.

## Notes

- 2026-11-07 [note] From astromech-se44's implementer: type-checking the preview fixtures against the generated types shows fields the real core always sends that the fixtures lack: nearby.json query.by_population; plot.json ship.fsd and waypoint jumps; stations*.json filters.max_distance_ly and min_volume; engineering.json and ships.json market_id on ships; exobiology.json genera.analysed; trade.json partial in the cargo plan; voice.json live_from. Typing the fakes here should surface and fix these.
- 2026-11-07 [note] From finding astromech-34i3 (i8vr review): comments and test names still say the shell resolves the pad or serves commands: app/src/lib/ui/SearchKit.svelte:97 and SearchKit.browser.test.ts:86, app/src/modules/trade/Route.svelte:31 and Route.browser.test.ts:18, docs/ARCHITECTURE.md:576, app/src/preview/shell.ts:2 and :311. Correct them here.
- 2026-11-07 [note] Decision (implementer): call() is call<K extends WindowCommand>(cmd: K, ...[args]: ArgList<K>): Promise<Reply<K>>, not (cmd, args: Args<K>). ArgList makes args optional only when the command's args admit null (no-arg commands, and set_ptt/set_kill_switch's Button | null), so the 40-odd no-arg calls aren't call("status", null); a command with args still requires them and their type. Also: replies the window can't get from the core are gone from its types. The preview answered engineering_plan/stats/trip and odyssey_shopping with null for inputs it has no sample for; the core never does, so the preview now fails those calls with the text the page showed for the null (stats and plan: identical on screen; trip and shopping: shown as an error now, preview only). And the browser preview's sample labels (source: "Sample journal history") are typed as Sampled<T> = T & {source?: string} in lib/transport.ts, which the core never sends. For the plan owner to accept or change.
- 2026-11-07 [decision] Accepted the implementer's deviations: call() takes ...[args]: ArgList<K>; the preview fails, as the core does, where it answered null for plans, picks and trips; Sampled<T>.
  Reason: Reviewer astromech-lzkb found them sound; they change the preview only, not the shipped app

## Report

Implementer report (commit 0d1c7c84 on feature/astromech-bp23). Outcome: lib/transport.ts derives WindowCommand (the map's exposure "window" names), Args<K>, Reply<K> and ArgList<K> from lib/api/commands.ts; call<K extends WindowCommand>(cmd, ...[args]: ArgList<K>): Promise<Reply<K>>. Transport.call stays the untyped wire (tests' transports). lib/core.ts: CORE_COMMANDS and the any-typed Reply are gone; CoreCmd is derived (window commands whose reply carries ok); coreCall<K extends CoreCmd>(cmd, ...args): Promise<Reply<K> | Failed>. Preview: preview/recorded.ts has Handler<K>/Handlers ({ [K in WindowCommand]: (args: Args<K>) => Reply<K> | Promise<Reply<K>> }) and recorded<T>(json), which checks a JSON import against reply type T (string literals and tuples widened, as JSON imports type them). previewCommands(): Handlers merges shell.ts (satisfies Partial<Handlers>), binds.ts and the fake core's (coreHandlers: one typed handler per CoreCmd, status included); it is built on first use so the release build drops the fake (check-dist). coverage.test.ts reads the window and local command names from commands.ts ?raw: the preview answers exactly the window ones, the app's sources call only those. Every hand-written reply interface in lib/*.ts that duplicated a generated type is now a re-export (same names kept, e.g. TradeSearch = Search, PlotSetup = PlotShip, PlotShip = PlotterShip, Level = LevelCheck); Sampled<T> adds the preview's sample label. Kept as window types: binds Press (an event), engineering Want (shopping_set's JSON text), odyssey Category, hotspots Want narrowed to the window's Ore and RingType. Generated types are looser in four places the window indexed by literal (mic verdict, intent decision, unlock kind, hotspot ring type); those lookups now take a string with the raw value as fallback.
Drift the typecheck found, all fixed in the fixture or the fake (no core bug): ships.json and engineering.json ships' market_id (re-ran astromech-ship dump-ships-fixture and astromech-engineering dump-fixture; the only change was that field: null for flown ships, 128666762 and 128016640 from the sample journal's StoredShips); trade.json cargo per_commodity partial: null on the two entries without it (as dump-trade gives; the hand-made platinum partial case is kept); nearby.json query.by_population false (galaxy dump-nearby); stations.json, stations-module.json, stations-ship.json filters max_distance_ly and min_volume null (plausible: those lookups don't offer them); exobiology.json genera analysed false (plausible: the only sampling is 1 of 3); voice.json live_from null (plausible: no sample phrase names a system); plot.json ship.fsd (the Asp Explorer drive from crates/astromech-galaxy/tests/plot_route.rs), the neutron route's waypoint jumps 0/10/1 (recorded: tests/plot.rs on spansh-route-result.json) and the local route's waypoint jumps (plausible, invented: 1 per leg and 2 on every fifth, summing to its recorded 180; the per-stop counts were never recorded). Fake core: status now sends update (idle, never checked), toasts [] and history null, which the core always sends; save_diagnostics gives a path, shopping_set the material count, voice_test_live the core's no-session failure, binds_set_active the start preset's path and backup (all answered with no or wrong fields before); the neutron header has name_group 256; inventory here is omitted, not null, under ?core=empty; the empty keybinds state keeps the keyboard and mouse; a typed origin moves the recorded here rather than replacing it with {system}.
Deviations: (1) call takes ...[args]: ArgList<K>, not args: Args<K>: args may be left out only when they admit null (decision note). (2) The preview no longer answers null for plans and picks it has no sample for (the core never does); it fails the call with the page's old message. On screen: identical for plan maths and ship stats; the engineer trip and the On foot shopping list for non-sample inputs now show it as an error (preview only). The window's dead null branches for these in Build.svelte and engineering View.svelte are removed. (3) Finding astromech-34i3: besides the listed lines, the other comments and test names that said the shell does what the core does were corrected (lib headers, Hotspots, Shipyard, NeutronFile, colonisation, help and stations/hotspots tests, fakecore). ARCHITECTURE §18.2 Types and CONTRIBUTING describe the typed call and recorded().
Proof: deliberate failures, each reverted: nearby.json without by_population fails pnpm check at shell.ts (recorded<Nearby>); call("galaxy_sytems") ("not assignable to parameter of type 'WindowCommand'"), call("galaxy_systems", { prefix: 42 }) ("Type 'number' is not assignable to type 'string'"), call("quit") (not a WindowCommand); neutron_check answering { newer: "no" } (not assignable to Handler<"neutron_check">); voice_preview without line (CoreHandler<"voice_preview">); shell_info's handler removed ("Property 'shell_info' is missing" in previewCommands). After revert pnpm check 0 errors. Screenshots: scripts/ui at 1200x800 of all 24 pages with ?core=ready and with no core, the 10 Places finders and ?route=exact, before (HEAD in a scratch copy with the same node_modules) and after: 59 of 59 PNGs byte-identical; page text identical except Settings' Updates line, "—" to "Not checked yet" (status.update now sent). Taller shots: Engineering ship pages for the stored Krait and Asp add a Spansh station link (market_id). Data-age lines ("21d") differ between runs taken minutes apart only as the clock crosses a day (rechecked on the old build). cd app && pnpm -s check (0 errors), pnpm -s lint (clean), pnpm -s test (74 files, 532 tests) pass; pnpm build then python3 scripts/check-dist.py app/dist: "74 files in app/dist, no browser preview" (first failed: top-level fake-core spreads kept the preview in the bundle, hence previewCommands()). No generated file or Rust changed, so cargo test -p astromech-core --lib bindings was not needed.
Open items: (1) plot.json's neutron route says jumps 128 (Spansh's old leg count; its trimmed recorded legs are 10 and 1, and today's plotter would say 11); left as is, so a preview neutron plot says 128 jumps and the route strip 11 left. (2) Tests' answer() and previewAnswer() stay untyped (out of scope); a test's own reply isn't checked against the map. (3) Review outstanding.
