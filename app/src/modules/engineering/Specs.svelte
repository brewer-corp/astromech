<script lang="ts">
  // Info tab: the fitted ship's numbers, laid out like the in-game ship specs bar, then the
  // rest of what the calculations already produce. Pitch / roll / yaw are a hull-table stand-in
  // until astromech-ship computes handling.
  import type { Headline, ShipStats } from "../../lib/engineering";
  import { handling } from "../../lib/manoeuvre";

  let {
    type,
    stats = null,
    headline = null,
    note = null,
    unknownHull = null,
  }: {
    type: string;
    stats?: ShipStats | null;
    headline?: Headline | null;
    note?: string | null;
    unknownHull?: string | null;
  } = $props();

  const turn = $derived(handling(type));
  const jump = $derived(stats?.jump);
  const power = $derived(stats?.power);
  const speed = $derived(stats?.speed);
  const boost = $derived(stats?.boost);
  const unresolved = $derived(stats?.unresolved ?? headline?.unresolved ?? []);

  function n(v: number | null | undefined, d: number): string {
    return v == null ? "–" : v.toFixed(d);
  }

  function fill(used: number | undefined, cap: number | undefined): string {
    if (!used || !cap || cap <= 0) return "0%";
    return `${Math.min(100, (used / cap) * 100)}%`;
  }
</script>

