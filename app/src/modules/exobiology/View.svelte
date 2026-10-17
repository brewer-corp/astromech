<script lang="ts">
  // The current system's biology, ranked by value: each body's genera (from the DSS, or predicted
  // from its scan before the DSS) with the species they could be and what they are worth; what
  // other players reported here (Spansh); and the sampling in progress with how far the last
  // samples are against the genus's colony range. Live while Elite runs.
  import { loadBiology, millions, shortName, valueRange, type BiologyView } from "../../lib/exobiology";

  let v = $state<BiologyView | null>(null);
  let failed = $state<string | null>(null);

  // Follows the game: scans and samples arrive while the window is open; on foot the distance
  // to the last sample changes by the second.
  $effect(() => {
    const tick = () =>
      loadBiology()
        .then((r) => {
          v = r;
          failed = null;
        })
        .catch((e: unknown) => (failed = e instanceof Error ? e.message : String(e)));
    void tick();
    const t = setInterval(() => void tick(), 2000);
    return () => clearInterval(t);
  });

  const total = $derived(
    v ? v.bodies.reduce<[number, number]>((a, b) => [a[0] + b.value[0], a[1] + b.value[1]], [0, 0]) : ([0, 0] as [number, number]),
  );
  const short = (name: string) => shortName(name, v?.system ?? null);
  const plural = (n: number, one: string, many: string) => `${n} ${n === 1 ? one : many}`;
  const sampledBody = $derived(v?.sampling ? v.bodies.find((b) => b.body_id === v?.sampling?.body_id) : undefined);
</script>

<section>
  {#if failed}
    <p class="sub">{failed} Biology follows the game while it runs.</p>
  {:else if !v}
    <p class="sub">Reading the journal…</p>
  {:else}
    <p class="sub">
      {v.system ?? "No system yet"}
      {#if v.bodies.length}· biology on {plural(v.bodies.length, "body", "bodies")} · {valueRange(total)}{/if}
      {#if v.source}· {v.source}{/if}
    </p>

    {#if v.sampling}
      {@const s = v.sampling}
      {@const c = s.clearance}
      <h3>Sampling</h3>
      <div class="tiles" aria-label="Sampling">
        <div><span class="k">Species</span><span class="v">{s.species}</span></div>
        <div><span class="k">Samples</span><span class="v">{s.count} of 3</span></div>
        {#if s.complete}
          <div><span class="k">Status</span><span class="v">Analysed</span></div>
        {:else if c}
          <div class:go={c.far_enough}>
            <span class="k">From the nearest sample</span>
            <span class="v">{Math.round(c.nearest_m)} m of {c.colony_range_m} m</span>
          </div>
          <div class:go={c.far_enough}>
            <span class="k">Sample {c.next}</span>
            <span class="v">{c.far_enough ? "Far enough" : `${Math.max(0, Math.ceil(c.colony_range_m - c.nearest_m))} m to go`}</span>
          </div>
        {:else if s.colony_range_m !== null}
          <div><span class="k">Colony range</span><span class="v">{s.colony_range_m} m</span></div>
        {/if}
      </div>
      {#if sampledBody}<p class="lo">On {short(sampledBody.name)}. Sample again at least the colony range from every earlier sample.</p>{/if}
    {/if}

    <h3>Bodies with biology</h3>
    {#if v.bodies.length}
      <table aria-label="Bodies with biology">
        <thead><tr><th>Body</th><th class="num">Signals</th><th>Genera</th><th class="num">Value</th></tr></thead>
        <tbody>
          {#each v.bodies as b (b.body_id)}
            <tr>
              <td class="body">{short(b.name)}</td>
              <td class="num">{b.signals ?? "–"}</td>
              <td>
                {#if b.genera.length}
                  <ul class="genera">
                    {#each b.genera as g (g.genus)}
                      <li class:guess={!g.known}>
                        <span class="genus">{g.genus}</span>
                        {#if g.species.length}<span class="lo">{g.species.map((c) => c.species.replace(`${g.genus} `, "")).join(", ")}</span>{/if}
                        <span class="mono">{valueRange(g.value)}</span>
                      </li>
                    {/each}
                  </ul>
                  <span class="lo small">{b.mapped ? "From the DSS" : "Predicted from the scan; the DSS will tell"}</span>
                {:else}
                  <span class="lo">Scan the body to predict its genera.</span>
                {/if}
              </td>
              <td class="num">{valueRange(b.value)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="lo">No biological signals found in this system yet. The FSS shows them.</p>
    {/if}

    {#if v.known?.length}
      <h3>Reported by other players</h3>
      <table aria-label="Reported by other players">
        <thead><tr><th>Body</th><th class="num">Signals</th><th>Genera and species</th><th class="num">Value</th></tr></thead>
        <tbody>
          {#each v.known as k (k.name)}
            <tr>
              <td class="body">{short(k.name)}</td>
              <td class="num">{k.signals}</td>
              <td>
                {k.genera.join(", ")}
                {#if k.species.length}<br /><span class="lo">{k.species.map((s) => `${s.name} ${millions(s.value)}`).join(" · ")}</span>{/if}
              </td>
              <td class="num">{valueRange(k.value)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
      <p class="lo small">From Spansh (sent to EDDN by other players). Values are Vista Genomics base values; a first logged species pays five times.</p>
    {/if}
  {/if}
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 14px 0 14px; }
  h3 { margin: 22px 0 8px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); }
  .tiles div { display: flex; flex-direction: column; gap: 4px; min-width: 11rem; padding: 10px 14px; background: var(--cell); }
  .tiles div.go { outline: 1px solid var(--accent); }
  .tiles div.go .v { color: var(--accent); }
  .k { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .v { color: var(--text-hi); font-family: var(--mono); font-size: var(--fs-l); }
  table { border-collapse: collapse; width: 100%; max-width: 60rem; margin-top: 6px; }
  th { text-align: left; color: var(--text-lo); font-size: var(--fs-s); font-weight: 400; text-transform: uppercase; letter-spacing: var(--caps); padding: 0 12px 6px 0; }
  td { padding: 5px 12px 5px 0; border-top: 1px solid var(--cell); vertical-align: baseline; }
  .num { text-align: right; font-family: var(--mono); white-space: nowrap; }
  .body { color: var(--text-hi); white-space: nowrap; }
  .genera { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 2px; }
  .genera li { display: flex; gap: 10px; align-items: baseline; flex-wrap: wrap; }
  .genus { color: var(--text-hi); }
  .guess .genus { color: var(--text); font-style: italic; }
  .lo { color: var(--text-lo); }
  .small { font-size: var(--fs-s); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
</style>
