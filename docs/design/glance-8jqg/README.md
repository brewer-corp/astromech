# Now's main value: glance captures (astromech-8jqg)

Captures of the Now page before and after astromech-8jqg, in three states (docked, in space,
Elite not running) at 2560 x 1440 at 145 percent and 1100 x 700 at 100 percent. They were
taken the way `docs/design/final/glance-plan.json` (astromech-crxa) takes them, with
`node scripts/browser/audit.mjs --base <vite> --plan docs/design/glance-8jqg/plan.json`
(`before-plan.json` writes to `before/`).

The answers and marks belong in `docs/design/final/glance-test.md`, row 2. That file is on
astromech-crxa's branch, which is not merged, so they are recorded here until it is.

## Readers

Each reader is a fresh general-purpose agent (Claude Opus 5.5) given only `after/now-space-2560x1440.png`,
copied as `01.png`, and the prompt from `glance-test.md` round 2, cut to one screenshot and
without the route panel question. Row 2 expects "the mining rate, 18 t per hour".

### Step-down only: every block but the main one in Value L, Options Grey

| Reader | Main value answer, verbatim | Mark |
|---|---|---|
| A | "Test System" (the system name in the title). No single figure leads: the eye goes first to the orange "18 t per hour" under MINING, although "183,674,700 cr" under SELL YOUR CARGO is the bigger figure | fail |
| B | No single value stands out. The orange "18 t per hour" under MINING draws the eye first, but the page seems to be about "183,674,700 cr" under SELL YOUR CARGO, since it is first and leads the row. | fail |

Both put the cargo total, the first block in the row, against the orange rate in the fourth.

### With the blocks reordered: Mining, Sell your cargo, This session, Powerplay

The blocks now stand in the order the main value is picked, so the block that holds it leads
the row. Two new readers, the same prompt, the new capture:

| Reader | Answer, verbatim (main value \| can change \| needs you) | Main value mark |
|---|---|---|
| C | 18 t per hour (the orange Mining figure) \| none: Mining, Sell my cargo, Session and Powerplay are only links, and the icons next to "Test System" and "LHS 1827" look like copy buttons \| none | pass |
| D | 18 t per hour (the orange mining rate under MINING; other figures compete with it: 183,674,700 cr, 14.6 M cr per hour, 1,829 merits) \| the two icon buttons after "Good Landing · LHS 1827 · 1 jump" (add as a waypoint, set as destination) and the copy icons beside "Test System" and "LHS 1827" \| none | pass |

Reader C's can change answer, "none", misses the Add stop and Set destination icons beside Good
Landing, which row 2 expects. Those icons are unchanged by this item, and that question passed
in both of astromech-crxa's rounds.
