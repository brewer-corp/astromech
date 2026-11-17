<script lang="ts">
  // Trade > Colonisation as progress toward building the site (astromech-m07p), on the List with
  // detail layout: the site's head (its name, system and how far it is built, the page's main
  // value), each commodity it requires as delivered against required with what is aboard marked,
  // the loads left for the ship flown, then Where to buy, whose station rows fill the side panel
  // when selected, as a Places finder's do. Below, outside the layout (its own search kit would
  // share the side panel), systems still open to claim near you.
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { untrack } from "svelte";
  import Colonies from "../../lib/ui/Colonies.svelte";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
  import Meter from "../../lib/ui/Meter.svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import ListDetail from "../../lib/ui/layout/ListDetail.svelte";
  import { tip } from "../../lib/ui/tip";
  import { buyList, buyResults, loadProjects, LOOKUP, needsOf, progressOf, projectName, type Need, type Project } from "../../lib/colonisation";
  import type { Filters, Origin, Results } from "../../lib/search";
  import { credits, DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS } from "../../lib/trade";

  let projects = $state<Project[] | null>(null);
  let err = $state<string | null>(null);
  let picked = $state<number | null>(null);

  // Polled like the other journal pages, so deliveries and purchases show within seconds (the
  // core replays the journals only when they or Cargo.json change). The picked project stays.
  /** A poll is waiting for the core: the next tick skips, so answers never overlap or race
   *  (a cold replay of a long history can take longer than the interval). */
  let inFlight = false;
  $effect(() => {
    const load = () => {
      if (inFlight) return;
      inFlight = true;
      return loadProjects()
        .then((p) => {
          projects = p;
          err = null;
          const keep = untrack(() => picked);
          if (keep == null || !p.some((x) => x.market_id === keep)) picked = p[0]?.market_id ?? null;
        })
        .catch((e: unknown) => (err = pollFailure("read the construction sites", e, projects !== null)))
        .finally(() => (inFlight = false));
    };
    void load();
    const t = setInterval(() => void load(), 2000);
    return () => clearInterval(t);
  });

  const project = $derived(projects?.find((p) => p.market_id === picked) ?? null);
  const rows = $derived(project ? progressOf(project) : []);
  /** What is left to buy; replaced only when it changes, so a poll doesn't re-run the search. */
  let lastNeeds: Need[] = [];
  const needs = $derived.by(() => {
    const next = project ? needsOf(project) : [];
    if (JSON.stringify(next) !== JSON.stringify(lastNeeds)) lastNeeds = next;
    return lastNeeds;
  });

  const run = $derived.by(() => {
    const list = needs;
    return async (origin: Origin, filters: Filters): Promise<Results> => buyResults(await buyList(list, origin, filters), list.length);
  });

  const percent = (p: Project) => Math.floor(p.progress * 100);
  function status(p: Project): string {
    if (p.complete) return "complete";
    if (p.failed) return "failed";
    return `${percent(p)} %`;
  }

  /** "13 loads at 784 t", or why there are none. */
  const loads = $derived.by(() => {
    const s = project?.shopping;
    if (!s) return "";
    if (s.loads == null || s.capacity == null) return "no cargo capacity known for the ship flown, so no loads";
    return `${s.loads} ${s.loads === 1 ? "load" : "loads"} at ${credits(s.capacity)} t`;
  });

  const ORDER =
    "Stations selling the most of the list first, nearest first among equals. Searches from the construction site's system; clear From under Change to search near you.";
</script>

