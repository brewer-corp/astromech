<script lang="ts">
  // Each callout's switch and cooldown, written to [callouts.<id>] in astromech.toml by the core.
  // A change applies from the next session. The cooldown is how long a callout stays quiet after
  // it speaks, so a burst of the same event gives one line.
  import { changeCallout, loadCallouts, parseCooldown, type CalloutSetting } from "../../lib/callouts";

  let list = $state<CalloutSetting[]>([]);
  let note = $state("");
  let error = $state("");

  async function reload() {
    list = await loadCallouts();
  }
  $effect(() => {
    reload().catch((e: unknown) => (error = String(e)));
  });

  async function change(c: CalloutSetting, what: { enabled: boolean } | { cooldown_s: number | null }) {
    try {
      await changeCallout(c.id, what);
      await reload();
      error = "";
      note = "Saved. It applies the next time Elite starts.";
    } catch (e) {
      error = `${c.title}: ${String(e)}`;
    }
  }

  function setCooldown(c: CalloutSetting, value: string) {
    const s = parseCooldown(value);
    if (s === "invalid") {
      error = `${c.title}: enter a number of seconds from 0 to 86400.`;
      return;
    }
    void change(c, { cooldown_s: s });
  }
</script>

<dl>
  {#each list as c (c.id)}
    <dt>{c.title}</dt>
    <dd>
      <button class:on={c.enabled} aria-pressed={c.enabled} aria-label={`${c.title} on or off`} onclick={() => change(c, { enabled: !c.enabled })}>
        {c.enabled ? "On" : "Off"}
      </button>
      <label>
        <input
          type="text"
          inputmode="decimal"
          aria-label={`${c.title} cooldown, seconds`}
          value={c.cooldown_s}
          onchange={(e) => setCooldown(c, e.currentTarget.value)}
        />
        <span class="lo">s quiet after it speaks</span>
      </label>
      {#if c.cooldown_s !== c.default_cooldown_s}
        <button onclick={() => change(c, { cooldown_s: null })}>Reset to {c.default_cooldown_s} s</button>
      {/if}
    </dd>
  {/each}
</dl>
{#if error}<p class="hint warn-text">{error}</p>{:else if note}<p class="hint">{note}</p>{/if}

<style>
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--text-lo); }
  dd { background: var(--cell-opt); color: var(--text); gap: 24px; justify-content: space-between; }
  label { display: flex; align-items: center; gap: 8px; margin-left: auto; }
  input { width: 4rem; background: var(--bg); color: var(--text-hi); border: 1px solid var(--accent-dim); padding: 4px 6px; font: inherit; font-family: var(--mono); }
  .lo { color: var(--text-lo); font-size: var(--fs-s); }
  .hint { color: var(--text-lo); margin: 8px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); line-height: 1.6; }
  .warn-text { color: var(--accent); }
  button { all: unset; cursor: pointer; flex: none; color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); min-width: 2rem; }
  button.on { color: var(--accent); }
  button:hover { color: var(--text-hi); }
</style>
