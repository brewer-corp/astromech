<script lang="ts">
  import type { Snippet } from "svelte";
  import Status from "../design/Status.svelte";
  let { active, children }: { active: string; children: Snippet } = $props();
  const nav = [
    { id: "keybinds", label: "Keybinds", icon: "⌗" },
    { id: "voice", label: "Voice & controls", icon: "◉" },
    { id: "callouts", label: "Callouts", icon: "◬" },
    { id: "diagnostics", label: "Diagnostics", icon: "⚙" },
  ];
</script>

<div class="shell">
  <aside>
    <div class="brand">
      <span class="logo">◢◣</span>
      <div><div class="name">ASTROMECH</div><div class="ver am-caps">v1.0 · stable</div></div>
    </div>
    <nav>
      {#each nav as n (n.id)}
        <a class:active={n.id === active} class="am-caps" href="#/mockup/{n.id}"><span class="ic">{n.icon}</span>{n.label}</a>
      {/each}
    </nav>
    <div class="foot">
      <Status level="ok" text="Elite running" />
      <div class="ship am-caps">Krait Mk II · "Dusk Wren"</div>
      <div class="sys">Shinrarta Dezhra</div>
      <div class="arm am-caps"><span class="dot"></span>Actions armed</div>
    </div>
  </aside>
  <main>{@render children()}</main>
</div>

<style>
  .shell { display: grid; grid-template-columns: 236px 1fr; height: 100vh; background: var(--am-bg); }
  aside {
    display: flex; flex-direction: column; gap: 24px; padding: 20px 0;
    background: linear-gradient(90deg, var(--am-bg-2), var(--am-bg));
    border-right: 1px solid var(--am-orange-line);
  }
  .brand { display: flex; gap: 12px; align-items: center; padding: 0 20px; }
  .logo { color: var(--am-orange); font-size: 22px; text-shadow: var(--am-orange-glow); letter-spacing: -4px; }
  .name { font: 400 15px var(--am-font-display); color: var(--am-orange); letter-spacing: 0.18em; }
  .ver { font: 500 var(--am-size-xs) var(--am-font-ui); color: var(--am-muted); }
  nav { display: flex; flex-direction: column; }
  nav a {
    display: flex; gap: 12px; align-items: center;
    padding: 11px 20px; text-decoration: none;
    font: 600 var(--am-size-s) var(--am-font-ui); color: var(--am-muted);
    border-left: 3px solid transparent;
  }
  nav a:hover { color: var(--am-orange); }
  nav a.active { color: var(--am-bg); background: var(--am-orange); border-left-color: var(--am-orange-bright); }
  .ic { width: 16px; text-align: center; }
  .foot { margin-top: auto; padding: 0 20px; display: flex; flex-direction: column; gap: 6px; font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-text); }
  .ship { color: var(--am-orange); font-size: var(--am-size-xs); margin-top: 6px; }
  .sys { color: var(--am-muted); }
  .arm { display: flex; align-items: center; gap: 8px; font-size: var(--am-size-xs); color: var(--am-green); }
  .dot { width: 7px; height: 7px; background: var(--am-green); }
  main { overflow: hidden; padding: 22px 26px; display: flex; flex-direction: column; gap: 16px; }
</style>
