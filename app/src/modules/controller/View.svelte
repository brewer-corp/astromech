<script lang="ts">
  // My controller (docs/V1-DESIGN.md §3.1-3.3), the default screen: one stick on VKB's sheet.
  // "All" shows every binding on it; choosing an activity lights that activity's actions and
  // lists what is still unbound (Bind this), what is on the other stick and what lives off the
  // sticks. Clicking an action shows its slots and issues, with Bind another control. The chosen
  // activity is remembered. Button numbers come
  // from VKB's factory profile. If a press lands somewhere the map doesn't expect (a remapped
  // stick), astromech asks about that one control and remembers the answer.
  import catalog from "../../../../data/catalog/activities.json";
  import { binds } from "../../lib/store.svelte";
  import { api, type Press, type Row } from "../../lib/binds";
  import { maps, controlsWithOverrides, controlFor, type MapControl } from "../../lib/devices";
  import { control, shortKey, keyName, bound, slotText } from "../../lib/format";
  import Sheet, { type Cell } from "../../lib/ui/Sheet.svelte";
  import About from "../../lib/ui/About.svelte";
  import BindDialog from "../../lib/ui/BindDialog.svelte";
  import PresetPicker from "../../lib/ui/PresetPicker.svelte";

  type Act = { id: string; title: string; actions: { id: string; important: boolean; alternatives: string[] }[] };
  const acts = catalog.activities as Act[];

  binds.start();
  let dev = $state(new URLSearchParams(location.search).get("device") ?? maps[0]!.device_id);
  const m = $derived(maps.find((x) => x.device_id === dev) ?? maps[0]!);
  const ov = $derived(binds.state?.overrides ?? {});
  const controls = $derived(controlsWithOverrides(m, ov));
  let picked = $state<MapControl | null>(null);
  let unknown = $state<Press | null>(null);
  let asking = $state<Press | null>(null);
  let w = $state(1600);
  /** "all" or an activity id. */
  const ACTIVITY_KEY = "astromech.activity";
  let actId = $state(new URLSearchParams(location.search).get("activity") ?? localStorage.getItem(ACTIVITY_KEY) ?? "all");
  const act = $derived(acts.find((a) => a.id === actId));
  let bind = $state<{ row: Row; slot: string } | null>(null);
  /** The action whose detail is open. */
  let selId = $state<string | null>(null);

  const byId = $derived(new Map(binds.rows.map((r) => [r.id, r])));
  const ids = $derived(new Set(act?.actions.flatMap((a) => [a.id, ...a.alternatives])));
  const isBound = (r: Row | undefined) => !!r && r.slots.some(bound);
  const slotHere = (r: Row) => r.slots.find((s) => bound(s) && s.device === m.device_id);
  /** The list: every action bound on this device, or only the chosen activity's. */
  const listed = $derived(binds.rows.filter((r) => slotHere(r) && (!act || ids.has(r.id))));
  const notBound = $derived(
    (act?.actions ?? []).filter((a) => !isBound(byId.get(a.id)) && !a.alternatives.some((x) => isBound(byId.get(x)))).map((a) => ({ ...a, row: byId.get(a.id) })),
  );
  const otherSticks = $derived(maps.filter((x) => x.device_id !== m.device_id));
  const onOther = (r: Row) => r.slots.find((s) => bound(s) && otherSticks.some((x) => x.device_id === s.device));
  /** The activity's actions bound only on the other stick: not on this one, and not "elsewhere". */
  const otherStick = $derived(
    (act?.actions ?? []).map((a) => byId.get(a.id)).filter((r): r is Row => !!r && !slotHere(r) && !!onOther(r)),
  );
  const sel = $derived(selId ? byId.get(selId) : undefined);
  const elsewhere = $derived(
    (act?.actions ?? []).map((a) => byId.get(a.id)).filter((r): r is Row => !!r && isBound(r) && !r.slots.some((s) => bound(s) && maps.some((x) => x.device_id === s.device))),
  );
  function firstFree(r: Row): string {
    if (r.analogue) return "Binding";
    return r.slots.find((s) => s.slot === "Primary" && !bound(s)) ? "Primary" : "Secondary";
  }
  function choose(id: string) {
    actId = id;
    localStorage.setItem(ACTIVITY_KEY, id);
    picked = null;
    selId = null;
  }
  /** Open an action's detail, and light its control when it is on this stick. */
  function showRow(r: Row) {
    selId = r.id;
    const s = slotHere(r);
    picked = (s && controls.find((c) => c.joy === s.key)) || null;
  }

  const cells = $derived.by(() => {
    const out: Record<string, Cell> = {};
    for (const c of controls) {
      if (!c.joy) continue;
      const rows = binds.rowsFor(m.device_id, c.joy);
      if (!rows.length) continue;
      const mine = rows.filter((r) => ids.has(r.id));
      // The box shows the activity's action, else the ship's or general one (the same control can
      // also be an SRV or on-foot action).
      const first = mine[0] ?? rows.find((r) => r.context === "Ship" || r.context === "General") ?? rows[0]!;
      const n = act ? mine.length : rows.length;
      const here = picked?.sheet_field === c.sheet_field;
      out[c.sheet_field] = {
        text: first.name + (n > 1 ? ` +${n - 1}` : ""),
        other: shortKey(first.slots.find((s) => s.device === "Keyboard")),
        sel: here,
        hi: here || mine.length > 0,
      };
    }
    if (picked && !out[picked.sheet_field]) out[picked.sheet_field] = { text: "Not bound", hi: true, sel: true };
    return out;
  });

  let seen = binds.presses;
  $effect(() => {
    const p = binds.last;
    if (binds.presses === seen || !p || binds.capturing) return;
    seen = binds.presses;
    const mm = maps.find((x) => x.device_id === p.device);
    if (!mm) return;
    dev = mm.device_id;
    const c = controlFor(p.device, p.key, ov);
    if (c) {
      picked = c;
      unknown = null;
    } else if (p.kind !== "axis") {
      picked = null;
      unknown = p;
    }
  });

  async function pick(c: MapControl) {
    if (asking) {
      await api.setOverride(m.device_id, c.sheet_field, asking.key);
      asking = null;
      unknown = null;
      await binds.load();
      picked = controlsWithOverrides(m, binds.state?.overrides ?? {}).find((x) => x.sheet_field === c.sheet_field) ?? c;
      return;
    }
    picked = c;
    selId = null;
    unknown = null;
  }
  async function reset(c: MapControl) {
    await api.setOverride(m.device_id, c.sheet_field, null);
    await binds.load();
    picked = null;
  }
  const rowsFor = (joy: string | null) => (joy ? binds.rowsFor(m.device_id, joy) : []);
  const connected = $derived(binds.state?.inventory.controllers.some((c) => c.elite_id === m.device_id) ?? false);
  const yours = $derived(Object.keys(ov[m.device_id] ?? {}).length);
