<script lang="ts">
  // The live system view (OD Explorer's system page): the bodies of the system you are in as you
  // scan them, with what each is worth now and mapped, which are worth mapping, and the facts
  // that decide where to fly (terraformable, landable, rings, gravity, signals).
  import Badge from "../../lib/ui/Badge.svelte";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { credits, loadSystem, loadUnsold, type Body, type SystemView, type Unsold } from "../../lib/exploration";
  import { when } from "../../lib/logbook";
  import { serviceRoute } from "../../lib/places";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  let view = $state<SystemView | null>(null);
  let failed = $state<string | null>(null);

  // Follows the journal: bodies appear as they are scanned.
  $effect(() => {
    const tick = () =>
      loadSystem()
        .then((v) => {
          view = v;
          failed = null;
        })
        .catch((e: unknown) => (failed = pollFailure("read the system", e, view !== null)));
    void tick();
    const t = setInterval(() => void tick(), 3000);
    return () => clearInterval(t);
  });

  // The data aboard, from the history (it changes on a scan or a sale, not every second).
  let unsold = $state<Unsold | null>(null);
  let unsoldLoaded = $state(false);
  let unsoldFailed = $state<string | null>(null);
  $effect(() => {
    const tick = () =>
      loadUnsold()
        .then((u) => {
          unsold = u;
          unsoldLoaded = true;
          unsoldFailed = null;
        })
        .catch((e: unknown) => (unsoldFailed = pollFailure("read the unsold data", e, unsold !== null)));
    void tick();
    const t = setInterval(() => void tick(), 15000);
    return () => clearInterval(t);
  });

  // Where to sell it: links into the Services finder (decision astromech-g8d5, item 5).
  const sellAt = [
    { route: serviceRoute("universal_cartographics"), label: "Universal Cartographics", what: "cartographic data" },
    { route: serviceRoute("vista_genomics"), label: "Vista Genomics", what: "organic data" },
  ];

  /** Short facts for the body's row. */
  function facts(b: Body): string[] {
    const f: string[] = [];
    if (b.terraformable && b.class !== "Earthlike body") f.push("Terraformable");
    if (b.landable) f.push("Landable");
    if (b.rings) f.push("Rings");
    if (b.belts) f.push("Belt");
    if (b.bio_signals) f.push(`Bio ${b.bio_signals}`);
    if (b.geo_signals) f.push(`Geo ${b.geo_signals}`);
    if (b.first_discovery) f.push("First discovery");
    return f;
  }

  const scanned = $derived(view ? view.bodies.length : 0);
</script>

