<script lang="ts">
  // Odyssey micro-resources: the ShipLocker + Backpack inventory with bartender values, a
  // blueprint picker (suit/weapon grade upgrades and modifications) with what each is missing and
  // one shopping list for them all, and the on-foot engineers' unlocks and referral chain.
  import { failure, pollFailure } from "../../lib/ui/errors";
  import Button from "../../lib/ui/Button.svelte";
  import Choice from "../../lib/ui/Choice.svelte";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
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
  import { hashParam, pageHash } from "../../lib/hashstate";

  // The inventory, the blueprints and the engineers are a choice on the page, not a third tab
  // row (DESIGN.md The Two Tab Rows Rule), kept in the link: #/odyssey?view=blueprints.
  const VIEWS = [
    { value: "inventory", label: "Inventory" },
    { value: "blueprints", label: "Blueprints" },
    { value: "engineers", label: "Engineers" },
  ] as const;
  type View = (typeof VIEWS)[number]["value"];
  const CATS: { value: Category; label: string }[] = [
    { value: "goods", label: "Goods" },
    { value: "assets", label: "Assets" },
    { value: "data", label: "Data" },
    { value: "consumables", label: "Consumables" },
  ];

  let view = $state<OdysseyView | null>(null);
  let error = $state<string | null>(null);
  const readView = (): View => VIEWS.find((v) => v.value === hashParam("view"))?.value ?? "inventory";
  let tab = $state<View>(readView());
  $effect(() => {
    const onHash = () => (tab = readView());
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });
  const chooseView = (v: View) => (location.hash = pageHash("odyssey", { view: v === "inventory" ? null : v }));
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
      .catch((e: unknown) => (error = pollFailure("read the inventory", e, view !== null)));
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

  // The cards come from the saved picks when there is no list for them (it failed, or is still
  // being worked out), so every pick can still be removed.
  // The list, if it is for the picks as they are now (not a reply from before a change).
  const list = $derived(shop && samePicks(shop.picks.map((r) => r.pick), picks) ? shop : null);
  const cards = $derived<PickResult[]>(
    list?.picks ??
      picks.map((p) => ({ pick: p, title: plainTitle(p), lines: [], credits: 0, credits_partial: false, fitted_by: [], ready: false, error: null })),
  );

  const shortOf = (lines: Need[]) => lines.filter((l) => l.short > 0);

  /** What the barter columns mean, in their heads' Tips. */
  const BARTER = "What one is worth traded at a bartender (assets only)";
  /** The Tip on a "(?)" after an engineer's mod. */
  const UNSURE = "The wiki disagrees with itself on whether this engineer fits this mod";
</script>

