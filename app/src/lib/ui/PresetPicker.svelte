<script lang="ts">
  // Which preset the keybinds screens show. Defaults to the one the game uses for the ship.
  import Dropdown from "./Dropdown.svelte";
  import { binds } from "../store.svelte";
  let { compact = false }: { compact?: boolean } = $props();
  const active = (ctx: string[]) => (ctx.length === 4 ? "active everywhere" : ctx.length ? `active: ${ctx.join(", ").toLowerCase()}` : "not active");
</script>

<label class="row" class:compact>
  <span class="lbl">Preset</span>
  <Dropdown value={binds.view?.path ?? ""} onchange={(e) => void binds.show(e.currentTarget.value)}>
    {#if binds.state && !binds.state.presets.length}<option value="">No presets found</option>{/if}
    {#each binds.state?.presets ?? [] as p (p.path)}
      <option value={p.path}>{p.name}{p.source === "built-in" ? " (built-in)" : ""} · {active(p.active_for)}</option>
    {/each}
  </Dropdown>
</label>
{#if binds.error}<p class="err" role="alert"><b>!</b> {binds.error}</p>{/if}

<style>
  .row { display: grid; grid-template-columns: 160px 1fr; align-items: center; margin-top: 12px; }
  .err { margin: 8px 0 0; padding-left: 172px; color: var(--alarm); }
  b { font-weight: 600; }
  .lbl { padding-left: 34px; color: var(--options-grey); }
  .compact { grid-template-columns: auto 1fr; gap: 12px; margin: 0; min-width: 16rem; }
  .compact .lbl { padding-left: 0; }
</style>
