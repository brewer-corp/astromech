<script lang="ts">
  // A ship's Plan view: what its build plan (made on the Loadout) does and costs. The stats now
  // against the plan done, the rolls per grade the costs assume, the shopping list against the
  // inventory, the trades, what is still to collect and the engineer trip. A selected material
  // fills the side panel with where it comes from.
  import { tip } from "../../lib/ui/tip";
  import { untrack } from "svelte";
  import { DEFAULT_ROLLS, loadEngineers, planTrip, shipStats, type Engineers, type Fleet, type MatLine, type Plan, type Ship, type StatsComparison, type Trip } from "../../lib/engineering";
  import Button from "../../lib/ui/Button.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import ListDetail from "../../lib/ui/layout/ListDetail.svelte";
  import Stats from "./Stats.svelte";
  import { jumps } from "../../lib/galaxy";
  import Nearby from "../../lib/ui/Nearby.svelte";
  import { failedNote, followedNote, followRoute, type FollowNote } from "../../lib/route";
  import FollowNoteLine from "../../lib/ui/FollowNote.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import { pollFailure } from "../../lib/ui/errors";
  import type { ShipPlan } from "./plan.svelte";

  let { fleet, ship, sp }: { fleet: Fleet; ship: Ship; sp: ShipPlan } = $props();
  let selMat = $state<string | null>(null);
  let tripSeq = 0;
  let engineersKey = "";
  let lastTripKey = "";
  let engineers = $state<Engineers | null>(null);
  /** The engineer progress poll failed: the trip was planned from the last progress read, if any. */
  let engineersErr = $state<string | null>(null);
  let trip = $state<Trip | null>(null);
  let tripErr = $state<string | null>(null);
  let followNote = $state<FollowNote | null>(null);
  let stats = $state<StatsComparison | null>(null);
  let unknownHull = $state<string | null>(null);
  let statsNote = $state<string | null>(null);
  let statsSeq = 0;

  const plan = $derived(sp.plan);
  const result = $derived(sp.result);
  const sel = $derived(result?.materials.find((m) => m.id === selMat));
  const selFarm = $derived(result?.farm.find((f) => f.id === selMat));
  const usedBy = $derived(result?.targets.filter((t) => t.materials.some(([id]) => id === selMat)) ?? []);
  const traded = $derived(
    (result?.trades ?? []).reduce((m, t) => m.set(t.get_id, (m.get(t.get_id) ?? 0) + t.get), new Map<string, number>()),
  );
  const hasJobs = $derived(result?.targets.some((t) => t.steps.length > 0 || t.experimental_needed) ?? false);
  const lookup = $derived((selFarm ?? sel)?.sources?.find((s) => s.query)?.query);

  // A new ship starts with no material selected.
  let lastShip: number | undefined;
  $effect.pre(() => {
    if (ship.id !== lastShip) selMat = null;
    lastShip = ship.id;
  });

  // Hand the trip to the core's route follower: the first engineer's system goes on the clipboard,
  // and the route it replaces is parked for Resume.
  async function followTrip() {
    if (!trip) return;
    try {
      followNote = followedNote(await followRoute("Engineer trip", "engineer trip", trip.stops.map((s) => ({ system: s.system, label: s.engineer }))));
    } catch (e) {
      followNote = failedNote("Could not follow it", e, () => void followTrip());
    }
  }

  $effect(() => {
    const tick = () => {
      // replace only on change, so the trip isn't re-planned on every poll
      void loadEngineers()
        .then((e) => {
          const k = JSON.stringify(e);
          if (k !== engineersKey) {
            engineersKey = k;
            engineers = e;
          }
          engineersErr = null;
        })
        .catch((e: unknown) => (engineersErr = pollFailure("read engineer progress", e, engineers !== null)));
    };
    tick();
    const t = setInterval(tick, 1000);
    return () => clearInterval(t);
  });

  // Stats now and with the plan done, on every plan edit and fleet re-read.
  $effect(() => {
    const p = plan;
    void fleet;
    if (!p) return;
    const mine = ++statsSeq;
    shipStats($state.snapshot(p) as Plan)
      .then((r) => {
        if (mine !== statsSeq) return;
        unknownHull = "unknown_hull" in r ? r.unknown_hull : null;
        stats = "current" in r ? r : null;
        statsNote = null;
      })
      .catch((e: unknown) => {
        if (mine !== statsSeq) return;
        stats = null;
        unknownHull = null;
        statsNote = String(e);
      });
  });

  // What the trip depends on: the jobs, engineer access and where you are. Materials don't
  // matter, so a poll that only re-reads the inventory doesn't re-plan.
  const tripKey = $derived(
    JSON.stringify({
      ship: plan?.ship_id,
      jobs: result?.targets.map((t) => [t.slot, t.blueprint, t.steps, t.experimental_needed]),
      engineers: engineers?.engineers.map((e) => [e.name, e.progress, e.rank]),
      here: engineers?.here?.system,
    }),
  );

  // Re-plan the trip when that changes (crafts, remote or not, shrink it).
  $effect(() => {
    const key = tripKey;
    if (key === lastTripKey) return;
    lastTripKey = key;
    const p = untrack(() => plan);
    if (!p || !untrack(() => hasJobs)) {
      trip = null;
      return;
    }
    const mine = ++tripSeq;
    planTrip($state.snapshot(p) as Plan)
      .then((tr) => {
        if (mine !== tripSeq) return;
        trip = tr;
        tripErr = null;
      })
      .catch((e: unknown) => {
        if (mine !== tripSeq) return;
        trip = null;
        tripErr = String(e);
      });
  });

  /** How much of the need is held, as a 2 px Mark Off track under the name that fills Prose Grey, Orange Lit when covered. */
  function track(m: MatLine) {
    const p = m.need > 0 ? Math.min(1, m.have / m.need) : 1;
    const ink = p >= 1 ? "var(--orange-lit)" : "var(--prose)";
    return `linear-gradient(90deg, ${ink} ${p * 100}%, var(--mark-off) ${p * 100}%) bottom / 100% 2px no-repeat`;
  }

  const pick = (id: string) => (selMat = selMat === id ? null : id);
