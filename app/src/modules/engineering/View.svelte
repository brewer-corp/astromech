<script lang="ts">
  // Build planner: pick a ship from the journal fleet, set a blueprint / grade / experimental per
  // module, and see the materials it costs against the inventory, trades and what to farm.
  import {
    DEFAULT_ROLLS, evaluatePlan, loadEngineers, loadFleet, planTrip, savePlan, savedPlan, shipTitle,
    type Engineers, type Fitted, type Fleet, type MatLine, type Plan, type PlanResult, type Target, type Trip,
  } from "../../lib/engineering";
  import { jumps } from "../../lib/galaxy";
  import Nearby from "../../lib/ui/Nearby.svelte";

  let fleet = $state<Fleet | null>(null);
  let shipId = $state<number | null>(null);
  let plan = $state<Plan | null>(null);
  let result = $state<PlanResult | null>(null);
  let failed = $state<string | null>(null);
  let selMat = $state<string | null>(null);
  let seq = 0;
  let engineers = $state<Engineers | null>(null);
  let trip = $state<Trip | null>(null);
  let tripErr = $state<string | null>(null);

  const ship = $derived(fleet?.ships.find((s) => s.id === shipId) ?? fleet?.ships[0]);
  const mods = $derived(ship?.modules.filter((m) => m.group && fleet?.groups[m.group]) ?? []);
  const sel = $derived(result?.materials.find((m) => m.id === selMat));
  const selFarm = $derived(result?.farm.find((f) => f.id === selMat));
  const usedBy = $derived(result?.targets.filter((t) => t.materials.some(([id]) => id === selMat)) ?? []);
  const traded = $derived(
    (result?.trades ?? []).reduce((m, t) => m.set(t.get_id, (m.get(t.get_id) ?? 0) + t.get), new Map<string, number>()),
  );

  const hasJobs = $derived(result?.targets.some((t) => t.steps.length > 0 || t.experimental_needed) ?? false);
  const lookup = $derived((selFarm ?? sel)?.sources?.find((s) => s.query)?.query);

  $effect(() => {
    const tick = () => {
      void loadFleet().then((f) => (fleet = f));
      void loadEngineers().then((e) => (engineers = e)).catch(() => {});
    };
    tick();
    const t = setInterval(tick, 5000);
    return () => clearInterval(t);
  });

  // New ship (or first load): restore its plan.
  $effect(() => {
    const id = ship?.id;
    if (id === undefined || plan?.ship_id === id) return;
    plan = savedPlan(id);
    selMat = null;
  });

  // Recompute on plan edits and when the fleet / inventory is re-read.
  $effect(() => {
    const p = plan;
    void fleet;
    if (!p) return;
    const snapshot = $state.snapshot(p) as Plan;
    const mine = ++seq;
    evaluatePlan(snapshot)
      .then((r) => {
        if (mine !== seq) return; // a newer request (e.g. another ship) is in flight
        result = r;
        failed = r ? null : "Plan maths runs in the app. Outside it, only the sample plan has numbers.";
      })
      .catch((e: unknown) => {
        if (mine !== seq) return;
        result = null;
        failed = String(e);
      });
  });

  // Re-plan the trip when the plan or the journal changes (crafts, remote or not, shrink it).
  $effect(() => {
    const p = plan;
    void result;
    void engineers;
    if (!p || !hasJobs) {
      trip = null;
      return;
    }
    planTrip($state.snapshot(p) as Plan)
      .then((tr) => {
        trip = tr;
        tripErr = null;
      })
      .catch((e: unknown) => {
        trip = null;
        tripErr = String(e);
      });
  });

  function target(slot: string): Target | undefined {
    return plan?.targets.find((t) => t.slot === slot);
  }

  function setTarget(m: Fitted, patch: Partial<Target> | null) {
    if (!plan) return;
    const others = plan.targets.filter((t) => t.slot !== m.slot);
    const old = target(m.slot);
    let next: Target | null = null;
    if (patch) {
      const blueprint = patch.blueprint ?? old?.blueprint ?? m.engineering?.blueprint ?? "";
      const choice = fleet?.groups[m.group ?? ""]?.blueprints.find((b) => b.id === blueprint);
      if (choice) {
        const grade = patch.grade ?? (old?.blueprint === blueprint ? old.grade : (choice.max_grade ?? 5));
        const experimental = patch.experimental !== undefined ? patch.experimental : (old?.experimental ?? m.engineering?.experimental ?? null);
        next = { slot: m.slot, blueprint, grade: Math.min(grade, choice.max_grade ?? 5), experimental: experimental || null };
      }
    }
    plan = { ...plan, targets: next ? [...others, next] : others };
    savePlan(plan);
  }

  function setRolls(i: number, v: number) {
    if (!plan || !Number.isFinite(v)) return;
    const rolls = [...plan.rolls];
    rolls[i] = Math.max(1, Math.min(20, Math.round(v)));
    plan = { ...plan, rolls };
    savePlan(plan);
  }

  function current(m: Fitted): string {
    const e = m.engineering;
    if (!e) return "Not engineered";
    const pct = e.quality >= 1 ? "" : ` ${Math.round(e.quality * 100)}%`;
    return `G${e.grade}${pct} ${e.blueprint_name}${e.experimental_name ? ` · ${e.experimental_name}` : ""}`;
  }

  function cost(slot: string): string {
    const c = result?.targets.find((t) => t.slot === slot);
    if (!c) return "";
    const steps = c.steps.map(([g, n]) => `G${g}×${n}`).join(" ");
    const ex = c.experimental_needed ? ` + ${c.experimental_name}` : "";
    return [steps + ex, c.note].filter(Boolean).join(" · ");
  }

  function maxGrade(m: Fitted, bp: string | undefined): number {
    return fleet?.groups[m.group ?? ""]?.blueprints.find((b) => b.id === bp)?.max_grade ?? 5;
  }

  function fill(m: MatLine) {
    const p = m.need > 0 ? Math.min(1, m.have / m.need) : 1;
    return `linear-gradient(90deg, var(--cell) ${p * 100}%, var(--cell-empty) ${p * 100}%)`;
  }

  function when(ts: string | null): string {
    return ts ? ts.replace("T", " ").replace(/:\d\dZ?$/, "") : "";
  }
