# Engineers and on foot

Two pages under **Engineering** that go beyond ships: **Engineers**, the unlock guide, and
**On foot**, your Odyssey inventory and blueprints.

## Engineers

A tech tree of every ship engineer and suit engineer, with your progress and what to do next.

**What it needs:** the journal (your engineer progress). No network.

- Choose **Ship engineers** or **Suit engineers**. The line under the tabs counts how many you
  have unlocked and how many you can work on now, and says how the next ones open: a ship
  engineer introduces the next ones once your rank with them passes grade 3; each suit engineer
  refers the next after a task.
- Columns run from **Common knowledge** (engineers anyone can find), through **Introduced** and **Introduced again**, to
  **Further**. An arrow points back to who introduces each engineer.
- Each engineer shows its status: **Unlocked** (with your rank, for ship engineers), **Invited**
  (the workshop still needs its unlock task), **Known**, **Locked**, or **Barred** (you have been
  barred from the workshop). The ones you can work on now stand out and say "Next:" with what is
  left to do. A permit-locked system says "permit".
- The system name has a copy button for the galaxy map.

The same unlocks decide which jobs the **engineer trip** in [Ships](ships.md) can use. Source:
the Elite Dangerous wiki, in `data/catalog/engineer-unlocks.json`. The wiki can be wrong or out of date.

## On foot

Odyssey micro-resources and blueprints. **What it needs:** the journal and the game's
`ShipLocker` file, which the game writes when you are on foot. Until then the page says "not
written yet (log in on foot once)". No network.

Three tabs:

### Inventory

Your ship locker and backpack by the game's own categories: **Goods**, **Assets**, **Data** and
**Consumables**. Each item shows how many are in the locker, in the backpack and tied to a mission,
what it is worth at the **barter** (assets only) and what a bartender pays you for it (**Sells for**). **Held only**
hides what you do not have. The line above says how much is held and what a bartender would pay
for it. Mission items go back when the mission ends and do not count for blueprints.

### Blueprints

Plan the micro-resources you need.

1. Choose **Grade upgrade** (a suit or weapon from one grade to another) or **Modification** (a
   suit or weapon mod, with the weapon maker for weapon mods).
2. **Add** it. Add as many as you like.
3. Each one shows what it still needs and how much you are short (have / need), its price in credits, and
   which engineers fit it. Upgrades are bought at Pioneer Supplies. Gear and mods you already have
   are marked and not offered again.

Below the cards, one **shopping list** adds them all up, with the credits. A **Bartender trade**
suggests swaps: which of your surplus assets to trade for the ones you are short of. It assumes a
bartender swaps equal barter value and only assets are traded, because the wiki gives barter values
for assets only.

### Engineers

On-foot engineers: where they are, your progress, what they offer, who refers them and the task
for the next referral. A "(?)" means the wiki disagrees with itself on whether this engineer fits
that mod.

## Limits

- Game data (items, values, who fits what) is from the Elite Dangerous wiki and EDCD/FDevIDs, as of the
  date the page shows at the bottom, with how many values are unsure or blank on the wiki.
- Your picks are remembered by the app window on this PC.
