<script lang="ts">
  // A lookup's filters, shown only where they apply (`offers`). The pad follows the current
  // ship until the commander picks one.
  import type { FilterKey, Filters, Pad } from "../search";

  let { filters, offers, onchange }: { filters: Filters; offers: FilterKey[]; onchange: (f: Filters) => void } = $props();

  const AGES = [1, 3, 7, 30, 90];
  const ARRIVALS = [1000, 5000, 25000];

  const padValue = $derived(!filters.pad_set ? "ship" : (filters.pad ?? "any"));

  function setPad(v: string) {
    if (v === "ship") onchange({ ...filters, pad_set: false });
    else onchange({ ...filters, pad_set: true, pad: v === "any" ? null : (v as Pad) });
  }

  const num = (v: string): number | null => (v ? Number(v) : null);
</script>

<div class="filters">
  {#if offers.includes("age")}
    <label>
      <span>Data age</span>
      <select value={filters.max_age_days ?? ""} onchange={(e) => onchange({ ...filters, max_age_days: num(e.currentTarget.value) })}>
        <option value="">Any</option>
        {#each AGES as d (d)}<option value={d}>Up to {d} day{d === 1 ? "" : "s"}</option>{/each}
      </select>
    </label>
  {/if}
  {#if offers.includes("pad")}
    <label>
      <span>Pad</span>
      <select value={padValue} onchange={(e) => setPad(e.currentTarget.value)}>
        <option value="ship">Ship's{!filters.pad_set ? ` (${filters.pad ?? "any"})` : ""}</option>
        <option value="S">S or larger</option>
        <option value="M">M or larger</option>
        <option value="L">L</option>
        <option value="any">Any</option>
      </select>
    </label>
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
  {#if offers.includes("carriers")}
    <label class="check">
      <input type="checkbox" checked={filters.exclude_carriers} onchange={(e) => onchange({ ...filters, exclude_carriers: e.currentTarget.checked })} />
      <span>No fleet carriers</span>
    </label>
  {/if}
  {#if offers.includes("odyssey")}
    <label class="check">
      <input type="checkbox" checked={filters.exclude_odyssey} onchange={(e) => onchange({ ...filters, exclude_odyssey: e.currentTarget.checked })} />
      <span>No Odyssey settlements</span>
    </label>
  {/if}
</div>

<style>
  .filters { display: flex; flex-direction: column; gap: 4px; margin: 0 0 10px; font-size: var(--fs-s); }
  label { display: flex; align-items: center; gap: 8px; }
  label > span { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; }
  label.check > span { text-transform: none; letter-spacing: normal; color: var(--text); min-width: 0; }
  select { all: unset; flex: 1; height: 26px; padding: 0 8px; background: var(--cell); color: var(--accent); cursor: pointer; }
  input[type="checkbox"] { accent-color: var(--accent); margin: 0; }
</style>
