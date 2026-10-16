<script lang="ts">
  // Nearest systems for one lookup (Famine, an HGE pool), with jumps at the current ship's range.
  import { ago, jumps, nearby, type Nearby } from "../galaxy";
  import SystemChip from "./SystemChip.svelte";

  let { query }: { query: string } = $props();
  let res = $state<Nearby | null>(null);
  let err = $state<string | null>(null);
  let busy = $state(false);

  $effect(() => {
    const q = query;
    busy = true;
    err = null;
    nearby(q)
      .then((r) => (res = r))
      .catch((e: unknown) => {
        res = null;
        err = String(e);
      })
      .finally(() => (busy = false));
  });

  function states(h: Nearby["hits"][number]): string {
    const f = h.factions[0];
    if (!f) return "";
    const st = f.active_states.length ? f.active_states.join(", ") : (f.state ?? "");
    return `${f.name} (${f.allegiance}${st ? `, ${st}` : ""})`;
  }
</script>

<div class="near">
  <h4>Nearest{#if res}&nbsp;to&nbsp;<SystemChip name={res.here.system} />{/if}</h4>
  {#if busy && !res}
    <p class="lo">Looking up live faction states…</p>
  {:else if err}
    <p class="lo">{err}</p>
  {:else if res}
    {#if res.warning}<p class="lo">{res.warning}</p>{/if}
    <ul>
      {#each res.hits as h (h.id64)}
        <li>
          <span class="sys"><SystemChip name={h.name} /></span>
          <span class="q">{jumps(h.jumps, h.exact)} · {h.ly.toFixed(1)} ly</span>
          <span class="lo">{states(h)} · reported {ago(h.updated_at)}</span>
        </li>
      {:else}
        <li class="lo">None found.</li>
      {/each}
    </ul>
    <p class="lo small">At {res.range.toFixed(1)} ly per jump · Spansh, {ago(res.fetched)}{res.hits.some((h) => !h.exact) ? " · ~ = straight-line estimate" : ""}</p>
  {/if}
</div>

<style>
  .near { margin-top: 14px; }
  h4 { margin: 0 0 6px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 8px; }
  li { display: flex; flex-direction: column; gap: 1px; font-size: var(--fs-s); line-height: 1.35; }
  .sys { color: var(--text); }
  .q { color: var(--accent); font-family: var(--mono); }
  .lo { color: var(--text-lo); }
  .small { font-size: var(--fs-s); margin-top: 8px; }
  p { margin: 0 0 6px; }
</style>
