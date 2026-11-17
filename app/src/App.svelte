<script lang="ts">
  // The window frame (docs/V1-DESIGN.md §2.2, §4; DESIGN.md Frame): title, plain text tabs for
  // the areas and one orange rule across the window; under it the route sidebar on the left (or
  // the no-route rail), and the page column: the route strip where the sidebar is collapsed or the
  // window is below the dock width, the area's pages as sub-tabs, and the page; then the status
  // bar (the ship and the core's one-line state). There is no Route page: whatever opened it
  // (#/route, the palette, the tray) shows the sidebar where the commander is and keeps the page. Opens on Now: what the
  // commander is doing at the moment. Onboarding replaces the frame on first launch (until finished or skipped) and
  // at #/setup or #/setup/<step>. The header's Find or plot box (PlotBar) finds a page, finder,
  // material, module or ship, or plots to a system, from any page; Ctrl+K puts the cursor in it.
  import { tip } from "./lib/ui/tip";
  import { groups, groupModules, findModule, modules } from "./lib/modules";
  import { MOVED_FINDERS } from "./lib/places";
  import { collect } from "./lib/palette";
  import Button from "./lib/ui/Button.svelte";
  import { coreStatus, onRoute } from "./lib/core";
  import { provideFrame } from "./lib/frame.svelte";
  import StatusBar from "./lib/ui/StatusBar.svelte";
  import Toasts from "./lib/ui/Toasts.svelte";
  import Onboarding from "./lib/onboarding/Onboarding.svelte";
  import { openHelp } from "./lib/help";
  import PlotBar from "./modules/route/PlotBar.svelte";
  import Strip from "./modules/route/Strip.svelte";
  import Sidebar from "./modules/route/Sidebar.svelte";
  import Rail from "./modules/route/Rail.svelte";
  import { provideFollow } from "./modules/route/follow.svelte";
  import { scale, startScale } from "./lib/scale.svelte";
  import { at100, DOCK } from "./lib/ui/layout/layout";

  const DEFAULT = "now";
  // Pages that were merged into another keep working as links: the route's first part moves,
  // and what followed it stays ("services/black_market" opens "places/services/black_market").
  const MOVED: Record<string, string> = { activities: "controller", ...MOVED_FINDERS };
  // On foot was split up (astromech-5a01): each of its views went to a different page, so its
  // links go by the view and drop the rest.
  const ON_FOOT: Record<string, string> = { blueprints: "suits", engineers: "unlocks?kind=suit" };
  const moved = (r: string) => {
    const [, head = "", rest = ""] = /^([^/?]*)(.*)$/.exec(r) ?? [];
    if (head === "odyssey") return ON_FOOT[/[?&]view=([^&]*)/.exec(rest)?.[1] ?? ""] ?? "materials?kind=onfoot";
    return Object.hasOwn(MOVED, head) ? MOVED[head] + rest : null;
  };
  const resolve = (r: string) => moved(r) ?? (r || DEFAULT);
  /** The route sidebar's own address, kept from when it was a page: it opens the sidebar. */
  const isRoute = (r: string) => /^route(?:[/?]|$)/.test(r);
  const raw = location.hash.replace(/^#\//, "");
  /** Opened at #/route (a bookmark, an old link): the default page, with the sidebar shown once
   * the frame knows whether it is docked, which needs its width and the saved Text size. */
  let showOnStart = $state(isRoute(raw));
  /** The saved Text size is read and drawn: docked or not is settled. */
  let sized = $state(false);
  const start = isRoute(raw) ? DEFAULT : resolve(raw);
  let route = $state(start);
  // A moved link opened from outside (a bookmark) is rewritten in place, as one followed in the
  // window is, so Back doesn't land on the old address.
  if (location.hash && location.hash !== `#/${start}`) history.replaceState(null, "", `#/${start}`);
  // A page may route within itself ("engineering/7" opens that ship) or keep its choices in a
  // query ("materials?kind=encoded", lib/hashstate.ts): the module is the first part.
  const current = $derived(findModule(route.split(/[/?]/)[0] ?? "") ?? findModule(DEFAULT));
  const pages = $derived(current ? groupModules(current.group) : []);

  // DESIGN.md Route sidebar: one poll of the route for the strip and the sidebar. Docked at or
  // above the dock width while a route is followed (or one is paused to resume), unless folded;
  // the no-route rail there with neither, unless the sidebar was asked for (#/route, Route).
  // Below the dock width, the strip, whose chevron opens the sidebar over the page.
  const follow = provideFollow();
  $effect(() => follow.start());
  let width = $state(0);
  const isDocked = () => width > 0 && at100(width, scale.step) >= DOCK;
  $effect(() => {
    follow.docked = isDocked();
  });
  const any = $derived(!!follow.view?.route || !!follow.view?.parked);
  const docked = $derived(follow.docked && !follow.collapsed && (any || follow.asked));
  const rail = $derived(follow.docked && !!follow.view && !any && !follow.asked);
  // A route that comes ends the request: once it ends, the rail is back.
  $effect(() => {
    if (any) follow.asked = false;
  });
  const over = $derived(!follow.docked && follow.overlay);
  $effect(() => {
    if (width > 0 && sized && showOnStart) {
      showOnStart = false;
      // At the size just restored, whichever effect runs first.
      follow.docked = isDocked();
      follow.show();
    }
  });
  $effect(() => {
    if (follow.docked) follow.overlay = false;
  });
  let overlay = $state<HTMLElement>();
  // Open over the page, the sidebar closes on Escape (unless something inside it, a dropdown or
  // the destination's suggestions, took it first, or a dialog is open) and on a click outside it
  // (the strip's chevron toggles it itself).
  $effect(() => {
    if (!over) return;
    const key = (e: KeyboardEvent) => {
      if (e.key !== "Escape" || e.defaultPrevented || (e.target instanceof Element && e.target.closest("dialog"))) return;
      follow.overlay = false;
    };
    const down = (e: PointerEvent) => {
      const t = e.target instanceof Element ? e.target : null;
      if (!t || overlay?.contains(t) || t.closest(".fold, dialog, .tip")) return;
      follow.overlay = false;
    };
    window.addEventListener("keydown", key);
    window.addEventListener("pointerdown", down);
    return () => {
      window.removeEventListener("keydown", key);
      window.removeEventListener("pointerdown", down);
    };
  });
  // The core's status and the ship's, polled once for the frame and any page that reads them.
  const frame = provideFrame();
  const status = $derived(frame.status);
  const ship = $derived(frame.ship);
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
    const inner = () => {
      const cs = getComputedStyle(h);
      return h.clientWidth - parseFloat(cs.paddingLeft) - parseFloat(cs.paddingRight);
    };
    const ro = new ResizeObserver(() => (stacked = oneRow(h) > inner()));
    ro.observe(h);
    for (const p of h.children) ro.observe(p);
    return () => ro.disconnect();
  });
  const onboarding = $derived(route === "setup" || route.startsWith("setup/") || status?.onboarded === false);

  function go(id: string) {
    if (isRoute(id)) return follow.show();
    route = resolve(id);
    location.hash = `#/${route}`;
  }

  $effect(() => {
    // An old link followed in the window is rewritten in place before the page reads it
    // (this listener is added before any page's).
    const hash = () => {
      const raw = location.hash.replace(/^#\//, "");
      if (isRoute(raw)) {
        // The page stays: the address goes back to it.
        history.replaceState(null, "", `#/${route}`);
        return follow.show();
      }
      const to = moved(raw);
      if (to) history.replaceState(null, "", `#/${to}`);
      route = to ?? resolve(raw);
    };
    window.addEventListener("hashchange", hash);
    return () => window.removeEventListener("hashchange", hash);
  });

  const groupTitle = (id: string) => groups.find((g) => g.id === id)?.title ?? id;
  /** Every page, and the route, which opens the sidebar. */
  const paletteItems = async () => [
    ...(await collect(modules, groupTitle)),
    { label: "Route", detail: "The route sidebar", route: "route", keywords: "route followed waypoints next stop timeline import" },
  ];

  async function finishSetup() {
    frame.status = await coreStatus();
    go(DEFAULT);
  }

  // DESIGN.md The Text Size Rule: the saved Text size, onboarding included. Asked again on each
  // status poll until the core has answered (it may start after the window).
  $effect(() => {
    void status;
    // Only a read that restored the size settles it: a failed one is tried again on the next poll.
    void startScale().then((ok) => ok && (sized = true));
  });

  $effect(() => onRoute((r) => go(r)));
  $effect(() => frame.start());
</script>

{#if onboarding}
  <Onboarding start={route.split("/")[1]} onfinish={finishSetup} />
{:else}
<div class="frame" bind:clientWidth={width}>
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
  <div class="body">
    {#if docked}
      <div class="dock"><Sidebar onfold={() => follow.fold()} /></div>
    {:else if rail}
      <Rail />
    {/if}
    <div class="col">
      {#if !docked && !rail}<Strip />{/if}
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
      {#if over}
        <div class="scrim"></div>
        <div class="over" bind:this={overlay}><Sidebar over onfold={() => (follow.overlay = false)} /></div>
      {/if}
    </div>
  </div>
  <StatusBar {status} {ship} seam={docked ? "var(--sidebar)" : rail ? "var(--rail)" : "0px"} />
  <Toasts {status} />
</div>
{/if}

<style>
  /* DESIGN.md Frame: the fixed parts, then the body: the route sidebar or rail, and the page
     column, which takes the rest. The page column is the container a page's narrow-window rules
     ask (`@container win`, in rem), so they measure the page beside the sidebar: a container
     query's rem is the root font size, so a breakpoint moves with Text size, where a media
     query's rem is the browser's default. The header measures the window itself. */
  .frame { height: 100vh; display: flex; flex-direction: column; }
  .frame > :global(*) { flex: none; }
  .frame > .body { flex: 1 1 0; display: flex; min-height: 0; }
  .body > :global(*) { flex: none; }
  /* DESIGN.md Route sidebar: 320 px of Hull from the header's rule to the status bar, with a Seam
     rule on its right, the page's edge. */
  .dock { width: var(--sidebar); min-height: 0; border-right: 1px solid var(--line); }
  .col { container: win / inline-size; position: relative; flex: 1 1 0; min-width: 0; display: flex; flex-direction: column; }
  .col > :global(*) { flex: none; }
  .col > main { flex: 1 1 0; }
  /* Below the dock width, the sidebar over the page, under the strip: a 1 px orange line on its
     open edge and the page dimmed behind it (The Dimmed Page Rule). */
  .scrim, .over { position: absolute; top: var(--strip); bottom: 0; z-index: 30; }
  .scrim { left: 0; right: 0; background: var(--scrim-route); }
  .over { left: 0; width: var(--sidebar); border-right: 1px solid var(--accent); }
  /* DESIGN.md Frame: a 64 px row with the 1 px orange rule under it; stacked, the tabs add 44 px. */
  /* The rule runs the full width of the window, over the route sidebar (The Full-Width Header Rule). */
  header { box-sizing: border-box; display: flex; align-items: center; gap: clamp(var(--sp-4), 2.5vw, var(--gutter)); height: 4rem; padding: 0 var(--side); border-bottom: 1px solid var(--accent); }
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
