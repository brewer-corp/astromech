# Glance test for Ships, after astromech-ng71

Ships with a module selected now passes the glance test. Two fresh readers each named the panel's
planned grade, G5, as the main value. In astromech-crxa's rounds 1 and 2, readers had picked the
head's 32.9 ly jump range. The change: the head's four stats are a row of equal facts in Value L,
Options Grey, so the panel's G5 is the page's one Value XL.

astromech-crxa copies the sheet rows, answers and marks below into
`docs/design/final/glance-test.md` when it resumes. The prompt, the answer sheet and the marking
rules are in that file on the `astromech-crxa` branch:
`git show astromech-crxa:docs/design/final/glance-test.md`.

## Captures

- `before/` is the page as rounds 1 and 2 saw it. `before/ships-module-2560x1440.png` is byte
  for byte the same as crxa's `docs/design/final/ships-module-2560x1440.png`.
- `after/` is the page with the change.

Each folder holds three states: a module selected (`ships-module`), nothing selected (`ships`)
and the Plan view (`ships-plan`). Each state is captured at 2560 x 1440 at 145 percent, the
glance test's size, and at 1100 x 700 at 100 percent. To take them, run:

    node scripts/browser/audit.mjs --base <dev server> --plan docs/design/glance-ng71/plan-after.json

Swap in `plan-before.json` for the before set. Both plans use the clock and query that crxa's
plans use. All twelve captures measure 0 for sideways, clipped, overlaps, small, contrast and
far.

## Sheet rows

Row 7 is crxa's, unchanged. Row 7a is new: the sheet had no row for Ships with nothing selected.
It was committed (a8203ebe) before any reader saw the after capture. Its main value comes from
the lead's decision on ng71. The other two answers come from astromech-8wr4's item for that
state.

| # | Page and state | Capture | Main value | Can change | Needs the commander |
|---|---|---|---|---|---|
| 7 | Ships, a module selected | `after/ships-module-2560x1440.png` | The selected module's planned grade, G5 (Increased range, Mass Manager), with its needs (astromech-8wr4) | The three dropdowns (Increased range, G5, Mass Manager) or ADD TO PLAN (8wr4) | The short materials' "!" marks in the Have column (8wr4) |
| 7a | Ships, nothing selected | `after/ships-2560x1440.png` | The ship's name, Long Way Round (ng71) | Picking a module row (the panel follows it) (8wr4) | none |

## Readers

Each reader was a new general-purpose agent (Claude Opus 5.5) that had not seen the sheet. It
got one capture, copied as `01.png` into an otherwise empty folder, and crxa's round 2 prompt.
The count in the prompt was changed to "1 screenshot, `01.png`", and the route panel question
was left out, as round 2 left it out. Each capture had two readers. The answers below are
verbatim. They are marked under crxa's round 1 marking notes. A main value answer that picks
one value and notes that others compete passes, as crxa's rows 25 and 26 did in round 1. An
answer of "unclear" fails.

| # | Reader | Answer (main value \| can change \| needs you) | M | C | N |
|---|---|---|---|---|---|
| 7 | 1 | "G5" (the planned grade for the selected 5A Frame Shift Drive), though "Long Way Round" and "32.9 ly" also compete for first glance \| the Increased range, G5 and Mass Manager dropdowns, the ADD TO PLAN button (it looks greyed, so it may be disabled), the Loadout/Plan toggle, and picking a module row such as "5A Frame Shift Drive" \| the materials marked "!" where you have fewer than you need: Arsenic !3 of 5, Chemical Distillery !2 of 4, Chemical Manipulators !1 of 5, Datamined Wake Exceptions !0 of 5, Galvanising Alloys !0 of 3 | pass | pass | pass |
| 7 | 2 | G5 (the planned grade for the 5A Frame Shift Drive) \| the Increased range, G5 and Mass Manager dropdowns, ADD TO PLAN, and the Loadout/Plan toggle \| the materials marked "!" where you have fewer than you need: Arsenic !3, Chemical Distillery !2, Chemical Manipulators !1, Datamined Wake Exceptions !0, Galvanising Alloys !0 | pass | pass | pass |
| 7a | 1 | the ship "Long Way Round" (Krait MkII). No single figure stands out; its stats sit as equal peers: 32.9 ly, 420 MJ, – m/s, – MW \| the Loadout / Plan toggle; the four ship cards at the bottom pick which ship is shown (Heavy Lifter, Long Way Round, Unnamed, Synthetic_Krait) \| none (the "–" top speed and power used, with the note beside them, are missing data, not an alert) | pass | **fail** | pass |
| 7a | 2 | the ship "Long Way Round" (Krait MkII) \| the Loadout / Plan toggle, and the ship cards along the bottom (Heavy Lifter, Long Way Round, Unnamed, Synthetic_Krait) that switch which ship is shown \| none (the orange grade pips on Thrusters and Frame Shift Drive are the closest candidate, but they read as "engineering not maxed", not as something urgent) | pass | **fail** | pass |

Row 7 passes on all three questions with both readers. Rounds 1 and 2 had failed it on main value.

Row 7a passes on main value with both readers, which is the answer ng71's decision expects. Both
readers fail it on can change, in the same way. Each named the Loadout / Plan choice and the
fleet strip, and neither named picking a module row. With no panel open, nothing on the loadout
rows suggests they can be picked. This is a separate question from ng71's, which is the head's
stats against the main value. The lead of astromech-xajc has two ways to settle it. One is to
accept the fleet strip in row 7a, since picking a ship changes what the page shows, as picking a
row does. The other is to give the unselected loadout a cue that its rows can be picked.
