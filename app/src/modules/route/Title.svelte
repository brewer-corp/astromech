<script lang="ts">
  // The Route page's title is the plotter (astromech-ok29's Design): "Khepri to [destination]",
  // the destination editable in place (Spansh suggests systems as you type); the plot setup's
  // Plotter and Worked out dropdowns (PlotSetup, shared with the header's plot panel); one
  // primary action, Replot from here for the route followed or Plot route to a new destination;
  // a line when the ship's range differs from the range the route was plotted at; then the
  // galaxy-map help behind ? and Import, Open on Spansh and Clear route in the ⋯ menu.
  // A failed plot is DESIGN.md's hard error, with TRY AGAIN.
  import { openUrl } from "../../lib/core";
  import { spanshSystem } from "../../lib/links";
  import { setup } from "../../lib/plot.svelte";
  import { clearRoute, replacedNote, type FollowNote, type Plotted, type RouteView } from "../../lib/route";
  import { enterChoice, systemNames } from "../../lib/search";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import FollowNoteLine from "../../lib/ui/FollowNote.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import Menu, { type MenuItem } from "../../lib/ui/Menu.svelte";
  import Suggestions from "../../lib/ui/Suggestions.svelte";
  import ImportDialog from "./ImportDialog.svelte";
  import PlotSetup from "./PlotSetup.svelte";
  import { plotWhere } from "./plotText";

  let {
    view,
    onview,
    run,
  }: {
    view: RouteView | null;
    /** A route this title followed (a plot, an import). */
    onview: (v: RouteView) => void;
    /** Run a route action the page reports (Clear route), as the page's own actions are. */
    run: (f: () => Promise<RouteView>) => Promise<void>;
  } = $props();

  const HELP =
    "In the galaxy map, select the search box, then say “paste” or press your paste button (controls.paste in the config). " +
    "astromech pastes the next stop and presses Enter, only while the galaxy map is open. " +
    "Jump into the stop and the next one is copied and read out. Say “next waypoint” to hear it again.";

  const route = $derived(view?.route ?? null);
  const plot = $derived(route?.plot ?? null);
  /** Where the route followed ends: what it was plotted to, else its last stop. */
  const end = $derived(plot?.to ?? route?.waypoints.at(-1)?.system ?? "");
  const here = $derived(setup.ship?.here ?? view?.here ?? null);
  const ship = $derived(setup.ship?.ship ?? null);

  let dest = $state("");
  /** The destination box was typed in since the route last changed. */
  let edited = $state(false);
  let err = $state<string | null>(null);
  let mine = $state(false);
  let done = $state<Plotted | null>(null);
  let note = $state<FollowNote | null>(null);
  let importing = $state(false);

  let hits = $state<string[]>([]);
  let sel = $state(0);
  let moved = false;
  let open = $state(false);
  let asked = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;

  $effect(() => setup.follow());
  // The page shows what the route followed was plotted with, and the header's panel follows.
  $effect(() => setup.adopt(plot));
  // A new route: the box shows where it goes, unless the commander is typing a new one.
  $effect(() => {
    const e = end;
    if (!edited) dest = e;
  });

  const target = $derived(dest.trim());
  /** The box names where the route followed goes: the action plots it again from here. */
  const again = $derived(!!plot && target.toLowerCase() === plot.to.toLowerCase());
  const ready = $derived(!!target && !!here && !!ship && !setup.busy);
  /** The range now against the range the route was plotted at, when they differ. */
  const ranges = $derived(plot && ship && ship.range.toFixed(2) !== plot.range.toFixed(2) ? { now: ship.range, then: plot.range } : null);

  /** Close the suggestions, and drop any reply still on its way, so it can't reopen them. */
  function dropHits() {
    clearTimeout(timer);
    asked++;
    open = false;
    hits = [];
  }

  async function go() {
    if (!ready) return;
    dropHits();
    mine = true;
    err = null;
    done = null;
    note = null;
    try {
      const r = await setup.plot(target);
      edited = false;
      done = r.plotted;
      note = replacedNote(r.view);
      onview(r.view);
    } catch (e) {
      err = String(e instanceof Error ? e.message : e);
    } finally {
      mine = false;
    }
  }

  function typed(v: string) {
    dest = v;
    edited = true;
    sel = 0;
    moved = false;
    dropHits();
    const q = v.trim();
    if (q.length < 2) return;
    const n = asked;
    timer = setTimeout(() => {
      systemNames(q)
        .then((h) => {
          if (n !== asked) return;
          hits = h;
          open = h.length > 0;
        })
        .catch(() => {
          if (n === asked) hits = [];
        });
    }, 250);
  }

  function pick(name: string) {
    dropHits();
    dest = name;
    edited = true;
  }

  function key(e: KeyboardEvent) {
    if (e.key === "ArrowDown" && hits.length) {
      open = true;
      moved = true;
      sel = (sel + 1) % hits.length;
    } else if (e.key === "ArrowUp" && hits.length) {
      moved = true;
      sel = (sel - 1 + hits.length) % hits.length;
    } else if (e.key === "Enter" && !e.isComposing) {
      if (open) pick(enterChoice(dest, hits, sel, moved));
      void go();
    } else if (e.key === "Escape") {
      if (open) open = false;
      else if (edited) {
        // Back to where the route goes.
        edited = false;
        dest = end;
      } else return;
      // A reply still on its way is for the edit just dismissed.
      dropHits();
    } else return;
    e.preventDefault();
  }

  const menu: MenuItem[] = $derived([
    { label: "Import a route…", hint: "A Spansh CSV or a list of systems", onselect: () => (importing = true) },
    { label: "Open on Spansh", hint: end ? `${end}, on Spansh's site` : undefined, disabled: !end, onselect: () => void openUrl(spanshSystem({ name: end })) },
    { label: "Clear route", disabled: !route, onselect: () => void run(clearRoute) },
  ]);
