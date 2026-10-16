<script lang="ts">
  // Ships page: a dense card per ship of the journal fleet, and the modules in storage with
  // where they are. Opening a card shows that ship's detail (Build.svelte) at #/engineering/<id>,
  // so back and reload keep it.
  import { loadFleet, type Fleet, type Ship } from "../../lib/engineering";
  import Build from "./Build.svelte";

  let fleet = $state<Fleet | null>(null);
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
    const tick = () => void loadFleet().then((f) => (fleet = f));
    tick();
    const t = setInterval(tick, 5000);
    return () => clearInterval(t);
  });

  /** "165M", "6.7M", "90k". */
  function cr(n: number | null): string {
    if (n === null) return "–";
    if (n >= 1e6) return `${+(n / 1e6).toFixed(n >= 1e8 ? 0 : 1)}M`;
    if (n >= 1e3) return `${Math.round(n / 1e3)}k`;
    return String(n);
  }

  function where(s: Ship): string {
    if (s.current) return "Flying now";
    if (s.in_transit) return s.system ? `In transit to ${s.system}` : "In transit";
    return s.system ? s.system : "Stored";
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
        Reading the journal…
      {:else}
        {fleet.ships.length} {fleet.ships.length === 1 ? "ship" : "ships"} · {fleet.stored_modules.length} stored
        {fleet.stored_modules.length === 1 ? "module" : "modules"} · {fleet.source}
      {/if}
    </p>

    <div class="cards">
      {#each fleet?.ships ?? [] as s (s.id)}
        {@const p = progress(s)}
        <button class="card" class:now={s.current} onclick={() => (location.hash = `#/engineering/${s.id}`)}>
          <span class="name">{s.name ?? "Unnamed"}</span>
          <span class="id lo">{s.ident ?? `#${s.id}`}</span>
          <span class="type lo">{s.type_name}</span>
          <span class="where" class:accent={s.current}>{where(s)}</span>
          <span class="k">Value</span><span class="q">{cr(s.value)}</span>
          <span class="k">Rebuy</span><span class="q">{cr(s.rebuy)}</span>
          <span class="k">Jump</span><span class="q">{s.max_jump_range ? `${s.max_jump_range.toFixed(1)} ly` : "–"}</span>
          <span class="k">Engineered</span>
          {#if s.loadout_ts}
            <span class="q">{p.done} / {p.total}</span>
            <span class="bar"><i style:width="{p.total ? (p.done / p.total) * 100 : 0}%"></i></span>
          {:else}
            <span class="q lo">modules unknown</span>
          {/if}
        </button>
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
          <div class:lo={!m.system}>{m.system ?? "In transit"}</div>
          <div class="r q">{cr(m.value)}</div>
        {/each}
      </div>
    {/if}
  </section>
{/if}

<style>
  section { padding: 8px 0 28px; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 14px 0; }
  .lo { color: var(--text-lo); }
  .cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr)); gap: 10px; }
  .card { display: grid; grid-template-columns: auto 1fr auto; column-gap: 10px; row-gap: 2px; align-content: start; padding: 10px 12px; background: var(--cell-empty); border-left: 2px solid var(--accent-dim); }
  .card:hover { background: var(--row-hover); border-left-color: var(--accent); }
  .card:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
  .card.now { border-left-color: var(--accent); }
  .name { grid-column: 1 / 3; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .id { font-family: var(--mono); font-size: var(--fs-s); text-align: right; align-self: center; }
  .type { grid-column: 1 / 3; font-size: var(--fs-s); }
  .where { grid-column: 3; font-size: var(--fs-s); text-align: right; color: var(--text); white-space: nowrap; }
  .accent { color: var(--accent); }
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
