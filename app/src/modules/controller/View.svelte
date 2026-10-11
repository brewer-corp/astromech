<script lang="ts">
  // My controller (docs/V1-DESIGN.md §3.1-3.2): one stick on VKB's sheet with every binding.
  // Button numbers come from VKB's factory profile. If a press lands somewhere the map doesn't
  // expect (a remapped stick), astromech asks about that one control and remembers the answer.
  import { binds } from "../../lib/store.svelte";
  import { api, type Press } from "../../lib/binds";
  import { maps, controlsWithOverrides, controlFor, type MapControl } from "../../lib/devices";
  import { control, shortKey, keyName } from "../../lib/format";
  import Sheet, { type Cell } from "../../lib/ui/Sheet.svelte";
  import About from "../../lib/ui/About.svelte";

  binds.start();
  let dev = $state(new URLSearchParams(location.search).get("device") ?? maps[0]!.device_id);
  const m = $derived(maps.find((x) => x.device_id === dev) ?? maps[0]!);
  const ov = $derived(binds.state?.overrides ?? {});
  const controls = $derived(controlsWithOverrides(m, ov));
  let picked = $state<MapControl | null>(null);
  let unknown = $state<Press | null>(null);
  let asking = $state<Press | null>(null);
  let w = $state(1600);

  const cells = $derived.by(() => {
    const out: Record<string, Cell> = {};
    for (const c of controls) {
      if (!c.joy) continue;
      const rows = binds.rowsFor(m.device_id, c.joy);
      if (!rows.length) continue;
      const first = rows[0]!;
      out[c.sheet_field] = {
        text: first.name + (rows.length > 1 ? ` +${rows.length - 1}` : ""),
        other: shortKey(first.slots.find((s) => s.device === "Keyboard")),
        sel: picked?.sheet_field === c.sheet_field,
        hi: picked?.sheet_field === c.sheet_field,
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
    {#if asking}<p class="ask">Which control did you just press ({keyName(asking.key)})? Click it on the sheet.</p>{/if}
    <Sheet image={m.image} controls={controls} {cells} width={Math.min(1180, w - 80 - 360 - 28)} onpick={(c) => void pick(c)} />
    <p class="credit">
      {connected ? "Connected" : "Not connected"} · button numbers from VKB's factory profile{yours ? `, ${yours} set by you` : ""} · sheet © VKB Gaming, design by UntoldForce
    </p>
  </section>
  {#if unknown}
    <About label="You pressed" title="{m.side} · {keyName(unknown.key)}">
      <p>VKB's factory profile has no control that sends {keyName(unknown.key)}. If you changed the stick's profile in VKBDevCfg, tell astromech which control it was and it remembers.</p>
      <p>Bound to: {binds.rowsFor(unknown.device, unknown.key).map((r) => r.name).join(", ") || "nothing"}</p>
      <button class="act" onclick={() => (asking = unknown)}>Show me which control</button>
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
    <About label="My controller" title={m.product}>
      <p>Every binding on this stick, in all contexts. Press a control or click a box to see everything on it.</p>
      <p class="lo">A box with "+2" has more actions on the same control (often in different modes: ship, SRV, FSS).</p>
    </About>
  {/if}
</div>

<style>
  button { all: unset; cursor: pointer; }
  .cols { display: grid; grid-template-columns: 1fr 360px; gap: 28px; padding-top: 14px; }
  .devices { display: flex; gap: 32px; text-transform: uppercase; letter-spacing: var(--caps); margin-bottom: 10px; }
  .devices button { color: var(--text-lo); }
  .devices .on { color: var(--accent); }
  .ask { color: var(--text-hi); background: var(--cell); padding: 8px 12px; margin: 0 0 8px; }
  .credit { color: var(--text-lo); font-size: var(--fs-s); }
  .lo { color: var(--text-lo); }
</style>
