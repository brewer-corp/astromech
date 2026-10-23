<script lang="ts">
  // Plot a route to `to` from the current system for the current ship and follow it, in the top
  // bar's search (PlotBar, docs/PLOTTING.md): neutron (boosts from neutron stars; its stops are
  // the neutron stars) or exact (every jump, with refuel stops).
  // The range and drive come from the ship's latest Loadout. A neutron route is worked out on
  // this PC in seconds when the neutron star file is downloaded (NeutronFile), else by Spansh;
  // an exact route always by Spansh, as a job that can take a minute.
  import { plotRoute, plotSetup, type PlotSetup, type Plotted, type Plotter, type RouteView } from "../../lib/route";
  import NeutronFile from "./NeutronFile.svelte";

  let { to, onfollow }: { to: string; onfollow?: (v: RouteView) => void } = $props();

  let plotter = $state<Plotter>("neutron");
  let efficiency = $state(60);
  let refuel = $state(true);
  let setup = $state<PlotSetup | null>(null);
  let busy = $state(false);
  let err = $state<string | null>(null);
  let done = $state<Plotted | null>(null);
  // The neutron star file is there, so neutron routes are plotted on this PC.
  let local = $state(false);
  // Bumped after a plot: the file's status may have changed (it failed to load).
  let plots = $state(0);
  const onThisPC = $derived(plotter === "neutron" && local);

  // The ship and cargo change as you play (a swap, cargo bought or sold): keep the line current.
  const readSetup = () =>
    plotSetup()
      .then((s) => (setup = s))
      .catch((e) => (setup = { here: null, ship: null, ship_error: String(e instanceof Error ? e.message : e) }));
  $effect(() => {
    void readSetup();
    const t = setInterval(() => void readSetup(), 5000);
    return () => clearInterval(t);
  });

  const ship = $derived(setup?.ship ?? null);
  const here = $derived(setup?.here ?? null);
  const ready = $derived(!!ship && !!to.trim() && !!here && !busy);

  // A new destination: the last one's result no longer applies.
  $effect(() => {
    void to;
    done = null;
    err = null;
  });

  /** Plot and follow (the top bar's Enter, or the button). */
  export async function go() {
    if (!ready) return;
    busy = true;
    err = null;
    done = null;
    try {
      const eff = Math.min(100, Math.max(1, Math.round(Number(efficiency) || 60)));
      const r = await plotRoute({ plotter, to: to.trim(), efficiency: eff, refuel_every_scoopable: refuel });
      done = r.plotted;
      onfollow?.(r.view);
    } catch (e) {
      err = String(e instanceof Error ? e.message : e);
    } finally {
      busy = false;
      plots += 1;
      void readSetup();
    }
  }

  const ly = (n: number) => Math.round(n).toLocaleString("en-GB");
  const count = (n: number, one: string, many: string) => `${n.toLocaleString("en-GB")} ${n === 1 ? one : many}`;
  function summary(p: Plotted): string {
    const parts = [count(p.jumps, "jump", "jumps"), `${ly(p.ly)} ly`];
    if (p.neutrons) parts.push(count(p.neutrons, "neutron boost", "neutron boosts"));
    if (p.refuels) parts.push(count(p.refuels, "refuel stop", "refuel stops"));
    return `${parts.join(" · ")}, at ${p.range.toFixed(2)} ly a jump`;
  }
  function where(p: Plotted): string {
    if (!p.local) return "Plotted by Spansh.";
    return p.fewest === false ? "Plotted on this PC, near the fewest jumps." : "Plotted on this PC.";
  }
</script>

<div class="plot">
  <h4><span class="lo">{here ?? "Your system"} to</span> {to}</h4>
  <div class="kinds" role="group" aria-label="Plotter">
    <button class:on={plotter === "neutron"} aria-pressed={plotter === "neutron"} onclick={() => (plotter = "neutron")}>Neutron</button>
    <button class:on={plotter === "exact"} aria-pressed={plotter === "exact"} onclick={() => (plotter = "exact")}>Exact</button>
  </div>
  <div class="fields">
    {#if plotter === "neutron" && !local}
      <label><span>Efficiency</span><input aria-label="Efficiency" type="number" min="1" max="100" bind:value={efficiency} /><em>%</em></label>
    {:else if plotter === "exact"}
      <label class="check"><input type="checkbox" bind:checked={refuel} /> Refuel at every scoopable star</label>
    {/if}
  </div>
  {#if plotter === "neutron"}<NeutronFile onlocal={(l) => (local = l)} refresh={plots} />{/if}
  {#if ship}
    <p class="lo">
      {ship.name}: {ship.range.toFixed(2)} ly a jump with a full tank{ship.cargo ? ` and ${Math.round(ship.cargo)} t of cargo` : ""}, from its last loadout.
      {#if ship.unresolved.length}Some modules are not counted, so the range may be off.{/if}
    </p>
  {:else if setup?.ship_error}
    <p class="lo">{setup.ship_error}</p>
  {/if}
  {#if setup && !here}<p class="lo">astromech doesn't know your current system yet. Log in to Elite, then plot.</p>{/if}
  <p class="lo">
    {plotter === "neutron"
      ? "Stops at the neutron stars to boost from. Plot each leg in the galaxy map."
      : "Every jump is a waypoint. Waypoints marked fuel are where to scoop."}
  </p>
  <button class="go" disabled={!ready} onclick={() => void go()}>{busy ? "Plotting…" : "Plot and follow"}</button>
  {#if busy}
    <p class="lo">{onThisPC ? "Plotting on this PC." : "Asking Spansh. A route can take a minute."}</p>
  {:else if err}
    <p class="err">{err}</p>
  {:else if done}
    <p class="ok">Following {done.name}: {summary(done)}. {where(done)} <a href="#/route">Show the route</a></p>
    {#if done.fallback}<p class="lo">The neutron star file can't be read, so Spansh plotted this route: {done.fallback}</p>{/if}
  {/if}
</div>

<style>
  .plot { display: flex; flex-direction: column; gap: 4px; }
  h4 { margin: 0 0 8px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-l); line-height: 1.2; overflow-wrap: anywhere; }
  h4 .lo { display: block; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  a { color: var(--text-hi); }
  p { margin: 0; font-size: var(--fs-s); }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .kinds { display: flex; gap: 18px; margin: 0 0 8px; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .kinds button { color: var(--text-lo); }
  .kinds button:hover { color: var(--text); }
  .kinds .on { color: var(--accent); }
  .fields { display: flex; flex-direction: column; gap: 4px; margin: 0 0 6px; font-size: var(--fs-s); }
  label { display: flex; align-items: center; gap: 8px; }
  label > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; }
  label em { font-style: normal; color: var(--text-lo); min-width: 1.5em; }
  input:not([type="checkbox"]) { all: unset; flex: 1; min-width: 0; height: 28px; padding: 0 8px; background: var(--cell-opt); color: var(--text-hi); font-family: var(--mono); }
  input::placeholder { color: var(--text-lo); }
  .check { color: var(--text); }
  .go { align-self: flex-start; margin: 6px 0 4px; height: var(--row); padding: 0 14px; background: var(--cell); color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); font-weight: 600; }
  .go:disabled { background: var(--cell-empty); color: var(--text-lo); cursor: default; }
  .lo { color: var(--text-lo); }
  .ok { color: var(--accent); }
  .err { color: var(--text-hi); background: var(--accent); padding: 8px 12px; }
</style>