<section>
  <Stale error={view ? failed : null} />
  {#if failed && !view}
    <p class="err">{failed}</p>
  {:else if !view}
    <p class="lo">Reading the journal…</p>
  {:else if !view.system}
    <p class="lo">No system in the journal yet. Jump somewhere and its bodies show here as you scan them.</p>
  {:else}
    <p class="sub head">
      <SystemChip name={view.system} />
      <span>
        {scanned}{view.body_count !== null ? ` of ${view.body_count}` : ""} bodies scanned{view.all_found ? " · all found" : ""}
      </span>
      {#if view.source}<span class="lo">· {view.source}</span>{/if}
    </p>
    <div class="tiles">
      <div><span class="k">Value now</span><span class="v">{credits(view.value)}</span></div>
      <div><span class="k">Mapping the best</span><span class="v">{credits(view.value_if_mapped)}</span></div>
    </div>

    {#if view.bodies.length}
      <table aria-label="Bodies">
        <thead>
          <tr><th>Body</th><th>Class</th><th class="num">Distance</th><th class="num">Gravity</th><th></th><th class="num">Value</th><th class="num">Mapped</th><th></th></tr>
        </thead>
        <tbody>
          {#each view.bodies as b (b.name)}
            <tr class:worth={b.worth_mapping}>
              <td class="name">{b.short_name}</td>
              <td>{b.class_name}</td>
              <td class="num">{b.distance_ls === null ? "–" : `${Math.round(b.distance_ls).toLocaleString()} ls`}</td>
              <td class="num">{b.gravity_g === null ? "" : `${b.gravity_g.toFixed(2)} g`}</td>
              <td><div class="facts">{#each facts(b) as f (f)}<Badge>{f}</Badge>{/each}</div></td>
              <td class="num">{credits(b.value)}</td>
              <td class="num">{b.star ? "" : credits(b.value_mapped)}</td>
              <td><div class="note">
                {#if b.worth_mapping}<Badge>Worth mapping</Badge>{:else if b.mapped}<span class="lo">Mapped{b.mapped_efficiently ? ", efficiently" : ""}</span>{/if}
                {#if b.first_footfall_likely}<Badge>First footfall likely</Badge>{/if}
                {#if b.landings}<span class="lo">Landed {b.landings === 1 ? "once" : `${b.landings} times`}</span>{/if}
              </div></td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="lo">Nothing scanned here yet. Honk and use the FSS: bodies show here as you scan them.</p>
    {/if}
    <p class="lo note">
      Values are estimates from the community's exploration value formulae, the same ones EDSM and OD Explorer use: "Mapped" is
      with the Detailed Surface Scanner within the efficiency target. "Worth mapping" is a planet worth at least 500 k cr mapped.
      "First footfall likely" is a landable body nobody had discovered in an unpopulated system; the journal can't say who
      walked there first.
    </p>
  {/if}

  <h3 class="section">Unsold data</h3>
  <Stale error={unsold ? unsoldFailed : null} />
  {#if unsoldFailed && !unsold}
    <p class="err">{unsoldFailed}</p>
  {:else if unsold}
    <div class="tiles">
      <div>
        <span class="k">Cartographic</span><span class="v">{credits(unsold.cartographic)}</span>
        <span class="lo small">{unsold.bodies.toLocaleString()} bodies in {unsold.systems.toLocaleString()} systems</span>
      </div>
      <div>
        <span class="k">Organic</span><span class="v">{credits(unsold.organic)}</span>
        <span class="lo small">{unsold.samples} samples{unsold.unknown_samples ? `, ${unsold.unknown_samples} of unknown value` : ""}</span>
      </div>
      <div><span class="k">Total</span><span class="v">{credits(unsold.total)}</span></div>
    </div>
    <p class="sub">
      CMDR {unsold.commander.name}{unsold.since ? ` · unsold since ${when(unsold.since)} UTC` : ""}{unsold.source ? ` · ${unsold.source}` : ""}
    </p>
    {#if unsold.risky}
      <p class="risk" role="alert"><b>!</b> {credits(unsold.total)} aboard. Dying loses all of it: sell it at the nearest station (links below).</p>
    {/if}
  {:else if unsoldLoaded}
    <p class="lo">No commander in the journal history yet.</p>
  {:else}
    <p class="lo">Reading the journal history…</p>
  {/if}

  <h3 class="section">Where to sell</h3>
  <ul class="sell">
    {#each sellAt as l (l.route)}
      <li><a href="#/{l.route}">Nearest {l.label}</a> <span class="lo">for {l.what}</span></li>
    {/each}
  </ul>
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  .sub { color: var(--orange-mid); font-size: var(--fs-s); margin: 14px 0 14px; }
  .head { display: flex; flex-wrap: wrap; align-items: baseline; gap: 10px; }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }
  .lo { color: var(--orange-mid); }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); margin-bottom: 18px; }
  .tiles div { display: flex; flex-direction: column; gap: 4px; min-width: 11rem; padding: 10px 14px; background: var(--surface); } /* read-only tiles: hull */
  .k { color: var(--orange-mid); font-size: var(--fs-s); }
  .v { color: var(--orange-lit); font-family: var(--mono); font-size: var(--fs-l); }
  table { border-collapse: collapse; width: 100%; }
  th { text-align: left; color: var(--orange-mid); font-weight: 400; font-size: var(--fs-s); padding: 0 14px 6px 0; }
  td { padding: 5px 14px 5px 0; border-top: 1px solid var(--line); vertical-align: baseline; }
  .num { text-align: right; font-family: var(--mono); white-space: nowrap; }
  .name { font-weight: 500; white-space: nowrap; }
  .facts { display: flex; flex-wrap: wrap; gap: 4px; }
  .note { display: flex; flex-wrap: wrap; gap: 6px; font-size: var(--fs-s); }
  tr.worth .name { color: var(--orange-lit); }
  p.note { margin-top: 18px; font-size: var(--fs-s); max-width: 52rem; }
  h3 { margin: 28px 0 10px; }
  .small { font-size: var(--fs-s); }
  /* A warning: an orange "!" and plain words, never a filled block (DESIGN.md States). */
  .risk { color: var(--accent); max-width: 52rem; }
  .risk b { font-weight: 600; }
  .sell { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 8px; }
  /* Links to somewhere else (DESIGN.md Buttons → Links). */
  .sell a { color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .sell a:hover { color: var(--orange-lit); }
</style>