<div class="specs">
  {#if unknownHull}
    <p class="warn" title="Hull: {unknownHull}"><span class="mark">!</span> Incomplete: this hull is not in the ship data, so there are no stats for it.</p>
  {:else if unresolved.length}
    <p class="warn" title={unresolved.join("\n")}>
      <span class="mark">!</span> Incomplete: {unresolved.length} {unresolved.length === 1 ? "item is not" : "items are not"} in the ship data
      and {unresolved.length === 1 ? "is" : "are"} left out of these numbers.
    </p>
  {:else if !stats && note}
    <p class="lo">{note}</p>
  {/if}

  <div class="band">
    <div class="stat" aria-label="Speed">
      <div class="k">Speed (top | boost)</div>
      <div class="v"><span class="q">{n(speed, 0)}</span> <span class="sep">|</span> <span class="q">{n(boost, 0)}</span> <span class="u">m/s</span></div>
    </div>
    <div class="stat" aria-label="Pitch / roll / yaw" title="Hull stand-in at four pips to engines, until the ship calculations include handling">
      <div class="k">Approximate pitch / roll / yaw</div>
      <div class="v">
        <span class="q">{n(turn?.pitch, 0)}</span> <span class="sep">|</span>
        <span class="q">{n(turn?.roll, 0)}</span> <span class="sep">|</span>
        <span class="q">{n(turn?.yaw, 0)}</span> <span class="u">°/s</span>
      </div>
    </div>
    <div class="stat" aria-label="Jump">
      <div class="k">Jump min / current / max</div>
      <div class="v">
        <span class="q" title="Full tank and full cargo">{n(jump?.laden ?? headline?.jump, 2)}</span> <span class="sep">|</span>
        <span class="q" title="Full tank, no cargo">{n(jump?.unladen ?? headline?.jump, 2)}</span> <span class="sep">|</span>
        <span class="q" title="One jump's fuel, no cargo">{n(jump?.max ?? headline?.jump, 2)}</span> <span class="u">ly</span>
      </div>
    </div>
    <div class="stat power" aria-label="Power">
      <div class="k">Power</div>
      <div class="prow">
        <span>Retracted</span>
        <span class="pbar" aria-hidden="true"><i style:width={fill(power?.retracted, power?.generation)}></i></span>
        <span class="q">{n(power?.retracted, 2)} MW</span>
      </div>
      <div class="prow">
        <span>Deployed</span>
        <span class="pbar" aria-hidden="true"><i style:width={fill(power?.deployed, power?.generation)}></i></span>
        <span class="q">{n(power?.deployed, 2)} MW</span>
      </div>
      <div class="prow tot">
        <span>Total power</span>
        <span class="q">{n(power?.generation, 2)} MW</span>
      </div>
    </div>
  </div>

  {#if stats}
    <div class="grid">
      <div class="block" aria-label="Shields">
        <h4>Shields</h4>
        <div class="row"><span>Strength</span><span class="q">{stats.shield ? `${n(stats.shield.strength, 0)} MJ` : "–"}</span></div>
        <div class="row"><span>Explosive</span><span class="q">{stats.shield ? `${n(stats.shield.explosive * 100, 1)}%` : "–"}</span></div>
        <div class="row"><span>Kinetic</span><span class="q">{stats.shield ? `${n(stats.shield.kinetic * 100, 1)}%` : "–"}</span></div>
        <div class="row"><span>Thermal</span><span class="q">{stats.shield ? `${n(stats.shield.thermal * 100, 1)}%` : "–"}</span></div>
      </div>
      <div class="block" aria-label="Weapons and hull">
        <h4>Weapons and hull</h4>
        <div class="row"><span>DPS</span><span class="q">{n(stats.weapons.dps, 1)}</span></div>
        <div class="row"><span>Armour</span><span class="q">{n(stats.armour.integrity, 0)}</span></div>
        <div class="row"><span>Explosive</span><span class="q">{n(stats.armour.explosive * 100, 1)}%</span></div>
        <div class="row"><span>Kinetic</span><span class="q">{n(stats.armour.kinetic * 100, 1)}%</span></div>
        <div class="row"><span>Thermal</span><span class="q">{n(stats.armour.thermal * 100, 1)}%</span></div>
      </div>
      <div class="block" aria-label="Distributor">
        <h4>Distributor</h4>
        <div class="row"><span>WEP capacity</span><span class="q">{stats.distributor ? `${n(stats.distributor.weapons.capacity, 1)} MJ` : "–"}</span></div>
        <div class="row"><span>WEP recharge</span><span class="q">{stats.distributor ? `${n(stats.distributor.weapons.recharge, 1)} MW` : "–"}</span></div>
        <div class="row"><span>Time to drain</span><span class="q">{stats.time_to_drain == null ? "never" : `${Math.floor(stats.time_to_drain / 60)}:${String(Math.floor(stats.time_to_drain) % 60).padStart(2, "0")}`}</span></div>
        <div class="row"><span>SYS recharge</span><span class="q">{stats.distributor ? `${n(stats.distributor.systems.recharge, 1)} MW` : "–"}</span></div>
        <div class="row"><span>ENG recharge</span><span class="q">{stats.distributor ? `${n(stats.distributor.engines.recharge, 1)} MW` : "–"}</span></div>
      </div>
      <div class="block" aria-label="Mass">
        <h4>Mass</h4>
        <div class="row"><span>Hull</span><span class="q">{n(stats.hull_mass, 0)} t</span></div>
        <div class="row"><span>Unladen</span><span class="q">{n(stats.unladen_mass, 0)} t</span></div>
        <div class="row"><span>Laden</span><span class="q">{n(stats.laden_mass, 0)} t</span></div>
        <div class="row"><span>Cargo</span><span class="q">{n(stats.cargo_capacity, 0)} t</span></div>
        <div class="row"><span>Fuel</span><span class="q">{n(stats.fuel_capacity, 1)} t</span></div>
      </div>
    </div>
  {:else if headline}
    <div class="grid">
      <div class="block" aria-label="Shields">
        <h4>Shields</h4>
        <div class="row"><span>Strength</span><span class="q">{headline.shield != null ? `${n(headline.shield, 0)} MJ` : "–"}</span></div>
      </div>
      <div class="block" aria-label="Weapons and hull">
        <h4>Weapons and hull</h4>
        <div class="row"><span>DPS</span><span class="q">{n(headline.dps, 1)}</span></div>
      </div>
    </div>
  {/if}
</div>

<style>
  .specs { padding: 8px 8px 28px; display: flex; flex-direction: column; gap: 28px; }
  .lo { color: var(--text-lo); margin: 0; }
  .warn { margin: 0; line-height: 1.4; }
  .mark { color: var(--accent); font-weight: 600; }
  .band { display: grid; grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr)); gap: 20px 28px; padding: 8px 12px 4px; background: var(--cell-empty); }
  .stat { display: flex; flex-direction: column; gap: 8px; min-width: 0; }
  .k { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .v { display: flex; flex-wrap: wrap; align-items: baseline; gap: 6px; color: var(--text-hi); }
  .q { font-family: var(--mono); font-size: var(--fs-l); color: var(--accent); }
  .sep { color: var(--text-lo); }
  .u { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .power { grid-column: 1 / -1; }
  @media (min-width: 1100px) {
    .power { grid-column: auto; }
  }
  .prow { display: grid; grid-template-columns: 7rem minmax(6rem, 1fr) auto; gap: 10px; align-items: center; font-size: var(--fs-s); color: var(--text-lo); }
  .prow.tot { margin-top: 4px; }
  .prow .q { font-size: var(--fs); }
  .pbar { height: 6px; background: var(--cell-opt); }
  .pbar i { display: block; height: 100%; background: var(--accent); }
  .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr)); gap: 8px 28px; }
  .block { padding: 14px 16px; background: var(--cell-empty); display: flex; flex-direction: column; gap: 6px; }
  h4 { margin: 0 0 6px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-s); letter-spacing: var(--caps); text-transform: uppercase; }
  .row { display: flex; justify-content: space-between; gap: 12px; font-size: var(--fs-s); color: var(--text); }
  .row .q { font-size: var(--fs); }
</style>
