<script lang="ts">
  // The current mining run: what the refinery has made since the last dock, by ore, and the rate.
  // A run starts at the first limpet, prospect or refine after docking or loading the game.
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
  import Stat from "../../lib/ui/Stat.svelte";
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
          failed = pollFailure("read the mining run", e, run !== null);
          loaded = true;
        });
    void tick();
    const t = setInterval(() => void tick(), 3000);
    return () => clearInterval(t);
  });

  const ores = $derived(Object.entries(run?.refined ?? {}).sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0])));
</script>

<section>
  <Stale error={run ? failed : null} />
  {#if !loaded}
    <p class="type-label sub">Reading the run…</p>
  {:else if failed && !run}
    <p class="type-label sub err" role="alert">{failed}</p>
  {:else if !run}
    <p class="type-label sub">No mining run yet.</p>
  {:else if !run.started}
    <p class="type-label sub">No mining yet. The run starts at your first limpet, prospect or refined ton after docking.</p>
  {:else}
    <div class="tiles">
      <Stat main label="Refined" value={run.total} unit="t" />
      <Stat label="Per hour" value={run.per_hour === null ? "–" : run.per_hour.toFixed(0)} unit={run.per_hour === null ? "" : "t/h"} {@attach tip("Measured to the latest journal line; shows once a minute has passed")} />
    </div>
    <h3 class="section">By ore</h3>
    {#if ores.length}
      <table aria-label="Tons refined by ore">
        <tbody>
          {#each ores as [ore, tons] (ore)}
            <tr><td>{oreName(ore)}</td><td class="num ranked">{tons} t</td></tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="lo">Nothing refined yet.</p>
    {/if}
    <p class="type-label note" {@attach tip("Docking ends the run")}>Since {run.started.replace("T", " ").replace(/:\d\dZ$/, "")} UTC</p>
  {/if}
</section>

<style>
  section { padding: 0.5rem 0 1.75rem; min-height: 0; }
  .sub { margin: 0.875rem 0; }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); margin-top: 0.875rem; }
  h3 { margin: 1.375rem 0 0.5rem; }
  table { border-collapse: collapse; }
  td { padding: 0.25rem 1.125rem 0.25rem 0; }
  .num { text-align: right; }
  /* Ores are ranked by tonnes: it is the table's one orange column. */
  .ranked { color: var(--accent); font-weight: 500; }
  .lo { color: var(--label-grey); }
  .note { margin-top: 0.875rem; }
</style>
