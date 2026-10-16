<script lang="ts">
  // The window frame (docs/V1-DESIGN.md §2.2, §4): title, plain text tabs for the three areas,
  // one orange rule, the area's pages as sub-tabs, the page, and a one-line core state. Opens on
  // My controller.
  import { groups, groupModules, findModule } from "./lib/modules";
  import { coreStatus, headerState, onRoute, type CoreStatus } from "./lib/core";

  const DEFAULT = "controller";
  // Pages that were merged into another keep working as links.
  const MOVED: Record<string, string> = { activities: "controller" };
  const resolve = (r: string) => MOVED[r] ?? (r || DEFAULT);
  const start = resolve(location.hash.replace(/^#\//, ""));
  let route = $state(start);
  if (location.hash && location.hash !== `#/${start}`) location.hash = `#/${start}`;
  const current = $derived(findModule(route) ?? findModule(DEFAULT));
  const pages = $derived(current ? groupModules(current.group) : []);
  let status = $state<CoreStatus | null>(null);
  const setup = $derived(status?.setup_needed ?? []);

  function go(id: string) {
    route = resolve(id);
    location.hash = `#/${route}`;
  }

  $effect(() => {
    onRoute((r) => go(r));
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
      {#each groups as g (g.id)}
        {@const first = groupModules(g.id)[0]}
        {#if first}
          <button class:on={g.id === current?.group} onclick={() => go(first.id)}>{g.title}</button>
        {/if}
      {/each}
    </nav>
    {#if setup.length && current?.id !== "settings"}
      <button class="setup" title={setup.join(", ")} onclick={() => go("settings")}>Finish setup</button>
    {/if}
    <span class="state">{headerState(status)}</span>
  </header>
  {#if pages.length > 1}
    <nav class="sub">
      {#each pages as m (m.id)}
        <button class:on={m.id === current?.id} title={m.summary} onclick={() => go(m.id)}>{m.title}</button>
      {/each}
    </nav>
  {/if}
  <main>
    {#if current}
      {#await current.load() then view}
        <view.default />
      {/await}
    {/if}
  </main>
</div>

<style>
  .frame { height: 100vh; display: grid; grid-template-rows: auto auto 1fr; }
  header { display: flex; align-items: baseline; gap: 40px; margin: 0 40px; padding: 26px 0 12px; border-bottom: 1px solid var(--accent); }
  .app { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); }
  nav { display: flex; gap: 28px; flex: 1; flex-wrap: wrap; }
  button { all: unset; cursor: pointer; color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 500; }
  button:hover { color: var(--text); }
  button.on { color: var(--accent); }
  nav.sub { flex: none; gap: 24px; margin: 0 40px; padding: 12px 0 0; }
  nav.sub button { font-size: var(--fs-s); font-weight: 600; padding-bottom: 4px; border-bottom: 2px solid transparent; }
  nav.sub button.on { color: var(--text-hi); border-bottom-color: var(--accent); }
  .setup { color: var(--accent); font-size: var(--fs-s); font-weight: 600; }
  .setup:hover { color: var(--text-hi); }
  .state { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); white-space: nowrap; }
  main { min-height: 0; overflow: auto; padding: 0 40px; scrollbar-gutter: stable; }
</style>
