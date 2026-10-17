<script lang="ts">
  // Odyssey micro-resources: the ShipLocker + Backpack inventory with bartender values, a
  // blueprint picker (suit/weapon grade upgrades and modifications) with what each is missing and
  // one shopping list for them all, and the on-foot engineers' unlocks and referral chain.
  import { failure } from "../../lib/ui/errors";
  import {
    fromGrades,
    hasPick,
    loadOdyssey,
    pickKey,
    samePicks,
    loadShopping,
    MAKERS,
    offered,
    ownedGrades,
    savedPicks,
    savePicks,
    type Category,
    type Need,
    type OdysseyView,
    type Pick,
    type PickResult,
    type Shopping,
  } from "../../lib/odyssey";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  const TABS = [
    { id: "inventory", title: "Inventory" },
    { id: "blueprints", title: "Blueprints" },
    { id: "engineers", title: "Engineers" },
  ] as const;
  const CATS: { id: Category; title: string }[] = [
    { id: "goods", title: "Goods" },
    { id: "assets", title: "Assets" },
    { id: "data", title: "Data" },
    { id: "consumables", title: "Consumables" },
  ];

  let view = $state<OdysseyView | null>(null);
  let error = $state<string | null>(null);
  let tab = $state<(typeof TABS)[number]["id"]>("inventory");
  let cat = $state<Category>("goods");
  let heldOnly = $state(true);

  let picks = $state<Pick[]>(savedPicks());
  let shop = $state<Shopping | null>(null);
  let shopError = $state<string | null>(null);

  // Picker state.
  let what = $state<"upgrade" | "mod">("upgrade");
  let gearId = $state("maverick");
  let from = $state(1);
  let to = $state(5);
  let modId = $state("");
  let maker = $state("");

  function refresh() {
    loadOdyssey()
      .then((v) => {
        view = v;
        error = null;
      })
      .catch((e: unknown) => (error = failure("read the inventory", e)));
  }

  $effect(() => {
    refresh();
    const t = setInterval(refresh, 2000);
    return () => clearInterval(t);
  });

  // Recompute the list when the picks, the inventory or an engineer's access change (not on
  // every poll: the derived key only changes when they do).
  const stateKey = $derived(
    view ? [view.locker_ts, view.backpack_ts, ...view.engineers.map((e) => e.progress)].join("|") : "",
  );
  $effect(() => {
    const ps = $state.snapshot(picks) as Pick[];
    void stateKey;
    loadShopping(ps)
      .then((s) => {
        shop = s;
        shopError = null;
      })
      .catch((e: unknown) => {
        shop = null;
        shopError = failure("work out the shopping list", e);
      });
  });

  const items = $derived(
    (view?.items ?? []).filter((i) => i.category === cat && (!heldOnly || i.locker + i.backpack + i.mission > 0)),
  );
  const held = $derived(
    (view?.items ?? []).filter((i) => i.category === cat).reduce((n, i) => n + i.locker + i.backpack, 0),
  );
  const worth = $derived(
    (view?.items ?? [])
      .filter((i) => i.category === cat)
      .reduce((n, i) => n + (i.value ?? 0) * (i.locker + i.backpack), 0),
  );
  const mod = $derived(view?.mods.find((m) => m.id === modId));
  const makers = $derived((mod?.makers ?? []).filter((mk) => !mod?.fitted_makers?.includes(mk)));
  const owned = $derived(view?.owned ?? []);
  const fromOptions = $derived(fromGrades(owned, gearId));
  const toOptions = $derived([2, 3, 4, 5].filter((g) => g > from));

  // An owned suit or weapon is upgraded from a grade it has; a mod already fitted everywhere is
  // no longer offered.
  $effect(() => {
    if (!fromOptions.includes(from)) from = fromOptions[0] ?? 1;
  });
  $effect(() => {
    if (to <= from) to = 5;
  });
  $effect(() => {
    if (modId && view && !view.mods.some((m) => m.id === modId && offered(m))) modId = "";
  });

  const ownedNote = (id: string) => (ownedGrades(owned, id).length ? ` · owned G${ownedGrades(owned, id).join(", G")}` : "");
  const modName = (id: string) => view?.mods.find((m) => m.id === id)?.name ?? id;

  function newPick(): Pick | null {
    if (what === "upgrade") return from < to ? { kind: "upgrade", gear: gearId, from, to } : null;
    if (!mod) return null;
    if (makers.length && !makers.includes(maker)) return null;
    return { kind: "mod", id: mod.id, maker: makers.length ? maker : null };
  }
  const candidate = $derived(newPick());

  function add() {
    if (!candidate || hasPick(picks, candidate)) return;
    picks = [...picks, candidate];
    savePicks($state.snapshot(picks) as Pick[]);
  }

  function remove(p: Pick) {
    picks = picks.filter((x) => pickKey(x) !== pickKey(p));
    savePicks($state.snapshot(picks) as Pick[]);
  }

  function plainTitle(p: Pick): string {
    if (p.kind === "upgrade") return `${view?.gear.find((g) => g.id === p.gear)?.name ?? p.gear} grade ${p.from} → ${p.to}`;
    const name = view?.mods.find((m) => m.id === p.id)?.name ?? p.id;
    return p.maker ? `${name} (${MAKERS[p.maker] ?? p.maker} weapons)` : name;
  }

  // The cards come from the saved picks when there is no list (it failed, or the preview has
  // only the sample's), so every pick can still be removed.
  // The list, if it is for the picks as they are now (not a reply from before a change).
  const list = $derived(shop && samePicks(shop.picks.map((r) => r.pick), picks) ? shop : null);
  const cards = $derived<PickResult[]>(
    list?.picks ??
      picks.map((p) => ({ pick: p, title: plainTitle(p), lines: [], credits: 0, credits_partial: false, fitted_by: [], ready: false, error: null })),
  );

  const shortOf = (lines: Need[]) => lines.filter((l) => l.short > 0);
