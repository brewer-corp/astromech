<script lang="ts">
  // The window frame (docs/V1-DESIGN.md §2.2, §4): title, six plain text tabs, one orange rule,
  // the page, and a one-line core state. Opens on Activities.
  import { modules, findModule } from "./lib/modules";
  import { coreStatus, headerState, onRoute, type CoreStatus } from "./lib/core";

  const DEFAULT = "activities";
  let route = $state(location.hash.replace(/^#\//, "") || DEFAULT);
  const current = $derived(findModule(route) ?? findModule(DEFAULT));
  let status = $state<CoreStatus | null>(null);

  function go(id: string) {
    route = id;
    location.hash = `#/${id}`;
  }

  $effect(() => {
    onRoute((r) => go(r || DEFAULT));
    const tick = async () => (status = await coreStatus());
    void tick();
    const t = setInterval(() => void tick(), 2000);
    return () => clearInterval(t);
  });
</script>

<div class="frame">
  <header>
    <span class="app">ASTROMECH</span>
    <nav>
      {#each modules as m (m.id)}
        <button class:on={m.id === current?.id} title={m.summary} onclick={() => go(m.id)}>{m.title}</button>
      {/each}
    </nav>
    <span class="state">{headerState(status)}</span>
  </header>
  <main>
    {#if current}
      {#await current.load() then view}
        <view.default />
      {/await}
    {/if}
  </main>
</div>

<style>
  .frame { height: 100vh; display: grid; grid-template-rows: auto 1fr; padding: 0 40px; }
  header { display: flex; align-items: baseline; gap: 40px; padding: 26px 0 12px; border-bottom: 1px solid var(--accent); }
  .app { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); }
  nav { display: flex; gap: 28px; flex: 1; flex-wrap: wrap; }
  nav button { all: unset; cursor: pointer; color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 500; }
  nav button:hover { color: var(--text); }
  nav button.on { color: var(--accent); }
  .state { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); white-space: nowrap; }
  main { min-height: 0; overflow: auto; }
</style>
