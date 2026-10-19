# Engineers and on foot

Two pages under **Engineering** that go beyond ships: **Engineers**, the unlock guide, and
**On foot**, your Odyssey inventory and blueprints.

## Engineers

A tech tree of every ship engineer and suit engineer, with your progress and what to do next.

**What it needs:** the journal (your engineer progress). No network.

- Choose **Ship engineers** or **Suit engineers**. The line under the tabs counts how many you
  have unlocked, and how the marks work: orange for rank, grey for no access yet, dimmer cards
  for engineers who have not been introduced. A ship engineer introduces the next ones once
  your rank with them passes grade 3; each suit engineer refers the next after a task.
- Each card shows the engineer, what they specialise in, and your **access** as engineer marks:
  one orange mark per rank once you have unlocked them (so rank 5 is five marks), a grey mark
  when they are **Known** but you have no access yet, a dimmer grey mark when they have **Not
  been introduced**, an envelope when you are **Invited**, and a struck mark if you are
  **Barred**. Unlocked engineers are full colour; known ones stay readable with "Known · no
  access yet"; cards that still need an introduction are dimmer. What is left to do stays on
  the card. Engineers you can work on now sit at the top of their column, with an orange edge.
  Hover a card to light who introduces them. A permit-locked system says "permit".
  An arrow points back to who introduces each engineer.
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
what it is worth at the **barter** (assets only) and what a bartender pays you for it (**Sells for**). **Held only** (orange) hides what you do not have; **All items** (grey) shows the rest of the
catalogue. The line above says how much is held and what a bartender would pay
for it. Mission items go back when the mission ends and do not count for blueprints.

### Blueprints

Plan the micro-resources you need.

1. Choose **Grade upgrade** (a suit or weapon from one grade to another) or **Modification** (a
   suit or weapon mod, with the weapon maker for weapon mods).
2. **Add** it. Add as many as you like.
3. Each one shows what it still needs and how much you are short (have / need), its price in credits, and
   which engineers fit it. Upgrades are bought at Pioneer Supplies. Gear and mods you already have
   are marked and not offered again.

Below the picks, one **shopping list** adds them all up, with the credits. A **Bartender trade**
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
