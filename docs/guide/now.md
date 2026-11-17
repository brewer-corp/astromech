# Now

The window opens on **Now**: what you are doing at the moment. It follows the game, so it
changes as you dock, undock and jump. **Now** also holds **Station**, **Session** and
[Mining](mining.md), the pages each of its answers comes from.

**What it needs:** the journal and Elite running. Sell your cargo asks Ardent Insight over the
network; everything else is read from your journal history on your PC.

## What it shows

The top of the page says where you are:

- **Docked:** the station's name with its type icon, then the system (with its copy icon) and the
  station's distance from the star. Under the answers, the station's **services**: every service
  in the game's station menu, in its order. A service this station has is bright; one it lacks is
  shown dim, so you can see at a glance what is missing. The tiles are facts, not buttons.
- **In space:** the system's name, with its copy icon. There are no services.
- **Elite isn't running:** the last place in your journal ("Last docked at Ehrlich City,
  Liaedin" or "Last in Liaedin"), the last session and Powerplay. What is in the hold and a
  mining run aren't known until Elite runs.
- **No game in the journal yet:** one line saying so, and nothing else.

If you have exploration data aboard that dying would lose, an orange **!** line under the place
says how much, with a link to [System](exploration.md).

Under that, a few answers, each shown only when it has something to say, with a link to the page
that has the rest. One figure is bright orange: the mining rate while you mine, else what the
hold would fetch, else the session's credits per hour.

| Answer | Shown | Its figure | Where it comes from |
|---|---|---|---|
| **Sell your cargo** | while the hold has cargo you can sell | what the best station pays for the hold, and how much of it that station takes ("takes all 600 t", or "takes 20 of 200 t"), then the station, its system, the jumps and the pad | the [Sell my cargo](places.md#sell-my-cargo) finder's first row, with that finder's filters |
| **This session** (Last session with Elite closed) | once there is a session | credits per hour, what was earned over how long, and the jumps | [Session](#session); hover the earnings for what they count |
| **Powerplay** | while you are pledged | merits this cycle, the time left in the cycle, your rank and the merits to the next | [Powerplay](powerplay.md) |
| **Mining** | while a mining run is going | tons per hour, and the tons refined | [Mining](mining.md) |

**Sell your cargo** has **Add stop** and **Set destination** beside the station, as the finder's
rows do. It asks Ardent again only when something it depends on changes: what you carry, your ship
(its pad and jump range) or the system you are in. Mission cargo, stolen goods and limpets don't
count. While it asks, the last answer stays, dimmed, with "Asking Ardent…"; with no station in
the finder's range that buys your hold it says so. If Ardent can't be reached, the block says so
with **Try again**; the other answers stay.

The next stop of a route isn't here: the [route sidebar](routes.md#the-route-sidebar) shows it on
every page.

## Session

What the latest game session earned and covered, like EDMC's hourly income.

- A line: your commander and the session's start, end and length (UTC).
- **Earned**, **Per hour**, **Jumps** and **Distance**.
- **Credits by activity**: Trade (with the profit), Exploration data, Organic data, Bounty vouchers,
  Combat bonds, Other vouchers, Missions, Search and rescue, Community goals, Odyssey goods and
  Powerplay salary, each with the number of payments.
- **Also**: data sold (already in the total), bounties awarded and their value (paid when you hand
  in the vouchers), and carrier jumps (not in jumps or distance).

Earnings are credits paid for what you did. **Selling ships, modules or drones is not counted.**

The same totals are the [session summary](callouts.md#session-summary) astromech says when you log
out, and by voice any time ("session summary").

## Station

The station you are docked at, and a search of every outfitting you have opened.

### Docked here

When you are docked: the station and system with Spansh and Inara buttons, its type, its
**faction** and the faction's state, government and allegiance, **economy** shares, **landing
pads** (large, medium, small), distance from the star and **services**. Not docked: "Dock
somewhere and the station's facts show here."

### Market

The market you are docked at, compared with the best prices elsewhere. It uses Ardent Insight, asks once
per dock and **Check again** asks anew. The comparison uses the filters of the
[Sell](places.md#sell-and-buy) finder (100 light years and 30 days unless you change them).

- **Your cargo**: for each commodity in the hold, what it sells for here, the best price nearby
  (in another system), the difference per ton and where.
- **For sale here**: for each commodity sold here, its price, the best price nearby and the profit
  per ton.
- At most a dozen commodities are looked up per dock: the biggest loads first, and those furthest
  below their average price. The rest are listed as not looked up. Mission cargo, stolen goods and
  limpets are left out.

The market is as of the last time you opened the commodity market ("Open it again to refresh").

### Outfitting

If you have opened this station's outfitting, the modules for sale that are **better than what you
have fitted**, slot by slot, with price. The comparison uses the ship you fly. If you have not
opened it: "Open the outfitting screen once and it is kept."

### Where did I see ...

Type a module by name, size and rating, or symbol ("5a frame shift", "fsd", "pulse laser",
"hyperdrive_size5") and see which stations
you have seen selling it, with price, when you saw it and the distance. It works offline: it searches only the outfitting
screens you opened while astromech was running.

## Limits

- The Station page shows only the station you are docked at.
- Sell your cargo needs the network (Ardent Insight); the rest of the page works offline.