</script>

<ListDetail panel={selFarm || sel ? about : null} selected={selMat} min={560}>
  <div class="plan">
    <section aria-label="Ship stats">
      <h3 class="section">Now and with the plan</h3>
      <Stats type={ship.ship_type} {stats} note={statsNote} {unknownHull} />
    </section>

    {#if plan}
      <section aria-label="Rolls per grade">
        <h3 class="section" {@attach tip("Each roll costs one full grade recipe; how many a grade takes depends on engineer rank and luck. A grade must be finished before the next opens, and a different blueprint starts again at grade 1.")}>Rolls per grade</h3>
        <div class="rolls">
          {#each plan.rolls as n, i (i)}
            <label class="type-label">G{i + 1}<Input type="number" min="1" max="20" value={n} onchange={(e) => sp.setRolls(i, Number(e.currentTarget.value))} /></label>
          {/each}
          {#if plan.rolls.join() !== DEFAULT_ROLLS.join()}
            <Button onclick={() => sp.resetRolls()}>Reset to 1–5</Button>
          {/if}
        </div>
      </section>
    {/if}

    {#if sp.failed}
      <p class="err" role="alert">{sp.failed}</p>
    {:else if result && result.materials.length}
      <section aria-label="Shopping list">
        <h3 class="section" class:current={result.materials.some((m) => m.id === selMat)}>Shopping list</h3>
        <div class="mats">
          <div class="mhead type-label">
            <span>Material</span><span class="r">Need</span><span class="r">Have</span><span class="r">Short</span>
            <span class="r" {@attach tip("Trades only spend what the plan doesn't need: 6 for 1 up a grade, 1 for 3 down a grade, 6 times worse across categories")}>Trade</span>
          </div>
          {#each result.materials as m (m.id)}
            <button class="mrow" class:sel={selMat === m.id} aria-pressed={selMat === m.id} onclick={() => pick(m.id)}>
              <span class="type-name mn" style:background={track(m)}>G{m.grade} {m.name}{#if m.over_cap}<b class="warn" {@attach tip("The plan needs more than you can store at once: engineer in batches")}> ⚠</b>{/if}</span>
              <span class="type-figures r">{m.need}</span>
              <span class="type-figures r">{m.have}</span>
              <span class="type-figures r" class:bad={m.short > 0} class:lo={m.short === 0}>{m.short}</span>
              <span class="type-figures r lo">{traded.get(m.id) ? `+${traded.get(m.id)}` : ""}</span>
            </button>
          {/each}
        </div>
      </section>

      {#if result.trades.length}
        <section aria-label="Trades">
          <h3 class="section">Trade at a material trader</h3>
          <ul class="plain">
            {#each result.trades as t, i (i)}
              <li><span class="type-figures">{t.give}</span> {t.give_name} → <span class="type-figures">{t.get}</span> {t.get_name} <span class="lo">· {t.trader}</span></li>
            {/each}
          </ul>
        </section>
      {/if}

      {#if result.farm.length}
        <section aria-label="Still to collect">
          <h3 class="section">Still to collect</h3>
          <ul class="plain">
            {#each result.farm as f (f.id)}
              <li>
                <button class="link" onclick={() => pick(f.id)}><span class="type-figures">{f.collect}</span> {f.name}</button>
                {#if f.over_cap}<span class="lo"> · over the {f.capacity} cap with what you hold: in batches</span>{/if}
                {#if f.sources?.[0]}<span class="lo"> · {f.sources[0].summary}</span>{/if}
                {#if f.then?.length}
                  <span class="then">then trade {f.then.map((t) => `${t.give} → ${t.get} ${t.get_name}`).join(", ")}</span>
                {/if}
              </li>
            {/each}
          </ul>
        </section>
      {:else}
        <p class="ok">You have everything{result.trades.length ? " after the trades above" : ""}.</p>
      {/if}
    {:else if result}
      <p class="type-label">Nothing planned yet: select a module on the <a href="#/engineering/{ship.id}">Loadout</a> to plan it.</p>
    {/if}

    {#if tripErr}
      <p class="err" role="alert">{tripErr}</p>
    {:else if trip}
      <section aria-label="Engineer trip">
        <div class="triphead">
          <h3 class="section">Engineer trip</h3>
          {#if trip.stops.length}<Button onclick={() => void followTrip()}>Follow</Button>{/if}
        </div>
        <Stale error={engineersErr} />
        <FollowNoteLine bind:note={followNote} />
        <p class="type-label">
          {#if trip.start}From <SystemChip name={trip.start} />{:else}Start unknown{/if} · {jumps(trip.total_jumps, trip.exact)} at {trip.range.toFixed(1)} ly
          {#if !trip.exact}<span {@attach tip("A ~ jump count is a straight-line estimate")}>· ~ estimated</span>{/if}
        </p>
        {#if trip.stops.length}
          <ol class="stops">
            {#each trip.stops as s, i (i)}
              <li>
                <span class="type-name">{s.engineer}</span>
                <span class="lo"> · <SystemChip name={s.system} /> · <span class="type-figures">{jumps(s.hop.jumps, s.hop.exact)}</span> ({s.hop.ly.toFixed(0)} ly){s.rank ? ` · rank ${s.rank}` : ""}</span>
                {#each s.jobs as j (j.slot + j.kind)}
                  <span class="then">{j.slot_name}: {j.what}{j.note ? ` · ${j.note}` : ""}</span>
                {/each}
              </li>
            {/each}
          </ol>
        {/if}
        {#if trip.blocked.length}
          <p class="type-label">Needs an engineer unlock</p>
          <ul class="plain">
            {#each trip.blocked as b (b.job.slot + b.job.kind)}
              <li>
                {b.job.slot_name}: {b.job.what} <span class="lo">· {b.reason}</span>
                {#if b.unlock.length}<span class="then">{b.unlock.map(([n, p]) => `${n} (${p.toLowerCase()})`).join(", ")}</span>{/if}
              </li>
            {/each}
          </ul>
        {/if}
      </section>
    {/if}
  </div>
</ListDetail>

{#snippet about()}
  <div class="about">
    {#if selFarm && !sel}
      <h3 class="type-title">{selFarm.name}</h3>
      <p class="type-label">Grade {selFarm.grade}</p>
      <p><span class="type-value-xl main">{selFarm.collect}</span><span class="type-unit">to collect{selFarm.then?.length ? ", then trade (topping up from stock):" : ""}</span></p>
      {#each selFarm.then ?? [] as t, i (i)}
        <p class="qty">{t.give} → {t.get} {t.get_name}</p>
      {/each}
      {#each selFarm.sources ?? [] as s, i (i)}
        <p>{s.summary}</p>
        <p class="type-label">{s.hint}</p>
      {/each}
      {#if lookup}<Nearby query={lookup} />{/if}
    {:else if sel}
      <h3 class="type-title">{sel.name}</h3>
      <p class="type-label">Grade {sel.grade}{sel.kind ? ` · ${sel.kind.charAt(0).toUpperCase()}${sel.kind.slice(1)}` : ""} · {sel.group}</p>
      <p><span class="type-value-xl main">{sel.need}</span><span class="type-unit">needed · {sel.have} held · cap {sel.capacity}</span></p>
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
        {#if selFarm.over_cap}<p class="type-label">That's more than the {selFarm.capacity} cap with what you hold: do it in batches.</p>{/if}
      {/if}
      {#each sel.sources ?? [] as s, i (i)}
        <p>{s.summary}</p>
        <p class="type-label">{s.hint}</p>
      {/each}
      <p class="type-label">Used by: {usedBy.map((t) => `${t.slot_name} (${t.blueprint_name})`).join(", ")}</p>
      {#if lookup}<Nearby query={lookup} />{/if}
    {/if}
  </div>
{/snippet}

<style>
  /* As wide as its widest section; spare width stays empty (The Next To It Rule). */
  .plan { display: grid; grid-template-columns: minmax(0, max-content); gap: var(--sp-8); }
  section { display: flex; flex-direction: column; gap: var(--sp-3); }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .rolls { display: grid; grid-template-columns: repeat(5, 4rem) max-content; align-items: end; gap: var(--sp-2); }
  .rolls label { display: flex; flex-direction: column; gap: var(--sp-1); }
  .mats { display: flex; flex-direction: column; }
  .mhead, .mrow { display: grid; grid-template-columns: 17rem repeat(4, 4.25rem); column-gap: var(--sp-4); align-items: center; }
  .mhead { padding: 0 var(--sp-2) var(--sp-2); }
  .mrow { padding: var(--sp-2); border-bottom: 1px solid var(--line); }
  .mrow:hover { background: var(--row-hover); }
  .mrow.sel, .mrow.sel:hover { background: var(--surface-hi); }
  .mrow.sel .mn { color: var(--text-hi); }
  .mn { padding: var(--sp-1) 0; }
  .warn { color: var(--accent); }
  .r { text-align: right; }
  .bad { color: var(--accent); }
  .lo { color: var(--label-grey); }
  .ok { margin: 0; color: var(--prose); }
  .plain { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: var(--sp-2); }
  .link { color: var(--accent); font-weight: 500; }
  .link:hover { color: var(--orange-lit); }
  .triphead { display: flex; align-items: center; gap: var(--sp-4); }
  .triphead .section { flex: 1; }
  .stops { margin: 0; padding: 0 0 0 var(--sp-6); display: flex; flex-direction: column; gap: var(--sp-3); }
  .then { display: block; margin: var(--sp-1) 0; color: var(--label-grey); }
  .err { margin: 0; color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }
  .about { display: flex; flex-direction: column; gap: var(--sp-3); }
  .about h3, .about p { margin: 0; }
  .qty { color: var(--accent); font-weight: 500; }
</style>
