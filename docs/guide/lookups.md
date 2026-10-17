# Voice lookups

A lookup is a question that searches the galaxy: the nearest station with a service, the best
place to sell, the closest raw material site. You ask by voice, astromech says "Looking that
up." where the search takes a moment, and then speaks the best answer. The same searches are on
screen as pages with more rows, and the two share their filters, so a page set up the way you like it is what
voice uses. Questions about your own ship, such as fuel and cargo, are in
[Voice commands](voice-commands.md#questions-about-your-ship).

## What lookups need

- **A network connection.** Searches use Spansh, Ardent Insight and EDSM (see
  [Using the app](using-the-app.md#what-needs-the-network)).
- **Your position.** It comes from the journal, from your last jump or login. Before that you hear
  "Your position is not known yet. It comes from the next jump or login."
- **The star map.** It is bundled for 500 light years around Sol and fills in from EDSM as you
  travel. If it cannot be opened: "Lookups are not available: the star map could not be opened."

Jump counts use your ship's range from the last loadout (20 light years if unknown). Inside the
known star map a count is worked out along real stars. Outside it, or if the search gives up, it
is a straight-line estimate and the page marks it with "~". Neutron boosts and fuel are not
counted. See [Galaxy](galaxy.md#jumps-and-distance).

## Stations: services, modules, ships

Ask: "nearest interstellar factors", "find material trader", "where can I buy a 5A FSD",
"nearest pulse laser", "where can I buy a Krait Mk II". The answer is the nearest station, its
system, how far, how far from the star, the price for a module or ship, and the age of the
report when it is more than a week old:

> Nearest Interstellar Factors: Jameson Memorial in Shinrarta Dezhra, 12 light years, 8 light
> seconds from the star.

Services you can ask for: Interstellar Factors, material trader (or raw, manufactured, encoded
data), technology broker (or human, Guardian), Universal Cartographics, Vista Genomics, Pioneer
Supplies, bartender, Frontline Solutions, Apex Interstellar, refuel, repair, rearm, black market,
search and rescue. Short forms work: "factors", "mat trader", "tech broker", "cartographics",
"bar", "restock".

Modules: say the module, with a class and rating if you want one ("5A", "class 5", "A-rated")
and a mount for weapons. Common short names work (FSD, scoop, AFMU, DSS, SCB, HRP). Ships:
the ship's name or a short form ("krait", "dropship", "gunship"). A shipyard is skipped if its pads are too small
for the ship.

The same searches are on the [Galaxy](galaxy.md) pages: Services, Outfitting and Shipyard. Filters
there (pad size, fleet carriers, Odyssey settlements, data age, arrival distance) apply to voice
too. The pad follows your current ship until you pick one.

Source: Spansh. Limits: Spansh data comes from players and can be days old; fleet carriers
and Odyssey settlements are included unless filtered.

## Trade

Ask: "where to sell gold", "best price for painite", "where can I buy tritium", "cheapest
tritium", "where should I sell my cargo".

> Best price for Painite: Haberlandt Port in LHS 1827, 19 light years. 211,587 credits, demand 4,220.

> Best for your cargo: Jameson Memorial in Shinrarta Dezhra, 12 light years. 1,203,400 credits for all 64 tonnes.

For your cargo, the answer is the one station that pays the most for the whole load. When no
market takes everything, it adds the best station for the commodity that is left over. Mission
cargo, stolen goods and limpets are left out ("Nothing in your hold to sell: mission cargo,
stolen goods and limpets are left out."). Data older than a day says how old it is.

Source: Ardent Insight. It searches within 100 light years and 30 days old by default, with the
filters of the [Trade](trade.md) page's Sell, Buy and My cargo modes. Say a commodity by its
game name; many have short names ("LTDs").

## Raw material sites

Ask: "nearest crystalline shards", "nearest brain trees", "nearest crashed ship", "nearest
geological sites", "nearest raw material farm".

> Nearest crystalline shards: HIP 36601 C 1 a, 22.4 light years, 2 jumps, 1,200 light seconds
> from the arrival star.

The answer is the nearest body with that kind of site, how far, how many jumps, and the distance
from the arrival star (a long supercruise is worth knowing about before you set off). The
page [Materials](materials.md#raw-material-sites) shows the list. Source: Spansh. Voice does
not use the page's filters for sites.

## Faction states: famine, boom, war and more

Ask: "nearest famine system", "nearest boom system", "nearest war system" (or "civil war"),
"nearest civil unrest system", "nearest outbreak system", "nearest federation system",
"nearest empire system".

These find systems where a faction is in a state, for farming materials. Boom, war, civil unrest and
outbreak give High Grade Emissions pools, and famine helps wake scans. The federation
and empire ones find the best system with an allegiance pool. The High Grade Emissions
searches are ranked by population and say "Best"; famine says "Nearest".

> Nearest famine system: Aleapa. 12 light years, 2 jumps. Reported 3 days ago.

Source: Spansh, with your own visits to a system taking priority when newer ("Seen on your
visit"). The page is under Materials (the side panel of a farming source). Voice uses the filters you set there.

## Powerplay targets

Ask: "where can I acquire near me", "where can I reinforce near me", "where can I undermine
near me". It uses the power you are pledged to, from the journal.

> Nearest system to undermine: Pranav Antal. 12 light years, 2 jumps. Zachary Hudson exploited, 8 percent.

If you are not pledged: "Not pledged to a power, so there are no targets." It searches within
40 light years. The same search is on [Powerplay](powerplay.md#where-to-work). Source: Spansh.

## Colonisation

Ask: "how is the build going", "what is left to deliver", "where can I buy steel for the
depot".

> Test Hub is 50 percent built. 300 tonnes left to deliver.

If there are several projects, it picks the one you are docked at, else the last you delivered
to, else the last the game updated. Source: your journals, and Ardent Insight for where to
buy. See [Colonisation](colonisation.md). With no project: "No construction project in progress."

## When a lookup fails

- "Didn't catch what to look for." It heard "nearest" but not the thing.
- "Which commodity? Say, for example, where can I sell gold."
- "No <thing> found near <system> with your search filters." Loosen the filters on the page.
- "Couldn't look that up." followed by the reason, such as no network.
- "Spansh doesn't know <system> yet, so I can't search from it." for a system too new for Spansh.

## Limits

- One answer per question: the best one. Use the pages to see more.
- Data comes from other players and tools, so it is as old as their last visit.
- Lookups search from where the journal says you are, not from a system you name.
  The pages let you type another system to search from.

## All the phrases

Slots in braces are filled by what you say: `{service}`, `{module}`, `{ship}`, `{commodity}`.


### Stations

| Question | Phrases |
|---|---|
| A station service | "nearest {service}", "where's the nearest {service}", "where is the nearest {service}", "find the nearest {service}", "find {service}" |
| A module | "nearest {module}", "where can i buy {module}", "where can i get {module}", "where's the nearest {module}", "where is the nearest {module}", "find {module}", "buy {module}" |
| A ship | "where can i buy {ship}", "where can i get {ship}", "nearest {ship}", "where's the nearest {ship}", "find {ship}", "buy {ship}", "who sells {ship}" |

### Trade

| Question | Phrases |
|---|---|
| Where to sell | "where to sell {commodity}", "where can i sell {commodity}", "best place to sell {commodity}", "best price for {commodity}", "sell {commodity}" |
| Where to buy | "where to buy {commodity}", "where can i buy {commodity}", "cheapest {commodity}", "buy {commodity}" |
| Sell your cargo | "where to sell my cargo", "where should i sell my cargo", "best place to sell my cargo", "sell my cargo" |

### Raw material sites

| Question | Phrases |
|---|---|
| Crystalline shards | "nearest crystalline shards", "find crystalline shards", "where are the nearest crystalline shards", "nearest crystalline structures", "nearest shard site" |
| Brain trees | "nearest brain trees", "find brain trees", "where are the nearest brain trees" |
| Crashed ships | "nearest crashed ship", "find a crashed ship", "nearest wreck site" |
| Geological sites | "nearest geological sites", "find geological sites", "nearest geology" |
| Raw material farms | "nearest raw material farm", "nearest raw farm", "where can i farm raw materials" |

### Faction states

| Question | Phrases |
|---|---|
| Famine | "nearest famine system", "nearest famine", "where is there a famine near me", "find a famine system", "famine near me" |
| Boom | "nearest boom system", "nearest boom", "where is there a boom near me", "find a boom system" |
| War | "nearest war system", "nearest war", "where is there a war near me", "nearest civil war", "find a war system" |
| Civil unrest | "nearest civil unrest system", "nearest civil unrest", "where is there civil unrest near me" |
| Outbreak | "nearest outbreak system", "nearest outbreak", "where is there an outbreak near me" |
| Federation | "nearest federation system", "nearest federal system", "nearest federation faction", "nearest federation", "nearest federal" |
| Empire | "nearest empire system", "nearest imperial system", "nearest empire faction", "nearest empire", "nearest imperial" |

### Powerplay targets

| Question | Phrases |
|---|---|
| Acquire | "where can i acquire near me", "nearest system to acquire", "nearest acquisition target", "nearest acquisition system", "where can i acquire" |
| Reinforce | "where can i reinforce near me", "nearest system to reinforce", "nearest reinforcement target", "nearest system to fortify", "where can i reinforce" |
| Undermine | "where can i undermine near me", "nearest system to undermine", "nearest undermining target", "nearest undermining system", "where can i undermine" |

### Colonisation

| Question | Phrases |
|---|---|
| Progress | "how is the build going", "how's the build going", "how is the construction going", "build progress", "construction progress", "colonisation progress", "how far along is the build" |
| What is left | "what is left to deliver", "what's left to deliver", "how much is left to deliver", "what does the build need", "what is left for the build", "construction shopping list" |
| Where to buy for the build | "where can i buy {commodity} for the depot", "where can i buy {commodity} for the build", "where can i buy {commodity} for the construction", "where can i buy {commodity} for the project", "where to buy {commodity} for the depot", "where to buy {commodity} for the build", "buy {commodity} for the depot" |
