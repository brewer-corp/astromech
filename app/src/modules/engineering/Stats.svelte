<script lang="ts">
  // A ship's stats as fitted ("Now"), with its build plan done ("Plan"), and the difference.
  // Numbers use the precision Coriolis shows, so they can be checked against it.
  import type { ShipStats, StatsComparison } from "../../lib/engineering";

  let {
    stats,
    note,
    unknownHull = null,
  }: { stats: StatsComparison | null; note: string | null; unknownHull?: string | null } = $props();

  interface Row {
    label: string;
    /** Longer explanation, shown on hover. */
    title?: string;
    get: (s: ShipStats) => number | null | undefined;
    digits: number;
    /** Lower is better (mass, draw). */
    lower?: boolean;
    /** Shown as minutes:seconds; null reads as `nullText`. */
    time?: boolean;
    pct?: boolean;
    nullText?: string;
  }

  const pct = (x: number | undefined) => (x === undefined ? undefined : x * 100);

  const groups: { title: string; rows: Row[] }[] = [
    {
      title: "Jump range (ly)",
      rows: [
        { label: "Max", title: "One jump's fuel aboard, no cargo", get: (s) => s.jump?.max, digits: 2 },
        { label: "Full tank", get: (s) => s.jump?.unladen, digits: 2 },
        { label: "Laden", title: "Full tank and full cargo", get: (s) => s.jump?.laden, digits: 2 },
        { label: "Total", title: "Jumping until the tank is empty, no cargo", get: (s) => s.jump?.total_unladen, digits: 2 },
      ],
    },
    {
      title: "Shields",
      rows: [
        { label: "Strength (MJ)", get: (s) => s.shield?.strength, digits: 0 },
        { label: "Explosive", get: (s) => pct(s.shield?.explosive), digits: 1, pct: true },
        { label: "Kinetic", get: (s) => pct(s.shield?.kinetic), digits: 1, pct: true },
        { label: "Thermal", get: (s) => pct(s.shield?.thermal), digits: 1, pct: true },
      ],
    },
    {
      title: "Distributor",
      rows: [
        { label: "WEP capacity (MJ)", get: (s) => s.distributor?.weapons.capacity, digits: 1 },
        { label: "WEP recharge (MW)", title: "Four pips to weapons", get: (s) => s.distributor?.weapons.recharge, digits: 1 },
        { label: "Weapon draw (MW)", title: "All weapons firing, counting reloads", get: (s) => s.weapons.sustained_eps, digits: 1, lower: true },
        {
          label: "Time to drain",
          title: "Firing everything with four pips to weapons; never: the recharge keeps up",
          get: (s) => s.time_to_drain,
          digits: 0,
          time: true,
          nullText: "never",
        },
        { label: "SYS recharge (MW)", get: (s) => s.distributor?.systems.recharge, digits: 1 },
        { label: "ENG recharge (MW)", get: (s) => s.distributor?.engines.recharge, digits: 1 },
      ],
    },
    {
      title: "Weapons and hull",
      rows: [
        { label: "DPS", title: "Damage per second while firing", get: (s) => s.weapons.dps, digits: 1 },
        { label: "Armour", get: (s) => s.armour.integrity, digits: 0 },
        { label: "Explosive", get: (s) => pct(s.armour.explosive), digits: 1, pct: true },
        { label: "Kinetic", get: (s) => pct(s.armour.kinetic), digits: 1, pct: true },
        { label: "Thermal", get: (s) => pct(s.armour.thermal), digits: 1, pct: true },
      ],
    },
    {
      title: "Ship",
      rows: [
        { label: "Speed (m/s)", get: (s) => s.speed, digits: 0, nullText: "–" },
        { label: "Boost (m/s)", get: (s) => s.boost, digits: 0, nullText: "–" },
        { label: "Mass (t)", title: "Unladen, full tank", get: (s) => s.unladen_mass, digits: 0, lower: true },
        { label: "Power use (MW)", title: "Hardpoints deployed", get: (s) => s.power.deployed, digits: 2, lower: true },
        { label: "Power plant (MW)", get: (s) => s.power.generation, digits: 2 },
      ],
    },
  ];

  /** Coriolis rounds times down to whole seconds. */
  function clock(sec: number): string {
    const t = Math.floor(Math.abs(sec));
    return `${sec < 0 ? "-" : ""}${Math.floor(t / 60)}:${String(t % 60).padStart(2, "0")}`;
  }

  function show(r: Row, v: number | null | undefined): string {
    if (v === null || v === undefined) return r.nullText ?? "–";
    return r.time ? clock(v) : v.toFixed(r.digits);
  }

  /** Planned minus current at the shown precision; blank when it rounds to nothing. */
  function delta(r: Row, now: number | null | undefined, plan: number | null | undefined): { text: string; better: boolean } {
    if (now == null || plan == null) return { text: "", better: false };
    const d = r.time ? Math.floor(plan) - Math.floor(now) : Number(plan.toFixed(r.digits)) - Number(now.toFixed(r.digits));
    if (Math.abs(d) < 10 ** -r.digits / 2) return { text: "", better: false };
    const sign = d > 0 ? "+" : "-";
    return { text: sign + (r.time ? clock(Math.abs(d)) : Math.abs(d).toFixed(r.digits)), better: r.lower ? d < 0 : d > 0 };
  }

  const unresolved = $derived([...new Set([...(stats?.current.unresolved ?? []), ...(stats?.planned.unresolved ?? [])])]);
