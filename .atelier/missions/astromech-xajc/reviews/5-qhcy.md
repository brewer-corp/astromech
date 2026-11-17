---
schema: atelier.review
schema_version: 3
round: 5
commit: 1879238dfd927276ab81230297c35800b3eb4e13
reviewers:
- 01a1e913-e7f0-7df3-b199-587cc52fe83a
model: grok-4.6
verdict: changes
created_at: 2026-11-17T10:36:29.794062234Z
---

## Summary

- Complexity: high. Increments 3 and 4 rebuild every remaining page, add a Now home, move three live views, and extend the core for loadouts, body parents, rank start, session payments and suit-engineer coordinates.
- Value: high. The pages are in daily use, increment 2 is on main, and the screens follow Sumeet's recorded stop and the approved mockups.
- Risk of merging it: medium. Visual misses show in the preview and revert per page. Core additions have named tests. One leftover of the suit-pick contract still silently double-costs a plan.

The outcome is worth the rest. Round 4's seven fix-now findings are closed in the items they named. One hole remains in how ADD TO PLAN treats the generic picks every current Suits user already has.

I read `mission.md`, `design.md`, `notes.md`, and every listed item (draft screens, w1lj, 70ts, zaqh, 8e22, gvy0, crxa, wo8s, the dropped checkpoint bugs, and the increment 2 items). I checked the round-4 claims against `scripts/browser/audit.mjs` and `docs/design/increment2/plan.json` on this commit, 8e22's second Done when line, wo8s's empty `mission`, `Pick` / `pickKey` / `shopping.rs:182`, `lookups.rs:377`, `cargo.rs` `sold` / `sellable`, `trade.ts:149`, `ShipStatus.sellable`, `DockedHere` on `FsdJump`, `SERVICE_NAMES`, `Hexes.svelte` `tone="grade"`, `NextRank`, `ScanBody.parents`, `session.rs` earnings, `fakecore.ts:421` and `:530`, `App.svelte` `DEFAULT` and the three `#/controller` cases, `gen-starmap.py` `SHIP_ENGINEER_IDS`, and the recorded `trade.json` cargo load of 600 t. I did not run the app, the journeys, or Windows WebView2.

## Finding 1: Let 8e22 close with the checkpoint failures already handed to later items
Recommendation: fix now
Was: 4-ayza.1
Now: fixed

`.atelier/items/astromech-8e22/item.md:38` now allows a filed bug to be fixed and recaptured, handed to the screen that rebuilds its page (lf3a → rj6i, 8og8 → nnn0, 5g9w → 8wr4), or filed on its own when older than increment 2 (wo8s). The boxes are ticked; atelier shows 8e22 as done on main (`faeb2878`). The three dropped items no longer say 8e22 waits on them.

## Finding 2: Make landing the audit tool a Done when of 8e22
Recommendation: fix now
Was: 4-ayza.2
Now: fixed

`scripts/browser/audit.mjs` and `docs/design/increment2/plan.json` are on this commit (merged with 8e22). The script takes `--base`, `--plan` and `--only` and writes `sideways`, `clipped`, `overlaps`, `small`, `far`, `contrast` and `bright`. The screens can tick their checks from main.

## Finding 3: Decide whether wo8s is in the mission
Recommendation: fix now
Was: 4-ayza.3
Now: fixed

`.atelier/items/astromech-wo8s/item.md` has `mission: null`. Its Scope says it is not part of astromech-xajc. `design.md:191-192` calls it a standalone Controls bug. The Out table and the item agree.

## Finding 4: Reconcile the checkpoint gate with the three fixes assigned after it
Recommendation: fix now
Was: 4-pi08.1
Now: fixed

The same Done when line as Finding 1 names the three handoffs. `design.md:186-190` records that 8e22 merged with them handed over, and the screen items still carry the cases as their first Done when lines.

## Finding 5: A suit pick cannot be treated as a plan for every compatible owned item
Recommendation: fix now
Was: 4-pi08.2
Now: fixed

`.atelier/items/astromech-w1lj/item.md:27-28` gives each owned item and loadout slot its journal id and adds optional `Pick.item`, costed once per pick, with tests for two items of one recipe and one item in two loadouts. `.atelier/items/astromech-nnn0/item.md:30-31` shows a pick only on the item it is tied to, lists untied old picks to tie or remove, and includes `item` in `pickKey` / `hasPick`. Finding 8 is the leftover of that contract, not this one.

## Finding 6: Refresh Now's cargo answer when its origin or sellable hold changes
Recommendation: fix now
Was: 4-pi08.3
Now: fixed

`.atelier/items/astromech-coq6/item.md:37-38` asks `trade_cargo` again when a key of `ship_status.sellable`, `ship_id`, `pad`, `max_jump_range` and `docked_here.system` changes, never on a timer; the old answer stays Grey Dim with its actions disabled. Those are the command's inputs (`lookups.rs:377`, `cargo.rs:263-268`, `ShipStatus.ts`). `docked_here.system` follows `GameState.system`, which `FsdJump` updates. The key is a pure function with a table test; `fakeJump` must move `docked_here` and has a browser case.

## Finding 7: Identify a partial cargo sale beside Now's recommended total
Recommendation: fix now
Was: 4-pi08.4
Now: fixed

`.atelier/items/astromech-coq6/item.md:32` always shows "takes all 600 t" or "takes 20 of 200 t" (`StationSale.sold` against `plan.load`) on the same line as the total, from one function shared with `cargoResults` (`trade.ts:149`), table-tested for the whole hold, part of one commodity, and one commodity of a mixed hold. An empty `stations` list has no recommendation and no destination action. The recorded `trade.json` cargo load is 600 t with `sold: 600` on the first station.

## Finding 8: ADD TO PLAN must not add a second pick of a recipe that is already saved without an item
Recommendation: fix now

`.atelier/items/astromech-nnn0/item.md:31` keeps every pick saved before this change untied, listed under the shopping list with a Dropdown that ties it. Line 36's ADD TO PLAN "adds, tied to the selected item by its journal id, the next grade of that item", and `pickKey` / `hasPick` include `item`, so a recipe-only pick and a tied pick of the same recipe are two keys. `.atelier/items/astromech-w1lj/item.md:28` costs each pick once and tests "a recipe pick with no `item` beside one with an `item`", so those two picks cost twice.

Every pick in localStorage today has no `item` (`app/src/lib/api/types/Pick.ts:3`, `odyssey.ts:63`). On the new page a Maverick with a saved "Maverick 1 to 3" pick shows no plan on its row (the pick is tied to nothing). ADD TO PLAN is the control on that row. Using it leaves the old pick in the shopping list and adds a second, and the commander pays for two upgrades of one suit. The Dropdown under the list is a different control; it is not what the loadout row offers.

ADD TO PLAN on an item that already has a matching untied pick must tie that pick (set `item` to the journal id) and must not append a second. The same when PLAN OTHER GEAR would add a recipe that is already saved without `item`. Cover it in nnn0's pick-to-item table test: a generic Maverick 1-to-3 in localStorage, ADD TO PLAN on one grade-1 Maverick, one pick in the list, that row showing the plan, the other Maverick unchanged.