</script>

<section>
  <div class="acts">
    {#each fleet?.ships ?? [] as s (s.id)}
      <button class:on={ship?.id === s.id} class:dim={!s.loadout_ts} onclick={() => (shipId = s.id)}>
        {s.name ?? s.type_name}{s.current ? " ●" : ""}
      </button>
    {/each}
  </div>
  <p class="sub">
    {#if !fleet}
      Reading the journal…
    {:else if !ship}
      No ships in the journal yet · {fleet.source}
    {:else}
      {shipTitle(ship)}{ship.ident ? ` (${ship.ident})` : ""}
      · {ship.current ? "flying now" : ship.system ? `stored at ${ship.system}` : "stored"}
      · {ship.loadout_ts ? `modules as of ${when(ship.loadout_ts)}` : "modules unknown"}
      · {fleet.source}
    {/if}
  </p>

  {#if ship && !ship.loadout_ts}
    <p class="lo wide">The journal has no loadout for this ship: it hasn't been flown since your oldest journal file. Fly it once and its modules show up here.</p>
  {:else if ship && plan}
    <div class="main">
      <div class="mods">
        <div class="h">Slot</div><div class="h">Module</div><div class="h">Now</div><div class="h">Blueprint</div><div class="h">Grade</div><div class="h">Experimental</div>
        {#each mods as m (m.slot)}
          {@const t = target(m.slot)}
          {@const g = fleet?.groups[m.group ?? ""]}
          <div class="slot">{m.slot_name}</div>
          <div>{m.name}</div>
          <div class="lo">{current(m)}</div>
          <select class:idle={!t} value={t?.blueprint ?? ""} onchange={(e) => setTarget(m, e.currentTarget.value ? { blueprint: e.currentTarget.value } : null)}>
            <option value="">No change</option>
            {#each g?.blueprints ?? [] as b (b.id)}
              <option value={b.id}>{b.name}</option>
            {/each}
          </select>
          <select disabled={!t} value={t?.grade ?? ""} onchange={(e) => setTarget(m, { grade: Number(e.currentTarget.value) })}>
            {#if !t}<option value="">–</option>{/if}
            {#each Array.from({ length: maxGrade(m, t?.blueprint) }, (_, i) => i + 1) as n (n)}
              <option value={n}>G{n}</option>
            {/each}
          </select>
          <select disabled={!t || !g?.experimentals.length} value={t?.experimental ?? ""} onchange={(e) => setTarget(m, { experimental: e.currentTarget.value || null })}>
            <option value="">{g?.experimentals.length ? "None" : "–"}</option>
            {#each g?.experimentals ?? [] as x (x.id)}
              <option value={x.id}>{x.name}</option>
            {/each}
          </select>
          {#if t}
            <div class="cost">{cost(m.slot)}</div>
          {/if}
        {:else}
          <p class="lo span">Nothing on this ship can be engineered.</p>
        {/each}
      </div>

      {#if failed}
        <p class="lo">{failed}</p>
      {:else if result && result.materials.length}
        <h4>Shopping list</h4>
        <div class="mats">
          <div class="h">Material</div><div class="h r">Need</div><div class="h r">Have</div><div class="h r">Short</div><div class="h r">Trade</div>
          {#each result.materials as m (m.id)}
            <button class="mrow" class:sel={selMat === m.id} onclick={() => (selMat = m.id)}>
              <span class="mn" style:background={fill(m)}>G{m.grade} {m.name}{m.over_cap ? " ⚠" : ""}</span>
              <span class="r q">{m.need}</span>
              <span class="r q">{m.have}</span>
              <span class="r q" class:bad={m.short > 0} class:lo={m.short === 0}>{m.short}</span>
              <span class="r q lo">{traded.get(m.id) ? `+${traded.get(m.id)}` : ""}</span>
            </button>
          {/each}
        </div>

        {#if result.trades.length}
          <h4>Trade at a material trader</h4>
          <ul class="plain">
            {#each result.trades as t, i (i)}
              <li><span class="q">{t.give}</span> {t.give_name} → <span class="q">{t.get}</span> {t.get_name} <span class="lo">· {t.trader}</span></li>
            {/each}
          </ul>
        {/if}

        {#if result.farm.length}
          <h4>Still to collect</h4>
          <ul class="plain farm">
            {#each result.farm as f (f.id)}
              <li>
                <button class="link" onclick={() => (selMat = f.id)}><span class="q">{f.collect}</span> {f.name}</button>
                {#if f.over_cap}<span class="lo"> · over the {f.capacity} cap with what you hold: in batches</span>{/if}
                {#if f.sources?.[0]}<span class="lo"> · {f.sources[0].summary}</span>{/if}
                {#if f.then?.length}
                  <span class="then">then trade {f.then.map((t) => `${t.give} → ${t.get} ${t.get_name}`).join(", ")}</span>
                {/if}
              </li>
            {/each}
          </ul>
        {:else}
          <p class="ok">You have everything{result.trades.length ? " after the trades above" : ""}.</p>
        {/if}
      {:else if result}
        <p class="lo">Pick a blueprint for a module to see what it costs.</p>
      {/if}

      {#if tripErr}
        <p class="lo">{tripErr}</p>
      {:else if trip}
        <h4>Engineer trip</h4>
        <p class="lo small">
          {trip.start ? `From ${trip.start}` : "Start unknown"} · {jumps(trip.total_jumps, trip.exact)} at {trip.range.toFixed(1)} ly
          {#if !trip.exact}· ~ = straight-line estimate{/if}
        </p>
        {#if trip.stops.length}
          <ol class="stops">
            {#each trip.stops as s, i (i)}
              <li>
                <span class="stop">{s.engineer}</span>
                <span class="lo"> · {s.system} · <span class="q">{jumps(s.hop.jumps, s.hop.exact)}</span> ({s.hop.ly.toFixed(0)} ly){s.rank ? ` · rank ${s.rank}` : ""}</span>
                {#each s.jobs as j (j.slot + j.kind)}
                  <span class="then">{j.slot_name}: {j.what}{j.note ? ` · ${j.note}` : ""}</span>
                {/each}
              </li>
            {/each}
          </ol>
        {/if}
        {#if trip.blocked.length}
          <p class="sublabel">Needs an engineer unlock</p>
          <ul class="plain">
            {#each trip.blocked as b (b.job.slot + b.job.kind)}
              <li>
                {b.job.slot_name}: {b.job.what} <span class="lo">· {b.reason}</span>
                {#if b.unlock.length}<span class="then">{b.unlock.map(([n, p]) => `${n} (${p.toLowerCase()})`).join(", ")}</span>{/if}
              </li>
            {/each}
          </ul>
        {/if}
      {/if}
    </div>
  {/if}

  <aside class="about">
    {#if selFarm && !sel}
      <h3>{selFarm.name}</h3>
      <p>Grade {selFarm.grade} · collect {selFarm.collect}{selFarm.then?.length ? ", then trade (topping up from stock):" : ""}</p>
      {#each selFarm.then ?? [] as t, i (i)}
        <p class="qty">{t.give} → {t.get} {t.get_name}</p>
      {/each}
      {#each selFarm.sources ?? [] as s, i (i)}
        <p>{s.summary}</p>
        <p class="lo">{s.hint}</p>
      {/each}
      {#if lookup}<Nearby query={lookup} />{/if}
    {:else if sel}
      <h3>{sel.name}</h3>
      <p>Grade {sel.grade}{sel.kind ? ` · ${sel.kind.charAt(0).toUpperCase()}${sel.kind.slice(1)}` : ""} · {sel.group}</p>
      <p class="qty">{sel.have} / {sel.need} needed · cap {sel.capacity}</p>
      {#if sel.over_cap}
        <p>The plan needs more than you can store at once: engineer in batches.</p>
      {/if}
      {#each result?.trades.filter((t) => t.get_id === sel.id) ?? [] as t, i (i)}
        <p>Trade {t.give} {t.give_name} for {t.get}.</p>
      {/each}
      {#each result?.farm.flatMap((f) => (f.then ?? []).filter((t) => t.get_id === sel.id)) ?? [] as t, i (i)}
        <p>Collect {t.give} {t.give_name}, then trade for {t.get}.</p>
      {/each}
      {#if selFarm}
        <p>Collect {selFarm.collect} more{selFarm.then?.length ? ", then trade:" : "."}</p>
        {#each selFarm.then ?? [] as t, i (i)}
          <p class="qty">{t.give} → {t.get} {t.get_name}</p>
        {/each}
        {#if selFarm.over_cap}<p class="lo">That's more than the {selFarm.capacity} cap with what you hold: do it in batches.</p>{/if}
      {/if}
      {#each sel.sources ?? [] as s, i (i)}
        <p>{s.summary}</p>
        <p class="lo">{s.hint}</p>
      {/each}
      <p class="lo">Used by: {usedBy.map((t) => `${t.slot_name} (${t.blueprint_name})`).join(", ")}</p>
      {#if lookup}<Nearby query={lookup} />{/if}
    {:else}
      <h3>Engineering</h3>
      <p>Choose a blueprint, grade and experimental for each module. The list adds up the remaining grade steps from where each module is now and compares the total with your materials.</p>
      <p class="lo">A grade must be finished before the next one opens; a different blueprint starts again at grade 1. Rolls per grade depend on engineer rank and luck, so these are estimates:</p>
      {#if plan}
        <div class="rolls">
          {#each plan.rolls as n, i (i)}
            <label>G{i + 1}<input type="number" min="1" max="20" value={n} onchange={(e) => setRolls(i, Number(e.currentTarget.value))} /></label>
          {/each}
        </div>
        {#if plan.rolls.join() !== DEFAULT_ROLLS.join()}
          <button class="link" onclick={() => plan && ((plan = { ...plan, rolls: DEFAULT_ROLLS }), savePlan(plan))}>Reset to 1–5</button>
        {/if}
      {/if}
      <p class="lo">Trades only spend what the plan doesn't need: 6:1 per grade up, 1:3 per grade down, ×6 across categories.</p>
    {/if}
  </aside>
</section>

<style>
  section { padding: 8px 0 28px; display: grid; grid-template-columns: 1fr 17rem; grid-template-rows: auto auto 1fr; gap: 0 40px; min-height: 0; }
  .acts, .sub, .wide { grid-column: 1 / -1; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; padding: 14px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .acts .dim:not(.on) { opacity: 0.6; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0 0 14px; }
  .main { min-width: 0; }
  .mods { display: grid; grid-template-columns: 9rem minmax(10rem, 1.2fr) minmax(9rem, 1fr) 11rem 4.5rem 10rem; gap: var(--gap) 10px; align-items: center; }
  .h { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); padding-bottom: 4px; }
  .r { text-align: right; }
  .slot { color: var(--text-lo); font-size: var(--fs-s); }
  .cost { grid-column: 4 / -1; color: var(--accent); font-family: var(--mono); font-size: var(--fs-s); margin: -2px 0 6px; }
  .span { grid-column: 1 / -1; }
  select { all: unset; height: 30px; line-height: 30px; padding: 0 10px; background: var(--cell); color: var(--accent); cursor: pointer; font-size: var(--fs-s); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  select.idle { background: var(--cell-empty); color: var(--text); }
  select:disabled { background: var(--cell-empty); color: var(--text-lo); cursor: default; }
  option { background: var(--bg); color: var(--text); }
  h4 { margin: 26px 0 8px; color: var(--text-hi); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .mats { display: grid; grid-template-columns: minmax(14rem, 22rem) 4rem 4rem 4rem 4rem; gap: var(--gap) 10px; }
  .mrow { display: contents; }
  .mn { padding: 6px 10px; font-size: var(--fs-s); }
  .mrow:hover .mn { outline: 1px solid var(--accent-dim); outline-offset: -1px; }
  .mrow.sel .mn { outline: 1px solid var(--accent); outline-offset: -1px; }
  .q { font-family: var(--mono); font-size: var(--fs-s); align-self: center; }
  .bad { color: var(--accent); }
  .ok { color: var(--accent); margin-top: 18px; }
  .plain { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 6px; font-size: var(--fs-s); }
  .link { color: var(--text); }
  .small { font-size: var(--fs-s); }
  .stops { margin: 0; padding-left: 1.4rem; display: flex; flex-direction: column; gap: 8px; font-size: var(--fs-s); }
  .stop { color: var(--text-hi); }
  .sublabel { margin: 16px 0 6px; color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }

  .then { display: block; margin: 2px 0 4px 1.6rem; color: var(--text-lo); }
  .link:hover { color: var(--accent); }
  .about { border-left: 2px solid var(--accent); padding-left: 24px; grid-row: 3; grid-column: 2; position: sticky; top: 0; align-self: start; }
  .about h3 { margin: 0 0 8px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .about p { margin: 0 0 8px; line-height: 1.45; }
  .qty { color: var(--accent); font-family: var(--mono); }
  .lo { color: var(--text-lo); }
  .rolls { display: grid; grid-template-columns: repeat(5, 1fr); gap: 6px; margin: 4px 0 10px; }
  .rolls label { display: flex; flex-direction: column; gap: 2px; color: var(--text-lo); font-size: var(--fs-s); }
  .rolls input { all: unset; box-sizing: border-box; width: 100%; height: 28px; padding: 0 6px; background: var(--cell); color: var(--accent); font-family: var(--mono); }
</style>
