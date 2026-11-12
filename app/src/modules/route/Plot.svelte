<script lang="ts">
  // Plot a route to `to` from the current system for the current ship and follow it, in the
  // header's Find or plot panel (PlotBar, docs/PLOTTING.md): neutron (boosts from neutron stars;
  // its stops are the neutron stars) or exact (every jump, with refuel stops). The setup is the
  // one the Route page's title shows (lib/plot.svelte.ts, PlotSetup). The range and drive come
  // from the ship's latest Loadout. A neutron route is worked out on this PC in seconds when the
  // neutron star file is downloaded, else by Spansh; an exact route always by Spansh, as a job
  // that can take a minute.
  import { setup } from "../../lib/plot.svelte";
  import { replacedNote, type FollowNote, type Plotted, type RouteView } from "../../lib/route";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import FollowNoteLine from "../../lib/ui/FollowNote.svelte";
  import PlotSetup from "./PlotSetup.svelte";
  import { plotWhere, plotSummary } from "./plotText";

  let { to, onfollow }: { to: string; onfollow?: (v: RouteView) => void } = $props();

  /** This panel's plot is running (the other place's plot only disables the button). */
  let mine = $state(false);
  let err = $state<string | null>(null);
  let done = $state<Plotted | null>(null);
  /** The route the plotted one replaced, parked for Resume, or what Resume did. */
  let note = $state<FollowNote | null>(null);

  $effect(() => setup.follow());

  const ship = $derived(setup.ship?.ship ?? null);
  const here = $derived(setup.ship?.here ?? null);
  const ready = $derived(!!ship && !!to.trim() && !!here && !setup.busy);

  // A new destination: the last one's result no longer applies.
  $effect(() => {
    void to;
    done = null;
    note = null;
    err = null;
  });

  /** A plot is running: the top bar keeps the panel open, so its result or error is seen. */
  export const plotting = () => mine;

  /** Plot and follow (the top bar's Enter, or the button). */
  export async function go() {
    if (!ready) return;
    mine = true;
    err = null;
    done = null;
    note = null;
    try {
      const r = await setup.plot(to);
      done = r.plotted;
      note = replacedNote(r.view);
      onfollow?.(r.view);
    } catch (e) {
      err = String(e instanceof Error ? e.message : e);
    } finally {
      mine = false;
    }
  }
</script>

<div class="plot">
  <h4><span class="lo">{here ?? "Your system"} to</span> {to}</h4>
  <div class="kinds"><PlotSetup inline /></div>
  {#if ship}
    <p class="lo">
      {ship.name}: {ship.range.toFixed(2)} ly a jump with a full tank{ship.cargo ? ` and ${Math.round(ship.cargo)} t of cargo` : ""}, from its last loadout.
      {#if ship.unresolved.length}Some modules are not counted, so the range may be off.{/if}
    </p>
  {:else if setup.ship?.ship_error}
    <p class="lo">{setup.ship.ship_error}</p>
  {/if}
  {#if setup.ship && !here}<p class="lo">astromech doesn't know your current system yet. Log in to Elite, then plot.</p>{/if}
  <Button kind="primary" class="go" disabled={!ready} onclick={() => void go()}>{mine ? "Plotting…" : "Plot and follow"}</Button>
  {#if mine}
    <p class="busy">{setup.worker === "local" ? "Plotting on this PC." : "Asking Spansh. A route can take a minute."}</p>
  {:else if setup.busy}
    <p class="busy">A route is being plotted on the Route page.</p>
  {:else if err}
    <Failure text={err} onaction={() => void go()} />
  {:else if done}
    <p class="ok">Following {done.name}: {plotSummary(done)}. {plotWhere(done)} <a href="#/route">Show the route</a></p>
    {#if done.fallback}<p class="lo">The neutron star file can't be read, so Spansh plotted this route: {done.fallback}</p>{/if}
  {/if}
  <FollowNoteLine bind:note onresume={() => (done = null)} />
</div>

<style>
  .plot { display: flex; flex-direction: column; gap: 4px; }
  h4 { margin: 0 0 8px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-l); line-height: 1.2; overflow-wrap: anywhere; }
  h4 .lo { display: block; font-size: var(--fs-s); color: var(--orange-mid); }
  a { color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  a:hover { color: var(--orange-lit); }
  p { margin: 0; font-size: var(--fs-s); }
  .kinds { margin: 0 0 8px; }
  .plot > :global(.go) { align-self: flex-start; margin: 6px 0 4px; }
  .lo { color: var(--prose); }
  .busy { color: var(--orange-mid); }
  .ok { color: var(--accent); }
</style>
