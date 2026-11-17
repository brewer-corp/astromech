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
  .row { display: grid; grid-template-columns: 10rem 1fr; align-items: center; margin-top: var(--sp-3); }
  .err { margin: var(--sp-2) 0 0; padding-left: 10.75rem; color: var(--alarm); }
  b { font-weight: 600; }
  .lbl { padding-left: 2.125rem; color: var(--options-grey); }
  .compact { grid-template-columns: auto 1fr; gap: var(--sp-3); margin: 0; min-width: 15rem; }
  .compact .lbl { padding-left: 0; }
</style>
