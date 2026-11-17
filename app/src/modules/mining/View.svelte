<script lang="ts">
  // The current mining run: what the refinery has made since the last dock, by ore, and the rate.
  // A run starts at the first limpet, prospect or refine after docking or loading the game.
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
  import Stat from "../../lib/ui/Stat.svelte";
  import { loadMining, oreName, type MiningRun } from "../../lib/mining";
  import Board from "../../lib/ui/layout/Board.svelte";

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

<Board>
  <Stale error={run ? failed : null} />
  {#if !loaded}
    <p class="type-label sub">Reading the run…</p>
  {:else if failed && !run}
    <p class="type-label sub err" role="alert">{failed}</p>
  {:else if !run}
    <p class="type-label sub">No mining run yet.</p>
  {:else if !run.started}
    <p class="type-label sub" {@attach tip("The run starts at your first limpet, prospect or refined ton after docking")}>No mining yet.</p>
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
            <tr><td class="type-name">{oreName(ore)}</td><td class="type-figures num ranked">{tons} t</td></tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="type-label">Nothing refined yet.</p>
    {/if}
    <p class="type-label note" {@attach tip("Docking ends the run")}>Since {run.started.replace("T", " ").replace(/:\d\dZ$/, "")} UTC</p>
  {/if}
</Board>

<style>
  p { margin: 0; }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); }
  h3 { margin: var(--sp-8) 0 var(--sp-3); }
  table { border-collapse: collapse; }
  td { padding: var(--sp-1) var(--sp-4) var(--sp-1) 0; }
  .num { text-align: right; }
  /* Ores are ranked by tonnes: it is the table's one orange column. */
  .ranked { color: var(--accent); font-weight: 500; }
  .note { margin-top: var(--sp-3); }
</style>
