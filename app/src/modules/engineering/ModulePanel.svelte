<script lang="ts">
  // The Loadout's side panel: the selected module, how it is engineered now, the plan's target for
  // it with that target's materials (need against have, a short one marked "!") and the engineers
  // who apply its top grade, the plan's dropdowns for it, ADD TO PLAN and ENGINEERS.
  import { tip } from "../../lib/ui/tip";
  import Button from "../../lib/ui/Button.svelte";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
  import Hexes from "../../lib/ui/Hexes.svelte";
  import type { Fitted, Fleet } from "../../lib/engineering";
  import { gradeUp, maxGrade, type ShipPlan } from "./plan.svelte";

  let { fleet, m, sp }: { fleet: Fleet; m: Fitted; sp: ShipPlan } = $props();

  let picks = $state<HTMLElement>();
  const g = $derived(fleet.groups[m.group ?? ""]);
  const t = $derived(sp.target(m.slot));
  const cost = $derived(sp.cost(m.slot));
  const up = $derived(gradeUp(fleet, m, t));
  const bpName = (id: string | undefined) => g?.blueprints.find((b) => b.id === id)?.name ?? id ?? "";
  const xName = (id: string | null | undefined) => (id ? (g?.experimentals.find((x) => x.id === id)?.name ?? id) : "");
  /** The target's materials, each with what is held and whether the plan is short of it. */
  const mats = $derived(
    (cost?.materials ?? []).map(([id, need]) => {
      const line = sp.result?.materials.find((l) => l.id === id);
      return { id, name: line?.name ?? id, need, have: line?.have ?? 0, short: (line?.short ?? 0) > 0 };
    }),
  );
  const steps = $derived(cost ? [...cost.steps.map(([gr, n]) => `G${gr} × ${n}`), ...(cost.experimental_needed && cost.experimental_name ? [cost.experimental_name] : [])].join(" · ") : "");

  function add() {
    if (up === "pick") {
      // Not engineered and not planned: the commander picks the blueprint first.
      const sel = picks?.querySelector("select");
      sel?.focus();
      try {
        sel?.showPicker();
      } catch {
        // Not every engine opens a select from script; focus is enough there.
      }
      return;
    }
    if (up !== "top") sp.set(fleet, m, up.add);
  }
</script>