<section>
  <div class="query">
    <div class="acts">
      <Choice label="On foot" options={[...VIEWS]} value={tab} onchange={chooseView} />
      <p class="sub" class:err={!!error && !view}>
        {#if error && !view}
          {error}
        {:else if view}
          {view.source}{view.locker_ts ? ` · ${view.locker_ts.replace("T", " ").replace("Z", "")}` : " · not written yet (log in on foot once)"}
        {:else}
          Reading the ShipLocker…
        {/if}
      </p>
    </div>
    <Stale error={view ? error : null} />
    {#if view && tab === "inventory"}
      <div class="acts">
        <Choice label="Item kind" options={CATS} value={cat} onchange={(v) => (cat = v)} />
        <Choice
          class="tog"
          label="Which items"
          value={heldOnly ? "held" : "all"}
          options={[{ value: "all", label: "All items" }, { value: "held", label: "Held only" }]}
          onchange={(v) => (heldOnly = v === "held")}
        />
      </div>
      <p class="sub">{held.toLocaleString()} held{cat === "consumables" ? "" : ` · a bartender pays ${worth.toLocaleString()} CR for them`}</p>
    {/if}
  </div>

  {#if view && tab === "inventory"}
    <div class="out">
      <table class="inv">
        <thead>
          <tr><th>Item</th><th>Ship locker</th><th>Backpack</th><th {@attach tip("Mission items go back when the mission ends and don't count towards blueprints")}>Mission</th><th {@attach tip(BARTER)}>Barter</th><th {@attach tip("What a bartender pays for one")}>Sells for</th></tr>
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
    </div>
  {/if}

  {#if view && tab === "blueprints"}
    <div class="out">
    {#if owned.length}
      <p class="sub owned">
        Owned (from the journal):
        {#each owned as g, i (i)}{i ? " · " : " "}{g.name} G{g.grade}{g.mods.length ? ` (${g.mods.map(modName).join(", ")})` : ""}{/each}
      </p>
    {/if}
    <div class="picker">
      <Dropdown bind:value={what} aria-label="Blueprint type">
        <option value="upgrade">Grade upgrade</option>
        <option value="mod">Modification</option>
      </Dropdown>
      {#if what === "upgrade"}
        <Dropdown bind:value={gearId} aria-label="Suit or weapon">
          <optgroup label="Suits">
            {#each view.gear.filter((g) => g.kind === "suit") as g (g.id)}<option value={g.id}>{g.name}{ownedNote(g.id)}</option>{/each}
          </optgroup>
          <optgroup label="Weapons">
            {#each view.gear.filter((g) => g.kind === "weapon") as g (g.id)}<option value={g.id}>{g.name}{ownedNote(g.id)}</option>{/each}
          </optgroup>
        </Dropdown>
        <label>from <Dropdown bind:value={from} aria-label="From grade">{#each fromOptions as g (g)}<option value={g}>G{g}</option>{/each}</Dropdown></label>
        <label>to <Dropdown bind:value={to} aria-label="To grade">{#each toOptions as g (g)}<option value={g}>G{g}</option>{/each}</Dropdown></label>
      {:else}
        <Dropdown bind:value={modId} aria-label="Modification">
          <option value="" disabled>Choose…</option>
          <optgroup label="Suit">
            {#each view.mods.filter((m) => m.kind === "suit" && offered(m)) as m (m.id)}<option value={m.id}>{m.name}</option>{/each}
          </optgroup>
          <optgroup label="Weapon">
            {#each view.mods.filter((m) => m.kind === "weapon" && offered(m)) as m (m.id)}<option value={m.id}>{m.name}</option>{/each}
          </optgroup>
        </Dropdown>
        {#if makers.length}
          <Dropdown bind:value={maker} aria-label="Weapon maker">
            <option value="" disabled>Weapon maker…</option>
            {#each makers as mk (mk)}<option value={mk}>{MAKERS[mk] ?? mk}</option>{/each}
          </Dropdown>
        {/if}
      {/if}
      <Button kind="primary" disabled={!candidate} onclick={add}>Add</Button>
    </div>

    {#if shopError}<p class="sub err" role="alert">{shopError}</p>{/if}
    {#if !picks.length}
      <p class="lo">Pick a suit or weapon upgrade, or a modification, to see what it needs and what you're short of.</p>
    {:else}
      <div class="picks">
        {#each cards as r (pickKey(r.pick))}
          <article class="pick" class:ready={r.ready} class:bad={!!r.error}>
            <header>
              <h3>{r.title}</h3>
              {#if list && !r.error}<span class="lo">{r.credits.toLocaleString()}{r.credits_partial ? "+" : ""} CR</span>{/if}
              <Button aria-label={`Remove ${r.title}`} onclick={() => remove(r.pick)}>Remove</Button>
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
                <p class="lo">Fitted by: {#each r.fitted_by as f, j (f.name)}{j ? ", " : ""}<span class:fit-on={f.unlocked}>{f.name}</span>{f.unconfirmed ? " " : ""}{#if f.unconfirmed}<span {@attach tip(UNSURE)}>(?)</span>{/if}{/each}</p>
              {:else if r.pick.kind === "upgrade"}
                <p class="lo">Upgraded at Pioneer Supplies.</p>
              {/if}
            {/if}
          </article>
        {/each}
      </div>

      {#if list}
      <h3 class="section list">Shopping list</h3>
      <p class="sub">
        {list.credits.toLocaleString()}{list.credits_partial ? "+" : ""} CR
        {#if list.barter_short}· short assets are worth {list.barter_short.toLocaleString()} barter at a bartender{/if}
      </p>
      <table class="inv">
        <thead><tr><th>Item</th><th>Need</th><th>Have</th><th>Short</th><th {@attach tip(BARTER)}>Barter each</th></tr></thead>
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
      <h3 class="section list" {@attach tip("Assumes a bartender swaps equal barter value, and trades assets only: the wiki gives barter values for assets only")}>Bartender trade</h3>
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
      {:else if !shopError}
        <p class="lo">Working out the shopping list…</p>
      {/if}
    {/if}
    </div>
  {/if}

  {#if view && tab === "engineers"}
    <div class="out engs">
      {#each view.engineers as e (e.id)}
        <article class="eng" class:unlocked={e.progress === "Unlocked"}>
          <header>
            <h3>{e.name}</h3>
            <span class="state">{e.progress === "Unknown" ? "Not yet known" : e.progress}</span>
          </header>
          <p><SystemChip name={e.system} />{e.colonia ? " · Colonia" : ""}</p>
          {#if e.next}<p class="next">{e.next}</p>{/if}
          <p class="lo">{#each e.mods as m, j (j)}{j ? " · " : ""}{m.name}{m.unconfirmed ? " " : ""}{#if m.unconfirmed}<span {@attach tip(UNSURE)}>(?)</span>{/if}{/each}</p>
          {#if e.referred_by.length}<p class="lo">Referred by {e.referred_by.join(", ")}</p>{/if}
          {#if e.refers.length}<p class="lo">Refers {e.refers.join(", ")}{e.referral_task ? `: ${e.referral_task}` : ""}</p>{/if}
        </article>
      {/each}
    </div>
  {/if}

  {#if view}
    <p class="lo note foot" {@attach tip(`${view.data_uncertain.length} values are unsure or blank on the wiki`)}>Game data: {view.data_source}, {view.data_date}</p>
  {/if}
</section>

<style>
  section { padding: 0 0 28px; min-height: 0; }
  .query { display: flex; flex-direction: column; gap: var(--sp-3); padding: var(--sp-6) 0 0; }
  .acts { display: flex; flex-wrap: wrap; gap: var(--sp-2) var(--sp-4); align-items: center; }
  .acts > :global(.tog) { margin-left: auto; }
  .sub { color: var(--orange-mid); font-size: var(--fs-s); margin: 0; }
  .out { padding: var(--sp-6) 0 8px; min-width: 0; }
  .lo { color: var(--orange-mid); }
  .note { font-size: var(--fs-s); margin-top: 16px; }
  .foot { padding: 0 8px; }
  table.inv { border-collapse: collapse; width: 100%; font-size: var(--fs); }
  .inv th { text-align: left; color: var(--orange-mid); font-weight: 400; font-size: var(--fs-s); padding: 0 12px 10px 8px; }
  .inv td { padding: 14px 12px 14px 8px; border-top: 1px solid var(--line); }
  .inv tbody tr:hover { background: var(--row-hover); }
  .q { text-align: right; }
  .inv th:not(:first-child) { text-align: right; }
  .inv tr.done td { color: var(--orange-dim); }
  .short { color: var(--accent); }
  .picker { display: flex; flex-wrap: wrap; gap: 10px; align-items: center; margin: 0 0 20px; }
  .picker label { color: var(--orange-mid); display: inline-flex; gap: 6px; align-items: center; }
  .picks { display: flex; flex-direction: column; }
  .pick, .eng { padding: 16px 8px; border-bottom: 1px solid var(--line); }
  .pick header, .eng header { display: flex; gap: 16px; align-items: baseline; }
  .pick h3, .eng h3 { margin: 0; font-size: var(--fs-l); font-weight: 500; flex: 1; }
  .pick.ready h3 { color: var(--orange-lit); }
  .missing { list-style: none; margin: 10px 0 0; padding: 0; display: flex; flex-direction: column; gap: 6px; font-size: var(--fs-s); }
  .missing li { display: grid; grid-template-columns: 1fr 4.5rem 3.5rem; gap: 12px; }
  .ok { color: var(--accent); font-size: var(--fs-s); }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }
  .pick.bad h3 { color: var(--orange-mid); }
  .pick p, .eng p { margin: 8px 0 0; font-size: var(--fs-s); }
  .fit-on { color: var(--accent); }
  .swaps { margin: 8px 0 12px; padding-left: 1.2rem; display: flex; flex-direction: column; gap: 10px; font-size: var(--fs-s); }
  h3.list { margin: 32px 8px 8px; }
  .engs { display: flex; flex-direction: column; }
  .eng .state { color: var(--orange-mid); font-size: var(--fs-s); }
  .eng.unlocked .state { color: var(--accent); }
  .owned { margin-bottom: 16px; }
</style>
