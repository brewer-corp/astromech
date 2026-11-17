---
schema: atelier.review
schema_version: 3
round: 4
commit: c6dbb689d79e6b5723b6e148a7f56b0b8ee2c79f
reviewers:
- 01a1e907-479b-7453-b4c0-7b77cc3a3d66
model: grok-4.6
verdict: changes
created_at: 2026-11-17T10:22:40.046994400Z
---

## Summary

- **Complexity:** high. Increments 3 and 4 rebuild every remaining page, add a Now home, move three live views, and extend the core for loadouts, body parents, rank start, session payments and suit-engineer coordinates.
- **Value:** high. The pages are in daily use, increment 1 is on main, and the screens follow Sumeet's recorded stop and the approved mockups.
- **Risk of merging it:** medium. Visual misses show in the preview and revert per page. Core additions have named tests. The plan as this commit holds it still cannot close increment 2, which every screen waits on.

The value is worth the rest. The screen items are specified from the current replies and are buildable. Three holes in the checkpoint gate have to close before those screens start.

I read `mission.md`, `design.md`, `notes.md`, and every listed item (draft screens, w1lj, 70ts, zaqh, 8e22, gvy0, crxa, wo8s, the dropped checkpoint bugs, and the approved increment 2 items). I checked the claims against `DESIGN.md` (colour list, Frame, Orange Means Act), `app/src/lib/modules.ts`, `App.svelte` `DEFAULT`/`MOVED`/`ON_FOOT`, `App.browser.test.ts` lines 340, 352 and 419, `app/eslint.config.js` `PAGE_SIZING` lists, `Hexes.svelte`, `station.ts` `SERVICE_NAMES`, `owned.rs:179`, `ScanBody.parents`, `NextRank` / `merits_for`, `session.rs` earnings, `colonisation` `in_cargo`, `Market.browser.test.ts:55`, `fakecore.ts:530`, `gen-starmap.py` `SHIP_ENGINEER_IDS`, and the odyssey journal lines w1lj names. The glow-up and nav-spike captures named in `design.md` are on this commit. `scripts/browser/audit.mjs` and `docs/design/increment2/` are not; they exist on astromech-8e22's unmerged branch, with the `--plan` / `--only` flags and the `sideways`, `clipped`, `overlaps`, `small`, `far`, `contrast` and `bright` keys the screens name. I did not run the app, the journeys, or Windows WebView2.

## Finding 1: Let 8e22 close with the checkpoint failures already handed to later items
Recommendation: fix now

`.atelier/items/astromech-8e22/item.md:37` still requires every filed failure to be "fixed before this item closes". The same commit dropped astromech-lf3a, astromech-8og8 and astromech-5g9w onto astromech-rj6i, astromech-nnn0 and astromech-8wr4, each of which lists `astromech-8e22` in `blocked_by`. astromech-wo8s is a fourth failure 8e22 found (My controller at 8 px, a page 8e22 itself captures) and is `ready` with an empty `blocked_by`, so it does not block 8e22 either.

Closing 8e22 with line 37 still in force leaves those Done when boxes false. Keeping 8e22 open until the screens fix the three handed-over pages deadlocks them. lf3a's Scope (line 30) still says "astromech-8e22, which waits on this fix"; the other two dropped items say the same.

Reword 8e22's second Done when line so a failure may be fixed and recaptured, handed to a named screen that rebuilds that page (lf3a → rj6i, 8og8 → nnn0, 5g9w → 8wr4), or filed on its own when it is older than increment 2 (wo8s). Drop the leftover "8e22 waits on this fix" sentences from the three dropped items.

## Finding 2: Make landing the audit tool a Done when of 8e22
Recommendation: fix now

Every screen item, and `design.md:204`, measures with `node scripts/browser/audit.mjs` "astromech-8e22's tool", from a `plan.json` in the form of `docs/design/increment2/plan.json`, and requires empty `sideways`, `clipped`, `overlaps`, `small`, `far` and `contrast` lists plus a `bright` list. On this commit that file does not exist, and 8e22's Done when never names creating or merging it. astromech-vbz3's Report (`.atelier/items/astromech-vbz3/item.md:39`) already had to copy `audit.mjs` and `docs/design/increment2/plan.json` from 8e22's unmerged branch.

8e22's Scope says it changes no page and files what it finds. A measuring script is evidence, and the screens cannot tick their checks until it is on the branch they wait on. Add to 8e22's Done when that `scripts/browser/audit.mjs` and `docs/design/increment2/plan.json` land with the flags and report keys above (`--base`, `--plan`, `--only`, and `bright`).

## Finding 3: Decide whether wo8s is in the mission
Recommendation: fix now

astromech-wo8s is `ready`, `mission: astromech-xajc`, and changes My controller's sheet so bound actions draw at 12 px or more at 1100 × 700. `mission.md:100` puts Controls pages out of scope: they already read best, and only the frame, scale and Tip reach them. Success criteria (line 65) add text size, colours and Tip. wo8s's own Scope (`.atelier/items/astromech-wo8s/item.md:31`) leaves the decision open: "this may belong outside the mission; the lead decides."

The lead has already readied it inside the mission. The Out table still forbids that work. Either amend the Controls row to name this one sheet-size exception, or take wo8s out of the mission (`mission` empty) and leave it as a standalone bug.