</script>

<section>
  <div class="acts">
    {#each TABS as t (t.id)}
      <button class:on={tab === t.id} onclick={() => (tab = t.id)}>{t.title}</button>
    {/each}
  </div>
  <p class="sub">
    {#if error}
      {error}
    {:else if view}
      {view.source}{view.locker_ts ? ` · ${view.locker_ts.replace("T", " ").replace("Z", "")}` : " · not written yet (log in on foot once)"}
    {:else}
      Reading the ShipLocker…
    {/if}
  </p>

  {#if view && tab === "inventory"}
    <div class="acts small">
      {#each CATS as c (c.id)}
        <button class:on={cat === c.id} onclick={() => (cat = c.id)}>{c.title}</button>
      {/each}
      <label class="held"><input type="checkbox" bind:checked={heldOnly} /> Held only</label>
    </div>
    <p class="sub">{held.toLocaleString()} held{cat === "consumables" ? "" : ` · a bartender pays ${worth.toLocaleString()} CR for them`}</p>
    <table class="inv">
      <thead>
        <tr><th>Item</th><th>Ship locker</th><th>Backpack</th><th>Mission</th><th>Barter</th><th>Sells for</th></tr>
      </thead>
      <tbody>
        {#each items as i (i.id)}
          <tr>
            <td>{i.name}</td>
            <td class="q">{i.locker || ""}</td>
            <td class="q">{i.backpack || ""}</td>
            <td class="q lo">{i.mission || ""}</td>
            <td class="q lo">{i.barter ?? ""}</td>
            <td class="q lo">{i.value != null ? `${i.value.toLocaleString()} CR` : cat === "consumables" ? "" : "?"}</td>
          </tr>
        {:else}
          <tr><td colspan="6" class="lo">Nothing held here.</td></tr>
        {/each}
      </tbody>
    </table>
    <p class="lo note">Barter is what one is worth trading at a bartender (assets); "Sells for" is what a bartender pays. Mission items go back when the mission ends and don't count towards blueprints.</p>
  {/if}

  {#if view && tab === "blueprints"}
    {#if owned.length}
      <p class="sub owned">
        Owned (from the journal):
        {#each owned as g, i (i)}{i ? " · " : " "}{g.name} G{g.grade}{g.mods.length ? ` (${g.mods.map(modName).join(", ")})` : ""}{/each}
      </p>
    {/if}
    <div class="picker">
      <select bind:value={what} aria-label="Blueprint type">
        <option value="upgrade">Grade upgrade</option>
        <option value="mod">Modification</option>
      </select>
      {#if what === "upgrade"}
        <select bind:value={gearId} aria-label="Suit or weapon">
          <optgroup label="Suits">
            {#each view.gear.filter((g) => g.kind === "suit") as g (g.id)}<option value={g.id}>{g.name}{ownedNote(g.id)}</option>{/each}
          </optgroup>
          <optgroup label="Weapons">
            {#each view.gear.filter((g) => g.kind === "weapon") as g (g.id)}<option value={g.id}>{g.name}{ownedNote(g.id)}</option>{/each}
          </optgroup>
        </select>
        <label>from <select bind:value={from} aria-label="From grade">{#each fromOptions as g (g)}<option value={g}>G{g}</option>{/each}</select></label>
        <label>to <select bind:value={to} aria-label="To grade">{#each toOptions as g (g)}<option value={g}>G{g}</option>{/each}</select></label>
      {:else}
        <select bind:value={modId} aria-label="Modification">
          <option value="" disabled>Choose…</option>
          <optgroup label="Suit">
            {#each view.mods.filter((m) => m.kind === "suit" && offered(m)) as m (m.id)}<option value={m.id}>{m.name}</option>{/each}
          </optgroup>
          <optgroup label="Weapon">
            {#each view.mods.filter((m) => m.kind === "weapon" && offered(m)) as m (m.id)}<option value={m.id}>{m.name}</option>{/each}
          </optgroup>
        </select>
        {#if makers.length}
          <select bind:value={maker} aria-label="Weapon maker">
            <option value="" disabled>Weapon maker…</option>
            {#each makers as mk (mk)}<option value={mk}>{MAKERS[mk] ?? mk}</option>{/each}
          </select>
        {/if}
      {/if}
      <button class="add" disabled={!candidate} onclick={add}>Add</button>
    </div>

    {#if shopError}<p class="sub">{shopError}</p>{/if}
    {#if !picks.length}
      <p class="lo">Pick a suit or weapon upgrade, or a modification, to see what it needs and what you're short of.</p>
    {:else}
      <div class="picks">
        {#each cards as r (pickKey(r.pick))}
          <article class="pick" class:ready={r.ready} class:bad={!!r.error}>
            <header>
              <h3>{r.title}</h3>
              {#if list && !r.error}<span class="lo">{r.credits.toLocaleString()}{r.credits_partial ? "+" : ""} CR</span>{/if}
              <button class="x" aria-label={`Remove ${r.title}`} onclick={() => remove(r.pick)}>Remove</button>
            </header>
            {#if r.error}
              <p class="err">{r.error}. Remove it and pick again.</p>
            {:else if list}
              {#if r.ready}
                <p class="ok">Everything's in the locker.</p>
              {:else}
                <ul class="missing">
                  {#each shortOf(r.lines) as l (l.id)}
                    <li><span>{l.name}</span><span class="q">{l.have} / {l.need}</span><span class="q short">−{l.short}</span></li>
                  {/each}
                </ul>
              {/if}
              {#if r.fitted_by.length}
                <p class="lo">Fitted by: {#each r.fitted_by as f, j (f.name)}{j ? ", " : ""}<span class:fit-on={f.unlocked}>{f.name}{f.unconfirmed ? " (?)" : ""}</span>{/each}</p>
              {:else if r.pick.kind === "upgrade"}
                <p class="lo">Upgraded at Pioneer Supplies.</p>
              {/if}
            {/if}
          </article>
        {/each}
      </div>

      {#if list}
      <h3 class="list">Shopping list</h3>
      <p class="sub">
        {list.credits.toLocaleString()}{list.credits_partial ? "+" : ""} CR
        {#if list.barter_short}· short assets are worth {list.barter_short.toLocaleString()} barter at a bartender{/if}
      </p>
      <table class="inv">
        <thead><tr><th>Item</th><th>Need</th><th>Have</th><th>Short</th><th>Barter each</th></tr></thead>
        <tbody>
          {#each list.total as l (l.id)}
            <tr class:done={l.short === 0}>
              <td>{l.name} <span class="lo">{l.category}</span></td>
              <td class="q">{l.need}</td>
              <td class="q">{l.have}</td>
              <td class="q" class:short={l.short > 0}>{l.short || ""}</td>
              <td class="q lo">{l.barter ?? ""}</td>
            </tr>
          {/each}
        </tbody>
      </table>
      <h3 class="list">Bartender trade</h3>
      {#if list.trade.swaps.length}
        <ul class="swaps">
          {#each list.trade.swaps as w (w.get_id)}
            <li>
              Trade {w.gives.map((g) => `${g.count} ${g.name}`).join(", ")} for {w.get} {w.get_name}
              <span class="lo">({w.barter_given} barter given for {w.barter_wanted}{w.barter_given > w.barter_wanted ? `, ${w.barter_given - w.barter_wanted} over` : ""})</span>
            </li>
          {/each}
        </ul>
      {:else}
        <p class="lo">{list.barter_short ? "No surplus assets to trade for the short ones." : "No short assets to trade for."}</p>
      {/if}
      {#if list.trade.left.length}
        <p class="sub">
          Still short after the trade: {list.trade.left.map((l) => `${l.short} ${l.name}`).join(", ")}
        </p>
      {:else if list.trade.swaps.length}
        <p class="ok">Nothing is short after the trade.</p>
      {/if}
      <p class="lo note">Assumes a bartender swaps equal barter value and that only assets are traded (the wiki gives barter values for assets only).</p>
      {:else if !shopError}
        <p class="lo">The shopping list is worked out in the app (the preview only has the sample picks).</p>
      {/if}
    {/if}
  {/if}

  {#if view && tab === "engineers"}
    <div class="engs">
      {#each view.engineers as e (e.id)}
        <article class="eng" class:unlocked={e.progress === "Unlocked"}>
          <header>
            <h3>{e.name}</h3>
            <span class="state">{e.progress === "Unknown" ? "Not yet known" : e.progress}</span>
          </header>
          <p><SystemChip name={e.system} />{e.colonia ? " · Colonia" : ""}</p>
          {#if e.next}<p class="next">{e.next}</p>{/if}
          <p class="lo">{e.mods.map((m) => m.name + (m.unconfirmed ? " (?)" : "")).join(" · ")}</p>
          {#if e.referred_by.length}<p class="lo">Referred by {e.referred_by.join(", ")}</p>{/if}
          {#if e.refers.length}<p class="lo">Refers {e.refers.join(", ")}{e.referral_task ? `: ${e.referral_task}` : ""}</p>{/if}
        </article>
      {/each}
    </div>
    <p class="lo note">(?) the wiki disagrees with itself on whether this engineer fits that mod.</p>
  {/if}

  {#if view}
    <p class="lo note">Game data: {view.data_source}, {view.data_date}. {view.data_uncertain.length} values are unsure or blank on the wiki.</p>
  {/if}
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; padding: 14px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); align-items: baseline; }
  .acts.small { gap: 18px; padding-top: 0; font-size: var(--fs-s); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .held { color: var(--text-lo); text-transform: none; letter-spacing: normal; margin-left: auto; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0 0 14px; }
  .lo { color: var(--text-lo); }
  .note { font-size: var(--fs-s); margin-top: 14px; }
  table.inv { border-collapse: collapse; width: 100%; max-width: 60rem; font-size: var(--fs-s); }
  .inv th { text-align: left; color: var(--text-lo); font-weight: 400; text-transform: uppercase; letter-spacing: var(--caps); padding: 4px 10px 6px 0; }
  .inv td { padding: 4px 10px 4px 0; border-top: 1px solid var(--cell); }
  .q { text-align: right; font-variant-numeric: tabular-nums; }
  .inv th:not(:first-child) { text-align: right; }
  .inv tr.done td { color: var(--text-lo); }
  .short { color: var(--accent); }
  .picker { display: flex; flex-wrap: wrap; gap: 10px; align-items: center; margin: 4px 0 16px; }
  .picker select { background: var(--cell); color: var(--text); border: 1px solid var(--accent-dim); padding: 4px 6px; font: inherit; }
  .picker label { color: var(--text-lo); display: inline-flex; gap: 6px; align-items: center; }
  .add { border: 1px solid var(--accent); color: var(--accent); padding: 4px 14px; text-transform: uppercase; letter-spacing: var(--caps); }
  .add:disabled { opacity: 0.4; cursor: default; }
  .picks { display: grid; grid-template-columns: repeat(auto-fill, minmax(20rem, 1fr)); gap: var(--gap); }
  .pick, .eng { background: var(--cell); padding: 10px 12px; }
  .pick header, .eng header { display: flex; gap: 10px; align-items: baseline; }
  .pick h3, .eng h3 { margin: 0; font-size: var(--fs-m, 1rem); flex: 1; }
  .pick.ready h3 { color: var(--accent); }
  .x { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .x:hover { color: var(--text-hi); }
  .missing { list-style: none; margin: 8px 0; padding: 0; display: flex; flex-direction: column; gap: 3px; font-size: var(--fs-s); }
  .missing li { display: grid; grid-template-columns: 1fr 4rem 3rem; gap: 8px; }
  .ok { color: var(--accent); font-size: var(--fs-s); }
  .err { color: var(--text); }
  .pick.bad h3 { color: var(--text-lo); }
  .pick p, .eng p { margin: 6px 0 0; font-size: var(--fs-s); }
  .fit-on { color: var(--accent); }
  .swaps { margin: 4px 0 8px; padding-left: 1.2rem; font-size: var(--fs-s); }
  h3.list { margin: 22px 0 4px; }
  .engs { display: grid; grid-template-columns: repeat(auto-fill, minmax(19rem, 1fr)); gap: var(--gap); }
  .eng .state { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .eng.unlocked .state { color: var(--accent); }
  .next { color: var(--text); }
</style>
