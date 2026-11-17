<script lang="ts" module>
  /**
   * The width of one group of rows, in px at 100 percent Text size: class 40, name 240, grade 76,
   * blueprint 256, three 12 px gaps and 12 px padding each side (the grid below, in rem). Beside
   * the side panel the loadout needs at least this, or the panel goes below it.
   */
  export const ROW_MIN = 672;
  /** Two groups side by side, 48 px apart. */
  const TWO = ROW_MIN * 2 + 48;
</script>

<script lang="ts">
  // A ship's loadout as the game's outfitting lists it: Hardpoints, Utility, Core and Optional,
  // one row per fitted module (class and rating, name, grade as engineer hexagons, then the
  // blueprint and experimental). A row selects the module for the side panel. Two columns of
  // groups where two fit beside the panel; worked out as if the panel were there, so selecting a
  // module never moves the rows.
  import { besidePanel } from "../../lib/ui/layout/layout";
  import { useDetail } from "../../lib/ui/layout/listDetail.svelte";
  import Hexes from "../../lib/ui/Hexes.svelte";
  import { loadoutGroups, type Fitted, type Fleet, type Ship } from "../../lib/engineering";
  import { maxGrade, type ShipPlan } from "./plan.svelte";

  let {
    fleet,
    ship,
    sp,
    selected,
    withPanel,
    onselect,
  }: { fleet: Fleet; ship: Ship; sp: ShipPlan; selected: string | null; withPanel: boolean; onselect: (m: Fitted) => void } = $props();

  const detail = useDetail();
  const groups = $derived(loadoutGroups(ship.modules));
  const room = $derived(detail ? (withPanel ? detail.room : besidePanel(detail.room)) : 0);
  const columns = $derived(room >= TWO && groups.length > 1 ? 2 : 1);
  /** Hardpoints and Utility in the first column, Core and Optional in the second. */
  const split = $derived(columns === 2 ? [groups.filter((g) => g.title === "Hardpoints" || g.title === "Utility"), groups.filter((g) => g.title === "Core" || g.title === "Optional")].filter((c) => c.length) : [groups]);

  /** "7A" and "Power Plant" from "7A Power Plant"; armour has no class. */
  function parts(name: string): [string, string] {
    const m = /^(\d+[A-I])\s+(.+)$/.exec(name);
    return m ? [m[1]!, m[2]!] : ["", name];
  }

  function grade(m: Fitted) {
    const t = sp.target(m.slot);
    const e = m.engineering;
    // The hexagons are the fitted blueprint's: a plan on the same blueprint lights the grades it
    // reaches; a plan for another blueprint (which starts again at grade 1) is named beside them.
    const same = !!t && (!e || t.blueprint === e.blueprint);
    const now = e?.grade ?? 0;
    const of = maxGrade(fleet, m, same ? t.blueprint : e?.blueprint);
    const other = t && !same ? (fleet.groups[m.group ?? ""]?.blueprints.find((b) => b.id === t.blueprint)?.name ?? t.blueprint) : null;
    const planned = t ? `, ${other ? `${other} ` : ""}grade ${t.grade} planned` : "";
    return { now, to: same ? t.grade : 0, of, label: `Grade ${now} of ${of}${planned}`, shown: !!e || !!t, other, otherGrade: t?.grade };
  }
</script>

<div class="loadout" class:two={columns === 2}>
  {#each split as col, c (c)}
    <div class="col">
      {#each col as g (g.title)}
        <section aria-label={g.title}>
          <h3 class="section" class:current={g.mods.some((m) => m.slot === selected)}>{g.title}</h3>
          <div class="rows">
            {#each g.mods as m (m.slot)}
              {@const [cls, name] = parts(m.name)}
              {@const gr = grade(m)}
              <button class="row" class:sel={m.slot === selected} aria-pressed={m.slot === selected} aria-label="{m.slot_name}: {m.name}" onclick={() => onselect(m)}>
                <span class="type-figures cls">{cls}</span>
                <span class="type-name name">{name}</span>
                <span class="hx">{#if gr.shown}<Hexes n={gr.now} to={gr.to} of={gr.of} tone="grade" label={gr.label} />{/if}</span>
                <span class="bp">
                  {#if m.engineering}{m.engineering.blueprint_name}{#if m.engineering.experimental_name}<span class="x">· {m.engineering.experimental_name}</span>{/if}{/if}{#if gr.other}<span class="x planned">→ {gr.other} G{gr.otherGrade}</span>{/if}
                </span>
              </button>
            {/each}
          </div>
        </section>
      {/each}
    </div>
  {/each}
</div>

<style>
  .loadout { display: grid; grid-template-columns: max-content; gap: var(--sp-8) var(--sp-12); align-items: start; }
  .loadout.two { grid-template-columns: max-content max-content; }
  .col { display: flex; flex-direction: column; gap: var(--sp-8); }
  section { display: flex; flex-direction: column; gap: var(--sp-3); }
  .rows { display: flex; flex-direction: column; gap: var(--gap); }
  /* A read-only row on Void; the selected one Hull Raised with its name White (DESIGN.md
     Selected row). Grades are the engineer hexagons (DESIGN.md Engineer rank and blueprint grade). */
  .row {
    all: unset; box-sizing: border-box; cursor: pointer;
    display: grid; grid-template-columns: 2.5rem 15rem 4.75rem 16rem; column-gap: var(--sp-3); align-items: center;
    min-height: var(--row); padding: var(--sp-1) var(--sp-3); outline: 1px solid var(--line); outline-offset: -1px;
  }
  .row:hover { background: var(--row-hover); }
  .row:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .row.sel, .row.sel:hover { background: var(--surface-hi); }
  .row.sel .name { color: var(--text-hi); }
  .row.sel .cls, .row.sel .bp { color: var(--options-grey); }
  .bp { color: var(--prose); }
  .x { margin-left: var(--sp-1); }
  /* A plan for another blueprint: Orange Mid, as a planned grade not reached (DESIGN.md's list). */
  .planned { color: var(--orange-mid); }
</style>