<div class="panel">
  <div class="title">
    <h3 class="type-title">{m.name}</h3>
    <p class="type-label">{m.slot_name}</p>
  </div>

  {#if g}
    <p class="now">
      {#if m.engineering}
        <Hexes n={m.engineering.grade} of={maxGrade(fleet, m, m.engineering.blueprint)} tone="grade" label="Grade {m.engineering.grade}" />
        <span>{m.engineering.blueprint_name}, grade {m.engineering.grade}{m.engineering.quality < 1 ? ` at ${Math.round(m.engineering.quality * 100)}%` : ""}{m.engineering.experimental_name ? ` · ${m.engineering.experimental_name}` : ""}</span>
      {:else}
        <span class="type-label">Not engineered</span>
      {/if}
    </p>

    {#if t}
      <div class="plan">
        <p class="type-label">Planned</p>
        <p><span class="type-value-xl main">G{t.grade}</span><span class="type-unit">{bpName(t.blueprint)}{t.experimental ? ` · ${xName(t.experimental)}` : ""}</span></p>
        {#if steps}<p class="type-label" {@attach tip("Rolls still to do at each grade, from the plan's rolls per grade")}>{steps}</p>{/if}
        {#if cost?.note}<p class="type-label">{cost.note}</p>{/if}
      </div>
    {/if}

    <!-- What can be changed comes before the materials, so a long list never pushes it off screen. -->
    <div class="picks" bind:this={picks}>
      <Dropdown aria-label="{m.slot_name} blueprint" value={t?.blueprint ?? ""} onchange={(e) => sp.set(fleet, m, e.currentTarget.value ? { blueprint: e.currentTarget.value } : null)}>
        <option value="">No change</option>
        {#each g.blueprints as b (b.id)}
          <option value={b.id}>{b.name}</option>
        {/each}
      </Dropdown>
      <Dropdown aria-label="{m.slot_name} grade" disabled={!t} value={t?.grade ?? ""} onchange={(e) => sp.set(fleet, m, { grade: Number(e.currentTarget.value) })}>
        {#if !t}<option value="">–</option>{/if}
        {#each Array.from({ length: maxGrade(fleet, m, t?.blueprint) }, (_, i) => i + 1) as n (n)}
          <option value={n}>G{n}</option>
        {/each}
      </Dropdown>
      <Dropdown aria-label="{m.slot_name} experimental" disabled={!t || !g.experimentals.length} value={t?.experimental ?? ""} onchange={(e) => sp.set(fleet, m, { experimental: e.currentTarget.value || null })}>
        <option value="">{g.experimentals.length ? "No experimental" : "–"}</option>
        {#each g.experimentals as x (x.id)}
          <option value={x.id}>{x.name}</option>
        {/each}
      </Dropdown>
    </div>

    <div class="acts">
      <Button kind="primary" disabled={up === "top"} onclick={add} {@attach tip(up === "top" ? "Already at its top grade" : up === "pick" ? "Choose a blueprint for it" : `Plan grade ${up.add.grade} of ${bpName(up.add.blueprint)}`)}>Add to plan</Button>
      <a class="type-action" href="#/unlocks?q={encodeURIComponent(g.name)}" {@attach tip(`Engineers for the ${g.name}`)}>Engineers</a>
    </div>

    {#if t}
      {#if mats.length}
        <div class="mats" role="table" aria-label="Materials for {m.name}">
          <div class="mrow" role="row">
            <span class="type-label" role="columnheader">Material</span><span class="type-label r" role="columnheader">Need</span><span class="type-label r" role="columnheader">Have</span>
          </div>
          {#each mats as x (x.id)}
            <div class="mrow" role="row">
              <span role="cell">{x.name}</span>
              <span class="type-figures r need" role="cell">{x.need}</span>
              <span class="type-figures r" class:short={x.short} role="cell">{#if x.short}<b {@attach tip("The plan needs more than you hold")}>!</b>{/if}{x.have}</span>
            </div>
          {/each}
        </div>
      {/if}
      {#if cost?.engineers.length}
        <p class="engs"><span class="type-label">Engineers</span> {cost.engineers.join(", ")}</p>
      {/if}
      {#if sp.failed}<p class="type-label">{sp.failed}</p>{/if}
    {/if}
  {/if}
</div>

<style>
  .panel { display: flex; flex-direction: column; gap: var(--sp-4); }
  .panel p { margin: 0; }
  .title { display: flex; flex-direction: column; gap: var(--sp-1); }
  .title h3 { margin: 0; }
  .now { display: flex; align-items: center; gap: var(--sp-2); color: var(--prose); }
  .plan { display: flex; flex-direction: column; gap: var(--sp-2); }
  .mats { display: flex; flex-direction: column; gap: var(--gap); }
  .mrow { display: grid; grid-template-columns: minmax(0, 1fr) 3rem 3rem; column-gap: var(--sp-3); align-items: baseline; }
  .mrow > :first-child { color: var(--options-grey); }
  .r { text-align: right; }
  /* A plan's need is Orange (DESIGN.md Orange: a plan); a short amount's "!" and have too. */
  .need { color: var(--accent); }
  .short, .short b { color: var(--accent); }
  .short b { font-weight: 600; margin-right: var(--sp-1); }
  .engs { color: var(--options-grey); }
  .picks { display: flex; flex-direction: column; gap: var(--gap); }
  .picks :global(.dropdown) { display: flex; }
  .acts { display: flex; align-items: center; gap: var(--sp-6); }
  .acts a { text-decoration: none; }
  .acts a:hover { color: var(--orange-lit); }
</style>
