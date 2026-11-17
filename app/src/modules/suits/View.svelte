<script module lang="ts">
  /** The loadout's rows at full width, in px at 100 percent Text size: the side panel goes below them when it can't sit beside. */
  const MIN = 920;
</script>

<script lang="ts">
  // Ships > Suits as a loadout (astromech-nnn0, tone5-suits-2560.png): the selected suit loadout
  // slot by slot (each item's grade, fitted mods and plan), one shopping list for every pick with
  // the bartender trade under it, and the other loadouts as a strip. A List with detail: selecting
  // a slot fills the side panel with that item's planned cost, what its picks need, where they are
  // fitted and ADD TO PLAN; PLAN OTHER GEAR opens the blueprint picker there for gear in no
  // loadout or not owned. A pick belongs to an item by its journal id (lib/suits.ts).
  import { untrack } from "svelte";
  import { failure, pollFailure } from "../../lib/ui/errors";
  import { pageHash, hashParam } from "../../lib/hashstate";
  import { failedNote, followedNote, followRoute, type FollowNote } from "../../lib/route";
  import Button from "../../lib/ui/Button.svelte";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import FollowNoteLine from "../../lib/ui/FollowNote.svelte";
  import Hexes from "../../lib/ui/Hexes.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import ListDetail from "../../lib/ui/layout/ListDetail.svelte";
  import { tip } from "../../lib/ui/tip";
  import {
    fromGrades,
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
    type OwnedGear,
    type Pick,
    type PickResult,
    type Shopping,
  } from "../../lib/odyssey";
  import { addPick, fits, inNoLoadout, modPick, modsFor, planFor, plannedGrade, slotRows, tie, untied } from "../../lib/suits";

  let view = $state<OdysseyView | null>(null);
  let error = $state<string | null>(null);
  let picks = $state<Pick[]>(savedPicks());
  /** The last shopping list worked out, kept when a later one fails. */
  let shop = $state<Shopping | null>(null);
  let shopError = $state<string | null>(null);
  /** Bumped by TRY AGAIN on a failed shopping list. */
  let attempt = $state(0);

  function refresh() {
    loadOdyssey()
      .then((v) => {
        view = v;
        error = null;
      })
      .catch((e: unknown) => (error = pollFailure("read the suits and loadouts", e, view !== null)));
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
    void attempt;
    loadShopping(ps)
      .then((s) => {
        shop = s;
        shopError = null;
      })
      .catch((e: unknown) => (shopError = failure("work out the shopping list", e)));
  });

  // The loadout shown: the one the link names, else the one worn (astromech-w1lj), else the first.
  let wantLoadout = $state(hashParam("loadout"));
  $effect(() => {
    const onHash = () => (wantLoadout = hashParam("loadout"));
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  });
  const loadouts = $derived(view?.loadouts ?? []);
  const loadout = $derived(loadouts.find((l) => String(l.id) === wantLoadout) ?? loadouts.find((l) => l.worn) ?? loadouts[0]);
  const rows = $derived(loadout ? slotRows(loadout) : []);
  const owned = $derived(view?.owned ?? []);
  const spare = $derived(inNoLoadout(owned, loadouts));
  const loose = $derived(view ? untied(picks, owned) : picks.filter((p) => p.item == null));
  function chooseLoadout(id: number) {
    wantLoadout = String(id);
    location.hash = pageHash("suits", { loadout: wantLoadout });
  }

  // The side panel: an owned item, or the picker for other gear, or nothing.
  let selId = $state<number | null>(null);
  let other = $state(false);
  const sel = $derived(selId === null ? undefined : owned.find((o) => o.id === selId));
  // Whenever the loadout shown changes (a click, Back, or the worn one changing in the journal),
  // the panel keeps an item that loadout has, or one in no loadout ("Also owned"); any other
  // closes with the loadout it belonged to.
  $effect(() => {
    const shown = new Set(rows.flatMap((r) => (r.item ? [r.item.id] : [])));
    untrack(() => {
      if (selId !== null && view && !shown.has(selId) && !spare.some((o) => o.id === selId)) selId = null;
    });
  });
  function select(item: OwnedGear) {
    selId = selId === item.id ? null : item.id;
    other = false;
  }
  function openOther() {
    other = !other;
    selId = null;
  }

  const modName = (id: string) => view?.mods.find((m) => m.id === id)?.name ?? id;
  const gearName = (id: string) => view?.gear.find((g) => g.id === id)?.name ?? id;

  /** What is planned for an item, from its picks: ["Grade 3", "Night Vision"]. */
  function planned(item: OwnedGear): string[] {
    const g = plannedGrade(picks, item);
    return [...(g > item.grade ? [`Grade ${g}`] : []), ...planFor(picks, item).flatMap((p) => (p.kind === "mod" ? [modName(p.id)] : []))];
  }
  /** "Grade 3 · Night Vision", on a row. */
  const planWords = (item: OwnedGear) => planned(item).join(" · ");
  /** "grade 3 and Night Vision", in the panel. */
  function planSaid(item: OwnedGear): string {
    const w = planned(item).map((x) => x.replace(/^Grade/, "grade"));
    return w.length > 1 ? `${w.slice(0, -1).join(", ")} and ${w.at(-1)}` : (w[0] ?? "");
  }
  const engineered = (item: OwnedGear) => item.grade > 1 || item.mods.length > 0 || planFor(picks, item).length > 0;
  const items = $derived(rows.flatMap((r) => (r.item ? [r.item] : [])));
  const where = (item: OwnedGear) => loadouts.filter((l) => l.suit.id === item.id || l.slots.some((s) => s.item.id === item.id)).map((l) => l.name);

  function setPicks(next: Pick[]) {
    picks = next;
    savePicks($state.snapshot(picks) as Pick[]);
  }
  const add = (p: Pick) => setPicks(addPick(picks, p));
  const remove = (p: Pick) => setPicks(picks.filter((x) => pickKey(x) !== pickKey(p)));

  function plainTitle(p: Pick): string {
    if (p.kind === "upgrade") return `${gearName(p.gear)} grade ${p.from} → ${p.to}`;
    return p.maker ? `${modName(p.id)} (${MAKERS[p.maker] ?? p.maker} weapons)` : modName(p.id);
  }

  // The list, if it is for the picks as they are now (not a reply from before a change). With
  // none, each pick still has its line from the saved picks, so every pick can be removed.
  const list = $derived(shop && samePicks(shop.picks.map((r) => r.pick), picks) ? shop : null);
  function resultOf(p: Pick): PickResult {
    return (
      list?.picks.find((r) => pickKey(r.pick) === pickKey(p)) ?? { pick: p, title: plainTitle(p), lines: [], credits: 0, credits_partial: false, fitted_by: [], ready: false, error: null }
    );
  }
  const total = $derived(list ? [...list.total].sort((a, b) => Number(b.short > 0) - Number(a.short > 0)) : []);
  const halves = $derived([total.slice(0, Math.ceil(total.length / 2)), total.slice(Math.ceil(total.length / 2))].filter((h) => h.length));
  const short = (lines: Need[]) => lines.filter((l) => l.short > 0).length;
  const cr = (n: number, partial: boolean) => `${partial ? "at least " : ""}${n.toLocaleString("en-GB")}`;

  /** "Grades 2 and 3 need": the grades an upgrade pick buys. */
  function gradesNeed(from: number, to: number): string {
    const g = Array.from({ length: to - from }, (_, i) => from + i + 1);
    return g.length === 1 ? `Grade ${g[0]} needs` : `Grades ${g.slice(0, -1).join(", ")} and ${g.at(-1)} need`;
  }

  // ADD TO PLAN for an owned item: a higher grade, or a modification it can still take.
  function choices(item: OwnedGear): { value: string; label: string; pick: Pick }[] {
    const g = plannedGrade(picks, item);
    const grades = Array.from({ length: 5 - g }, (_, i) => g + i + 1).map((to) => ({
      value: `g${to}`,
      label: `Grade ${to}`,
      pick: { kind: "upgrade", gear: item.gear, from: item.grade, to, item: item.id } as Pick,
    }));
    const ms = view ? modsFor(item, view.mods, picks).map((m) => ({ value: `m:${m.id}`, label: m.name, pick: modPick(item, m) })) : [];
    return [...grades, ...ms];
  }
  let choice = $state("");
  let forItem = $state("");
  const forOwned = $derived(spare.find((o) => String(o.id) === forItem));
  // A new item in the panel starts on its first choice (the next grade, else a modification).
  const forChoice = $derived(sel ?? (other ? forOwned : undefined));
  // Again after an add (the next grade moves on), but not on every poll of the view.
  const choiceKey = $derived(`${forChoice?.id ?? ""}|${picks.map(pickKey).join(",")}`);
  $effect(() => {
    void choiceKey;
    choice = untrack(() => (forChoice ? (choices(forChoice)[0]?.value ?? "") : ""));
  });

  // PLAN OTHER GEAR: for an owned item in no loadout (forItem, tied to it), or any suit or weapon (tied to nothing).
  let what = $state<"upgrade" | "mod">("upgrade");
  let gearId = $state("maverick");
  let from = $state(1);
  let to = $state(5);
  let modId = $state("");
  let maker = $state("");
  const mod = $derived(view?.mods.find((m) => m.id === modId));
  const makers = $derived((mod?.makers ?? []).filter((mk) => !mod?.fitted_makers?.includes(mk)));
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
  function recipe(): Pick | null {
    if (what === "upgrade") return from < to ? { kind: "upgrade", gear: gearId, from, to } : null;
    if (!mod) return null;
    if (makers.length && !makers.includes(maker)) return null;
    return { kind: "mod", id: mod.id, maker: makers.length ? maker : null };
  }
  const candidate = $derived(recipe());

  // Set destination to an engineer who fits a picked mod; the note sits under that engineer.
  let note = $state<FollowNote | null>(null);
  let noteFor = $state<string | null>(null);
  async function go(name: string, system: string) {
    noteFor = name;
    try {
      note = followedNote(await followRoute(system, "suits", [{ system, label: name }]));
    } catch (e) {
      note = failedNote("Couldn't follow it", e, () => void go(name, system));
    }
  }

  /** The Tip on a "(?)" after an engineer's mod, as on Engineers. */
  const UNSURE = "The wiki disagrees with itself on whether this engineer fits this mod";
  const BARTER = "Assumes a bartender swaps equal barter value, and trades assets only: the wiki gives barter values for assets only";
  const source = $derived(
    view ? `Grades and modifications from the journal (${view.source}${view.locker_ts ? `, ${view.locker_ts.replace("T", " ").replace("Z", "")}` : ""})` : "",
  );
  const panel = $derived(sel ? itemPanel : other ? otherPanel : null);
