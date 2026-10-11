<script lang="ts">
  // Activities (docs/V1-DESIGN.md §3.3), the default screen: your sticks on VKB's own sheets
  // with one activity's actions lit, then what's not bound yet and what's bound elsewhere
  // (keyboard, mouse). An action can be in several activities.
  import catalog from "../../../../data/catalog/activities.json";
  import { binds } from "../../lib/store.svelte";
  import { maps, controlsWithOverrides, controlFor, type DeviceMap } from "../../lib/devices";
  import { bound, slotText, shortKey, control } from "../../lib/format";
  import type { Row } from "../../lib/binds";
  import Sheet, { type Cell } from "../../lib/ui/Sheet.svelte";
  import BindDialog from "../../lib/ui/BindDialog.svelte";
  import PresetPicker from "../../lib/ui/PresetPicker.svelte";

  type Act = { id: string; title: string; actions: { id: string; important: boolean; alternatives: string[] }[] };
  const acts = catalog.activities as Act[];
  let actId = $state(new URLSearchParams(location.search).get("activity") ?? localStorage.getItem("astromech.activity") ?? "flight-basics");
  const act = $derived(acts.find((a) => a.id === actId) ?? acts[0]!);
  let selId = $state<string | null>(null);
  let bind = $state<{ row: Row; slot: string } | null>(null);
  let pressedField = $state<string | null>(null);
  let pressedText = $state<string | null>(null);
  let w = $state(1600);

  binds.start();

  function pick(id: string) {
    actId = id;
    localStorage.setItem("astromech.activity", id);
    selId = null;
  }
  const byId = $derived(new Map(binds.rows.map((r) => [r.id, r])));
  const ids = $derived(new Set(act.actions.flatMap((a) => [a.id, ...a.alternatives])));
  const isBound = (r: Row | undefined) => !!r && r.slots.some(bound);
  const onSticks = (r: Row) => r.slots.some((s) => bound(s) && maps.some((m) => m.device_id === s.device));
  const notBound = $derived(
    act.actions.filter((a) => !isBound(byId.get(a.id)) && !a.alternatives.some((x) => isBound(byId.get(x)))).map((a) => ({ ...a, row: byId.get(a.id) })),
  );
  const elsewhere = $derived(act.actions.map((a) => byId.get(a.id)).filter((r): r is Row => !!r && isBound(r) && !onSticks(r)));
  const keyOf = (r: Row) => shortKey(r.slots.find((s) => s.device === "Keyboard"));
  const sheetW = $derived(Math.max(560, Math.min(980, (w - 80 - 40) / 2)));

  function cells(m: DeviceMap): Record<string, Cell> {
    const out: Record<string, Cell> = {};
    for (const c of controlsWithOverrides(m, binds.state?.overrides ?? {})) {
      if (!c.joy) continue;
      const rows = binds.rowsFor(m.device_id, c.joy);
      const mine = rows.filter((r) => ids.has(r.id));
      const r = mine[0] ?? rows.find((x) => x.context === "Ship" || x.context === "General") ?? rows[0];
      if (!r) continue;
      out[c.sheet_field] = { text: r.name + (mine.length > 1 ? ` +${mine.length - 1}` : ""), other: keyOf(r), hi: mine.length > 0, sel: pressedField === `${m.device_id}:${c.sheet_field}` || (!!selId && rows.some((x) => x.id === selId)) };
    }
    return out;
  }

  function firstFree(r: Row): string {
    if (r.analogue) return "Binding";
    return r.slots.find((s) => s.slot === "Primary" && !bound(s)) ? "Primary" : "Secondary";
  }

  let seen = binds.presses;
  $effect(() => {
    const p = binds.last;
    if (binds.presses === seen || !p || binds.capturing) return;
    seen = binds.presses;
    const c = binds.state ? controlFor(p.device, p.key, binds.state.overrides) : undefined;
    pressedField = c ? `${p.device}:${c.sheet_field}` : null;
    const rows = binds.rowsFor(p.device, p.key);
    pressedText = `${control(p.device, p.key)}${c ? ` (${c.control})` : ""}: ${rows.map((r) => r.name).join(", ") || "not bound"}`;
  });
  const sel = $derived(selId ? byId.get(selId) : undefined);
</script>

<svelte:window bind:innerWidth={w} />

