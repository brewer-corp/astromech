<script lang="ts">
  // DESIGN.md's route strip: the route followed, one 36 px line across the top of the page column
  // where the route sidebar is collapsed or the window is below the dock width (decision
  // astromech-bi94). A chevron that opens the sidebar · "Next" · the next system · copy · "3 of 41"
  // · what is there as badges, then on the right the destination and how far is left: jumps where
  // every stop knows them (plotted routes and the galaxy map's), else stops (decision
  // astromech-g8d5). One orange "!" with plain words when the commander should act. With no
  // route, "No route" and the no-route rail's two actions: Find or plot and Import a route. The
  // destination opens the sidebar too. A read or action that failed, a failed clipboard write
  // included, is DESIGN.md's hard error: a red "!", the words, and TRY AGAIN, which repeats it.
  import { tip } from "../../lib/ui/tip";
  import { copyNext, resumeRoute, stopFlags, type RouteView } from "../../lib/route";
  import { failure } from "../../lib/ui/errors";
  import Badge from "../../lib/ui/Badge.svelte";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import ImportDialog from "./ImportDialog.svelte";
  import { focusFindOrPlot } from "./PlotBar.svelte";
  import { useFollow } from "./follow.svelte";

  const { follow, own } = useFollow();
  if (own) $effect(() => follow.start());

  /** The last action failed: TRY AGAIN repeats it. */
  let err = $state<{ text: string; retry: () => void } | null>(null);
  let importing = $state(false);

  async function act(f: () => Promise<RouteView>, what: string) {
    try {
      await follow.run(f);
      err = null;
    } catch (e) {
      err = { text: failure(what, e), retry: () => void act(f, what) };
    }
  }

  const view = $derived(follow.view);
  const same = (a: string | null | undefined, b: string | null | undefined) => !!a && !!b && a.trim().toLowerCase() === b.trim().toLowerCase();

  const route = $derived(view?.route ?? null);
  const next = $derived(view?.next ?? null);
  const prev = $derived(route && route.next > 0 ? (route.waypoints[route.next - 1] ?? null) : null);
  const dest = $derived(route?.waypoints.at(-1) ?? null);
  /** There is a route to show in the sidebar: one followed, or one paused to resume. */
  const any = $derived(!!route || !!view?.parked);
  /** "38 jumps left" where every stop ahead knows its jumps, else "3 stops left". */
  const left = $derived.by(() => {
    const jumps = view?.jumps_left ?? null;
    const n = jumps ?? view?.remaining ?? 0;
    return { n: n.toLocaleString("en"), unit: `${jumps !== null ? "jump" : "stop"}${n === 1 ? "" : "s"} left` };
  });

  /** The one thing the commander should act on, most pressing first. A failed clipboard write is
   * the hard error below, not one of these. */
  const warning = $derived.by((): string | null => {
    if (!view || !next) return null;
    // Off route only where the next waypoint is one jump on (an exact route, the galaxy map's):
    // between the stops of a neutron route or an engineer trip the commander is elsewhere anyway.
    if (prev && next.jumps === 1 && view.here && !same(view.here, prev.system) && !same(view.here, next.system)) return "Off route";
    if (prev && same(view.here, prev.system) && stopFlags(prev.label).includes("fuel")) return "Refuel before the next jump";
    return null;
  });

  /** The chevron: unfold the docked sidebar, or open and close it over the page. */
  const open = $derived(!follow.docked && follow.overlay);
  function fold() {
    if (follow.docked) follow.collapsed = false;
    else follow.overlay = !follow.overlay;
  }
</script>