</script>

<ListDetail {panel} selected={sel?.id ?? (other ? "other" : null)} min={MIN}>
  <div class="page">
    <div class="head">
      <h2 class="type-title">{loadout?.name ?? "Suits"}</h2>
      <Button class="plan-other" aria-pressed={other} onclick={openOther}>Plan other gear</Button>
    </div>
    {#if error && !view}
      <Failure text={error} onaction={refresh} />
    {:else if !view}
      <p class="type-label">Reading the journal…</p>
    {:else}
      {#if loadout}
        <p class="type-label" {@attach tip(source)}>
          Loadout {loadouts.indexOf(loadout) + 1} of {loadouts.length} · {loadout.suit.name} · {items.filter(engineered).length} of {items.length} items engineered or planned
        </p>
      {:else}
        <p class="type-body" {@attach tip(source)}>No suit loadout in the journal yet.</p>
      {/if}
      <Stale {error} />

      {#if loadout}
        <section aria-label="Loadout">
          <h3 class="section" class:current={!!sel && items.some((i) => i.id === sel.id)}>Loadout</h3>
          <div class="slots">
            <div class="slot heads type-label" aria-hidden="true"><span>Slot</span><span>Item</span><span>Grade</span><span>Modifications</span><span>Plan</span></div>
            {#each rows as r (r.key)}
              {#if r.item}
                {@const item = r.item}
                {@const g = plannedGrade(picks, item)}
                <button
                  class="slot"
                  class:sel={sel?.id === item.id}
                  aria-label="{r.slot}: {item.name}"
                  aria-current={sel?.id === item.id ? "true" : undefined}
                  onclick={() => select(item)}
                >
                  <span class="type-label">{r.slot}</span>
                  <span class="type-name name">{item.name}</span>
                  <Hexes n={item.grade} plan={g} label={g > item.grade ? `Grade ${item.grade} of 5, planned ${g}` : `Grade ${item.grade} of 5`} />
                  {#if item.mods.length}<span class="type-figures mods">{item.mods.map(modName).join(" · ")}</span>{:else}<span class="type-label">None</span>{/if}
                  <span class="type-name plan">{planWords(item)}</span>
                </button>
              {:else}
                <div class="slot empty">
                  <span class="type-label">{r.slot}</span>
                  <span class="type-label">Empty</span>
                </div>
              {/if}
            {/each}
          </div>
        </section>
      {/if}

      <section aria-label="Shopping list">
        <h3 class="section">Shopping list{#if list}<span class="type-label count">{cr(list.credits, list.credits_partial)} cr · {short(list.total)} of {list.total.length} short</span>{/if}</h3>
        {#if !picks.length}
          <p class="type-label">Nothing planned yet.</p>
        {:else if list}
          <Stale error={shopError} />
          <div class="needs">
            {#each halves as half, i (i)}
              <div class="need" role="table" aria-label={i ? "Shopping list, continued" : "Shopping list"}>
                <div class="nrow heads type-label" role="row" class:second={i > 0}>
                  <span role="columnheader">Item</span><span role="columnheader">Need</span><span role="columnheader">Have</span>
                </div>
                {#each half as l (l.id)}
                  <div class="nrow" role="row" class:short={l.short > 0}>
                    <span class="type-name" role="cell">{l.name}</span>
                    <span class="type-figures q" role="cell">{l.need}</span>
                    <span class="type-figures q have" role="cell">{#if l.short > 0}<b>!</b>{/if}{l.have}</span>
                  </div>
                {/each}
              </div>
            {/each}
          </div>
        {:else if shopError}
          <Failure text={shopError} onaction={() => attempt++} />
        {:else}
          <p class="type-label">Working out the shopping list…</p>
        {/if}

        {#if loose.length}
          <h4 class="sub type-label">Not tied to an item</h4>
          <ul class="loose">
            {#each loose as p (pickKey(p))}
              {@const r = resultOf(p)}
              {@const can = view ? fits(p, owned, view.mods) : []}
              <li aria-label={r.title}>
                <span class="type-name">{r.title}</span>
                {#if can.length}
                  <Dropdown aria-label="Tie {r.title} to" value="" onchange={(e) => setPicks(tie(picks, p, Number(e.currentTarget.value)))}>
                    <option value="" disabled>Tie to…</option>
                    {#each can as o (o.id)}<option value={String(o.id)}>{o.name} G{o.grade}{where(o).length ? ` · ${where(o).join(", ")}` : ""}</option>{/each}
                  </Dropdown>
                {/if}
                <Button aria-label="Remove {r.title}" onclick={() => remove(p)}>Remove</Button>
                {#if r.error}<p class="err">{r.error}. Remove it and pick again.</p>{/if}
              </li>
            {/each}
          </ul>
        {/if}

        {#if list}
          <h4 class="sub type-label" {@attach tip(BARTER)}>Bartender trade</h4>
          {#if list.barter_short}<p class="type-label">The short assets are worth {list.barter_short.toLocaleString("en-GB")} barter.</p>{/if}
          {#if list.trade.swaps.length}
            <ul class="swaps" aria-label="Bartender trade">
              {#each list.trade.swaps as w (w.get_id)}
                <li class="type-figures">
                  Trade {w.gives.map((g) => `${g.count} ${g.name}`).join(", ")} for {w.get} {w.get_name}
                  <span class="type-label">({w.barter_given} barter given for {w.barter_wanted}{w.barter_given > w.barter_wanted ? `, ${w.barter_given - w.barter_wanted} over` : ""})</span>
                </li>
              {/each}
            </ul>
          {:else}
            <p class="type-label">{list.barter_short ? "No surplus assets to trade for the short ones." : "No short assets to trade for."}</p>
          {/if}
          {#if list.trade.left.length}
            <p class="type-label">Still short after the trade: {list.trade.left.map((l) => `${l.short} ${l.name}`).join(", ")}</p>
          {:else if list.trade.swaps.length}
            <p class="type-label">Nothing is short after the trade.</p>
          {/if}
        {/if}
      </section>

      {#if loadouts.length}
        <section aria-label="Loadouts">
          <h3 class="section">Loadouts</h3>
          <nav class="strip" aria-label="Your loadouts">
            {#each loadouts as l (l.id)}
              <button class="tile" class:on={l === loadout} aria-current={l === loadout ? "true" : undefined} aria-label="Loadout {l.name}" onclick={() => chooseLoadout(l.id)}>
                <span class="type-name">{l.name}</span>
                <span class="type-label tl">{l.suit.name.replace(/ Suit$/, "")} <Hexes n={l.suit.grade} label="Grade {l.suit.grade} of 5" /></span>
              </button>
            {/each}
          </nav>
        </section>
      {/if}
      {#if spare.length}
        <p class="type-label also">
          Also owned:
          {#each spare as o, i (o.id)}{i ? " · " : " "}<button class="link" class:on={sel?.id === o.id} onclick={() => select(o)}>{o.name}</button> G{o.grade}{o.mods.length ? ` (${o.mods.map(modName).join(", ")})` : ""}{#if planWords(o)}<span class="plan"> → {planWords(o)}</span>{/if}{/each}
        </p>
      {/if}
      <p class="type-meta foot" {@attach tip(`${view.data_uncertain.length} values are unsure or blank on the wiki`)}>Game data: {view.data_source}, {view.data_date}</p>
    {/if}
  </div>
</ListDetail>

{#snippet itemPanel()}
  {#if sel}
    {@const item = sel}
    {@const mine = planFor(picks, item)}
    {@const up = mine.find((p) => p.kind === "upgrade")}
    {@const g = plannedGrade(picks, item)}
    {@const opts = choices(item)}
    <div class="about">
      <h3 class="type-title">{item.name}</h3>
      <p class="type-label line">
        <Hexes n={item.grade} plan={g} label={g > item.grade ? `Grade ${item.grade} of 5, planned ${g}` : `Grade ${item.grade} of 5`} />
        {mine.length ? `Plan: ${planSaid(item)}` : "No plan"}
      </p>
      {#if mine.length && list}
        {@const main = up ? resultOf(up) : null}
        {@const mods = mine.filter((p) => p.kind === "mod").map(resultOf)}
        <p class="qty">
          {#if main && up?.kind === "upgrade"}
            <span class="type-value-xl main">{cr(main.credits, main.credits_partial)}</span><span class="type-unit">cr to grade {up.to}</span>
          {:else}
            <span class="type-value-xl main">{cr(mods.reduce((n, r) => n + r.credits, 0), mods.some((r) => r.credits_partial))}</span><span class="type-unit">cr for {mods.map((r) => modName(r.pick.kind === "mod" ? r.pick.id : "")).join(" and ")}</span>
          {/if}
        </p>
      {:else if mine.length}
        <p class="type-label">{shopError ? "" : "Working out what it needs…"}</p>
      {/if}

      <div class="add">
        {#if opts.length}
          <Dropdown bind:value={choice} aria-label="Add to plan for {item.name}">
            {#each opts as o (o.value)}<option value={o.value}>{o.label}</option>{/each}
          </Dropdown>
          <Button kind="primary" onclick={() => add((opts.find((o) => o.value === choice) ?? opts[0]!).pick)}>Add to plan</Button>
        {:else}
          <p class="type-label">Grade 5, every modification fitted or planned.</p>
        {/if}
      </div>

      {#each mine as p (pickKey(p))}
        {@const r = resultOf(p)}
        <div class="pick" aria-label={r.title}>
          <div class="ph">
            <h4 class="section">{p.kind === "upgrade" ? gradesNeed(p.from, p.to) : modName(p.id)}</h4>
            <Button aria-label="Remove {r.title}" onclick={() => remove(p)}>Remove</Button>
          </div>
          {#if r.error}
            <p class="err">{r.error}. Remove it and pick again.</p>
          {:else if list}
            {#if p.kind === "mod"}<p class="type-label">{cr(r.credits, r.credits_partial)} cr · {short(r.lines)} of {r.lines.length} short</p>{/if}
            <div class="lines" role="table" aria-label="What {r.title} needs">
              <div class="nrow heads type-label" role="row"><span role="columnheader">Item</span><span role="columnheader">Need</span><span role="columnheader">Have</span></div>
              {#each r.lines as l (l.id)}
                <div class="nrow" role="row" class:short={l.short > 0}>
                  <span class="type-name" role="cell">{l.name}</span>
                  <span class="type-figures q" role="cell">{l.need}</span>
                  <span class="type-figures q have" role="cell">{#if l.short > 0}<b>!</b>{/if}{l.have}</span>
                </div>
              {/each}
            </div>
            {#if p.kind === "upgrade"}
              <p class="type-label">At Pioneer Supplies</p>
            {:else}
              <ul class="fitters" aria-label="Engineers who fit {modName(p.id)}">
                {#each r.fitted_by as f (f.name)}
                  <li>
                    <span class="fl">
                      <span class="type-name">{f.name}</span>{#if f.unconfirmed}<span class="type-label" {@attach tip(UNSURE)}>(?)</span>{/if}
                      <span class="type-label">{f.system}</span>
                      <SystemChip name={f.system} icon />
                      <IconButton icon="destination" label="Set {f.system} as the destination" tip="Set {f.system} as the destination (replaces the route, which Resume brings back)" onclick={() => void go(f.name, f.system)} />
                    </span>
                    <span class="type-label">{f.unlocked ? "Unlocked" : "Not unlocked"}</span>
                    {#if note && noteFor === f.name}<FollowNoteLine bind:note />{/if}
                  </li>
                {/each}
              </ul>
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  {/if}
{/snippet}

{#snippet otherPanel()}
  <div class="about">
    <h3 class="type-title">Plan other gear</h3>
    {#if view}
      <div class="add col">
        <Dropdown bind:value={forItem} aria-label="For">
          {#each spare as o (o.id)}<option value={String(o.id)}>{o.name} G{o.grade} (owned, in no loadout)</option>{/each}
          <option value="">Any suit or weapon</option>
        </Dropdown>
        {#if forOwned}
          {@const opts = choices(forOwned)}
          {#if opts.length}
            <Dropdown bind:value={choice} aria-label="Add to plan for {forOwned.name}">
              {#each opts as o (o.value)}<option value={o.value}>{o.label}</option>{/each}
            </Dropdown>
            <Button kind="primary" onclick={() => add((opts.find((o) => o.value === choice) ?? opts[0]!).pick)}>Add to plan</Button>
          {/if}
        {:else}
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
            <span class="grades">
              <label class="type-label">from <Dropdown bind:value={from} aria-label="From grade">{#each fromOptions as g (g)}<option value={g}>G{g}</option>{/each}</Dropdown></label>
              <label class="type-label">to <Dropdown bind:value={to} aria-label="To grade">{#each toOptions as g (g)}<option value={g}>G{g}</option>{/each}</Dropdown></label>
            </span>
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
          <Button kind="primary" disabled={!candidate} onclick={() => candidate && add(candidate)}>Add to plan</Button>
        {/if}
      </div>
    {/if}
  </div>
{/snippet}

<style>
  /* A row, tile or link that is a button: reset at no specificity, so its own rules win. */
  :where(.slot, .tile, .link) { all: unset; cursor: pointer; box-sizing: border-box; }
  :where(.slot, .tile, .link):focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  /* As wide as its columns, so nothing is far from what it acts on (DESIGN.md The Next To It Rule). */
  .page { display: flex; flex-direction: column; width: max-content; max-width: 100%; min-width: 0; }
  .page p { margin: 0; }
  .head { display: flex; align-items: center; gap: var(--sp-6); }
  .head h2 { margin: 0; }
  .page > .type-label, .page > .type-body { margin-top: var(--sp-1); }
  section { margin-top: var(--sp-8); }
  section > .section { margin-bottom: var(--sp-3); }
  .count { text-transform: none; letter-spacing: 0; font-weight: 400; }

  /* The loadout: one row per slot in the mockup's groups (DESIGN.md The Aligned Columns Rule). */
  .slots { display: flex; flex-direction: column; gap: var(--hair); }
  .slot { display: grid; grid-template-columns: 5.5rem 12rem 5.5rem 15rem 12rem; align-items: center; column-gap: var(--sp-6); min-height: var(--row); padding: var(--sp-2) var(--sp-3); }
  .slot > * { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .slot.heads { min-height: 0; padding-bottom: var(--sp-2); border-bottom: 1px solid var(--line); }
  button.slot:hover { background: var(--row-hover); }
  .slot.sel, .slot.sel:hover { background: var(--surface-hi); }
  .slot.sel .name { color: var(--text-hi); }
  .mods { white-space: normal; }
  .plan { color: var(--accent); }

  /* The shopping list: need against have, in two columns where they fit. */
  .needs { display: grid; grid-template-columns: repeat(2, max-content); column-gap: var(--sp-12); }
  .need, .lines { display: flex; flex-direction: column; }
  .nrow { display: grid; grid-template-columns: 14rem 3.5rem 3.5rem; column-gap: var(--sp-3); align-items: baseline; padding: var(--sp-1) 0; border-bottom: 1px solid var(--line); }
  .nrow.heads { border-bottom-color: var(--line); }
  .nrow .type-name { color: var(--prose); font-weight: 400; }
  .nrow.short .type-name { color: var(--options-grey); }
  .q { text-align: right; }
  .short .have { color: var(--accent); }
  .have b { font-weight: 600; margin-right: var(--sp-1); }
  .sub { margin: var(--sp-6) 0 var(--sp-2); font-weight: 600; }
  .loose { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: var(--sp-2); }
  .loose li { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-4); }
  .loose .err { flex-basis: 100%; }
  .swaps { margin: var(--sp-2) 0; padding-left: var(--sp-4); display: flex; flex-direction: column; gap: var(--sp-2); }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }

  /* The loadouts: each a Seam-outlined tile, the shown one raised. */
  .strip { display: flex; flex-wrap: wrap; gap: var(--sp-2); }
  .tile { display: flex; flex-direction: column; gap: var(--sp-1); padding: var(--sp-2) var(--sp-3); min-width: 12rem; outline: 1px solid var(--line); outline-offset: -1px; }
  .tile:hover { background: var(--row-hover); }
  .tile.on, .tile.on:hover { background: var(--surface-hi); }
  .tile.on .type-name { color: var(--text-hi); }
  .tl { display: inline-flex; align-items: center; gap: var(--sp-2); }
  .also { margin-top: var(--sp-4); }
  .link { color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .link:hover { color: var(--orange-lit); }
  .link.on { color: var(--text-hi); }
  .foot { margin-top: var(--sp-8); }

  /* The side panel (DESIGN.md Side panel): the title White, its main value Value XL Orange Lit. */
  /* The panel's own width even below the loadout, so Remove stays beside its pick. */
  .about { display: grid; grid-template-columns: minmax(0, var(--panel)); gap: var(--sp-2); }
  .about h3 { margin: 0; }
  .about p { margin: 0; }
  .line { display: flex; align-items: center; gap: var(--sp-2); }
  .qty { margin: var(--sp-2) 0; }
  .add { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2); margin: var(--sp-2) 0; }
  .add.col { flex-direction: column; align-items: flex-start; }
  .grades { display: flex; gap: var(--sp-3); }
  .grades label { display: inline-flex; align-items: center; gap: var(--sp-2); }
  .pick { display: flex; flex-direction: column; gap: var(--sp-2); margin-top: var(--sp-6); }
  .ph { display: flex; align-items: center; gap: var(--sp-2); }
  .ph .section { flex: 1; }
  .about .nrow { grid-template-columns: minmax(0, 1fr) 3rem 3rem; }
  .fitters { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: var(--sp-2); }
  .fitters li { display: flex; flex-direction: column; }
  .fl { display: flex; align-items: center; gap: var(--sp-2); }

  @container win (max-width: 75rem) {
    .needs { grid-template-columns: max-content; }
    .nrow.heads.second { display: none; }
  }
</style>
