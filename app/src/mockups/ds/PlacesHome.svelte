<script lang="ts">
  // DESIGN.md mockup: the Places home, a visible grid of finders under section headings.
  // Each tile is a read-only link (hull surface, never a cell); a live answer where one is known.
  import Chrome from "./Chrome.svelte";

  type Tile = { name: string; what: string; answer?: [string, string, string] };
  const groups: [string, Tile[]][] = [
    ["Trade", [
      { name: "Sell", what: "Where a commodity fetches the most", answer: ["Your 64 t Painite", "313,239", "cr"] },
      { name: "Buy", what: "Where a commodity is cheapest" },
      { name: "Sell my cargo", what: "The best stations for the whole hold", answer: ["Whole hold", "20.0 M", "cr"] },
    ]],
    ["Ship", [
      { name: "Services", what: "Interstellar Factors, traders, brokers, repair", answer: ["Interstellar Factors", "2", "jumps"] },
      { name: "Outfitting", what: "Stations selling a module" },
      { name: "Shipyard", what: "Stations selling a ship" },
    ]],
    ["Engineering", [
      { name: "Material traders", what: "Raw, manufactured and encoded", answer: ["Raw trader", "4", "jumps"] },
      { name: "Raw sites", what: "Crystalline shards, brain trees, geological sites" },
      { name: "Faction states", what: "Boom, war, outbreak, for High Grade Emissions" },
    ]],
    ["Mining", [
      { name: "Mining hotspots", what: "Rings with hotspots for a mineral", answer: ["Painite near next stop", "3", "hotspots"] },
    ]],
    ["Route", [
      { name: "Find along route", what: "A service or sale with the smallest detour", answer: ["Nearest refuel on route", "+0", "jumps"] },
    ]],
  ];
</script>

<Chrome top="Galaxy" tabs={["Places", "Route", "System"]} tab="Places"
  strip={{ next: "Wregoe OI-C b54-6", of: "3 of 41", flags: ["Scoopable"], dest: "Colonia", left: "38 jumps left" }}>
  <div class="home">
    {#each groups as [g, tiles] (g)}
      <section>
        <h2 class="section">{g}</h2>
        <div class="tiles">
          {#each tiles as t (t.name)}
            <a class="tile" class:hover={t.name === "Sell"} href="#/mockup/{t.name === 'Mining hotspots' ? 'ds-mining' : 'ds-sell'}">
              <span class="name lit">{t.name}</span>
              <span class="label">{t.what}</span>
              {#if t.answer}
                <span class="answer"><span class="label">{t.answer[0]}</span><span><span class="num v">{t.answer[1]}</span><span class="unit">{t.answer[2]}</span></span></span>
              {/if}
            </a>
          {/each}
        </div>
      </section>
    {/each}
  </div>
</Chrome>

<style>
  .home { max-width: var(--list-w); display: grid; grid-template-columns: 1fr; gap: var(--sp-6); }
  .section { margin-bottom: var(--sp-3); max-width: calc(3 * 220px + 2 * var(--sp-2)); }
  .tiles { display: grid; grid-template-columns: repeat(auto-fill, 220px); gap: var(--sp-2); max-width: calc(3 * 220px + 2 * var(--sp-2)); }
  .tile { display: grid; grid-template-rows: auto auto 1fr; gap: var(--sp-1); min-height: 104px; padding: var(--sp-3); background: var(--surface); text-decoration: none; color: inherit; }
  .tile.hover { background: var(--hover); outline: 1px solid var(--line); outline-offset: -1px; }
  .name { font-weight: 500; }
  .answer { align-self: end; display: flex; justify-content: space-between; align-items: baseline; gap: var(--sp-2); }
  .v { font-size: var(--fs-l); font-weight: 600; color: var(--orange-lit); }
</style>
