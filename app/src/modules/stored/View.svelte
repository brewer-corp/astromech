<script lang="ts">
  // Ships > Stored modules (astromech-gvy0), on List with detail: the modules in storage grouped by
  // where each one is ("In transit" for one on its way), a group's count in its heading. A row is
  // the module's name over its blueprint and grade, with its value; the rows are grouped by place,
  // not ranked, so no figure is orange. Selecting one shows its facts in the side panel.
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { loadFleet, type Fleet } from "../../lib/engineering";
  import Hexes from "../../lib/ui/Hexes.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import ListDetail from "../../lib/ui/layout/ListDetail.svelte";
  import { tip } from "../../lib/ui/tip";

  type Module = Fleet["stored_modules"][number];

  /** The rows' full width, in px at 100 percent Text size: the panel goes below them when it can't sit beside. */
  const MIN = 464;
  const TRANSIT = "In transit";

  let fleet = $state<Fleet | null>(null);
  let fleetErr = $state<string | null>(null);
  /** The selected module, by key: the module itself, and which of identical ones it is. */
  let sel = $state<string | null>(null);

  $effect(() => {
    let key = "";
    const tick = () => {
      void loadFleet()
        .then((f) => {
          fleetErr = null;
          const k = JSON.stringify(f);
          if (k !== key) {
            key = k;
            fleet = f;
            if (sel !== null && !keyed(f.stored_modules).some((x) => x.k === sel)) sel = null;
          }
        })
        .catch((e: unknown) => (fleetErr = pollFailure("read the stored modules", e, fleet !== null)));
    };
    tick();
    const t = setInterval(tick, 1000);
    return () => clearInterval(t);
  });

  type Row = { m: Module; k: string };
  /** Each module with its key, which stays the same while it is in storage, whatever moves around it. */
  function keyed(mods: Module[]): Row[] {
    const seen: Record<string, number> = {};
    return mods.map((m) => {
      const id = JSON.stringify(m);
      const n = seen[id] ?? 0;
      seen[id] = n + 1;
      return { m, k: `${id}#${n}` };
    });
  }
  /** The modules by where they are, in the order each place first appears; in transit last. */
  const groups = $derived.by(() => {
    const out: [string, Row[]][] = [];
    for (const r of keyed(fleet?.stored_modules ?? [])) {
      const place = r.m.system ?? TRANSIT;
      const g = out.find(([p]) => p === place);
      if (g) g[1].push(r);
      else out.push([place, [r]]);
    }
    return out.sort(([a], [b]) => Number(a === TRANSIT) - Number(b === TRANSIT));
  });
  const chosen = $derived(sel === null ? null : (keyed(fleet?.stored_modules ?? []).find((r) => r.k === sel)?.m ?? null));

  function cr(n: number | null): string {
    if (n === null) return "–";
    if (n >= 1e6) return `${+(n / 1e6).toFixed(n >= 1e8 ? 0 : 1)}M`;
    if (n >= 1e3) return `${Math.round(n / 1e3)}k`;
    return String(n);
  }
  const grade = (m: Module) => (m.grade === null ? "grade not known" : `grade ${m.grade} of 5`);
</script>