<div class="acts">
  {#each acts as a (a.id)}<button class:on={a.id === act.id} onclick={() => pick(a.id)}>{a.title}</button>{/each}
</div>
<div class="sub">
  <span>{pressedText ? `Pressed ${pressedText}` : "Press any control to find it. Click a box to see what's on it."}</span>
  <span class="sp"></span>
  <span class="pp"><PresetPicker /></span>
</div>
<div class="sheets">
  {#each maps as m (m.device_id)}
    <div>
      <div class="cap">{m.title}{binds.state && !binds.state.inventory.controllers.some((c) => c.elite_id === m.device_id) ? " · not connected" : ""}</div>
      <Sheet image={m.image} controls={controlsWithOverrides(m, binds.state?.overrides ?? {})} cells={cells(m)} width={sheetW} dimOthers onpick={(c) => { const r = c.joy ? binds.rowsFor(m.device_id, c.joy)[0] : undefined; selId = r?.id ?? null; pressedField = `${m.device_id}:${c.sheet_field}`; pressedText = `${m.side} · ${c.control}: ${c.joy ? binds.rowsFor(m.device_id, c.joy).map((x) => x.name).join(", ") || "not bound" : "unknown button"}`; }} />
    </div>
  {/each}
</div>
<div class="band">
  <section>
    <h3>Not bound yet</h3>
    {#each notBound.filter((a) => a.important) as a (a.id)}
      <div class="row" class:sel={selId === a.id}>
        <button class="n" onclick={() => (selId = a.id)}>{a.row?.name ?? a.id}</button>
        {#if a.row}<button class="bind" onclick={() => a.row && (bind = { row: a.row, slot: firstFree(a.row) })}>Bind this</button>{/if}
      </div>
    {:else}
      <p class="lo">Everything important for {act.title.toLowerCase()} is bound.</p>
    {/each}
    {#if notBound.some((a) => !a.important)}
      <p class="lo more">Also unbound: {notBound.filter((a) => !a.important).map((a) => a.row?.name ?? a.id).join(", ")}</p>
    {/if}
  </section>
  <section>
    <h3>Elsewhere for {act.title.toLowerCase()}</h3>
    {#each elsewhere as r (r.id)}
      <button class="row" class:sel={selId === r.id} onclick={() => (selId = r.id)}><span class="n">{r.name}</span><span class="v">{slotText(r.slots.find(bound))}</span></button>
    {:else}
      <p class="lo">Nothing: every bound action is on your sticks.</p>
    {/each}
  </section>
  <section class="about">
    {#if sel}
      <h3>{sel.name}</h3>
      {#each sel.slots as s (s.slot)}<p>{sel.analogue ? "Axis" : s.slot}: {slotText(s) || "nothing"}</p>{/each}
      {#each sel.issues as i (i.text)}<p class="warn">! {i.text}</p>{/each}
      <button class="act" onclick={() => (bind = { row: sel, slot: firstFree(sel) })}>{isBound(sel) ? "Bind another control" : "Bind this"}</button>
    {:else}
      <h3>{act.title}</h3>
      <p>{act.actions.length} actions, {act.actions.filter((a) => a.important).length} of them important. Lit boxes are this activity's actions; the key after a name is its keyboard slot.</p>
      <p class="lo">Bind this: press a free control and it's written to your preset (backed up first), or to a copy if the preset is built in.</p>
    {/if}
  </section>
</div>
<p class="credit">VKB reference sheets © VKB Gaming, design by UntoldForce · button numbers from VKB's factory profile</p>
{#if bind}<BindDialog row={bind.row} slot={bind.slot} onclose={() => (bind = null)} />{/if}

<style>
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .acts { display: flex; flex-wrap: wrap; gap: 26px; padding: 14px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); }
  .acts button { color: var(--text-lo); }
  .acts button:hover { color: var(--text); }
  .acts .on { color: var(--accent); }
  .sub { display: flex; align-items: center; gap: 20px; color: var(--text-lo); font-size: var(--fs-s); min-height: 40px; }
  .sub .pp { width: 520px; }
  .sub .pp :global(.row) { margin: 0; }
  .sp { flex: 1; }
  .sheets { display: flex; gap: 40px; margin-top: 6px; }
  .cap { text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); color: var(--text-lo); margin-bottom: 6px; }
  .band { display: grid; grid-template-columns: 1fr 1fr 1.2fr; gap: 40px; margin-top: 14px; }
  h3 { margin: 0 0 8px; color: var(--accent); font-weight: 600; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .row { display: flex; justify-content: space-between; align-items: center; width: 100%; height: 30px; margin-bottom: var(--gap); padding: 0 12px; background: var(--cell-empty); }
  .row.sel { outline: 1px solid var(--accent); outline-offset: -1px; }
  .row.sel .n { color: var(--accent); }
  .bind, .act { color: var(--accent); font-weight: 600; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .bind:hover, .act:hover { color: var(--text-hi); }
  .v { color: var(--text); }
  .about { border-left: 2px solid var(--accent); padding-left: 24px; }
  .about h3 { color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; text-transform: none; letter-spacing: 0; }
  .about p { margin: 0 0 8px; line-height: 1.45; }
  .lo { color: var(--text-lo); }
  .more { margin-top: 10px; font-size: var(--fs-s); line-height: 1.5; }
  .warn { color: var(--accent); }
  .credit { color: var(--text-lo); font-size: var(--fs-s); margin: 18px 0; }
</style>
