<script lang="ts">
  // The "search near me" kit every lookup uses: pick where to search from (current or typed
  // system) and the lookup's remembered filters, run the lookup, and list the nearest results
  // with jumps at the current range, distance, pad size, arrival distance and data age.
  import { ago, jumps } from "../galaxy";
  import { changeFilters, openFilters, type FilterStore } from "../filterstate";
  import { arrivalLabel, DEFAULT_FILTERS, loadFilters, padLabel, saveFilters, type FilterHints, type FilterKey, type Filters, type Origin, type Results } from "../search";
  import OriginInput from "./OriginInput.svelte";
  import SearchFilters from "./SearchFilters.svelte";
  import SystemChip from "./SystemChip.svelte";

  let {
    lookup,
    offers,
    hints = {},
    run,
    title = "Nearest",
    joiner = "to",
    busyText = "Searching…",
    start = null,
  }: {
    /** Stable id of the lookup: filters are remembered under it, and a change re-runs the search. */
    lookup: string;
    /** The filters this lookup can apply. */
    offers: FilterKey[];
    hints?: FilterHints;
    /** A new function (e.g. another commodity) re-runs the search. */
    run: (origin: Origin, filters: Filters) => Promise<Results>;
    title?: string;
    /** Between the title and the origin: "Nearest to Sol", "Best prices near Sol". */
    joiner?: string;
    busyText?: string;
    /** Where to search from until the commander picks another system; a new one resets the box. */
    start?: Origin;
  } = $props();

  // Follows `start`; the commander's pick (or ×) overrides it until `start` changes.
  let origin = $derived<Origin>(start);
  /** Filters loaded for `lookup` (null until they arrive). */
  let saved = $state<{ lookup: string; filters: Filters } | null>(null);
  let res = $state<Results | null>(null);
  let err = $state<string | null>(null);
  let filterErr = $state<string | null>(null);
  const store: FilterStore = { load: loadFilters, save: saveFilters };
  let busy = $state(false);
  let asked = 0;
  let changed = 0; // filter changes made, so a slow earlier one can't overwrite a later one

  $effect(() => {
    const l = lookup;
    filterErr = null;
    void openFilters(store, l, DEFAULT_FILTERS).then((st) => {
      if (l === lookup) saved = { lookup: l, filters: st.filters };
    });
  });

  async function change(next: Filters) {
    const l = lookup;
    const prev = saved?.filters ?? DEFAULT_FILTERS;
    saved = { lookup: l, filters: next };
    const n = ++changed;
    const st = await changeFilters(store, l, prev, next);
    if (l !== lookup || n !== changed) return;
    filterErr = st.error;
    if (JSON.stringify(st.filters) !== JSON.stringify(next)) saved = { lookup: l, filters: st.filters };
  }

  $effect(() => {
    const o = origin;
    const s = saved;
    if (!s || s.lookup !== lookup) return;
    const n = ++asked;
    busy = true;
    err = null;
    run(o, s.filters)
      .then((r) => {
        if (n === asked) res = r;
      })
      .catch((e: unknown) => {
        if (n !== asked) return;
        res = null;
        err = String(e);
      })
      .finally(() => {
        if (n === asked) busy = false;
      });
  });

  function facts(r: Results["rows"][number]): string {
    return [padLabel(r.pad), arrivalLabel(r.arrival_ls), r.detail ?? "", `reported ${ago(r.updated_at)}`].filter(Boolean).join(" · ");
  }
</script>

<div class="kit">
  <h4>{title}{#if res}&nbsp;{joiner}&nbsp;<SystemChip name={res.origin} />{/if}</h4>
  <OriginInput {origin} onchange={(o) => (origin = o)} />
  {#if saved && offers.length}<SearchFilters filters={saved.filters} {offers} {hints} onchange={(f) => void change(f)} />{/if}
  {#if filterErr}<p class="lo">{filterErr}</p>{/if}
  {#if busy && !res}
    <p class="lo">{busyText}</p>
  {:else if err}
    <p class="lo">{err}</p>
  {:else if res}
    {#if res.warning}<p class="lo">{res.warning}</p>{/if}
    <ul class:stale={busy}>
      {#each res.rows as r (r.key)}
        <li>
          <span class="sys">{r.station ? `${r.station} · ` : ""}<SystemChip name={r.system} /></span>
          <span class="q">{r.lead ? `${r.lead} · ` : ""}{jumps(r.jumps, r.exact)} · {r.ly.toFixed(1)} ly</span>
          <span class="lo">{facts(r)}</span>
        </li>
      {:else}
        <li class="lo">None found.</li>
      {/each}
    </ul>
    <p class="lo small">At {res.range.toFixed(1)} ly per jump · {res.source}, {ago(res.fetched)}{res.rows.some((r) => !r.exact) ? " · ~ = straight-line estimate" : ""}</p>
  {/if}
</div>

<style>
  .kit { margin-top: 14px; }
  h4 { margin: 0 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 8px; }
  ul.stale { opacity: 0.5; }
  li { display: flex; flex-direction: column; gap: 1px; font-size: var(--fs-s); line-height: 1.35; }
  .sys { color: var(--text); }
  .q { color: var(--accent); font-family: var(--mono); }
  .lo { color: var(--text-lo); }
  .small { font-size: var(--fs-s); margin-top: 8px; }
  p { margin: 0 0 6px; }
</style>
