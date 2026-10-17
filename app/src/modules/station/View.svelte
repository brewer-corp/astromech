<script lang="ts">
  // The station you are docked at (services, economies, faction and its state, pads, its market
  // and the hold against nearby prices, and what its outfitting sells that the ship flown would
  // be better for), and a search over every outfitting the core has seen: "where did I see a 5A
  // drive?" works offline.
  import { when } from "../../lib/logbook";
  import { loadHere, searchOutfitting, serviceName, type DockedHere, type OutfittingSearch } from "../../lib/station";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import Market from "./Market.svelte";

  let here = $state<DockedHere | null>(null);
  let failed = $state<string | null>(null);
  let query = $state("");
  let result = $state<OutfittingSearch | null>(null);
  let searchFailed = $state<string | null>(null);
  let seq = 0;

  // The docked view follows the journal: the commander docks and undocks while the window is open.
  $effect(() => {
    const tick = () =>
      loadHere()
        .then((v) => {
          here = v;
          failed = null;
        })
        .catch((e: unknown) => (failed = String(e)));
    void tick();
    const t = setInterval(() => void tick(), 5000);
    return () => clearInterval(t);
  });

  async function search(q: string) {
    const mine = ++seq;
    try {
      const r = await searchOutfitting(q);
      if (mine !== seq) return; // a newer search is in flight
      result = r;
      searchFailed = null;
    } catch (e) {
      if (mine === seq) searchFailed = String(e);
    }
  }

  $effect(() => {
    const q = query;
    const t = setTimeout(() => void search(q), 200);
    return () => clearTimeout(t);
  });

  const cr = (n: number) => `${n.toLocaleString()} cr`;
  const pct = (x: number) => `${Math.round(x * 100)}%`;
  const trimmed = $derived(query.trim());
</script>

