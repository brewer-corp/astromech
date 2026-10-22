<script lang="ts">
  // The window frame (docs/V1-DESIGN.md §2.2, §4): title, plain text tabs for the three areas,
  // one orange rule, the area's pages as sub-tabs, the page, and a one-line core state. Opens on
  // My controller. Onboarding replaces the frame on first launch (until finished or skipped) and
  // at #/setup or #/setup/<step>. Ctrl+K opens the command palette from any page.
  import { groups, groupModules, findModule, modules } from "./lib/modules";
  import { collect } from "./lib/palette";
  import Palette from "./lib/ui/Palette.svelte";
  import { coreStatus, headerState, onRoute, type CoreStatus } from "./lib/core";
  import Toasts from "./lib/ui/Toasts.svelte";
  import Onboarding from "./lib/onboarding/Onboarding.svelte";
  import { openHelp } from "./lib/help";
  import Rail from "./modules/route/Rail.svelte";

  const DEFAULT = "controller";
  // Pages that were merged into another keep working as links.
  const MOVED: Record<string, string> = { activities: "controller" };
  const resolve = (r: string) => MOVED[r] ?? (r || DEFAULT);
  const fromHash = () => resolve(location.hash.replace(/^#\//, ""));
  const start = fromHash();
  let route = $state(start);
  if (location.hash && location.hash !== `#/${start}`) location.hash = `#/${start}`;
  // A page may route within itself ("engineering/7" opens that ship): the module is the first part.
  const current = $derived(findModule(route.split("/")[0] ?? "") ?? findModule(DEFAULT));
  const pages = $derived(current && current.group !== "galaxy" ? groupModules(current.group) : []);
  const galaxyFind = $derived(current?.group === "galaxy" && current.id !== "route");
  let status = $state<CoreStatus | null>(null);
  const setup = $derived(status?.setup_needed ?? []);
  const showSetup = $derived(setup.length > 0 && current?.id !== "settings");

  // The tabs share a row with the name and status while all of it fits (the status runs from "Not
  // running" to "Waiting for Elite · Voice paused · Kill switch on"); in a narrower window they
  // take a row of their own under the name.
  let header = $state<HTMLElement>();
  let stacked = $state(false);
  /** The header's width on one row: each part at its own width (the tabs at their buttons'). */
  function oneRow(h: HTMLElement): number {
    const gap = (el: Element) => parseFloat(getComputedStyle(el).columnGap) || 0;
    const parts = [...h.children] as HTMLElement[];
    const width = (el: HTMLElement) => {
      if (el.tagName !== "NAV") return el.offsetWidth;
      const tabs = [...el.children] as HTMLElement[];
      return tabs.reduce((w, t) => w + t.offsetWidth, 0) + gap(el) * Math.max(0, tabs.length - 1);
    };
    return parts.reduce((w, p) => w + width(p), 0) + gap(h) * Math.max(0, parts.length - 1);
  }
  $effect(() => {
    const h = header;
    void showSetup; // the Finish setup button comes and goes: observe the parts there are now
    if (!h || typeof ResizeObserver === "undefined") return;
    const ro = new ResizeObserver(() => (stacked = oneRow(h) > h.clientWidth));
    ro.observe(h);
    for (const p of h.children) ro.observe(p);
    return () => ro.disconnect();
  });
  const onboarding = $derived(route === "setup" || route.startsWith("setup/") || status?.onboarded === false);

  function go(id: string) {
    route = resolve(id);
    location.hash = `#/${route}`;
  }

  $effect(() => {
    const hash = () => (route = fromHash());
    window.addEventListener("hashchange", hash);
    return () => window.removeEventListener("hashchange", hash);
  });

  const groupTitle = (id: string) => groups.find((g) => g.id === id)?.title ?? id;
  const paletteItems = () => collect(modules, groupTitle);

  async function finishSetup() {
    status = await coreStatus();
    go(DEFAULT);
  }

  $effect(() => {
    onRoute((r) => go(r));
    const tick = async () => (status = await coreStatus());
    void tick();
    const t = setInterval(() => void tick(), 2000);
    return () => clearInterval(t);
  });
</script>

{#if onboarding}
  <Onboarding start={route.split("/")[1]} onfinish={finishSetup} />
{:else}
<div class="frame">
  <header bind:this={header} class:stacked>
    <span class="app">ASTROMECH</span>
    <nav>
      {#each groups as g (g.id)}
        {@const first = groupModules(g.id)[0]}
        {#if first}
          <button class:on={g.id === current?.group} onclick={() => go(first.id)}>{g.title}</button>
        {/if}
      {/each}
    </nav>
    {#if showSetup}
      <button class="setup" title={setup.join(", ")} onclick={() => go("setup/health")}>Finish setup</button>
    {/if}
    <span class="right">
      <span class="state" class:warn={!status || status.game === "not_running"}>{headerState(status)}</span>
      {#if current}
        <button class="help" title="Open this page's guide in your browser" onclick={() => void openHelp(current.help)}>Help</button>
      {/if}
    </span>
  </header>
  {#if pages.length > 1}
    <nav class="sub">
      {#each pages as m (m.id)}
        <button class:on={m.id === current?.id} title={m.summary} onclick={() => go(m.id)}>{m.title}</button>
      {/each}
    </nav>
  {/if}
  <main class:split={galaxyFind}>
    {#if current}
      {#if galaxyFind}
        <div class="find">
          {#await current.load() then view}
            <view.default />
          {/await}
        </div>
        <aside class="rail" aria-label="Route">
          <Rail />
        </aside>
      {:else}
        {#await current.load() then view}
          <view.default />
        {/await}
      {/if}
    {/if}
  </main>
  <Toasts {status} />
</div>
<Palette load={paletteItems} onpick={go} />
{/if}

<style>
  .frame { height: 100vh; display: grid; grid-template-rows: auto auto 1fr; grid-template-columns: minmax(0, 1fr); }
  header { display: flex; align-items: center; gap: clamp(16px, 2.5vw, 40px); margin: 0 clamp(16px, 3vw, 40px); padding: 26px 0 12px; border-bottom: 1px solid var(--accent); }
  .app { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); }
  nav { display: flex; gap: clamp(16px, 2.5vw, 28px); flex: 1; flex-wrap: wrap; }
  header nav { flex-wrap: nowrap; }
  button { all: unset; cursor: pointer; color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 500; }
  button:hover { color: var(--text); }
  button.on { color: var(--text-hi); }
  nav.sub { flex: none; gap: 24px; margin: 0 clamp(16px, 3vw, 40px); padding: 12px 0 0; }
  nav.sub button { font-size: var(--fs-s); font-weight: 600; padding-bottom: 4px; border-bottom: 2px solid transparent; }
  nav.sub button.on { color: var(--text-hi); border-bottom-color: var(--accent); }
  .setup { color: var(--accent); font-size: var(--fs-s); font-weight: 600; white-space: nowrap; }
  .setup:hover { color: var(--text-hi); }
  .help { font-size: var(--fs-s); font-weight: 600; text-decoration: underline; text-underline-offset: 3px; }
  .right { display: flex; align-items: baseline; gap: 16px; }
  .state { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); white-space: nowrap; }
  .state.warn { color: var(--accent); font-weight: 600; }
  main { min-height: 0; overflow: auto; padding: 0 clamp(16px, 3vw, 40px); scrollbar-gutter: stable; }
  main.split { display: grid; grid-template-columns: minmax(0, 1fr) minmax(18rem, 24rem); gap: 0; overflow: hidden; padding-right: 0; }
  .find { min-width: 0; overflow: auto; padding: 20px 24px 40px 0; scrollbar-gutter: stable; }
  .rail { min-width: 0; overflow: auto; padding: 20px clamp(16px, 3vw, 28px) 40px 24px; border-left: 1px solid var(--accent-dim); background: var(--bg-list); }
  header.stacked { flex-wrap: wrap; row-gap: 14px; }
  header.stacked .app { margin-right: auto; }
  header.stacked nav { order: 1; flex-basis: 100%; }
  @media (max-width: 1100px) {
    header { gap: 16px; }
    nav { gap: 14px; }
    nav.sub { gap: 16px; }
    main.split { display: flex; flex-direction: column; overflow: auto; padding-right: clamp(16px, 3vw, 40px); }
    .find { padding: 20px 0 24px; overflow: visible; flex: none; }
    .rail { border-left: 0; border-top: 1px solid var(--accent-dim); padding: 16px 0 20px; max-height: 38vh; overflow: auto; position: sticky; bottom: 0; z-index: 2; }
  }
</style>
