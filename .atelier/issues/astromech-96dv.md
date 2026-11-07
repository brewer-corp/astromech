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
status: "todo"
title: "Type the window's call() and the fake's handlers against the generated commands"
updated_at: "2026-11-07T09:08:04.796667048+00:00"
---

## Goal

The window's `call()` and the fake's handlers are typed against the generated command map. A misspelt command, wrong args or a fixture that doesn't match the core's reply type then fails `pnpm check`.

## Done when

- [ ] `call` is typed as `call<K extends WindowCommand>(cmd: K, args: Args<K>): Promise<Reply<K>>`, from `app/src/lib/api/commands.ts`. `WindowCommand` is the `window`-exposure names, so a `local` command doesn't typecheck from app code.
- [ ] The preview transport's handlers are typed `{ [K in WindowCommand]: (args: Args<K>) => Reply<K> | Promise<Reply<K>> }`, and their fixtures are checked against the reply types. Each drift the typecheck finds is fixed in the fixture, or, when the core is wrong, in a filed bug. List them in the Close Reason. `app/src/preview/coverage.test.ts` takes its command list from the map.
- [ ] Hand-written reply interfaces in `app/src/lib/*.ts` that duplicate a generated type are replaced by the generated one, or re-exported from it.
- [ ] `cd app && pnpm -s check && pnpm -s lint && pnpm -s test` pass; `scripts/ui` screenshots of each top-level page in the preview look as before.

## Scope

app/src only. No Rust change unless a generated type is wrong. If so, fix it in the Rust type and regenerate.

## Notes

- 2026-11-07 [note] From astromech-se44's implementer: type-checking the preview fixtures against the generated types shows fields the real core always sends that the fixtures lack: nearby.json query.by_population; plot.json ship.fsd and waypoint jumps; stations*.json filters.max_distance_ly and min_volume; engineering.json and ships.json market_id on ships; exobiology.json genera.analysed; trade.json partial in the cargo plan; voice.json live_from. Typing the fakes here should surface and fix these.
