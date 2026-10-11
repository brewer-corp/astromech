<script lang="ts">
  import type { Snippet } from "svelte";
  // The window frame, modelled on the game's Options screen: title row, one orange rule,
  // content, one orange rule, text actions. Navigation is plain text tabs.
  let { active, children, footer }: { active: string; children: Snippet; footer?: Snippet } = $props();
  const tabs = [
    ["activities", "Activities"],
    ["keybinds", "All keybinds"],
    ["controller", "My controller"],
    ["presets", "Presets"],
    ["settings", "Settings"],
    ["diagnostics", "Diagnostics"],
  ];
</script>

<div class="frame">
  <header>
    <span class="app">ASTROMECH</span>
    <nav>
      {#each tabs as [id, label] (id)}
        <a href="#/mockup/{id}" class:on={id === active}>{label}</a>
      {/each}
    </nav>
    <span class="state">Elite running · Voice ready</span>
  </header>
  <div class="body">{@render children()}</div>
  <footer>{@render footer?.()}</footer>
</div>

<style>
  .frame { height: 100vh; display: grid; grid-template-rows: auto 1fr auto; padding: 0 40px; }
  header { display: flex; align-items: baseline; gap: 40px; padding: 26px 0 12px; border-bottom: 1px solid var(--accent); }
  .app { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); }
  nav { display: flex; gap: 28px; flex: 1; }
  nav a { color: var(--text-lo); text-decoration: none; text-transform: uppercase; letter-spacing: var(--caps); font-weight: 500; }
  nav a.on { color: var(--accent); }
  .state { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .body { min-height: 0; overflow: hidden; }
  footer { display: flex; gap: 40px; align-items: center; min-height: 46px; border-top: 1px solid var(--accent); color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); font-weight: 600; }
</style>
