<script lang="ts">
  // The Station page (DESIGN.md Board), under NOW: what is left of the station docked at once the
  // Now page has shown its services, read as answers. The shared station head, one compact fact
  // list, the market against nearby prices (Market), the upgrades its outfitting sells for the
  // ship flown, and a search over every outfitting the core has seen: "where did I see a 5A
  // drive?" works offline.
  import Input from "../../lib/ui/Input.svelte";
  import { tip } from "../../lib/ui/tip";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { when } from "../../lib/logbook";
  import { figure } from "../../lib/search";
  import { loadHere, MENU_SERVICES, searchOutfitting, serviceName, type DockedHere, type OutfittingSearch } from "../../lib/station";
  import Board from "../../lib/ui/layout/Board.svelte";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import Head from "./Head.svelte";
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
        .catch((e: unknown) => (failed = pollFailure("read the docked station", e, here !== null)));
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

  const pct = (x: number) => `${Math.round(x * 100)}%`;
  const trimmed = $derived(query.trim());
  /** The services the station reports that the Now page's board doesn't draw: none is lost. */
  const BOARD = new Set<string>(MENU_SERVICES);
  const also = $derived(here?.services.filter((s) => !BOARD.has(s)) ?? []);
</script>

<Board>
  <div class="page">
    <Stale error={here ? failed : null} />
    {#if failed && !here}
      <!-- The page reads again every 5 s on its own: the next read is the recovery, so no action. -->
      <p class="type-label err" role="alert"><b>!</b> {failed}</p>
    {:else if !here}
      <p class="type-label">Reading the journal…</p>
    {:else if !here.docked}
      <div class="lead">
        {#if here.system}
          <h2 class="type-title system"><span>{here.system}</span><SystemChip name={here.system} icon /></h2>
          <p class="links"><ExtLinks system={here.system} /></p>
        {/if}
        <p class="type-label">Not docked. Dock somewhere and the station's facts show here.</p>
      </div>
    {:else}
      <div class="lead">
        <Head {here} />
        <dl class="facts">
          {#if here.faction}
            <dt class="type-label">Faction</dt>
            <dd class="type-figures"><span>{here.faction.name}</span>{#if here.faction.state}<span class="type-label">in</span><span>{here.faction.state}</span>{/if}</dd>
          {/if}
          {#if here.government || here.allegiance}
            <dt class="type-label">Government</dt>
            <dd class="type-figures">{[here.government, here.allegiance].filter(Boolean).join(" · ")}</dd>
          {/if}
          {#if here.economies.length}
            <dt class="type-label">Economy</dt>
            <dd class="type-figures">{here.economies.map((e) => `${e.name_localised ?? e.name} ${pct(e.proportion)}`).join(" · ")}</dd>
          {/if}
          {#if here.landing_pads}
            <dt class="type-label">Landing pads</dt>
            <dd class="type-figures">
              <span>{here.landing_pads.large} large · {here.landing_pads.medium} medium · {here.landing_pads.small} small</span>
              {#if here.max_pad}<span class="type-label">up to {here.max_pad.toLowerCase()}</span>{/if}
            </dd>
          {/if}
          <dt class="type-label">Links</dt>
          <dd class="links">
            {#if here.station}<span class="type-label">Station</span><ExtLinks system={here.system ?? ""} station={here.station} marketId={here.market_id} />{/if}
            {#if here.system}<span class="type-label">System</span><ExtLinks system={here.system} />{/if}
          </dd>
        </dl>
        {#if also.length}<p class="type-label">Also: {also.map(serviceName).join(", ")}</p>{/if}
        {#if here.source}<p class="type-label">{here.source}</p>{/if}
      </div>

      <section aria-labelledby="market-head">
        <h3 class="section" id="market-head">Market</h3>
        {#if !here.services.includes("commodities")}
          <p class="type-label">This station has no market.</p>
        {:else}
          <Market marketId={here.market_id} />
        {/if}
      </section>

      <section aria-labelledby="outfitting-head">
        <h3 class="section" id="outfitting-head">Outfitting{here.ship ? ` for ${here.ship}` : ""}</h3>
        {#if !here.services.includes("outfitting") && !here.outfitting}
          <p class="type-label">This station has no outfitting.</p>
        {:else if !here.outfitting}
          <p class="type-body">No outfitting seen here yet. Open the outfitting screen once and it is kept.</p>
        {:else if here.outfitting.upgrades.length}
          <p class="type-label">Of {here.outfitting.items.toLocaleString("en")} modules for sale, seen {when(here.outfitting.seen)} UTC</p>
          <div class="list upgrades" role="table" aria-label="Upgrades on sale">
            <div class="row heads type-label" role="row">
              <span role="columnheader">Slot</span>
              <span role="columnheader">Fitted</span>
              <span role="columnheader">Better on sale</span>
              <span class="r" role="columnheader">Price</span>
            </div>
            {#each here.outfitting.upgrades as u (u.slot)}
              <div class="row one" role="row">
                <span class="type-label" role="cell">{u.slot_name}</span>
                <span class="type-figures" role="cell">{u.fitted}</span>
                <span class="type-name" role="cell">{u.name}</span>
                <span class="type-figures r" role="cell">{figure(u.price)}<span class="type-unit">cr</span></span>
              </div>
            {/each}
          </div>
        {:else}
          <p class="type-label">Nothing here beats what you have fitted ({here.outfitting.items.toLocaleString("en")} modules for sale, seen {when(here.outfitting.seen)} UTC).</p>
        {/if}
      </section>
    {/if}

    <section aria-labelledby="seen-head">
      <h3 class="section" id="seen-head">Where did I see…</h3>
      <label class="search type-label">
        <span>Module</span>
        <Input class="q" type="search" {@attach tip("A module's name, size and rating, or symbol")} bind:value={query} placeholder="5a frame shift, fsd, pulse laser, hyperdrive_size5" spellcheck="false" autocomplete="off" />
      </label>
      {#if searchFailed}
        <p class="type-label err" role="alert"><b>!</b> {searchFailed}</p>
      {:else if result}
        {#if result.snapshots === 0}
          <p class="type-label">No outfitting kept yet. The core keeps each outfitting you open, from now on.</p>
        {:else if !trimmed}
          <p class="type-label">{result.stations.toLocaleString("en")} {result.stations === 1 ? "station" : "stations"} remembered ({result.snapshots.toLocaleString("en")} outfitting files)</p>
        {:else if result.found.length}
          <!-- Newest seen first, as the core orders them: no column ranks the list. -->
          <div class="list found" role="table" aria-label="Stations that sold it">
            <div class="row heads type-label" role="row">
              <span role="columnheader">Module</span>
              <span role="columnheader">Station · system</span>
              <span class="r" role="columnheader">Price</span>
              <span class="r" role="columnheader">Seen · distance</span>
            </div>
            {#each result.found as f (`${f.market_id}:${f.item}`)}
              <div class="row result" role="row">
                <span class="g" role="cell"><span class="type-name ell">{f.name}</span></span>
                <span class="g" role="cell">
                  <span class="type-name ell">{f.station_name}</span>
                  <span class="l1"><span class="type-label ell">{f.star_system}</span><SystemChip name={f.star_system} icon /></span>
                </span>
                <span class="g r" role="cell"><span class="type-figures">{figure(f.price)}<span class="type-unit">cr</span></span></span>
                <span class="g r" role="cell">
                  <span class="type-figures">{when(f.seen)}</span>
                  <span class="type-label">{f.distance_ly === null ? "–" : `${figure(f.distance_ly, 1)} ly`}</span>
                </span>
              </div>
            {/each}
          </div>
        {:else}
          <p class="type-label">No remembered station sold “{trimmed}”.</p>
        {/if}
        {#if result.source}<p class="type-label">{result.source}</p>{/if}
      {/if}
    </section>
  </div>
</Board>

<style>
  p, h2, h3, dl, dd { margin: 0; }
  .page { display: flex; flex-direction: column; gap: var(--sp-8); min-width: 0; }
  .lead, section { display: flex; flex-direction: column; gap: var(--sp-3); min-width: 0; }
  .system { display: flex; align-items: center; gap: var(--sp-1); }
  .err { color: var(--alarm); }
  .err b { font-weight: 600; }
  /* DESIGN.md Fact list: the label in Label, the value in Figures; one compact column. */
  .facts { display: grid; grid-template-columns: max-content minmax(0, max-content); align-items: baseline; gap: var(--sp-1) var(--sp-6); }
  .facts dd { display: flex; flex-wrap: wrap; align-items: baseline; gap: 0 var(--sp-2); }
  .links { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); }
  .search { display: flex; align-items: center; gap: var(--sp-3); }
  .search > :global(.q) { flex: 0 1 28rem; }

  /* DESIGN.md Lookup result row: groups on one set of tracks, a row as wide as its groups. */
  .list { display: grid; column-gap: var(--sp-6); width: fit-content; max-width: 100%; }
  .upgrades { grid-template-columns: max-content max-content max-content max-content; }
  .found { grid-template-columns: minmax(0, max-content) minmax(0, max-content) max-content max-content; }
  .row { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center; padding: 0 var(--sp-3); white-space: nowrap; }
  .heads { height: var(--sp-8); border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); }
  .one { height: var(--row); }
  .result { height: var(--row-result); }
  .row:not(.heads):hover { background: var(--row-hover); }
  .r { text-align: right; }
  .g { display: flex; flex-direction: column; justify-content: center; min-width: 0; overflow: hidden; }
  .g.r { align-items: flex-end; }
  .l1 { display: flex; align-items: center; min-width: 0; }
  .l1 :global(.chip) { flex: none; }
  .l1 :global(.chip .ibtn) { height: 1.25rem; }
  .ell { overflow: hidden; text-overflow: ellipsis; }
</style>