<div class="strip" role="region" aria-label="Route">
  {#if any}
    <button class="fold" class:open aria-expanded={open} aria-label={open ? "Close the route" : "Show the route"} onclick={fold} {@attach tip(open ? "Close the route" : "Show the whole route")}>
      <Glyph name="chevron" />
    </button>
  {/if}
  {#if route && next}
    <span class="label">Next</span>
    <span class="sys" title={next.system}>{next.system}</span>
    <IconButton icon="copy" label={`Copy ${next.system}`} tip={`Copy ${next.system} for the galaxy map's search box`} onclick={() => act(copyNext, "copy the next waypoint")} />
    <span class="of">{route.next + 1} of {route.waypoints.length}</span>
    {#each stopFlags(next.label) as f, i (i)}<Badge>{f}</Badge>{/each}
  {:else if route}
    <span class="none">Route finished</span>
    <Button onclick={focusFindOrPlot}>Find or plot</Button>
  {:else if view}
    <span class="none">No route</span>
    <Button onclick={focusFindOrPlot}>Find or plot</Button>
    <Button onclick={() => (importing = true)}>Import a route</Button>
  {/if}

  {#if err}
    <Failure inline text={err.text} onaction={err.retry} />
  {:else if next && view?.clipboard_error}
    <Failure inline text={`Not on the clipboard: ${view.clipboard_error}`} onaction={() => act(copyNext, "copy the next waypoint")} />
  {:else if follow.readErr}
    <Failure inline text={follow.readErr} onaction={() => void follow.read()} />
  {:else if follow.stale}
    <span class="warn" title={follow.stale}><b>!</b>{follow.stale}</span>
  {:else if warning}
    <span class="warn" title={warning}><b>!</b>{warning}</span>
  {/if}

  <span class="grow"></span>
  {#if route && next && dest}
    <button class="to" onclick={() => follow.show()} {@attach tip(`${route.name}: show the whole route`)}>
      <span class="label">To</span>
      <span class="dest">{dest.system}</span>
      <span class="left"><span class="n">{left.n}</span> <span class="label">{left.unit}</span></span>
    </button>
  {:else if view?.parked && !next}
    <span class="label">Paused</span>
    <span class="dest">{view.parked.name}</span>
    <Button {@attach tip(`Follow ${view.parked.name} again`)} onclick={() => act(resumeRoute, "resume the route")}>Resume</Button>
  {/if}
</div>

{#if importing}
  <ImportDialog onclose={() => (importing = false)} onview={(v) => { importing = false; follow.view = v; }} />
{/if}

<style>
  /* One line, always: the warning gives way first, then the destination; the next system, the
     strip's main value, keeps its width up to a third of the strip. */
  .strip { display: flex; align-items: center; gap: var(--sp-3); height: var(--strip); min-width: 0; padding: 0 var(--side); background: var(--surface); white-space: nowrap; overflow: hidden; }
  .label { color: var(--label-grey); font-size: var(--fs-s); }
  .sys { color: var(--orange-lit); font-size: var(--fs); font-weight: 500; max-width: 33%; overflow: hidden; text-overflow: ellipsis; flex: none; margin-right: calc(-1 * var(--sp-2)); }
  .of { color: var(--prose); font-size: var(--fs); flex: none; }
  .none { color: var(--label-grey); font-size: var(--fs); flex: none; }
  .warn { color: var(--accent); font-size: var(--fs-s); min-width: 3em; overflow: hidden; text-overflow: ellipsis; flex: 0 1000 auto; }
  .warn b { font-weight: 600; margin-right: var(--sp-1); }
  .strip > :global(.failure) { flex: 0 1000 auto; }
  .grow { flex: 1 1 0; min-width: 0; }
  button { all: unset; cursor: pointer; }
  /* The chevron sits in the side margin, at the window's left edge; it points at the sidebar it
     opens, and back once the sidebar is open over the page. */
  .fold { display: grid; place-items: center; flex: none; width: var(--sp-6); height: var(--strip); margin-left: calc(-1 * var(--side)); color: var(--label-grey); }
  .fold:hover { color: var(--orange-lit); }
  .fold.open { background: var(--accent); color: var(--on-accent); }
  .fold :global(.glyph) { transform: rotate(-90deg); }
  .fold.open :global(.glyph) { transform: rotate(90deg); }
  .fold:focus-visible, .to:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .to { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; flex: 0 1 auto; }
  .to:hover .dest { color: var(--orange-lit); }
  .dest { color: var(--accent); font-size: var(--fs-s); overflow: hidden; text-overflow: ellipsis; min-width: 3em; }
  .n { color: var(--prose); font-size: var(--fs); }
  .left { flex: none; }
</style>
