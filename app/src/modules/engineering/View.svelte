<script lang="ts">
  // Ships > Ships: one ship of the fleet. Its head (name, where it is, the builder links and four
  // hero values), then its Loadout (the fitted modules in the game's outfitting groups, a selected
  // module in the side panel) or its Plan (what the build plan costs), and the rest of the fleet as
  // a strip under it. The link is #/engineering/<id>, or #/engineering/<id>/plan for the Plan.
  import { tip } from "../../lib/ui/tip";
  import Button from "../../lib/ui/Button.svelte";
  import Choice from "../../lib/ui/Choice.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import { failure, pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { DEFAULT_ROLLS, loadFleet, loadHeadlines, shipStats, type Compared, type Fitted, type Fleet, type Headline, type Ship } from "../../lib/engineering";
  import { openUrl } from "../../lib/core";
  import ListDetail from "../../lib/ui/layout/ListDetail.svelte";
  import ShipIcon from "../../lib/ui/ShipIcon.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";
  import Build from "./Build.svelte";
  import Loadout, { ROW_MIN } from "./Loadout.svelte";
  import ModulePanel from "./ModulePanel.svelte";
  import { ShipPlan } from "./plan.svelte";

  let fleet = $state<Fleet | null>(null);
  let headlines = $state<Record<string, Headline>>({});
  let headlinesErr = $state<string | null>(null);
  let fleetErr = $state<string | null>(null);
  let linkErr = $state<string | null>(null);
  let compared = $state<Compared | null>(null);
  let statsNote = $state<string | null>(null);
  let statsSeq = 0;
  /** The selected module's slot, on the Loadout. */
  let selSlot = $state<string | null>(null);

  const readHash = () => {
    const m = /^#\/engineering(?:\/(-?\d+)(?:\/(plan))?)?$/.exec(location.hash);
    return { id: m?.[1] ? Number(m[1]) : null, view: (m?.[2] === "plan" ? "plan" : "loadout") as View };
  };
  type View = "loadout" | "plan";
  let want = $state(readHash());

  $effect(() => {
    const onHash = () => (want = readHash());
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  $effect(() => {
    let fleetKey = "";
    let headlinesKey = "";
    const tick = () => {
      void loadFleet()
        .then((f) => {
          fleetErr = null;
          const k = JSON.stringify(f);
          if (k !== fleetKey) {
            fleetKey = k;
            fleet = f;
          }
        })
        .catch((e: unknown) => (fleetErr = pollFailure("read the fleet", e, fleet !== null)));
      void loadHeadlines()
        .then((h) => {
          const k = JSON.stringify(h);
          if (k !== headlinesKey) {
            headlinesKey = k;
            headlines = h;
          }
          headlinesErr = null;
        })
        .catch((e: unknown) => {
          headlinesKey = "";
          headlines = {};
          headlinesErr = failure("work out shield, DPS and jump range", e);
        });
    };
    tick();
    const t = setInterval(tick, 1000);
    return () => clearInterval(t);
  });

  const ship = $derived.by((): Ship | undefined => {
    const ships = fleet?.ships ?? [];
    if (!ships.length) return undefined;
    return ships.find((s) => s.id === want.id) ?? ships.find((s) => s.current) ?? ships[0];
  });
  /** The current ship first, then the fleet in the journal's order. */
  const strip = $derived([...(fleet?.ships ?? [])].sort((a, b) => Number(b.current) - Number(a.current)));
  const view = $derived(want.view);
  const VIEWS: { value: View; label: string }[] = [
    { value: "loadout", label: "Loadout" },
    { value: "plan", label: "Plan" },
  ];
  const sp = new ShipPlan(() => fleet, () => ship?.id);
  const stats = $derived(compared && "current" in compared ? compared.current : null);
  const unknownHull = $derived(compared && "unknown_hull" in compared ? compared.unknown_hull : null);
  const headline = $derived(ship ? headlines[ship.id] : undefined);
  const unresolved = $derived(stats?.unresolved ?? headline?.unresolved ?? []);
  const selected = $derived(ship?.modules.find((m) => m.slot === selSlot));

  // A new ship starts with nothing selected.
  let lastShip: number | undefined;
  $effect.pre(() => {
    const id = ship?.id;
    if (id !== lastShip) selSlot = null;
    lastShip = id;
  });

  // The hero values: the ship as fitted, on its own request so a plan the stats can't work out
  // never blanks them.
  $effect(() => {
    const id = ship?.id;
    if (id === undefined) {
      compared = null;
      statsNote = null;
      return;
    }
    const mine = ++statsSeq;
    shipStats({ ship_id: id, targets: [], rolls: DEFAULT_ROLLS })
      .then((r) => {
        if (mine !== statsSeq) return;
        compared = r;
        statsNote = null;
      })
      .catch((e: unknown) => {
        if (mine !== statsSeq) return;
        compared = null;
        statsNote = String(e);
      });
  });

  function href(id: number, next: View = view) {
    return next === "plan" ? `#/engineering/${id}/plan` : `#/engineering/${id}`;
  }

  function cr(n: number | null): string {
    if (n === null) return "–";
    if (n >= 1e6) return `${+(n / 1e6).toFixed(n >= 1e8 ? 0 : 1)}M`;
    if (n >= 1e3) return `${Math.round(n / 1e3)}k`;
    return String(n);
  }

  function progress(s: Ship): { done: number; total: number } {
    const mods = s.modules.filter((m) => m.group);
    return { done: mods.filter((m) => m.engineering).length, total: mods.length };
  }

  function when(ts: string | null): string {
    return ts ? ts.replace("T", " ").replace(/:\d\dZ?$/, "") : "";
  }

  /** Why a hero figure is "–". */
  function missing(what: string): string {
    if (unknownHull) return "This hull is not in the ship data.";
    if (!stats) return statsNote ? `The stats could not be worked out: ${statsNote}` : "Working out the stats…";
    return what;
  }

  type Hero = { label: string; value: string; unit: string; why: string };
  const heroes = $derived.by((): Hero[] => {
    const jump = stats?.jump?.max ?? headline?.jump ?? null;
    const shield = stats ? (stats.shield?.strength ?? null) : (headline?.shield ?? null);
    const power = stats?.power;
    return [
      {
        label: "Jump range",
        value: jump == null ? "–" : jump.toFixed(1),
        unit: "ly",
        why: jump == null
          ? missing("No frame shift drive, or it is switched off.")
          : `With one jump's fuel and no cargo${stats?.jump ? `; ${stats.jump.unladen.toFixed(2)} ly on a full tank, ${stats.jump.laden.toFixed(2)} ly laden` : ""}.`,
      },
      {
        label: "Shields",
        value: shield == null ? "–" : shield.toFixed(0),
        unit: "MJ",
        why: shield == null ? missing("No shield generator.") : "Shield strength with the boosters fitted.",
      },
      {
        label: "Top speed",
        value: stats?.speed == null ? "–" : stats.speed.toFixed(0),
        unit: "m/s",
        why: stats?.speed == null ? missing("The ship can't thrust: its thrusters are missing or switched off.") : `With four pips to engines${stats.boost == null ? "" : `; ${stats.boost.toFixed(0)} m/s boosting`}.`,
      },
      {
        label: "Power used",
        value: power ? power.deployed.toFixed(1) : "–",
        unit: power ? `of ${power.generation.toFixed(1)} MW` : "MW",
        why: power ? `With hardpoints deployed; ${power.retracted.toFixed(1)} MW retracted.` : missing(""),
      },
    ];
  });

  async function open(url: string) {
    linkErr = null;
    try {
      await openUrl(url);
    } catch (e) {
      linkErr = `Could not open the link: ${String(e)}`;
    }
  }

  function select(m: Fitted) {
    selSlot = selSlot === m.slot ? null : m.slot;
  }
</script>

{#if !fleet}
  <p class="type-label state">{fleetErr ?? "Reading the journal…"}</p>
{:else if !ship}
  <p class="type-body state">No ships in the journal yet.</p>
{:else}
  {@const p = progress(ship)}
  <div class="page">
    <Choice label="Ship" options={VIEWS} value={view} onchange={(v) => (location.hash = href(ship.id, v))} />
    <div class="head">
      <ShipIcon type={ship.ship_type} size={72} />
      <div class="who">
        <div class="titlerow">
          <h2 class="type-title">{ship.name ?? "Unnamed"}</h2>
          {#if ship.links}
            {@const links = ship.links}
            <span class="open type-label">
              Open in
              <Button {@attach tip("Open this loadout in Coriolis")} onclick={() => open(links.coriolis)}>Coriolis <Glyph name="external" /></Button>
              <Button {@attach tip("Open this loadout in EDSY")} onclick={() => open(links.edsy)}>EDSY <Glyph name="external" /></Button>
            </span>
          {/if}
        </div>
        <p class="type-label line">
          <span>{ship.type_name}</span>
          {#if ship.ident}<span>{ship.ident}</span>{/if}
          {#if ship.loadout_ts}
            <span {@attach tip(`Modules as of ${when(ship.loadout_ts)}, of those that can be engineered. Read from: ${fleet.source}.`)}>{p.done} of {p.total} modules engineered</span>
          {:else}
            <span>modules unknown</span>
          {/if}
          {#if ship.current}
            <span>flying now</span>
          {:else if ship.system}
            <span class="where">{ship.in_transit ? "in transit to" : "stored at"} <SystemChip name={ship.system} /> <ExtLinks system={ship.system} />
              {#if ship.market_id != null && !ship.in_transit}station <ExtLinks system={ship.system} marketId={ship.market_id} />{/if}</span>
          {:else}
            <span>{ship.in_transit ? "in transit" : "stored"}</span>
          {/if}
          <span {@attach tip("What the ship is worth: the shipyard's value for a stored ship, else hull and modules")}>value {cr(ship.value)}</span>
          <span {@attach tip("Insurance to pay if the ship is lost")}>rebuy {cr(ship.rebuy)}</span>
        </p>
        {#if !ship.links && ship.loadout_ts && ship.modules.some((m) => m.engineering)}
          <p class="type-label" {@attach tip("You engineered a module since the game last saved this loadout, so the link would show the old build.")}>Re-dock or relog to refresh the Coriolis / EDSY link</p>
        {/if}
      </div>
    </div>
    {#if linkErr}<p class="err" role="alert"><b>!</b> {linkErr}</p>{/if}
    <Stale error={fleetErr} />

    {#if ship.loadout_ts}
      <div class="heroes">
        {#each heroes as h, i (h.label)}
          <div class="hero" {@attach tip(h.why)}>
            <span class="type-label">{h.label}</span>
            <span><span class="type-value-xl" class:main={i === 0 && !selected && h.value !== "–"}>{h.value}</span><span class="type-unit">{h.unit}</span></span>
          </div>
        {/each}
        <div class="notes">
          {#if unknownHull}
            <p class="type-label" {@attach tip(`Hull: ${unknownHull}`)}><b class="mark">!</b> Incomplete: this hull is not in the ship data</p>
          {:else if unresolved.length}
            <p class="type-label" {@attach tip(unresolved.join("\n"))}>
              <b class="mark">!</b> Incomplete: {unresolved.length} {unresolved.length === 1 ? "item is not" : "items are not"} in the ship data
            </p>
          {:else if statsNote && !stats && view !== "plan"}
            <p class="type-label">{statsNote}</p>
          {/if}
          {#if headlinesErr}<p class="type-label"><b class="mark">!</b> {headlinesErr}</p>{/if}
        </div>
      </div>
    {/if}
  </div>

  {#if !ship.loadout_ts}
    <p class="type-body state">The journal has no loadout for this ship, so its modules are unknown: it has not been flown since your oldest journal file.</p>
  {:else if view === "plan"}
    <Build {fleet} {ship} {sp} />
  {:else}
    <ListDetail panel={selected ? modulePanel : null} selected={selSlot} min={ROW_MIN}>
      <Loadout {fleet} {ship} {sp} selected={selSlot} withPanel={!!selected} onselect={select} />
    </ListDetail>
  {/if}

  <nav class="fleet" aria-label="Your ships">
    {#each strip as s (s.id)}
      <a class="pick" class:on={s.id === ship.id} href={href(s.id)} aria-label="{s.name ?? s.ident ?? `#${s.id}`}, {s.type_name}" aria-current={s.id === ship.id ? "page" : undefined}>
        <ShipIcon type={s.ship_type} size={48} />
        <span class="pick-who">
          <span class="type-name pick-name">{s.name ?? "Unnamed"}</span>
          <span class="type-label">{s.type_name} · {#if s.current}Here{:else if s.system}{s.in_transit ? `to ${s.system}` : s.system}{:else}{s.in_transit ? "in transit" : "stored"}{/if}</span>
        </span>
      </a>
    {/each}
  </nav>
{/if}

{#snippet modulePanel()}
  {#if selected && fleet}<ModulePanel {fleet} m={selected} {sp} />{/if}
{/snippet}

<style>
  .state { margin: 0; padding: var(--sp-6) 0; }
  .page { display: flex; flex-direction: column; align-items: start; gap: var(--sp-4); padding-top: var(--sp-6); }
  /* The head is as wide as what it holds: the builder links sit right after the ship's name and its
     line, never at the far edge of a wide page (The Next To It Rule). */
  .head { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-6); }
  .who { display: flex; flex-direction: column; gap: var(--sp-1); min-width: 0; }
  .who h2 { margin: 0; }
  .titlerow { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-2) var(--sp-6); }
  .who p { margin: 0; }
  .line { display: flex; flex-wrap: wrap; align-items: baseline; column-gap: var(--sp-2); }
  .line > span + span::before { content: "·"; margin-right: var(--sp-2); }
  .where { display: inline-flex; flex-wrap: wrap; align-items: baseline; column-gap: var(--sp-1); }
  .open { display: inline-flex; align-items: center; gap: var(--sp-1); }
  .err { margin: 0; color: var(--alarm); }
  .err b { font-weight: 600; }
  .mark { color: var(--accent); font-weight: 600; }
  .heroes { display: flex; flex-wrap: wrap; align-items: end; gap: var(--sp-4) var(--sp-8); }
  .hero { display: flex; flex-direction: column; gap: var(--sp-2); }
  .notes { display: flex; flex-direction: column; gap: var(--sp-1); align-self: end; }
  .notes p { margin: 0; }
  /* The fleet strip: read-only tiles in a Seam outline, the shown ship raised with its name White
     (DESIGN.md Selected row or tile). Tiles keep their width and wrap; spare width stays empty. */
  .fleet { display: flex; flex-wrap: wrap; gap: var(--sp-2); padding: var(--sp-4) 0 var(--sp-8); }
  .pick { display: flex; align-items: center; gap: var(--sp-3); padding: var(--sp-2) var(--sp-3); outline: 1px solid var(--line); outline-offset: -1px; color: inherit; text-decoration: none; }
  .pick:hover { background: var(--row-hover); }
  .pick.on, .pick.on:hover { background: var(--surface-hi); }
  .pick-who { display: flex; flex-direction: column; gap: var(--gap); }
  .pick-name { white-space: nowrap; }
  .pick.on .pick-name { color: var(--text-hi); }
</style>
