<script lang="ts">
  // The window frame (docs/V1-DESIGN.md §2.2, §4; DESIGN.md Frame): title, plain text tabs for
  // the areas, one orange rule, the route strip, the area's pages as sub-tabs, the page, and the
  // status bar (the ship and the core's one-line state). Opens on My controller. Onboarding replaces the frame on first launch (until finished or skipped) and
  // at #/setup or #/setup/<step>. The header's Find or plot box (PlotBar) finds a page, finder,
  // material, module or ship, or plots to a system, from any page; Ctrl+K puts the cursor in it.
  import { tip } from "./lib/ui/tip";
  import { groups, groupModules, findModule, modules } from "./lib/modules";
  import { MOVED_FINDERS } from "./lib/places";
  import { collect } from "./lib/palette";
  import Button from "./lib/ui/Button.svelte";
  import { coreStatus, onRoute, shipStatus, type CoreStatus, type ShipStatus } from "./lib/core";
  import StatusBar from "./lib/ui/StatusBar.svelte";
  import Toasts from "./lib/ui/Toasts.svelte";
  import Onboarding from "./lib/onboarding/Onboarding.svelte";
  import { openHelp } from "./lib/help";
  import PlotBar from "./modules/route/PlotBar.svelte";
  import Strip from "./modules/route/Strip.svelte";
  import { startScale } from "./lib/scale.svelte";

  const DEFAULT = "controller";
  // Pages that were merged into another keep working as links: the route's first part moves,
  // and what followed it stays ("services/black_market" opens "places/services/black_market").
  const MOVED: Record<string, string> = { activities: "controller", ...MOVED_FINDERS };
  const moved = (r: string) => {
    const [, head = "", rest = ""] = /^([^/?]*)(.*)$/.exec(r) ?? [];
    return Object.hasOwn(MOVED, head) ? MOVED[head] + rest : null;
  };
  const resolve = (r: string) => moved(r) ?? (r || DEFAULT);
  const fromHash = () => resolve(location.hash.replace(/^#\//, ""));
  const start = fromHash();
  let route = $state(start);
  // A moved link opened from outside (a bookmark) is rewritten in place, as one followed in the
  // window is, so Back doesn't land on the old address.
  if (location.hash && location.hash !== `#/${start}`) history.replaceState(null, "", `#/${start}`);
  // A page may route within itself ("engineering/7" opens that ship) or keep its choices in a
  // query ("materials?kind=encoded", lib/hashstate.ts): the module is the first part.
  const current = $derived(findModule(route.split(/[/?]/)[0] ?? "") ?? findModule(DEFAULT));
  const pages = $derived(current ? groupModules(current.group) : []);
  let status = $state<CoreStatus | null>(null);
  let ship = $state<ShipStatus | null>(null);
  const setup = $derived(status?.setup_needed ?? []);
  const showSetup = $derived(setup.length > 0 && current?.id !== "settings");

  // DESIGN.md Frame: one 64 px header row while the name, tabs, Find or plot and Help fit on it
  // (with Finish setup while setup is unfinished); in a narrower window the tabs take a 44 px row
  // of their own under the name.
  let header = $state<HTMLElement>();
  let stacked = $state(false);
  /** The header's width on one row: each part at its own width (the tabs at their buttons', Find
   * or plot at its full 320 px even while it has given way). */
  function oneRow(h: HTMLElement): number {
    const gap = (el: Element) => parseFloat(getComputedStyle(el).columnGap) || 0;
    const parts = [...h.children] as HTMLElement[];
    const width = (el: HTMLElement) => {
      if (el.tagName !== "NAV") return Math.max(el.offsetWidth, parseFloat(getComputedStyle(el).flexBasis) || 0);
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
    // An old link followed in the window is rewritten in place before the page reads it
    // (this listener is added before any page's).
    const hash = () => {
      const raw = location.hash.replace(/^#\//, "");
      const to = moved(raw);
      if (to) history.replaceState(null, "", `#/${to}`);
      route = to ?? resolve(raw);
    };
    window.addEventListener("hashchange", hash);
    return () => window.removeEventListener("hashchange", hash);
  });

  const groupTitle = (id: string) => groups.find((g) => g.id === id)?.title ?? id;
  const paletteItems = () => collect(modules, groupTitle);

  async function finishSetup() {
    status = await coreStatus();
    go(DEFAULT);
  }

  // DESIGN.md The Text Size Rule: the saved Text size, onboarding included. Asked again on each
  // status poll until the core has answered (it may start after the window).
  $effect(() => {
    void status;
    void startScale();
  });

  $effect(() => {
    onRoute((r) => go(r));
    // The status bar's fuel and cargo follow the game's Status.json writes on the same tick.
    const tick = async () => {
      [status, ship] = await Promise.all([coreStatus(), shipStatus()]);
    };
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
    <PlotBar load={paletteItems} onpick={go} />
    {#if showSetup}
      <Button onclick={() => go("setup/health")} {@attach tip(setup.join(", "))}>Finish setup</Button>
    {/if}
    {#if current}
      <button class="help" onclick={() => void openHelp(current.help)} {@attach tip("Open this page's guide in your browser")}>Help</button>
    {/if}
  </header>
  <Strip />
  {#if pages.length > 1}
    <nav class="sub">
      {#each pages as m (m.id)}
        <button class:on={m.id === current?.id} onclick={() => go(m.id)} {@attach tip(m.summary)}>{m.title}</button>
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
  <StatusBar {status} {ship} />
  <Toasts {status} />
</div>
{/if}

<style>
  /* DESIGN.md Frame: the fixed parts, then the page, which takes the rest and scrolls. The frame is
     the container a page's narrow-window rules ask (`@container win`, in rem): a container query's
     rem is the root font size, so a breakpoint moves with Text size, where a media query's rem
     is the browser's default. */
  .frame { container: win / inline-size; height: 100vh; display: flex; flex-direction: column; }
  .frame > :global(*) { flex: none; }
  .frame > main { flex: 1 1 0; }
  /* DESIGN.md Frame: a 64 px row with the 1 px orange rule under it; stacked, the tabs add 44 px. */
  header { box-sizing: border-box; display: flex; align-items: center; gap: clamp(var(--sp-4), 2.5vw, var(--gutter)); height: 4rem; margin: 0 var(--side); border-bottom: 1px solid var(--accent); }
  .app { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); }
  nav { display: flex; gap: var(--sp-1); flex: 1; flex-wrap: wrap; }
  header nav { flex-wrap: nowrap; align-items: center; }
  button { all: unset; cursor: pointer; }
  /* DESIGN.md Tabs: top tabs are Nav caps in Label Grey at 400, the active one a solid orange box
     with On Orange text at 500 (the frame's one fill); page tabs are Name in mixed case, Label
     Grey, the active one Options Grey on a 2 px orange underline. An inactive tab hovers to Orange. */
  header nav button { display: flex; align-items: center; height: var(--row); padding: 0 var(--sp-3); color: var(--label-grey); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 400; }
  header nav button:hover { color: var(--accent); }
  header nav button.on { background: var(--accent); color: var(--on-accent); font-weight: 500; }
  header nav button.on:focus-visible { outline: 1px solid var(--on-accent); outline-offset: -3px; }
  nav.sub { flex: none; gap: var(--sp-6); height: 2.75rem; margin: 0 var(--side); }
  nav.sub button { display: flex; align-items: center; font-weight: 500; color: var(--label-grey); border-bottom: 2px solid transparent; }
  nav.sub button:hover { color: var(--accent); }
  nav.sub button.on { color: var(--options-grey); border-bottom-color: var(--accent); }
  /* A link to somewhere else (DESIGN.md Buttons → Links): Orange text, a 1 px Orange Mid underline. */
  .help { color: var(--accent); font-size: var(--fs-s); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .help:hover { color: var(--orange-lit); }
  main { min-height: 0; overflow: auto; padding: 0 var(--side); scrollbar-gutter: stable; }
  header.stacked { height: 6.75rem; flex-wrap: wrap; align-content: flex-start; row-gap: 0; }
  header.stacked .app { display: flex; align-items: center; height: 3.9375rem; margin-right: auto; }
  header.stacked nav { order: 1; flex-basis: 100%; align-items: center; height: 2.75rem; }
  /* The header's gaps scale with the window only (no breakpoint), so the width one row needs grows
     steadily with it: the header stacks once as the window narrows and stays stacked below that. */
  @container win (max-width: 68.75rem) {
    nav.sub { gap: var(--sp-4); }
  }
</style>