<ListDetail panel={chosen ? about : null} selected={sel} min={MIN}>
  {#if !fleet && fleetErr}
    <!-- A hard error: the page reads again on its own, so the next read is the recovery (DESIGN.md States). -->
    <p class="type-body sub err" role="alert">{fleetErr}</p>
  {:else}
    <p class="type-label sub" {@attach tip(fleet?.source ?? null)}>
      {#if !fleet}
        Reading the journal…
      {:else}
        {fleet.stored_modules.length} stored {fleet.stored_modules.length === 1 ? "module" : "modules"}
      {/if}
    </p>
  {/if}
  <Stale error={fleet ? fleetErr : null} />
  {#if fleet && !fleet.stored_modules.length}
    <p class="type-body">No modules in storage.</p>
  {/if}
  {#if groups.length}
    <div class="row heads type-label" aria-hidden="true"><span>Module · blueprint</span><span class="v">Value</span></div>
  {/if}
  {#each groups as [place, mods] (place)}
    <section aria-label={place}>
      <h3 class="section" class:current={mods.some((x) => x.k === sel)}>{place}<span class="type-label count">{mods.length}</span></h3>
      <div class="rows">
        {#each mods as { m, k } (k)}
          <button class="row" class:sel={sel === k} aria-current={sel === k ? "true" : undefined} onclick={() => (sel = sel === k ? null : k)}>
            <span class="what">
              <span class="type-name name">{m.name}</span>
              {#if m.blueprint_name}
                <span class="type-label bp"><Hexes n={m.grade ?? 0} label={`${m.blueprint_name}, ${grade(m)}`} />{m.blueprint_name}</span>
              {/if}
            </span>
            <span class="type-figures v">{cr(m.value)}{#if m.value !== null}<span class="type-unit">cr</span>{/if}</span>
          </button>
        {/each}
      </div>
    </section>
  {/each}
</ListDetail>

<!-- DESIGN.md Side panel: the module's name in white, where it is, its value as the panel's main
     value, then its facts. -->
{#snippet about()}
  {#if chosen}
    <h2 class="type-title ptitle">{chosen.name}</h2>
    <p class="type-label psub">{chosen.system ?? TRANSIT}</p>
    {#if chosen.value !== null}
      <p class="pmain"><span class="type-value-xl main">{chosen.value.toLocaleString("en-GB")}</span><span class="type-unit">cr</span></p>
    {/if}
    <dl class="pfacts">
      <dt class="type-label">Where</dt>
      <dd class="type-figures">{#if chosen.system}<SystemChip name={chosen.system} />{:else}{TRANSIT}{/if}</dd>
      <dt class="type-label">Blueprint</dt>
      <dd class="type-figures">
        {#if chosen.blueprint_name}
          <Hexes n={chosen.grade ?? 0} />
          {chosen.blueprint_name}, {grade(chosen)}
        {:else}
          Not engineered
        {/if}
      </dd>
    </dl>
  {/if}
{/snippet}

<style>
  p { margin: 0; }
  .sub { margin-bottom: var(--sp-6); }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }
  section { margin-top: var(--sp-6); }
  section > .section { margin-bottom: var(--sp-3); }
  .count { margin-left: calc(-1 * var(--sp-1)); letter-spacing: 0; text-transform: none; }
  .rows { display: flex; flex-direction: column; gap: var(--gap); }
  /* One two-line row per module (DESIGN.md The Aligned Columns Rule): the name over its blueprint,
     then the value, right-aligned. */
  :where(button.row) { all: unset; box-sizing: border-box; cursor: pointer; }
  .row { display: grid; grid-template-columns: 20rem 6rem; column-gap: var(--sp-6); align-items: baseline; padding: var(--sp-2) var(--sp-3); }
  .row.heads { border-bottom: 1px solid var(--line); }
  button.row:hover { background: var(--row-hover); }
  button.row:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .row.sel, .row.sel:hover { background: var(--surface-hi); }
  .row.sel .name { color: var(--text-hi); }
  .what { display: flex; flex-direction: column; gap: var(--gap); min-width: 0; }
  .what > * { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .bp { display: flex; align-items: center; gap: var(--sp-2); }
  .v { text-align: right; }
  .type-figures.v { color: var(--options-grey); }
  .ptitle { margin: 0; }
  .psub { margin-top: var(--sp-1); }
  .pmain { margin-top: var(--sp-4); }
  .pfacts { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: var(--sp-2) var(--sp-6); margin: var(--sp-6) 0 0; }
  .pfacts dt { align-self: baseline; }
  .pfacts dd { margin: 0; display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2); }
</style>
