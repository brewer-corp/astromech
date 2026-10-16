<script lang="ts">
  // The "search near me" kit every lookup uses: pick where to search from (current or typed
  // system), run the lookup, and list the nearest results with jumps at the current range,
  // distance, pad size, arrival distance and data age.
  import { ago, jumps } from "../galaxy";
  import { arrivalLabel, padLabel, type Origin, type Results } from "../search";
  import OriginInput from "./OriginInput.svelte";

  let {
    lookup,
    run,
    title = "Nearest",
    busyText = "Searching…",
  }: {
    /** Stable id of the lookup; a change re-runs the search. */
    lookup: string;
    run: (origin: Origin) => Promise<Results>;
    title?: string;
    busyText?: string;
  } = $props();

  let origin = $state<Origin>(null);
  let res = $state<Results | null>(null);
  let err = $state<string | null>(null);
  let busy = $state(false);
  let asked = 0;

  $effect(() => {
    void lookup;
    const o = origin;
    const n = ++asked;
    busy = true;
    err = null;
    run(o)
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
  <h4>{title}{res ? ` to ${res.origin}` : ""}</h4>
  <OriginInput {origin} onchange={(o) => (origin = o)} />
  {#if busy && !res}
    <p class="lo">{busyText}</p>
  {:else if err}
    <p class="lo">{err}</p>
  {:else if res}
    {#if res.warning}<p class="lo">{res.warning}</p>{/if}
    <ul class:stale={busy}>
      {#each res.rows as r (r.key)}
        <li>
          <span class="sys">{r.station ? `${r.station} · ` : ""}{r.system}</span>
          <span class="q">{jumps(r.jumps, r.exact)} · {r.ly.toFixed(1)} ly</span>
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
