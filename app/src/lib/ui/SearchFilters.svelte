<script lang="ts">
  // A lookup's filters, shown only where they apply (`offers`). Pad is S/M/L (S is any pad);
  // the ship you are flying is marked on its size. Carriers and Odyssey are include/exclude cells.
  import type { FilterHints, FilterKey, Filters, Pad } from "../search";
  import Seg from "./Seg.svelte";

  let {
    filters,
    offers,
    hints = {},
    shipPad,
    onchange,
  }: {
    filters: Filters;
    offers: FilterKey[];
    hints?: FilterHints;
    /** The current ship's pad as of the latest search; `undefined`: as the filters loaded. */
    shipPad?: Pad | null;
    onchange: (f: Filters) => void;
  } = $props();

  const AGES = [1, 3, 7, 30, 90, 365];
  const ARRIVALS = [1000, 5000, 25000];
  const DISTANCES = [20, 50, 100, 200, 500];
  const VOLUMES = [100, 500, 1000, 5000];

  const shipSize = $derived<Pad>((shipPad === undefined ? filters.pad : shipPad) ?? "S");
  const padValue = $derived(!filters.pad_set ? shipSize : (filters.pad ?? "S"));

  function setPad(v: string) {
    const pad = v as Pad;
    if (pad === shipSize) onchange({ ...filters, pad_set: false });
    else onchange({ ...filters, pad_set: true, pad });
  }

  const num = (v: string): number | null => (v ? Number(v) : null);

  /**
   * The choices after the default: the usual ones without the default (it is the first option),
   * plus a saved value that isn't among them, so a select never shows blank.
   */
  const choices = (usual: number[], dflt: number | undefined, saved: number | null): number[] =>
    [...new Set([...usual, ...(saved == null ? [] : [saved])])].filter((v) => v !== dflt).sort((a, b) => a - b);
  /** A saved value equal to the default is the default's option. */
  const selected = (saved: number | null, dflt: number | undefined) => (saved == null || saved === dflt ? "" : saved);
</script>

<div class="filters">
  {#if offers.includes("distance")}
    <label>
      <span>Within</span>
      <select value={selected(filters.max_distance_ly, hints.distance)} onchange={(e) => onchange({ ...filters, max_distance_ly: num(e.currentTarget.value) })}>
        <option value="">{hints.distance ? `${hints.distance} ly` : "Default"}</option>
        {#each choices(DISTANCES, hints.distance, filters.max_distance_ly) as d (d)}<option value={d}>{d} ly</option>{/each}
      </select>
    </label>
  {/if}
  {#if offers.includes("volume")}
    <label>
      <span>{hints.volume ?? "Volume"}</span>
      <select value={filters.min_volume ?? ""} onchange={(e) => onchange({ ...filters, min_volume: num(e.currentTarget.value) })}>
        <option value="">Any</option>
        {#each VOLUMES as v (v)}<option value={v}>At least {v.toLocaleString("en")} t</option>{/each}
      </select>
    </label>
  {/if}
  {#if offers.includes("age")}
    <label>
      <span>Data age</span>
      <select value={selected(filters.max_age_days, hints.age)} onchange={(e) => onchange({ ...filters, max_age_days: num(e.currentTarget.value) })}>
        <option value="">{hints.age ? `Up to ${hints.age} days` : "Any"}</option>
        {#each choices(AGES, hints.age, filters.max_age_days) as d (d)}<option value={d}>Up to {d} day{d === 1 ? "" : "s"}</option>{/each}
      </select>
    </label>
  {/if}
  {#if offers.includes("pad")}
    <Seg
      label="Pad"
      value={padValue}
      options={[
        { value: "S", label: "S", title: shipSize === "S" ? "Your ship · S or larger" : "S or larger", mark: shipSize === "S" ? "ship" : undefined },
        { value: "M", label: "M", title: shipSize === "M" ? "Your ship · M or larger" : "M or larger", mark: shipSize === "M" ? "ship" : undefined },
        { value: "L", label: "L", title: shipSize === "L" ? "Your ship · L" : "L", mark: shipSize === "L" ? "ship" : undefined },
      ]}
      onchange={setPad}
    />
  {/if}
  {#if offers.includes("arrival")}
    <label>
      <span>Arrival</span>
      <select value={filters.max_arrival_ls ?? ""} onchange={(e) => onchange({ ...filters, max_arrival_ls: num(e.currentTarget.value) })}>
        <option value="">Any distance</option>
        {#each ARRIVALS as ls (ls)}<option value={ls}>Up to {ls.toLocaleString("en")} ls</option>{/each}
      </select>
    </label>
  {/if}
  {#if offers.includes("carriers") || offers.includes("odyssey")}
    <div class="togs">
      {#if offers.includes("carriers")}
        <button type="button" class="tog" class:on={!filters.exclude_carriers} aria-pressed={!filters.exclude_carriers} onclick={() => onchange({ ...filters, exclude_carriers: !filters.exclude_carriers })}>{filters.exclude_carriers ? "No carriers" : "Carriers"}</button>
      {/if}
      {#if offers.includes("odyssey")}
        <button type="button" class="tog" class:on={!filters.exclude_odyssey} aria-pressed={!filters.exclude_odyssey} onclick={() => onchange({ ...filters, exclude_odyssey: !filters.exclude_odyssey })}>{filters.exclude_odyssey ? "No Odyssey" : "Odyssey"}</button>
      {/if}
    </div>
  {/if}
</div>

<style>
  .filters { display: flex; flex-wrap: wrap; gap: 10px 18px; align-items: center; font-size: var(--fs-s); }
  label { display: flex; align-items: center; gap: 8px; min-width: 0; }
  label > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; }
  select { all: unset; min-width: 8rem; height: 30px; padding: 0 8px; background: var(--cell); color: var(--accent); cursor: pointer; }
  .togs { display: flex; flex-wrap: wrap; gap: var(--gap); }
  .tog { all: unset; cursor: pointer; box-sizing: border-box; height: 30px; padding: 0 12px; background: var(--cell-opt); color: var(--text); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; }
  .tog:hover { color: var(--text-hi); }
  .tog.on { background: var(--cell); color: var(--accent); }
</style>
