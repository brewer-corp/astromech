<script lang="ts">
  // Per-ore prospector thresholds: a rock with this much of the ore or more is called out.
  // Empty turns the ore off (a motherlode is always called out).
  import Button from "../../lib/ui/Button.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import { loadThresholds, parseThreshold, setThreshold, type OreThreshold } from "../../lib/prospector";
  import { announcer } from "../../lib/ui/announce";

  let ores = $state<OreThreshold[]>([]);
  let error = $state("");
  const say = announcer((t) => (error = t));

  $effect(() => {
    loadThresholds()
      .then((o) => (ores = o))
      .catch((e: unknown) => say(String(e)));
  });

  /** `box`: the threshold box the change came from; it goes back to the saved value if refused. */
  async function change(o: OreThreshold, value: string | "reset", box?: HTMLInputElement) {
    const saved = String(o.threshold ?? "");
    const t = value === "reset" ? "reset" : parseThreshold(value);
    if (t === "invalid") {
      if (box) box.value = saved;
      void say(`${o.name}: enter a percentage from 0 to 100, or leave it empty to turn it off.`);
      return;
    }
    try {
      ores = await setThreshold(o.ore, t);
      error = "";
    } catch (e) {
      if (box) box.value = saved;
      await say(String(e));
    }
  }
</script>

<dl>
  {#each ores.filter((o) => o.default !== null || o.threshold !== null) as o (o.ore)}
    <dt class="type-name">{o.name}</dt>
    <dd>
      <label>
        <Input
          class="num"
          type="text"
          inputmode="decimal"
          aria-label={`${o.name} threshold, percent`}
          value={o.threshold ?? ""}
          placeholder="Off"
          onchange={(e) => change(o, e.currentTarget.value, e.currentTarget)}
        />
        <span class="type-label">% or more</span>
      </label>
      {#if o.threshold !== o.default}
        <Button onclick={() => change(o, "reset")}>Reset to {o.default ?? "off"}{o.default !== null ? "%" : ""}</Button>
      {/if}
    </dd>
  {/each}
</dl>
<!-- Always there, so a screen reader announces each error, repeats included. -->
<p class="hint type-label" class:warn-text={error} aria-live="polite">{error}</p>

<style>
  dl { display: grid; grid-template-columns: var(--form-label) minmax(0, 1fr); gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--options-grey); }
  /* The threshold is a control of its own, so the row has no cell around it. */
  dd { padding: 0; gap: var(--sp-3); }
  label { display: flex; align-items: center; gap: 8px; }
  label > :global(.num) { width: 4.5rem; }
  .hint { color: var(--prose); margin: 8px 0 0 calc(var(--form-label) + var(--gap)); }
  .hint:empty { margin: 0; }
  /* A failed save or a refused value: a red "!" and plain words under the field. */
  .warn-text { color: var(--alarm); }
  .warn-text::before { content: "! "; font-weight: 600; }
</style>
