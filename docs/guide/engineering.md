# Engineers

**Engineering, Engineers** is the unlock guide for every ship and suit engineer. The suit
engineers were on the On foot page until it was split up: they are here beside the ship engineers,
with every mod each one fits. On foot's inventory is now a kind on [Materials](materials.md#on-foot),
and its blueprints are [Ships, Suits](ships.md#suits).

## Engineers

A tech tree of every ship engineer and suit engineer, with your progress and what to do next.

**What it needs:** the journal (your engineer progress). No network.

### Search by module

Type a module in the box at the top, in your own words: "laser", "fsd", "shield", "beam",
"thrusters", "multi-cannon". The page answers where to engineer it:

- The block at the top is the best grade you can get today: of the engineers you have unlocked,
  the one whose rank gives the highest grade, nearest first, with **Set destination** and Add stop.
  If none who work on it is unlocked yet, it names the one nearest to unlocking and the step left.
- Below it, every engineer who works on that module. First what they reach: orange hexagons for
  the grade you can get from them now, dimmer orange for grades that need more rank or an unlock,
  and the grade per module type ("Pulse 4 · burst 3 · beam 3"). Then the engineer with your rank
  as white hexagons, the rank a higher grade needs, or the step to unlock them. Then where they
  are, with a copy button, and how far from your current system.
- Suit engineers have no grades, so search them by the modification you want: "range",
  "magazine", "night vision". Those you have unlocked come first, then those closest to
  unlocking; at equal standing the nearest comes first, with how far it is. A "(?)" after a mod
  means the wiki disagrees with itself on whether that engineer fits it. A suit or weapon name
  ("pistol", "Maverick") finds nothing, and the page says so.
- A word the page doesn't know says so and suggests close ones.
- Clear the box (or press Escape) to get the tree back. The search stays in the page's link.

Which engineer offers which grade is from EDCD's coriolis-data, in `data/catalog/engineering.json`;
the words are in `data/catalog/engineer-search.json`. Where every engineer is, ship and suit, is
in `data/catalog/engineers.json`, from EDCD/FDevIDs and Spansh's systems dump.

### The tree

- Choose **Ship engineers** or **Suit engineers** at the top. The line beside them counts how many you
  have unlocked, and how the marks work: white hexagons for rank, a dim mark for no access yet, dimmer cards
  for engineers who have not been introduced. A ship engineer introduces the next ones once
  your rank with them passes grade 3; each suit engineer refers the next after a task.
- Each card shows the engineer, what they specialise in (for a suit engineer, every suit and
  weapon mod they fit, from the Odyssey data; a "(?)" means the wiki disagrees with itself on
  whether they fit that mod), and your **access** as engineer marks:
  one white hexagon per rank once you have unlocked them (so rank 5 is five lit of five), a dim mark
  when they are **Known** but you have no access yet, the same dim mark on a dimmer card when
  they have **Not been introduced**, an envelope when you are **Invited**, and a struck mark if you are
  **Barred**. Unlocked engineers have bright names; known and invited ones, whom you can work
  toward, have orange names with "Known · no access yet"; a barred engineer's name and reason are
  red; cards that still need an introduction are dim, unless you can work on them now. What is left to do stays on the card. Engineers you can work on now sit at the top of
  their column.
  Hover a card to light who introduces them. A permit-locked system says "permit", and a suit
  engineer in Colonia says "Colonia".
  An arrow points back to who introduces each engineer.
- The system name has a copy button for the galaxy map.

The same unlocks decide which jobs the **engineer trip** in [Ships](ships.md) can use. Source:
the Elite Dangerous wiki, in `data/catalog/engineer-unlocks.json`. The wiki can be wrong or out of date.

## Limits

- Which mods a suit engineer fits is from the Elite Dangerous wiki and EDCD/FDevIDs; the date is
  at the bottom of [Ships, Suits](ships.md#suits).