</script>

<svelte:window bind:innerWidth={w} />

<div class="cols">
  <section>
    <div class="devices">
      {#each maps as x (x.device_id)}
        <button class:on={x.device_id === m.device_id} onclick={() => { dev = x.device_id; picked = null; }}>{x.title} · {x.device_id}</button>
      {/each}
    </div>
    <PresetPicker />
    <div class="acts">
      <button class:on={!act} onclick={() => choose("all")}>All</button>
      {#each acts as a (a.id)}<button class:on={a.id === act?.id} onclick={() => choose(a.id)}>{a.title}</button>{/each}
    </div>
    {#if asking}<p class="ask">Which control did you just press ({keyName(asking.key)})? Click it on the sheet.</p>{/if}
    <Sheet image={m.image} controls={controls} {cells} width={Math.min(1180, w - 80 - 360 - 28)} dimOthers={!!act} onpick={(c) => void pick(c)} />
    <p class="credit">
      {connected ? "Connected" : "Not connected"} · button numbers from VKB's factory profile{yours ? `, ${yours} set by you` : ""} · sheet © VKB Gaming, design by UntoldForce
    </p>
    <div class="band">
      <section>
        <h3>{act ? `${act.title} on this stick` : "Everything on this stick"}</h3>
        {#each listed as r (r.id)}
          <button class="row" class:sel={selId === r.id} onclick={() => showRow(r)}>
            <span class="n">{r.name}</span><span class="v">{slotText(slotHere(r))}</span>
          </button>
        {:else}
          <p class="lo">{act ? `Nothing for ${act.title.toLowerCase()} is bound on this stick.` : "Nothing is bound on this stick."}</p>
        {/each}
      </section>
      {#if act}
        <section>
          <h3>Not bound yet</h3>
          {#each notBound.filter((a) => a.important) as a (a.id)}
            <div class="row" class:sel={selId === a.id}>
              {#if a.row}<button class="n" onclick={() => a.row && showRow(a.row)}>{a.row.name}</button>{:else}<span class="n">{a.id}</span>{/if}
              {#if a.row}<button class="bind" onclick={() => a.row && (bind = { row: a.row, slot: firstFree(a.row) })}>Bind this</button>{/if}
            </div>
          {:else}
            <p class="lo">Everything important for {act.title.toLowerCase()} is bound.</p>
          {/each}
          {#if notBound.some((a) => !a.important)}
            <p class="lo more">Also unbound: {notBound.filter((a) => !a.important).map((a) => a.row?.name ?? a.id).join(", ")}</p>
          {/if}
          {#if otherStick.length}
            <h3 class="gap">On the other stick</h3>
            {#each otherStick as r (r.id)}
              <button class="row" class:sel={selId === r.id} onclick={() => showRow(r)}>
                <span class="n">{r.name}</span><span class="v">{slotText(onOther(r))}</span>
              </button>
            {/each}
          {/if}
          {#if elsewhere.length}
            <h3 class="gap">Elsewhere (keyboard, mouse)</h3>
            {#each elsewhere as r (r.id)}
              <button class="row" class:sel={selId === r.id} onclick={() => showRow(r)}>
                <span class="n">{r.name}</span><span class="v">{slotText(r.slots.find(bound))}</span>
              </button>
            {/each}
          {/if}
        </section>
      {/if}
    </div>
  </section>
  {#if unknown}
    <About label="You pressed" title="{m.side} · {keyName(unknown.key)}">
      <p>VKB's factory profile has no control that sends {keyName(unknown.key)}. If you changed the stick's profile in VKBDevCfg, tell astromech which control it was and it remembers.</p>
      <p>Bound to: {binds.rowsFor(unknown.device, unknown.key).map((r) => r.name).join(", ") || "nothing"}</p>
      <button class="act" onclick={() => (asking = unknown)}>Show me which control</button>
    </About>
  {:else if sel}
    <About label="Action" title={sel.name}>
      <p class="lo">{sel.context}</p>
      {#each sel.slots as s (s.slot)}<p>{sel.analogue ? "Axis" : s.slot}: {slotText(s) || "nothing"}</p>{/each}
      {#each sel.issues as i (i.text)}<p class="warn">! {i.text}</p>{/each}
      <button class="act" onclick={() => (bind = { row: sel, slot: firstFree(sel) })}>{isBound(sel) ? "Bind another control" : "Bind this"}</button>
    </About>
  {:else if picked}
    {@const rows = rowsFor(picked.joy)}
    <About label={binds.last && picked.joy === binds.last.key ? "You pressed" : "Control"} title={picked.control}>
      <p>{picked.joy ? `${control(m.device_id, picked.joy)} on the ${m.side === "L" ? "left" : "right"} stick.` : "astromech doesn't know this control's number yet."}
        {#if picked.confidence === "low"}<span class="lo"> Not confirmed: press it to check.</span>{/if}
        {#if picked.confidence === "yours"}<span class="lo"> Set by you.</span>{/if}</p>
      <div class="k">Bound to</div>
      {#each rows as r (r.id)}<p>{r.name}<span class="lo"> · {r.context}</span></p>{:else}<p class="lo">Nothing in this preset.</p>{/each}
      <div class="k">Wrong box?</div>
      <p class="lo">If pressing this control lights a different box, your stick was remapped. Press it again: astromech asks which control it was.</p>
      {#if picked.confidence === "yours"}<button class="act" onclick={() => picked && void reset(picked)}>Use VKB's number again</button>{/if}
    </About>
  {:else}
    <About label="My controller" title={act?.title ?? m.product}>
      {#if act}
        <p>{act.actions.length} actions, {act.actions.filter((a) => a.important).length} of them important. Lit boxes are this activity's actions.</p>
        <p class="lo">Bind this: press a free control and it's written to your preset (backed up first), or to a copy if the preset is built in.</p>
      {:else}
        <p>Every binding on this stick, in all contexts. Press a control or click a box to see everything on it. Pick an activity above to light just its actions and see what is still unbound.</p>
      {/if}
      <p class="lo">A box with "+2" has more actions on the same control (often in different modes: ship, SRV, FSS).</p>
    </About>
  {/if}
</div>
{#if bind}<BindDialog row={bind.row} slot={bind.slot} onclose={() => (bind = null)} />{/if}

<style>
  button { all: unset; cursor: pointer; }
  .cols { display: grid; grid-template-columns: 1fr 360px; gap: 28px; padding-top: 14px; }
  .devices { display: flex; gap: 32px; text-transform: uppercase; letter-spacing: var(--caps); margin-bottom: 10px; }
  .devices button { color: var(--text-lo); }
  .devices .on { color: var(--accent); }
  .acts { margin-top: 12px; display: flex; flex-wrap: wrap; gap: 22px; margin-bottom: 8px; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .band { display: grid; grid-template-columns: 1fr 1fr; gap: 40px; margin-top: 14px; }
  h3 { margin: 0 0 8px; color: var(--accent); font-weight: 600; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  h3.gap { margin-top: 18px; }
  .row { display: flex; justify-content: space-between; align-items: center; width: 100%; box-sizing: border-box; height: 30px; margin-bottom: 4px; padding: 0 12px; background: var(--cell-empty); }
  .row.sel { outline: 1px solid var(--accent); outline-offset: -1px; }
  .row.sel .n { color: var(--accent); }
  .row .n { color: var(--text); }
  .warn { color: var(--accent); }
  .bind { color: var(--accent); font-weight: 600; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .bind:hover { color: var(--text-hi); }
  .more { margin-top: 10px; font-size: var(--fs-s); line-height: 1.5; }
  .ask { color: var(--text-hi); background: var(--cell); padding: 8px 12px; margin: 0 0 8px; }
  .credit { color: var(--text-lo); font-size: var(--fs-s); }
  .lo { color: var(--text-lo); }
</style>
