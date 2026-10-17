<script lang="ts">
  // The current mining run: what the refinery has made since the last dock, by ore, and the rate.
  // A run starts at the first limpet, prospect or refine after docking or loading the game.
  import { failure } from "../../lib/ui/errors";
  import { loadMining, oreName, type MiningRun } from "../../lib/mining";

  let run = $state<MiningRun | null>(null);
  let loaded = $state(false);
  let failed = $state<string | null>(null);

  $effect(() => {
    const tick = () =>
      loadMining()
        .then((v) => {
          run = v;
          failed = null;
          loaded = true;
        })
        .catch((e: unknown) => {
          failed = failure("read the mining run", e);
          loaded = true;
        });
    void tick();
    const t = setInterval(() => void tick(), 3000);
    return () => clearInterval(t);
  });

  const ores = $derived(Object.entries(run?.refined ?? {}).sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0])));
</script>

<section>
  {#if !loaded}
    <p class="sub">Reading the run…</p>
  {:else if failed && !run}
    <p class="sub">{failed}</p>
  {:else if !run}
    <p class="sub">The core isn't running, so there is no mining run to show.</p>
  {:else if !run.started}
    <p class="sub">No mining yet. The run starts at your first limpet, prospect or refined ton after docking.</p>
  {:else}
    <div class="tiles">
      <div><span class="k">Refined</span><span class="v">{run.total} t</span></div>
      <div><span class="k">Per hour</span><span class="v">{run.per_hour === null ? "–" : `${run.per_hour.toFixed(0)} t/h`}</span></div>
    </div>
    <h3>By ore</h3>
    {#if ores.length}
      <table aria-label="Tons refined by ore">
        <tbody>
          {#each ores as [ore, tons] (ore)}
            <tr><td>{oreName(ore)}</td><td class="num">{tons} t</td></tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="lo">Nothing refined yet.</p>
    {/if}
    <p class="lo note">Since {run.started.replace("T", " ").replace(/:\d\dZ$/, "")} UTC, measured to the latest journal line. The rate shows once a minute has passed. Docking ends the run.</p>
  {/if}
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 14px 0; }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); margin-top: 14px; }
  .tiles div { display: flex; flex-direction: column; gap: 4px; min-width: 11rem; padding: 10px 14px; background: var(--cell); }
  .k { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .v { color: var(--text-hi); font-family: var(--mono); font-size: var(--fs-l); }
  h3 { margin: 22px 0 8px; }
  table { border-collapse: collapse; }
  td { padding: 4px 18px 4px 0; }
  .num { text-align: right; font-family: var(--mono); color: var(--text-hi); }
  .lo { color: var(--text-lo); }
  .note { font-size: var(--fs-s); margin-top: 14px; }
</style>