<div class="page">
  <ListDetail>
    <Stale error={projects ? err : null} />
    {#if err && !projects}
      <p class="type-label">{err}</p>
    {:else if !projects}
      <p class="type-label">Reading the journals…</p>
    {:else if !projects.length}
      <p class="type-label">No construction sites in your journals yet. Dock at one and it shows here with what it still needs.</p>
    {:else}
      {#if projects.length > 1}
        <!-- The site is a dropdown on the page, not a third tab row (DESIGN.md The Two Tab Rows
             Rule); a coloniser can have many. -->
        <div class="pick">
          <Dropdown bind:value={picked} aria-label="Construction site">
            {#each projects as p (p.market_id)}<option value={p.market_id}>{projectName(p)} · {status(p)}</option>{/each}
          </Dropdown>
        </div>
      {/if}
      {#if project}
        <div class="site">
        <header class="head">
          <h1 class="type-title">{projectName(project)}</h1>
          {#if project.system_name}<p class="type-name"><SystemChip name={project.system_name} /></p>{/if}
          <p class="built">
            {#if project.complete}<span class="type-value-xl">Complete</span>
            {:else if project.failed}<span class="type-value-xl">Failed</span>
            {:else}<span class="type-value-xl main">{percent(project)}</span><span class="type-unit">% built</span>{/if}
          </p>
          <Meter value={project.complete ? 1 : project.progress} />
        </header>

        <ul class="rows" aria-label="Commodities">
          {#each rows as r (r.name)}
            <li class:done={r.complete}>
              <span class="what">
                <span class="type-name">{r.name}</span>
                <span class="type-label">{credits(r.provided)} of {credits(r.required)} t{#if r.aboard} · <span>{credits(r.aboard)} t aboard</span>{/if}</span>
              </span>
              <Meter value={r.complete ? 1 : r.provided / r.required} coming={r.aboard / r.required} />
              <span class="buy">
                <span class="type-value-l" class:short={r.toBuy > 0}>{credits(r.toBuy)}</span><span class="type-unit">t to buy</span>
              </span>
            </li>
          {/each}
        </ul>
        </div>

        {#if needs.length}
          <p class="type-label loads">{credits(project.shopping.to_buy_total)} t to buy · {loads}</p>
          <h2 class="section where" {@attach tip(ORDER)}>Where to buy</h2>
          <SearchKit
            lookup={LOOKUP}
            offers={["distance", "volume", "pad", "age", "arrival", "carriers", "odyssey"]}
            hints={{ volume: "Stock", distance: DEFAULT_DISTANCE_LY, age: DEFAULT_MAX_AGE_DAYS }}
            {run}
            start={project.system_name}
            title="Where to buy"
            joiner="near"
            noun="stations"
            busyText="Asking Ardent Insight…"
          />
        {:else}
          <p class="type-label loads">{project.complete || project.failed ? "Nothing left to deliver." : "Everything still needed is aboard: deliver it."}</p>
        {/if}
      {/if}
    {/if}
  </ListDetail>
  <Colonies />
</div>

<style>
  p { margin: 0; }
  .pick { padding: 0 0 var(--sp-4); }
  /* The site's head and its rows as one block as wide as the rows, so the head's meter spans them. */
  .site { display: grid; width: fit-content; max-width: 100%; }
  .head { display: grid; gap: var(--sp-2); justify-items: start; margin: 0 0 var(--sp-6); }
  .head h1 { margin: 0; }
  .built { margin-top: var(--sp-2); }
  .head > :global(.meter) { justify-self: stretch; }

  /* Each commodity: its name over delivered of required, the meter, then what is still to buy.
     The columns are shared (subgrid), so the meters and the figures line up down the list. */
  .rows {
    list-style: none; margin: 0; padding: 0;
    display: grid; grid-template-columns: minmax(10rem, max-content) minmax(8rem, 20rem) max-content;
    gap: var(--sp-3) var(--sp-6); align-items: center;
  }
  .rows li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center; }
  .what { display: grid; gap: var(--gap); }
  .buy { justify-self: end; white-space: nowrap; }
  .short { color: var(--accent); }
  .done .type-name, .done .type-label, .done .type-value-l, .done .type-unit { color: var(--grey-dim); }
  .loads { margin: var(--sp-4) 0 var(--sp-8); }
  .where { margin: 0 0 var(--sp-2); }
</style>
