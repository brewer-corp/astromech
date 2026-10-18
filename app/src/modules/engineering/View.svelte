<script lang="ts">
  // Ships page: a dense card per ship of the journal fleet (shield, DPS and jump range from the
  // ship calculations, astromech-ship), and the modules in storage with where they are. Opening
  // a card shows that ship's detail (Build.svelte) at #/engineering/<id>, so back and reload
  // keep it.
  import { failure, pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { loadFleet, loadHeadlines, type Fleet, type Headline, type Ship } from "../../lib/engineering";
  import Build from "./Build.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  let fleet = $state<Fleet | null>(null);
  let headlines = $state<Record<string, Headline>>({});
  let headlinesErr = $state<string | null>(null);
  let fleetErr = $state<string | null>(null);
  const readOpen = () => {
    const m = /^#\/engineering\/(-?\d+)$/.exec(location.hash);
    return m ? Number(m[1]) : null;
  };
  let open = $state<number | null>(readOpen());

  $effect(() => {
    const onHash = () => (open = readOpen());
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });

  $effect(() => {
    // Replace only on change: the page polls every second and most polls find nothing new.
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

  // A literal trailing space would be trimmed; the flex gap spaces it on screen, this for readers.
  const IN_TRANSIT_TO = "In transit to ";

  /** "165M", "6.7M", "90k". */
  function cr(n: number | null): string {
    if (n === null) return "–";
    if (n >= 1e6) return `${+(n / 1e6).toFixed(n >= 1e8 ? 0 : 1)}M`;
    if (n >= 1e3) return `${Math.round(n / 1e3)}k`;
    return String(n);
  }

  /** Engineered modules out of those that can be engineered. */
  function progress(s: Ship): { done: number; total: number } {
    const mods = s.modules.filter((m) => m.group);
    return { done: mods.filter((m) => m.engineering).length, total: mods.length };
  }
</script>

{#if fleet && open !== null}
  <Build {fleet} shipId={open} onback={() => (location.hash = "#/engineering")} />
{:else}
  <section>
    <p class="sub">
      {#if !fleet}
        {fleetErr ?? "Reading the journal…"}
      {:else}
        {fleet.ships.length} {fleet.ships.length === 1 ? "ship" : "ships"} · {fleet.stored_modules.length} stored
        {fleet.stored_modules.length === 1 ? "module" : "modules"} · {fleet.source}
      {/if}
    </p>
    <Stale error={fleet ? fleetErr : null} />
    {#if headlinesErr && fleet}
      <p class="sub"><b class="mark">!</b> {headlinesErr}</p>
    {/if}

    <div class="cards">
      {#each fleet?.ships ?? [] as s (s.id)}
        {@const p = progress(s)}
        {@const h = headlines[s.id]}
        {@const jump = h?.jump ?? s.max_jump_range}
        <!-- The name link covers the card; the system chip sits above it (a button can't hold one). -->
        <div class="card" class:now={s.current}>
          <a class="name" href="#/engineering/{s.id}" aria-label="{s.name ?? s.ident ?? `#${s.id}`}, {s.type_name}">{s.name ?? "Unnamed"}</a>
          <span class="id lo">{s.ident ?? `#${s.id}`}</span>
          <span class="row">
            <span class="type lo">{s.type_name}</span>
            <span class="where" class:accent={s.current} title={!s.current && s.system ? s.system : undefined}>
              {#if s.current}Flying now{:else if s.system}{#if s.in_transit}<span>{IN_TRANSIT_TO}</span>{/if}<SystemChip name={s.system} />{:else}{s.in_transit ? "In transit" : "Stored"}{/if}
            </span>
          </span>
          <span class="k">Value</span><span class="q">{cr(s.value)}</span>
          <span class="k">Rebuy</span><span class="q">{cr(s.rebuy)}</span>
          <span class="k">Jump</span><span class="q">{jump ? `${jump.toFixed(1)} ly` : "–"}</span>
          <span class="k">Shield</span><span class="q">{h?.shield != null ? `${h.shield.toFixed(0)} MJ` : "–"}</span>
          <span class="k">DPS</span><span class="q">{h?.dps != null ? h.dps.toFixed(1) : "–"}</span>
          {#if h?.unresolved.length}
            <span class="k">Stats</span>
            <span class="q inc" title={h.unresolved.join("\n")}><b>!</b> Incomplete</span>
          {/if}
          <span class="k">Engineered</span>
          {#if s.loadout_ts}
            <span class="q">{p.done} / {p.total}</span>
            <span class="bar"><i style:width="{p.total ? (p.done / p.total) * 100 : 0}%"></i></span>
          {:else}
            <span class="q lo">modules unknown</span>
          {/if}
        </div>
      {:else}
        {#if fleet}<p class="lo">No ships in the journal yet.</p>{/if}
      {/each}
    </div>

    {#if fleet?.stored_modules.length}
      <h4>Stored modules</h4>
      <div class="stored">
        <div class="h">Module</div><div class="h">Engineering</div><div class="h">Where</div><div class="h r">Value</div>
        {#each fleet.stored_modules as m, i (i)}
          <div>{m.name}</div>
          <div class:lo={!m.blueprint_name}>{m.blueprint_name ? `G${m.grade ?? "?"} ${m.blueprint_name}` : "–"}</div>
          <div class:lo={!m.system}>{#if m.system}<SystemChip name={m.system} />{:else}In transit{/if}</div>
          <div class="r q">{cr(m.value)}</div>
        {/each}
      </div>
    {/if}
  </section>
{/if}

<style>
  section { padding: 8px 0 28px; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 14px 0; }
  .lo { color: var(--text-lo); }
  .cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr)); gap: 10px; }
  .card { position: relative; display: grid; grid-template-columns: auto 1fr auto; column-gap: 10px; row-gap: 2px; align-content: start; padding: 10px 12px; background: var(--cell-empty); border-left: 2px solid var(--accent-dim); }
  .card:hover { background: var(--row-hover); border-left-color: var(--accent); }
  .card:has(.name:focus-visible) { outline: 2px solid var(--accent); outline-offset: 2px; }
  .name::after { content: ""; position: absolute; inset: 0; }
  .name:focus-visible { outline: none; }
  .where :global(.chip) { position: relative; z-index: 1; max-width: 100%; min-width: 0; }
  .where :global(.chip .name) { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .card.now { border-left-color: var(--accent); }
  .name { grid-column: 1 / 3; color: var(--text-hi); text-decoration: none; font-size: var(--fs-l); font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .id { font-family: var(--mono); font-size: var(--fs-s); text-align: right; align-self: center; }
  /* Type left, location right; a long location wraps under "In transit to" and its name ellipsizes. */
  .row { grid-column: 1 / -1; display: flex; justify-content: space-between; align-items: baseline; gap: 10px; min-width: 0; }
  .type { flex: none; font-size: var(--fs-s); white-space: nowrap; }
  .where { min-width: 0; display: flex; flex-wrap: wrap; justify-content: flex-end; align-items: baseline; column-gap: 4px; font-size: var(--fs-s); text-align: right; color: var(--text); white-space: nowrap; }
  .mark { color: var(--accent); }
  .accent { color: var(--accent); }
  .inc b { color: var(--accent); }
  .k { grid-column: 1 / 3; color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .card .q { grid-column: 3; font-family: var(--mono); font-size: var(--fs-s); text-align: right; }
  .bar { grid-column: 1 / -1; height: 4px; background: var(--cell-opt); margin-top: 2px; }
  .bar i { display: block; height: 100%; background: var(--accent); }
  h4 { margin: 26px 0 8px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .stored { display: grid; grid-template-columns: minmax(12rem, 1.4fr) minmax(10rem, 1fr) minmax(8rem, 1fr) 6rem; gap: var(--gap) 12px; font-size: var(--fs-s); max-width: 52rem; }
  .q { font-family: var(--mono); }
  .stored > div { padding: 4px 0; }
  .h { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); }
  .r { text-align: right; }
</style>
