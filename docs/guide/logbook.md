# Logbook

Four pages under **Logbook** use your journal history: **Logbook**, **Session**, **Mining**
([Mining](mining.md) has its own page) and **Station**.

**The history.** astromech reads every journal file Elite has written into a database on your PC
(`history.sqlite` in the data folder), the first time it runs, and then keeps it up to date while it runs.
The first import can take a while on a long history: the Logbook page shows "Importing your journals:
42%" as it goes, and fills in as it works. It never changes your journals. The history database stays on your PC; sharing with EDDN or EDSM is separate and off until you turn it on.
Deleting `history.sqlite` starts over. If you point astromech at a second journal folder, its
history is added to the first.

**What they need:** the journal. No network, except the Station page's market comparison (Ardent
Insight).

## Logbook

Every system and station you have visited.

- A line at the top: your commander (and "Legacy" if the journal is from the Legacy game) and
  the totals: systems, stations, arrivals and dockings.
- **Search** by system or station name. It searches as you type.
- **Places**: each visited place with the number of visits, the first and the last time. Stations
  show with their system. It lists the most recent first, and says "Showing the N most recent of M" when there
  are more; search to narrow it down.
- **Timeline**: arrivals ("Arrived in", or "Carrier jumped to", with the distance) and dockings, newest
  first.

The arrival briefing uses this history for "Visited 2 times. Last here 3 days ago." and for your landings
(see [Callouts](callouts.md#arrival-briefing)).

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
per dock and **Check again** asks anew. The comparison uses the filters of Commodities, Sell
(100 light years and 30 days unless you change them).

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

Type a module ("5a frame shift", "fsd", "pulse laser", "hyperdrive_size5") and see which stations
you have seen selling it, with price, when you saw it and the distance. It works offline: it searches only the outfitting
screens you opened while astromech was running.

## Limits

- The history starts at your oldest journal file. Journals you deleted are gone.
- A very large history takes a while to import the first time.
- The Station page shows only the station you are docked at.
