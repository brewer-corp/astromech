<script lang="ts">
  // Ships > Suits: the suits and weapons owned (from the journal), a blueprint picker (suit and
  // weapon grade upgrades and modifications) with what each is missing, one shopping list for
  // them all and the bartender trade that covers it. Moved whole from the On foot page
  // (astromech-5a01); the inventory is Materials' On foot kind, the suit engineers are Engineers'.
  import { failure, pollFailure } from "../../lib/ui/errors";
  import Button from "../../lib/ui/Button.svelte";
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
    type Need,
    type OdysseyView,
    type Pick,
    type PickResult,
    type Shopping,
  } from "../../lib/odyssey";

  let view = $state<OdysseyView | null>(null);
  let error = $state<string | null>(null);
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

  /** What the barter column means, in its head's Tip. */
  const BARTER = "What one is worth traded at a bartender (assets only)";
  /** The Tip on a "(?)" after an engineer's mod. */
  const UNSURE = "The wiki disagrees with itself on whether this engineer fits this mod";
</script>

<section>
  <div class="query">
    <p class="sub" class:err={!!error && !view}>
      {#if error && !view}
        {error}
      {:else if view}
        {view.source}{view.locker_ts ? ` · ${view.locker_ts.replace("T", " ").replace("Z", "")}` : " · not written yet (log in on foot once)"}
      {:else}
        Reading the ShipLocker…
      {/if}
    </p>
    <Stale error={view ? error : null} />
  </div>

  {#if view}
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

  {#if view}
    <p class="lo note foot" {@attach tip(`${view.data_uncertain.length} values are unsure or blank on the wiki`)}>Game data: {view.data_source}, {view.data_date}</p>
  {/if}
</section>

<style>
  section { padding: 0 0 1.75rem; min-height: 0; }
  .query { display: flex; flex-direction: column; gap: var(--sp-3); padding: var(--sp-6) 0 0; }
  .sub { color: var(--label-grey); font-size: var(--fs-s); margin: 0; }
  .out { padding: var(--sp-6) 0 var(--sp-2); min-width: 0; }
  .lo { color: var(--label-grey); }
  .note { font-size: var(--fs-s); margin-top: var(--sp-4); }
  .foot { padding: 0 var(--sp-2); }
  table.inv { border-collapse: collapse; width: 100%; font-size: var(--fs); }
  .inv th { text-align: left; color: var(--label-grey); font-weight: 400; font-size: var(--fs-s); padding: 0 var(--sp-3) 0.625rem var(--sp-2); }
  .inv td { padding: 0.875rem var(--sp-3) 0.875rem var(--sp-2); border-top: 1px solid var(--line); }
  .inv tbody tr:hover { background: var(--row-hover); }
  .q { text-align: right; }
  .inv th:not(:first-child) { text-align: right; }
  .inv tr.done td { color: var(--grey-dim); }
  .short { color: var(--accent); }
  .picker { display: flex; flex-wrap: wrap; gap: 0.625rem; align-items: center; margin: 0 0 1.25rem; }
  .picker label { color: var(--label-grey); display: inline-flex; gap: 0.375rem; align-items: center; }
  .picks { display: flex; flex-direction: column; }
  .pick { padding: var(--sp-4) var(--sp-2); border-bottom: 1px solid var(--line); }
  .pick header { display: flex; gap: var(--sp-4); align-items: baseline; }
  .pick h3 { margin: 0; font-size: var(--fs-l); font-weight: 500; flex: 1; color: var(--options-grey); }
  .pick.ready h3 { color: var(--text-hi); }
  .missing { list-style: none; margin: 0.625rem 0 0; padding: 0; display: flex; flex-direction: column; gap: 0.375rem; font-size: var(--fs-s); }
  .missing li { display: grid; grid-template-columns: 1fr 4.21875rem 3.28125rem; gap: var(--sp-3); }
  .ok { color: var(--prose); font-size: var(--fs-s); }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }
  .pick.bad h3 { color: var(--label-grey); }
  .pick p { margin: var(--sp-2) 0 0; font-size: var(--fs-s); }
  .fit-on { color: var(--options-grey); }
  .swaps { margin: var(--sp-2) 0 var(--sp-3); padding-left: 1.125rem; display: flex; flex-direction: column; gap: 0.625rem; font-size: var(--fs-s); }
  h3.list { margin: var(--sp-8) var(--sp-2) var(--sp-2); }
  .owned { margin-bottom: var(--sp-4); }
</style>