</script>

<div class="title">
  {#if route && !plot}<p class="type-label">Following {route.name}</p>{/if}
  <h1 class="type-title">
    <span class="from">{here ?? "Your system"}</span>
    <span class="to">to</span>
    <span class="dest">
      <input
        value={dest}
        placeholder="Destination"
        aria-label="Destination"
        spellcheck="false"
        autocomplete="off"
        role="combobox"
        aria-autocomplete="list"
        aria-expanded={open}
        aria-controls="dest-list"
        aria-activedescendant={open ? `dest-list-${sel}` : undefined}
        oninput={(e) => typed(e.currentTarget.value)}
        onkeydown={key}
        onblur={() => setTimeout(dropHits, 150)}
      />
      {#if open}
        <Suggestions id="dest-list" items={hits} {sel} key={(h) => h} onpick={pick}>
          {#snippet item(h)}{h}{/snippet}
        </Suggestions>
      {/if}
    </span>
  </h1>

  <div class="controls">
    <PlotSetup />
    <Button kind="primary" disabled={!ready} onclick={() => void go()}>{mine ? "Plotting…" : again ? "Replot from here" : "Plot route"}</Button>
    {#if ranges && !mine}
      <span class="range">Your range is <b class="type-figures">{ranges.now.toFixed(2)}</b> ly now; this route was plotted at {ranges.then.toFixed(2)}.</span>
    {/if}
    <span class="end">
      <IconButton icon="help" label="How to use the route in the galaxy map" tip={HELP} />
      <Menu label="More route actions" items={menu} />
    </span>
  </div>

  {#if mine}
    <p class="type-label">{setup.worker === "local" ? "Plotting on this PC." : "Asking Spansh. A route can take a minute."}</p>
  {:else if setup.busy}
    <p class="type-label">A route is being plotted in the header.</p>
  {:else if err}
    <Failure text={err} onaction={() => void go()} />
  {:else if done}
    <p class="type-label">{plotWhere(done)}{done.fallback ? ` The neutron star file can't be read, so Spansh plotted it: ${done.fallback}` : ""}</p>
  {/if}
  {#if !setup.busy && setup.ship && !here}<p class="type-label">astromech doesn't know your current system yet. Log in to Elite, then plot.</p>{/if}
  {#if !setup.busy && setup.ship?.ship_error}<p class="type-label">{setup.ship.ship_error}</p>{/if}
  <FollowNoteLine bind:note onresume={(v) => onview(v)} />
</div>

{#if importing}
  <ImportDialog
    onclose={() => (importing = false)}
    onview={(v) => {
      importing = false;
      edited = false;
      onview(v);
    }}
  />
{/if}

<style>
  .title { display: flex; flex-direction: column; gap: var(--sp-3); }
  p { margin: 0; }
  h1 { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-1) var(--sp-3); margin: 0; }
  .to { color: var(--orange-mid); font-weight: 400; }
  .dest { position: relative; display: inline-flex; min-width: 0; }
  /* The destination is the title's own text until it is pointed at: then it is a cell. */
  input { all: unset; box-sizing: border-box; field-sizing: content; min-width: 10ch; height: calc(var(--row) + var(--sp-1)); padding: 0 var(--sp-2); margin-left: calc(-1 * var(--sp-2)); color: var(--text-hi); border-bottom: 1px dashed var(--orange-mid); }
  input::placeholder { color: var(--placeholder); opacity: 1; }
  input:hover { background: var(--cell); border-bottom-color: transparent; }
  input:focus { background: var(--cell); border-bottom-color: var(--accent); }
  .dest :global(.hits) { font-weight: 500; }
  .controls { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-1); }
  .controls > :global(.primary) { margin-left: var(--sp-1); }
  .range { margin-left: var(--sp-3); }
  .range b { color: var(--orange-lit); }
  .end { display: inline-flex; align-items: center; margin-left: var(--sp-3); }
</style>
