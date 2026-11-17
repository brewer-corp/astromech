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
- Below it, every engineer who works on that module. First what they reach: white hexagons for
  the grade you can get from them now, and grey outlines without a centre for grades that need more
  rank or an unlock,
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

- **Ship engineers** and **Suit engineers** are two trees side by side; in a narrow window, or
  with an engineer selected when the two no longer fit beside the details, the suit engineers
  go below the ship engineers. The line beside the search box counts how many of each you have
  unlocked, and how many you can work on now.
- Each column is a step further down the chain: the first holds the engineers everyone knows,
  and a line joins each engineer to the one who introduces them. A ship engineer introduces the
  next ones once your rank with them passes grade 3; a suit engineer refers the next after a
  task of their own. Colonia's engineers sit where their introducers put them.
- Each engineer shows their name and one line, by where you stand with them:
  - **Unlocked**: a light grey name, your rank as white hexagons (five of five at rank 5; a
    suit engineer, who has no ranks, shows one) and their system.
  - **You can work on them now** (known, invited, or every introducer unlocked): an orange name
    and the step to take.
  - **Barred**: the name and reason in red.
  - **Not introduced**: dim, saying "Not introduced".
  Point at an engineer for what their state means.
- **Select an engineer** to see their details beside the trees (below them in a narrow window).
  The engineers who introduce them, and the lines to them, light up. The details show where
  they are, with a copy button for the galaxy map, and how far from your current system; how
  far is left out until the journal says where you are. Then where you stand, every step left to
  unlock them in order (the first, what to do now, is the brightest), and **Works on**: each
  module type a ship engineer offers with the best grade as hexagons (white for the grade your
  rank gets you today, grey outlines for grades that need more rank), or each mod a suit engineer fits, where a "(?)" means the wiki
  disagrees with itself on whether they fit it. Last, who they introduce, a suit engineer's
  referral task, and **Set destination** and **Add stop**. Select the engineer again to close
  the details.
- With no engineer progress in the journal yet, the engineers everyone knows show as known and
  the rest as not introduced.

The same unlocks decide which jobs the **engineer trip** in [Ships](ships.md) can use. Source:
the Elite Dangerous wiki, in `data/catalog/engineer-unlocks.json`. The wiki can be wrong or out of date.

## Limits

- Which mods a suit engineer fits is from the Elite Dangerous wiki and EDCD/FDevIDs; the date is
  at the bottom of [Ships, Suits](ships.md#suits).
