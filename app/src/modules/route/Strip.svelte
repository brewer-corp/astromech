<script lang="ts">
  // DESIGN.md's route strip: the route followed, one 36 px line under the header on every page
  // (decision astromech-bi94). "Next" · the next system · copy · "3 of 41" · what is there as
  // badges, then on the right the destination and how far is left: jumps where every stop knows
  // them (plotted routes and the galaxy map's), else stops (decision astromech-g8d5). One orange
  // "!" with plain words when the commander should act. With no route, "No route" and the
  // header box's Find or plot shortcut. The destination opens the Route page, which has the whole
  // list, import and Resume. A read or action that failed is DESIGN.md's hard error: a red "!",
  // the words, and TRY AGAIN, which repeats it.
  import { copyNext, getRoute, resumeRoute, stopFlags, watchRoute, type RouteView } from "../../lib/route";
  import { failure, pollFailure } from "../../lib/ui/errors";
  import Badge from "../../lib/ui/Badge.svelte";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import { focusFindOrPlot } from "./PlotBar.svelte";

  let view = $state<RouteView | null>(null);
  /** The last read or action failed: a route on screen is kept, and this says so. */
  let err = $state<string | null>(null);
  /** It failed outright (an action, or the first read), rather than a refresh of what is shown: TRY AGAIN repeats it. */
  let retry = $state<(() => void) | null>(null);

  async function run(f: () => Promise<RouteView>, what: string | null) {
    try {
      view = await f();
      err = null;
      retry = null;
    } catch (e) {
      retry = what || !view ? () => void run(f, what) : null;
      err = what ? failure(what, e) : pollFailure("read the route", e, !!view);
    }
  }

  // The core moves the route on as you jump; pages that change the route ping watchRoute.
  $effect(() => {
    const read = () => void run(getRoute, null);
    read();
    const t = setInterval(read, 2000);
    const stop = watchRoute(read);
    return () => {
      clearInterval(t);
      stop();
    };
  });

  const same = (a: string | null | undefined, b: string | null | undefined) => !!a && !!b && a.trim().toLowerCase() === b.trim().toLowerCase();

  const route = $derived(view?.route ?? null);
  const next = $derived(view?.next ?? null);
  const prev = $derived(route && route.next > 0 ? (route.waypoints[route.next - 1] ?? null) : null);
  const dest = $derived(route?.waypoints.at(-1) ?? null);
  /** "38 jumps left" where every stop ahead knows its jumps, else "3 stops left". */
  const left = $derived.by(() => {
    const jumps = view?.jumps_left ?? null;
    const n = jumps ?? view?.remaining ?? 0;
    return { n: n.toLocaleString("en"), unit: `${jumps !== null ? "jump" : "stop"}${n === 1 ? "" : "s"} left` };
  });

  /** The one thing the commander should act on, most pressing first. */
  const warning = $derived.by((): string | null => {
    if (!view || !next) return null;
    if (view.clipboard_error) return `Not on the clipboard: ${view.clipboard_error}`;
    // Off route only where the next waypoint is one jump on (an exact route, the galaxy map's):
    // between the stops of a neutron route or an engineer trip the commander is elsewhere anyway.
    if (prev && next.jumps === 1 && view.here && !same(view.here, prev.system) && !same(view.here, next.system)) return "Off route";
    if (prev && same(view.here, prev.system) && stopFlags(prev.label).includes("fuel")) return "Refuel before the next jump";
    return null;
  });
</script>

<div class="strip" role="region" aria-label="Route">
  {#if route && next}
    <span class="label">Next</span>
    <span class="sys" title={next.system}>{next.system}</span>
    <IconButton icon="copy" label={`Copy ${next.system}`} tip={`Copy ${next.system} for the galaxy map's search box`} onclick={() => run(copyNext, "copy the next waypoint")} />
    <span class="of">{route.next + 1} of {route.waypoints.length}</span>
    {#each stopFlags(next.label) as f, i (i)}<Badge>{f}</Badge>{/each}
  {:else if route}
    <span class="none">Route finished</span>
    <Button onclick={focusFindOrPlot}>Find or plot</Button>
  {:else if view}
    <span class="none">No route</span>
    <Button onclick={focusFindOrPlot}>Find or plot</Button>
  {/if}

  {#if err && retry}
    <Failure inline text={err} onaction={retry} />
  {:else if err}
    <span class="warn" title={err}><b>!</b>{err}</span>
  {:else if warning}
    <span class="warn" title={warning}><b>!</b>{warning}</span>
  {/if}

  <span class="grow"></span>
  {#if route && next && dest}
    <a class="to" href="#/route" title={`${route.name}: show the whole route`}>
      <span class="label">To</span>
      <span class="dest">{dest.system}</span>
      <span class="left"><span class="n">{left.n}</span> <span class="label">{left.unit}</span></span>
    </a>
  {:else if view?.parked && !next}
    <span class="label">Paused</span>
    <span class="dest">{view.parked.name}</span>
    <Button title={`Follow ${view.parked.name} again`} onclick={() => run(resumeRoute, "resume the route")}>Resume</Button>
  {/if}
</div>

<style>
  /* One line, always: the warning gives way first, then the destination; the next system, the
     strip's main value, keeps its width up to a third of the strip. */
  .strip { display: flex; align-items: center; gap: var(--sp-3); height: var(--strip); min-width: 0; padding: 0 clamp(16px, 3vw, 40px); background: var(--surface); white-space: nowrap; overflow: hidden; font-family: var(--face); font-stretch: var(--face-stretch); }
  .label { color: var(--orange-mid); font-size: var(--fs-s); }
  .sys { color: var(--orange-lit); font-size: var(--fs); font-weight: 500; max-width: 33%; overflow: hidden; text-overflow: ellipsis; flex: none; margin-right: calc(-1 * var(--sp-2)); }
  .of { color: var(--orange-mid); font-size: var(--fs); font-variant-numeric: tabular-nums; flex: none; }
  .none { color: var(--orange-mid); font-size: var(--fs); flex: none; }
  .warn { color: var(--accent); font-size: var(--fs-s); min-width: 3em; overflow: hidden; text-overflow: ellipsis; flex: 0 1000 auto; }
  .warn b { font-weight: 600; margin-right: var(--sp-1); }
  .strip > :global(.failure) { flex: 0 1000 auto; }
  .grow { flex: 1 1 0; min-width: 0; }
  .to { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; flex: 0 1 auto; color: inherit; text-decoration: none; }
  .to:hover .dest { color: var(--orange-lit); }
  .to:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .dest { color: var(--accent); font-size: var(--fs-s); overflow: hidden; text-overflow: ellipsis; min-width: 3em; }
  .n { color: var(--accent); font-size: var(--fs); font-variant-numeric: tabular-nums; }
  .left { flex: none; }
</style>
