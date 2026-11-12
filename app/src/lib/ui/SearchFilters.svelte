<script lang="ts">
  // A lookup's filters as DESIGN.md's Collapsed filters: at rest one summary line (`summary`,
  // which the parent builds) with CHANGE, and no controls. CHANGE opens the controls in place
  // below the line; they stay open across edits and close only on DONE, Escape or CHANGE again,
  // so nothing the parent does (a search starting, finishing or failing) opens or closes them.
  // The controls show only the filters that apply (`offers`). Pad is S/M/L (S is any pad); the
  // ship you are flying is marked on its size. Carriers and Odyssey are a multiple choice of
  // what to include: filled is included.
  import { AGES, ARRIVALS, DISTANCES, VOLUMES, type FilterHints, type FilterKey, type Filters, type Pad } from "../search";
  import Button from "./Button.svelte";
  import Choice from "./Choice.svelte";
  import Dropdown from "./Dropdown.svelte";

  let {
    filters,
    offers,
    hints = {},
    shipPad,
    summary,
    onchange,
  }: {
    filters: Filters;
    offers: FilterKey[];
    hints?: FilterHints;
    /** The current ship's pad as of the latest search; `undefined`: as the filters loaded. */
    shipPad?: Pad | null;
    /** The constraints in force, one line: "Within 100 ly of Sol · large pad · reports up to 30 days old". */
    summary: string;
    onchange: (f: Filters) => void;
  } = $props();

  let open = $state(false);
  const id = $props.id();
  let toggle = $state<HTMLButtonElement | null>(null);

  /** DONE or Escape: close, and keep the keyboard on CHANGE so it can reopen. */
  function close() {
    open = false;
    toggle?.focus();
  }

  let root = $state<HTMLElement | null>(null);

  /** Escape from CHANGE or any control closes the controls; Escape elsewhere is someone else's. */
  function keydown(e: KeyboardEvent) {
    if (e.key !== "Escape" || !open || e.defaultPrevented || !(e.target instanceof Node) || !root?.contains(e.target)) return;
    e.preventDefault();
    close();
  }

  const shipSize = $derived<Pad>((shipPad === undefined ? filters.pad : shipPad) ?? "S");
  const padValue = $derived(!filters.pad_set ? shipSize : (filters.pad ?? "S"));

  function setPad(v: string) {
    const pad = v as Pad;
    if (pad === shipSize) onchange({ ...filters, pad_set: false });
    else onchange({ ...filters, pad_set: true, pad });
  }

  const num = (v: string): number | null => (v ? Number(v) : null);

  const included = $derived([...(filters.exclude_carriers ? [] : ["carriers"]), ...(filters.exclude_odyssey ? [] : ["odyssey"])]);
  const include = (v: string[]) => onchange({ ...filters, exclude_carriers: !v.includes("carriers"), exclude_odyssey: !v.includes("odyssey") });

  /**
   * The choices after the default: the usual ones without the default (it is the first option),
   * plus a saved value that isn't among them, so a select never shows blank.
   */
  const choices = (usual: number[], dflt: number | undefined, saved: number | null): number[] =>
    [...new Set([...usual, ...(saved == null ? [] : [saved])])].filter((v) => v !== dflt).sort((a, b) => a - b);
  /** A saved value equal to the default is the default's option. */
  const selected = (saved: number | null, dflt: number | undefined) => (saved == null || saved === dflt ? "" : saved);
</script>

<svelte:window onkeydown={keydown} />

<div class="search-filters" bind:this={root}>
  <p class="summary">
    <span class="text">{summary}</span>
    {#if offers.length}
      <Button bind:ref={toggle} aria-expanded={open} aria-controls={`${id}-filters`} aria-label="Change filters" onclick={() => (open = !open)}>Change</Button>
    {/if}
  </p>
  {#if open}
    <div class="filters" id={`${id}-filters`} role="group" aria-label="Filters">
      {#if offers.includes("distance")}
        <label>
          <span>Within</span>
          <Dropdown value={selected(filters.max_distance_ly, hints.distance)} onchange={(e) => onchange({ ...filters, max_distance_ly: num(e.currentTarget.value) })}>
            <option value="">{hints.distance ? `${hints.distance} ly` : "Default"}</option>
            {#each choices(DISTANCES, hints.distance, filters.max_distance_ly) as d (d)}<option value={d}>{d} ly</option>{/each}
          </Dropdown>
        </label>
      {/if}
      {#if offers.includes("volume")}
        <label>
          <span>{hints.volume ?? "Volume"}</span>
          <Dropdown value={filters.min_volume ?? ""} onchange={(e) => onchange({ ...filters, min_volume: num(e.currentTarget.value) })}>
            <option value="">Any</option>
            {#each VOLUMES as v (v)}<option value={v}>At least {v.toLocaleString("en")} t</option>{/each}
          </Dropdown>
        </label>
      {/if}
      {#if offers.includes("age")}
        <label>
          <span>Data age</span>
          <Dropdown value={selected(filters.max_age_days, hints.age)} onchange={(e) => onchange({ ...filters, max_age_days: num(e.currentTarget.value) })}>
            <option value="">{hints.age ? `Up to ${hints.age} days` : "Any"}</option>
            {#each choices(AGES, hints.age, filters.max_age_days) as d (d)}<option value={d}>Up to {d} day{d === 1 ? "" : "s"}</option>{/each}
          </Dropdown>
        </label>
      {/if}
      {#if offers.includes("pad")}
        <div class="field">
          <span>Pad</span>
          <Choice
            label="Pad"
            value={padValue}
            options={(["S", "M", "L"] as const).map((p) => ({
              value: p,
              label: p,
              title: `${shipSize === p ? "Your ship · " : ""}${p === "L" ? "L" : `${p} or larger`}`,
              ariaLabel: `Pad ${p}${shipSize === p ? ", current ship" : ""}`,
              mark: shipSize === p ? "ship" : undefined,
            }))}
            onchange={setPad}
          />
        </div>
      {/if}
      {#if offers.includes("arrival")}
        <label>
          <span>Arrival</span>
          <Dropdown value={filters.max_arrival_ls ?? ""} onchange={(e) => onchange({ ...filters, max_arrival_ls: num(e.currentTarget.value) })}>
            <option value="">Any distance</option>
            {#each ARRIVALS as ls (ls)}<option value={ls}>Up to {ls.toLocaleString("en")} ls</option>{/each}
          </Dropdown>
        </label>
      {/if}
      {#if offers.includes("carriers") || offers.includes("odyssey")}
        <div class="field">
          <span>Include</span>
          <Choice
            label="Include"
            value={included}
            options={[
              ...(offers.includes("carriers") ? [{ value: "carriers", label: "Carriers", title: "Include fleet carriers" }] : []),
              ...(offers.includes("odyssey") ? [{ value: "odyssey", label: "Odyssey", title: "Include Odyssey settlements" }] : []),
            ]}
            onchange={include}
          />
        </div>
      {/if}
      <Button onclick={close}>Done</Button>
    </div>
  {/if}
</div>

<style>
  .search-filters { display: flex; flex-direction: column; gap: var(--sp-2); min-width: 0; }
  .summary { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); margin: 0; font-size: var(--fs-s); color: var(--orange-mid); }
  .text { min-width: 0; }
  .filters { display: flex; flex-wrap: wrap; gap: 10px 18px; align-items: center; font-size: var(--fs-s); }
  label, .field { display: flex; align-items: center; gap: 8px; min-width: 0; }
  label > span, .field > span { color: var(--orange-mid); font-size: var(--fs-s); min-width: 5.5em; }
</style>
