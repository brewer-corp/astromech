# Logbook

**Logbook** lists where you have been, from your journal history. The pages that follow what you
are doing now (**Now**, **Station**, **Session** and **Mining**) are under **Now**: see [Now](now.md).
They use the same history.

**The history.** astromech reads every journal file Elite has written into a database on your PC
(`history.sqlite` in the data folder), the first time it runs, and then keeps it up to date while it runs.
The first import can take a while on a long history: the Logbook page shows "Importing your journals:
42%" as it goes, and fills in as it works. It never changes your journals. The history database stays on your PC; sharing with EDDN or EDSM is separate and off until you turn it on.
Deleting `history.sqlite` starts over. If you point astromech at a second journal folder, its
history is added to the first.

**What it needs:** the journal. No network.

## Logbook

Every system and station you have visited, and your travel day by day.

- At the top: your commander (and "Legacy" if the journal is from the Legacy game) and the totals:
  systems, stations, arrivals and dockings.
- **Search** by system or station name. It searches as you type, and narrows both lists.
- **Places**: each visited place, the most recent first, with its number of visits and the first and
  last time. Stations show with their system. It says "Showing the N most recent of M" when there are
  more; search to narrow it down.
- **Days**: your arrivals and dockings, newest first, grouped by day (UTC), each day with its jumps
  and dockings counted. Each row is the time, the system (or the station and its system), and the
  jump's distance, or "carrier jump". The counts are of the rows listed: the logbook lists your 200
  latest arrivals and dockings.

On a wide window the places and the days sit side by side; on a narrow one the places come first.

The arrival briefing uses this history for "Visited 2 times. Last here 3 days ago." and for your landings
(see [Callouts](callouts.md#arrival-briefing)).

## Limits

- The history starts at your oldest journal file. Journals you deleted are gone.
- A very large history takes a while to import the first time.
