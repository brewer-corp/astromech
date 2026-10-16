<script lang="ts">
  // Colonisation construction projects from the journals: what is left to buy for the picked
  // project, in loads for the current ship, and where to buy it, stations stocking the most of
  // the list first, searched from the construction site's system (or near you).
  import { untrack } from "svelte";
  import SearchKit from "../../lib/ui/SearchKit.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import { buyList, buyResults, loadProjects, LOOKUP, needsOf, projectName, type Need, type Project } from "../../lib/colonisation";
  import type { Filters, Origin, Results } from "../../lib/search";
  import { credits, DEFAULT_DISTANCE_LY, DEFAULT_MAX_AGE_DAYS } from "../../lib/trade";

  let projects = $state<Project[] | null>(null);
  let err = $state<string | null>(null);
  let picked = $state<number | null>(null);

  // Polled like the other journal pages, so deliveries and purchases show within seconds (the
  // shell replays the journals only when they or Cargo.json change). The picked project stays.
  /** A poll is waiting for the shell: the next tick skips, so answers never overlap or race
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
        .catch((e: unknown) => (err = String(e)))
        .finally(() => (inFlight = false));
    };
    void load();
    const t = setInterval(() => void load(), 2000);
    return () => clearInterval(t);
  });

  const project = $derived(projects?.find((p) => p.market_id === picked) ?? null);
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

  function status(p: Project): string {
    if (p.complete) return "complete";
    if (p.failed) return "failed";
    return `${Math.floor(p.progress * 100)}%`;
  }

  const loads = $derived.by(() => {
    const s = project?.shopping;
    if (!s || s.loads == null || s.capacity == null) return "";
    return ` · ${s.loads} ${s.loads === 1 ? "load" : "loads"} at ${credits(s.capacity)} t`;
  });
</script>

<section>
  {#if err && !projects}
    <p class="lo">{err}</p>
  {:else if !projects}
    <p class="lo">Reading the journals…</p>
  {:else if !projects.length}
    <p class="lo">No construction sites in your journals yet. Dock at one and it shows here with what it still needs.</p>
  {:else}
    {#if projects.length > 1}
      <div class="acts">
        {#each projects as p (p.market_id)}
          <button class:on={p.market_id === picked} onclick={() => (picked = p.market_id)}>{projectName(p)} <span class="lo">{status(p)}</span></button>
        {/each}
      </div>
    {/if}
    {#if project}
      <div class="main">
        <h3>{projectName(project)}</h3>
        <p class="sub">
          {#if project.system_name}<SystemChip name={project.system_name} />&nbsp;·&nbsp;{/if}{project.complete || project.failed ? status(project) : `${status(project)} built`}
        </p>
        {#if needs.length}
          <h4>Still to buy</h4>
          <ul class="list">
            {#each project.shopping.lines as l (l.symbol)}
              <li>
                <span>{l.name_localised ?? l.symbol}</span>
                <span class="q">{credits(l.to_buy)} t</span>
                <span class="lo">{l.in_cargo ? `${credits(l.in_cargo)} t aboard` : ""}</span>
              </li>
            {/each}
          </ul>
          <p class="total">{credits(project.shopping.to_buy_total)} t in all{loads}</p>
          <p class="sub">
            Stations selling the most of the list first, nearest first among equals. Searches from the construction site's system; clear it to search near you.
          </p>
          <SearchKit
            lookup={LOOKUP}
            offers={["distance", "volume", "pad", "age", "arrival", "carriers", "odyssey"]}
            hints={{ volume: "Stock", distance: DEFAULT_DISTANCE_LY, age: DEFAULT_MAX_AGE_DAYS }}
            {run}
            start={project.system_name}
            title="Where to buy"
            joiner="near"
            busyText="Asking Ardent Insight…"
          />
        {:else}
          <p class="lo">{project.complete || project.failed ? "Nothing left to deliver." : "Everything still needed is aboard: deliver it."}</p>
        {/if}
      </div>
    {/if}
  {/if}
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; padding: 14px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .main { max-width: 40rem; }
  h3 { margin: 14px 0 2px; color: var(--text-hi); font-weight: 500; }
  h4 { margin: 14px 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0 0 10px; }
  .list { list-style: none; margin: 0; padding: 0; display: grid; grid-template-columns: max-content max-content 1fr; column-gap: 18px; row-gap: 3px; font-size: var(--fs-s); }
  .list li { display: contents; }
  .q { color: var(--accent); font-family: var(--mono); text-align: right; }
  .total { font-size: var(--fs-s); color: var(--text); margin: 8px 0 14px; }
  .lo { color: var(--text-lo); }
</style>