</script>

<div class="stats">
  {#if unknownHull}
    <p class="warn" title="Hull: {unknownHull}">
      <span class="mark">!</span> Incomplete: this hull isn't in the ship data, so there are no stats for it.
    </p>
  {:else if !stats}
    <p class="lo">{note ?? "Working out the stats…"}</p>
  {:else}
    {#if unresolved.length}
      <p class="warn" title={unresolved.join("\n")}>
        <span class="mark">!</span> Incomplete: {unresolved.length} {unresolved.length === 1 ? "item isn't" : "items aren't"} in the ship data
        and {unresolved.length === 1 ? "is" : "are"} left out of these numbers.
      </p>
    {/if}
    <table>
      <thead>
        <tr><th></th><th scope="col">Now</th><th scope="col">Plan</th><th scope="col">Change</th></tr>
      </thead>
      {#each groups as g (g.title)}
        <tbody aria-label={g.title}>
          <tr class="group"><th colspan="4" scope="rowgroup">{g.title}</th></tr>
          {#each g.rows as r (r.label)}
            {@const now = r.get(stats.current)}
            {@const plan = r.get(stats.planned)}
            {@const d = delta(r, now, plan)}
            <tr>
              <th scope="row" title={r.title}>{r.label}</th>
              <td>{show(r, now)}</td>
              <td class:changed={d.text !== ""}>{show(r, plan)}</td>
              <td class="d" class:better={d.better}>{d.text}</td>
            </tr>
          {/each}
        </tbody>
      {/each}
    </table>
  {/if}
</div>

<style>
  .stats { font-size: var(--fs-s); }
  .lo { color: var(--text-lo); margin: 0; }
  .warn { margin: 0 0 8px; line-height: 1.4; }
  .mark { color: var(--accent); font-weight: 600; }
  table { border-collapse: collapse; width: 100%; }
  th, td { padding: 5px 0; }
  thead th { color: var(--text-lo); font-weight: 400; text-transform: uppercase; letter-spacing: var(--caps); text-align: right; padding-left: 12px; }
  th[scope="row"] { text-align: left; font-weight: 400; color: var(--text); white-space: nowrap; padding-right: 12px; }
  .group th { text-align: left; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); padding: 16px 0 6px; }
  tbody:first-of-type .group th { padding-top: 4px; }
  td { font-family: var(--mono); text-align: right; color: var(--text-lo); white-space: nowrap; padding-left: 12px; }
  td.changed { color: var(--text-hi); }
  td.d { color: var(--text); }
  td.better { color: var(--accent); }
</style>