<section>
  <h3>Docked here</h3>
  {#if failed}
    <p class="err">{failed}</p>
  {:else if !here}
    <p class="lo">Reading the journal…</p>
  {:else if !here.docked}
    <p class="lo">Not docked. Dock somewhere and the station's facts show here.</p>
  {:else}
    <p class="sub head">
      <span class="station">{here.station}</span>
      {#if here.station}<ExtLinks system={here.system ?? ""} station={here.station} marketId={here.market_id} />{/if}
      {#if here.system}<SystemChip name={here.system} /><ExtLinks system={here.system} />{/if}
      {#if here.station_type}<span class="lo">{here.station_type}</span>{/if}
      {#if here.source}<span class="lo">· {here.source}</span>{/if}
    </p>
    <dl class="facts">
      {#if here.faction}
        <dt>Faction</dt>
        <dd class="faction">
          <span>{here.faction.name}</span>
          {#if here.faction.state}<span class="lo">in</span> <span class="state">{here.faction.state}</span>{/if}
        </dd>
      {/if}
      {#if here.government || here.allegiance}
        <dt>Government</dt>
        <dd>{[here.government, here.allegiance].filter(Boolean).join(" · ")}</dd>
      {/if}
      {#if here.economies.length}
        <dt>Economy</dt>
        <dd>{here.economies.map((e) => `${e.name_localised ?? e.name} ${pct(e.proportion)}`).join(" · ")}</dd>
      {/if}
      {#if here.landing_pads}
        <dt>Landing pads</dt>
        <dd>
          <span class="mono">{here.landing_pads.large} large · {here.landing_pads.medium} medium · {here.landing_pads.small} small</span>
          {#if here.max_pad}<span class="lo">(up to {here.max_pad.toLowerCase()})</span>{/if}
        </dd>
      {/if}
      {#if here.dist_from_star_ls !== null}
        <dt>From the star</dt>
        <dd class="mono">{Math.round(here.dist_from_star_ls).toLocaleString()} ls</dd>
      {/if}
      {#if here.services.length}
        <dt>Services</dt>
        <dd class="services">{#each here.services as s (s)}<span class="svc">{serviceName(s)}</span>{/each}</dd>
      {/if}
    </dl>

    <h3>Market</h3>
    {#if !here.services.includes("commodities")}
      <p class="lo">This station has no market.</p>
    {:else}
      <Market marketId={here.market_id} />
    {/if}

    <h3>Outfitting{here.ship ? ` for ${here.ship}` : ""}</h3>
    {#if !here.services.includes("outfitting") && !here.outfitting}
      <p class="lo">This station has no outfitting.</p>
    {:else if !here.outfitting}
      <p class="lo">No outfitting seen here yet. Open the outfitting screen once and it is kept.</p>
    {:else if here.outfitting.upgrades.length}
      <p class="sub">
        Better than what you have fitted, of {here.outfitting.items.toLocaleString()} modules for sale (seen {when(here.outfitting.seen)} UTC):
      </p>
      <table aria-label="Upgrades on sale">
        <thead><tr><th>Slot</th><th>Fitted</th><th>On sale</th><th class="num">Price</th></tr></thead>
        <tbody>
          {#each here.outfitting.upgrades as u (u.slot)}
            <tr><td>{u.slot_name}</td><td class="lo">{u.fitted}</td><td class="station">{u.name}</td><td class="num">{cr(u.price)}</td></tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="lo">Nothing here beats what you have fitted ({here.outfitting.items.toLocaleString()} modules for sale, seen {when(here.outfitting.seen)} UTC).</p>
    {/if}
  {/if}

  <h3>Where did I see…</h3>
  <label class="search">
    <span>Module</span>
    <input type="search" bind:value={query} placeholder="5a frame shift, fsd, pulse laser, hyperdrive_size5" spellcheck="false" autocomplete="off" />
  </label>
  {#if searchFailed}
    <p class="err">{searchFailed}</p>
  {:else if result}
    {#if result.snapshots === 0}
      <p class="lo">No outfitting kept yet. The core keeps each outfitting you open, from now on.</p>
    {:else if !trimmed}
      <p class="lo">{result.stations.toLocaleString()} {result.stations === 1 ? "station" : "stations"} remembered ({result.snapshots.toLocaleString()} outfitting files). Search for a module by name, size and rating, or symbol.</p>
    {:else if result.found.length}
      <table aria-label="Stations that sold it">
        <thead><tr><th>Module</th><th>Station</th><th class="num">Price</th><th>Seen</th><th class="num">Distance</th></tr></thead>
        <tbody>
          {#each result.found as f (`${f.market_id}:${f.item}`)}
            <tr>
              <td class="station">{f.name}</td>
              <td>{f.station_name} <SystemChip name={f.star_system} /></td>
              <td class="num">{cr(f.price)}</td>
              <td class="mono">{when(f.seen)}</td>
              <td class="num">{f.distance_ly === null ? "–" : `${f.distance_ly.toFixed(1)} ly`}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="lo">No remembered station sold “{trimmed}”.</p>
    {/if}
    {#if result.source}<p class="lo">{result.source}</p>{/if}
  {/if}
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  h3 { margin: 22px 0 8px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 8px 0; }
  .err { color: var(--accent); }
  .lo { color: var(--text-lo); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .station { color: var(--text-hi); }
  .head { display: flex; align-items: baseline; gap: 10px; }
  .faction { display: flex; gap: 6px; }
  .state { color: var(--accent); }
  .facts { display: grid; grid-template-columns: max-content 1fr; gap: 6px 18px; margin: 10px 0 0; max-width: 60rem; }
  dt { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); padding-top: 2px; }
  dd { margin: 0; }
  .services { display: flex; flex-wrap: wrap; gap: 4px; }
  .svc { background: var(--cell); padding: 1px 8px; font-size: var(--fs-s); }
  .search { display: flex; align-items: center; gap: 12px; margin: 10px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); color: var(--text-lo); font-size: var(--fs-s); }
  .search input { flex: 0 1 30rem; background: var(--cell); color: var(--text); border: 1px solid var(--accent-dim); padding: 6px 8px; font: inherit; text-transform: none; letter-spacing: normal; font-size: var(--fs); }
  .search input:focus { outline: 1px solid var(--accent); outline-offset: -1px; }
  table { border-collapse: collapse; width: 100%; max-width: 60rem; margin-top: 6px; }
  th { text-align: left; color: var(--text-lo); font-size: var(--fs-s); font-weight: 400; text-transform: uppercase; letter-spacing: var(--caps); padding: 0 12px 6px 0; }
  td { padding: 5px 12px 5px 0; border-top: 1px solid var(--cell); vertical-align: baseline; }
  .num { text-align: right; font-family: var(--mono); }
</style>
