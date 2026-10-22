<script lang="ts">
  // Per-ore prospector thresholds: a rock with this much of the ore or more is called out.
  // Empty turns the ore off (a motherlode is always called out).
  import { loadThresholds, parseThreshold, setThreshold, type OreThreshold } from "../../lib/prospector";

  let ores = $state<OreThreshold[]>([]);
  let error = $state("");

  $effect(() => {
    loadThresholds()
      .then((o) => (ores = o))
      .catch((e: unknown) => (error = String(e)));
  });

  /** `box`: the threshold box the change came from; it goes back to the saved value if refused. */
  async function change(o: OreThreshold, value: string | "reset", box?: HTMLInputElement) {
    const saved = String(o.threshold ?? "");
    const t = value === "reset" ? "reset" : parseThreshold(value);
    if (t === "invalid") {
      error = `${o.name}: enter a percentage from 0 to 100, or leave it empty to turn it off.`;
      if (box) box.value = saved;
      return;
    }
    try {
      ores = await setThreshold(o.ore, t);
      error = "";
    } catch (e) {
      error = String(e);
      if (box) box.value = saved;
    }
  }
</script>

<dl>
  {#each ores.filter((o) => o.default !== null || o.threshold !== null) as o (o.ore)}
    <dt>{o.name}</dt>
    <dd>
      <label>
        <input
          type="text"
          inputmode="decimal"
          aria-label={`${o.name} threshold, percent`}
          value={o.threshold ?? ""}
          placeholder="off"
          onchange={(e) => change(o, e.currentTarget.value, e.currentTarget)}
        />
        <span class="lo">% or more</span>
      </label>
      {#if o.threshold !== o.default}
        <button onclick={() => change(o, "reset")}>Reset to {o.default ?? "off"}{o.default !== null ? "%" : ""}</button>
      {/if}
    </dd>
  {/each}
</dl>
<!-- Always there, so a screen reader announces each new error. -->
<p class="hint warn-text" aria-live="polite">{error}</p>

<style>
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--text-lo); }
  dd { background: var(--cell-opt); color: var(--text); gap: 24px; justify-content: space-between; }
  label { display: flex; align-items: center; gap: 8px; }
  input { width: 4rem; background: var(--bg); color: var(--text-hi); border: 1px solid var(--accent-dim); padding: 4px 6px; font: inherit; font-family: var(--mono); }
  .lo { color: var(--text-lo); font-size: var(--fs-s); }
  .hint { color: var(--text-lo); margin: 8px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); line-height: 1.6; }
  .hint:empty { margin: 0; }
  .warn-text { color: var(--accent); }
  button { all: unset; cursor: pointer; flex: none; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:hover, button:focus-visible { color: var(--text-hi); }
  button:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
</style>
