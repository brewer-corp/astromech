<script lang="ts">
  // Ships page: one selected ship as the main pane (info or engineering), the rest of the
  // fleet as a bottom rail. Stored modules live on their own Engineering tab.
  import { failure, pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { DEFAULT_ROLLS, loadFleet, loadHeadlines, shipStats, shipTitle, type Compared, type Fleet, type Headline, type Ship } from "../../lib/engineering";
  import { openUrl } from "../../lib/core";
  import Build from "./Build.svelte";
  import Specs from "./Specs.svelte";
  import ShipIcon from "../../lib/ui/ShipIcon.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";

  let fleet = $state<Fleet | null>(null);
  let headlines = $state<Record<string, Headline>>({});
  let headlinesErr = $state<string | null>(null);
  let fleetErr = $state<string | null>(null);
  let linkErr = $state<string | null>(null);
  let compared = $state<Compared | null>(null);
  let statsNote = $state<string | null>(null);
  let statsSeq = 0;

  const readHash = () => {
    const m = /^#\/engineering(?:\/(-?\d+)(?:\/(plan))?)?$/.exec(location.hash);
    return { id: m?.[1] ? Number(m[1]) : null, tab: (m?.[2] === "plan" ? "plan" : "content") as "content" | "plan" };
  };
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
  const tab = $derived(want.tab);
  const IN_TRANSIT_TO = "In transit to ";
  const stats = $derived(compared && "current" in compared ? compared.current : null);
  const unknownHull = $derived(compared && "unknown_hull" in compared ? compared.unknown_hull : null);

  $effect(() => {
    const s = ship;
    if (!s) {
      compared = null;
      statsNote = null;
      return;
    }
    const mine = ++statsSeq;
    shipStats({ ship_id: s.id, targets: [], rolls: DEFAULT_ROLLS })
      .then((r) => {
        if (mine !== statsSeq) return;
        compared = r;
        statsNote = r ? null : "Ship stats are worked out in the app. Outside it, only the sample Krait's sample plans have them.";
      })
      .catch((e: unknown) => {
        if (mine !== statsSeq) return;
        compared = null;
        statsNote = String(e);
      });
  });

  function href(id: number, next: "content" | "plan" = tab) {
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

  async function open(url: string) {
    linkErr = null;
    try {
      await openUrl(url);
    } catch (e) {
      linkErr = `Could not open the link: ${String(e)}`;
    }
  }
</script>

<section>
  <div class="query">
    <div class="acts" role="tablist" aria-label="Ship">
      {#if ship}
        <a role="tab" aria-selected={tab === "content"} class:on={tab === "content"} href={href(ship.id, "content")}>Info</a>
        <a role="tab" aria-selected={tab === "plan"} class:on={tab === "plan"} href={href(ship.id, "plan")}>Engineering</a>
      {/if}
      {#if ship?.links}
        <span class="open">
          Open in
          <button title="Open this loadout in Coriolis" onclick={() => ship?.links && open(ship.links.coriolis)}>Coriolis</button>
          <button title="Open this loadout in EDSY" onclick={() => ship?.links && open(ship.links.edsy)}>EDSY</button>
        </span>
      {:else if ship?.loadout_ts && ship.modules.some((m) => m.engineering)}
        <span class="open lo" title="You engineered a module since the game last saved this loadout, so the link would show the old build.">Re-dock or relog to refresh the Coriolis / EDSY link</span>
      {/if}
    </div>
    {#if linkErr}<p class="lo">{linkErr}</p>{/if}
    <p class="sub">
      {#if !fleet}
        {fleetErr ?? "Reading the journal…"}
      {:else if !ship}
        {fleet.ships.length} {fleet.ships.length === 1 ? "ship" : "ships"} · {fleet.source}
      {:else}
        {shipTitle(ship)}{ship.ident ? ` (${ship.ident})` : ""}
        · {#if ship.current}flying now{:else if ship.system}{ship.in_transit ? "in transit to" : "stored at"} <SystemChip name={ship.system} />
          <ExtLinks system={ship.system} />
          {#if ship.market_id != null && !ship.in_transit}<span class="lo">station</span> <ExtLinks system={ship.system} marketId={ship.market_id} />{/if}{:else}stored{/if}
        · {ship.loadout_ts ? `modules as of ${when(ship.loadout_ts)}` : "modules unknown"}
        · {fleet.source}
      {/if}
    </p>
    <Stale error={fleet ? fleetErr : null} />
    {#if headlinesErr && fleet}
      <p class="sub"><b class="mark">!</b> {headlinesErr}</p>
    {/if}
  </div>

  <div class="stage">
    {#if ship && tab === "plan" && fleet}
      <Build {fleet} shipId={ship.id} embedded onback={() => (location.hash = href(ship.id, "content"))} />
    {:else if ship}
      {@const p = progress(ship)}
      {@const h = headlines[ship.id]}
      <div class="ship" class:now={ship.current}>
        <div class="art">
          <ShipIcon type={ship.ship_type} name={ship.type_name} size={180} />
        </div>
        <div class="who">
          <h2>{ship.name ?? "Unnamed"}</h2>
          <p class="id lo">{ship.ident ?? `#${ship.id}`}</p>
          <p class="type">{ship.type_name}</p>
          <p class="where" class:accent={ship.current} title={!ship.current && ship.system ? ship.system : undefined}>
            {#if ship.current}Flying now{:else if ship.system}{#if ship.in_transit}<span>{IN_TRANSIT_TO}</span>{/if}<SystemChip name={ship.system} />{:else}{ship.in_transit ? "In transit" : "Stored"}{/if}
          </p>
        </div>
        <div class="more">
          <span class="pair"><span class="k">Value</span> <span class="q">{cr(ship.value)}</span></span>
          <span class="pair"><span class="k">Rebuy</span> <span class="q">{cr(ship.rebuy)}</span></span>
          {#if ship.loadout_ts}
            <span class="pair eng"><span class="k">Engineered</span> <span class="q">{p.done} / {p.total}</span></span>
            <span class="bar" aria-hidden="true"><i style:width="{p.total ? (p.done / p.total) * 100 : 0}%"></i></span>
          {:else}
            <span class="pair"><span class="k">Engineered</span> <span class="q lo">modules unknown</span></span>
          {/if}
        </div>
      </div>
      {#if ship.loadout_ts}
        <Specs type={ship.ship_type} {stats} headline={h} note={statsNote} {unknownHull} />
      {:else}
        <p class="none">The journal has no loadout for this ship: it has not been flown since your oldest journal file.</p>
      {/if}
    {:else if fleet}
      <p class="none">No ships in the journal yet.</p>
    {/if}
  </div>

  {#if fleet?.ships.length}
    <nav class="rail" aria-label="Your ships">
      {#each fleet.ships as s (s.id)}
        <a class="pick" class:on={s.id === ship?.id} class:now={s.current} href={href(s.id)} aria-label="{s.name ?? s.ident ?? `#${s.id}`}, {s.type_name}" aria-current={s.id === ship?.id ? "page" : undefined}>
          <ShipIcon type={s.ship_type} name={s.type_name} size={56} />
          <span class="pick-who">
            <span class="pick-name">{s.name ?? "Unnamed"}</span>
            <span class="pick-type">{s.type_name}</span>
            {#if s.current}<span class="pick-now">Current ship</span>{/if}
          </span>
        </a>
      {/each}
    </nav>
  {/if}
</section>

<style>
  section { padding: 0; display: flex; flex-direction: column; height: 100%; min-height: 0; overflow: hidden; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .query { display: flex; flex-direction: column; gap: 12px; padding: 18px 20px 20px; background: var(--bg-list); border-bottom: 1px solid var(--accent-dim); flex: none; }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; align-items: baseline; text-transform: uppercase; letter-spacing: var(--caps); }
  .acts a, .acts button { color: var(--text-lo); text-decoration: none; }
  .acts a:hover, .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .open { color: var(--text-lo); text-transform: none; letter-spacing: 0; margin-left: auto; }
  .open button { color: var(--accent); margin-left: 6px; text-transform: uppercase; letter-spacing: var(--caps); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0; }
  .stage { flex: 1 1 auto; min-width: 0; min-height: 0; overflow: auto; }
  .ship { display: grid; grid-template-columns: auto minmax(12rem, 1.4fr) minmax(12rem, 1fr); gap: 18px 28px; align-items: start; padding: 18px 20px 4px; }
  .art { display: grid; place-items: center; min-width: 12rem; }
  .who h2 { margin: 0 0 4px; color: var(--text-hi); font-size: var(--fs-xl); font-weight: 500; }
  .id { font-family: var(--mono); font-size: var(--fs-s); margin: 0 0 8px; }
  .type { margin: 0 0 8px; font-size: var(--fs-l); color: var(--text); }
  .where { display: flex; flex-wrap: wrap; align-items: baseline; column-gap: 4px; margin: 0; font-size: var(--fs); color: var(--text-lo); }
  .lo { color: var(--text-lo); }
  .mark { color: var(--accent); }
  .accent { color: var(--accent); }
  .more { display: flex; flex-wrap: wrap; align-items: baseline; gap: 10px 18px; min-width: 0; padding-top: 8px; }
  .pair { display: inline-flex; align-items: baseline; gap: 6px; white-space: nowrap; }
  .k { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .q { font-family: var(--mono); font-size: var(--fs-l); }
  .eng { align-items: center; }
  .bar { flex: 1 1 6rem; min-width: 4rem; height: 4px; background: var(--cell-opt); align-self: center; }
  .bar i { display: block; height: 100%; background: var(--accent); }
  .none { color: var(--text-lo); padding: 28px 20px; }
  .rail { display: flex; gap: 8px; overflow-x: auto; flex: none; padding: 12px 12px 16px; background: var(--bg-list); border-top: 1px solid var(--accent-dim); }
  .pick { display: flex; align-items: center; gap: 12px; min-width: 13rem; padding: 10px 12px; background: var(--cell-empty); color: inherit; text-decoration: none; }
  .pick:hover { background: var(--row-hover); }
  .pick.on { background: var(--cell); }
  .pick-who { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
  .pick-name { color: var(--text-hi); font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .pick.on .pick-name { color: var(--accent); }
  .pick-type, .pick-now { font-size: var(--fs-s); color: var(--text-lo); }
  .pick.now .pick-now { color: var(--accent); }
  @media (max-width: 900px) {
    .ship { grid-template-columns: minmax(0, 1fr); }
    .art { justify-content: start; }
  }
</style>
