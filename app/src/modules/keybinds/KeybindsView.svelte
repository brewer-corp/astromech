<script lang="ts">
  // All keybinds (docs/V1-DESIGN.md §3.1): the game's Controls list, improved. Search, context
  // and state filters, issue counts per group, inline health markers, empty slots that look
  // empty, and pressing any control jumps to what it does.
  import { tip } from "../../lib/ui/tip";
  import Button from "../../lib/ui/Button.svelte";
  import Choice from "../../lib/ui/Choice.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import { binds } from "../../lib/store.svelte";
  import { slotText, bound, control } from "../../lib/format";
  import { controlFor } from "../../lib/devices";
  import { isWindowClick, type Issue, type Row } from "../../lib/binds";
  import About from "../../lib/ui/About.svelte";
  import BindDialog from "../../lib/ui/BindDialog.svelte";
  import PresetPicker from "../../lib/ui/PresetPicker.svelte";
  import { previewOption } from "../../lib/transport";

  const CTX = ["All", "Ship", "SRV", "On foot", "General"] as const;
  const STATE = ["All", "Issues", "Unbound"] as const;
  let q = $state(previewOption("q") ?? "");
  let ctx = $state<(typeof CTX)[number]>("All");
  let only = $state<(typeof STATE)[number]>("All");
  let open = $state<Record<string, boolean>>({});
  let selId = $state<string | null>(null);
  let hits = $state<string[]>([]);
  let bind = $state<{ row: Row; slot: string } | null>(null);
  let pressed = $state<string | null>(null);
  let search: HTMLInputElement | HTMLTextAreaElement | undefined = $state();

  binds.start();

  const issues = (r: Row) => r.issues.filter((i) => i.level !== "info");
  const mark = (i: Issue) =>
    i.code === "no_keyboard" ? "No keyboard key" : i.code === "device_missing" ? "Device not connected" : i.code === "paste_key" ? "Clashes with route paste" : (i.text.split(".")[0] ?? i.text);
  const unbound = (r: Row) => !r.slots.some(bound);
  const words = $derived(q.trim().toLowerCase().split(/\s+/).filter(Boolean));
  const groups = $derived(
    (binds.view?.groups ?? [])
      .filter((g) => ctx === "All" || g.context === ctx)
      .map((g) => ({
        ...g,
        all: g.rows,
        rows: g.rows.filter((r) => {
          if (only === "Issues" && !issues(r).length) return false;
          if (only === "Unbound" && !unbound(r)) return false;
          if (hits.length && !q) return true;
          const hay = `${r.name} ${r.id} ${r.slots.map(slotText).join(" ")}`.toLowerCase();
          return words.every((w) => hay.includes(w));
        }),
      }))
      .filter((g) => g.rows.length),
  );
  const totalIssues = $derived(binds.rows.reduce((n, r) => n + issues(r).length, 0));
  // A panel you toggled stays as you left it. Otherwise panels open only when a search or state
  // filter asks to show their rows (a pressed control opens its own group); how many groups remain never decides.
  const isOpen = (name: string) => {
    if (open[name] !== undefined) return open[name]!;
    if (q || only !== "All") return true;
    const g = groups.find((x) => x.name === name);
    return !!g?.all.some((r) => issues(r).length);
  };
  const sel = $derived(binds.rows.find((r) => r.id === selId) ?? null);

  // Press a control: jump to everything bound to it.
  let seen = binds.presses;
  $effect(() => {
    const p = binds.last;
    if (binds.presses === seen || !p || binds.capturing) return;
    seen = binds.presses;
    if (isWindowClick(p)) return;
    if (document.activeElement === search && p.device === "Keyboard") return;
    const rows = binds.rowsFor(p.device, p.key);
    const ctl = binds.state ? controlFor(p.device, p.key, binds.state.overrides) : undefined;
    pressed = `${control(p.device, p.key)}${ctl ? ` (${ctl.control})` : ""}: ${rows.length ? rows.map((r) => r.name).join(", ") : "not bound to anything"}`;
    hits = rows.map((r) => r.id);
    if (rows[0]) {
      q = "";
      selId = rows[0].id;
      for (const r of rows) open[r.group] = true;
      setTimeout(() => document.getElementById(`row-${rows[0]?.id}`)?.scrollIntoView({ block: "center" }), 0);
    }
  });

  // ?bind=Action:Slot (browser preview / screenshots) opens the bind dialog.
  const want = previewOption("bind");
  let opened = false;
  $effect(() => {
    if (!want || opened || !binds.rows.length) return;
    const [id, slot = "Primary"] = want.split(":");
    const r = binds.rows.find((x) => x.id === id);
    if (r) {
      opened = true;
      selId = r.id;
      bind = { row: r, slot };
    }
  });

  function keydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "f") {
      e.preventDefault();
      search?.focus();
    }
  }
</script>

<svelte:window onkeydown={keydown} />

<div class="cols">
  <section class="list">
    <Input bind:input={search} bind:value={q} aria-label="Search actions or keys" placeholder="Search actions or keys, or press any control" oninput={() => (hits = [])}>
      {#snippet end()}Ctrl+F{/snippet}
    </Input>
    <PresetPicker />
    <div class="filters">
      <Choice label="Where" value={ctx} options={CTX.map((c) => ({ value: c, label: c }))} onchange={(v) => (ctx = v)} />
      <span class="sp"></span>
      <Choice label="Show" value={only} options={STATE.map((s) => ({ value: s, label: `${s}${s === "Issues" && totalIssues ? ` ${totalIssues}` : ""}` }))} onchange={(v) => (only = v)} />
    </div>
    {#if pressed}<div class="pressed">Pressed {pressed}</div>{/if}
    <div class="head"><span>Action</span><span>Primary</span><span>Secondary</span><span></span></div>
    <div class="scroll">
      {#each groups as g (g.name)}
        {@const n = g.all.reduce((k, r) => k + issues(r).length, 0)}
        <button class="row grp" class:open={isOpen(g.name)} onclick={() => (open[g.name] = !isOpen(g.name))}>
          <span class="box">{isOpen(g.name) ? "−" : "+"}</span>
          <span class="gname">{g.name}</span>
          <span class="iss">{n ? `${n} issue${n > 1 ? "s" : ""}` : ""}</span>
        </button>
        {#if isOpen(g.name)}
          {#each g.rows as r (r.id)}
            {@const iss = issues(r)}
            <div class="row item" id="row-{r.id}" class:sel={selId === r.id} class:hit={hits.includes(r.id)}>
              <button class="name" onclick={() => (selId = r.id)}>{r.name}</button>
              {#if r.analogue}
                <button class="cell wide" class:empty={!bound(r.slots[0])} {@attach tip(bound(r.slots[0]) ? "Click to select, click again to rebind" : "Set this axis")} onclick={() => { const again = selId === r.id; selId = r.id; if (!bound(r.slots[0]) || again) bind = { row: r, slot: "Binding" }; }}>{slotText(r.slots[0]) || "Set"}</button>
              {:else}
                {#each ["Primary", "Secondary"] as s (s)}
                  {@const sl = r.slots.find((x) => x.slot === s)}
                  <button
                    class="cell"
                    class:empty={!bound(sl)}
                    class:bad={sl && /^Joy_(3[3-9]|[4-9]\d|\d{3})$/.test(sl.key)}
                    {@attach tip(bound(sl) ? "Click to select, click again to rebind" : "Set this slot")}
                    onclick={() => {
                      const again = selId === r.id;
                      selId = r.id;
                      if (!bound(sl) || again) bind = { row: r, slot: s };
                    }}>{slotText(sl) || "Set"}</button>
                {/each}
              {/if}
              <span class="mark">{iss[0] ? `! ${mark(iss[0])}` : ""}</span>
            </div>
          {/each}
        {/if}
      {:else}
        <p class="lo pad">{binds.view ? "Nothing matches." : binds.state && !binds.state.presets.length ? "No keybindings found. Run Elite once, or set the Bindings folder in Settings." : binds.error ? "" : "Loading…"}</p>
      {/each}
    </div>
  </section>
  {#if sel}
    <About label={sel.group} title={sel.name}>
      <div class="k">Bound to</div>
      {#each sel.slots as s (s.slot)}
        <p>{sel.analogue ? "Axis" : s.slot}: {slotText(s) || "nothing"}{#if bound(s) && binds.state}{@const c = controlFor(s.device, s.key, binds.state.overrides)}{#if c}<span class="lo"> · {c.control}</span>{/if}{/if}</p>
      {/each}
      {#each sel.issues as i (i.text)}
        <div class="k">{i.level === "info" ? "Note" : "! Issue"}</div>
        <p>{i.text}</p>
      {/each}
      {#if !sel.in_file}<p class="lo">Not in this preset's file, so the game has nothing bound.</p>{/if}
      {#if sel.analogue}
        <Button class="act" onclick={() => (bind = { row: sel, slot: "Binding" })}>Bind an axis…</Button>
      {:else}
        <Button class="act" onclick={() => (bind = { row: sel, slot: "Primary" })}>Set primary…</Button>
        <Button class="act" onclick={() => (bind = { row: sel, slot: "Secondary" })}>Set secondary…</Button>
      {/if}
      <p class="lo id">Elite name: {sel.id}</p>
    </About>
  {:else}
    <About label="All keybinds" title="Press any control">
      <p>Press a button, hat or key and the list jumps to what it does. Or search by name or key.</p>
      <p>Click an empty slot to bind it. Click a filled slot to select it, then again to rebind.</p>
      {#if binds.view}<div class="k">This preset</div><p>{binds.view.issue_count} issue{binds.view.issue_count === 1 ? "" : "s"} · {binds.view.devices.map((d) => control(d, "").replace(" · ", "") || d).join(", ") || "keyboard and mouse only"}</p>{/if}
      {#each binds.view?.problems ?? [] as p (p)}<p class="warn">! {p}</p>{/each}
    </About>
  {/if}
</div>
{#if bind}<BindDialog row={bind.row} slot={bind.slot} onclose={() => (bind = null)} />{/if}

<style>
  .cols { display: grid; grid-template-columns: minmax(0, 1fr) 360px; gap: 28px; height: 100%; padding-top: 18px; min-height: 0; }
  .list { display: flex; flex-direction: column; min-height: 0; }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .row { display: grid; grid-template-columns: 1fr 200px 200px 190px; gap: var(--gap); align-items: center; min-height: var(--row); }
  .filters { display: flex; gap: var(--sp-4); margin: 12px 0 6px; }
  .sp { flex: 1; }
  .pressed { padding: 6px 0 6px 34px; color: var(--orange-lit); }
  .head { display: grid; grid-template-columns: 1fr 200px 200px 190px; gap: var(--gap); padding: 6px 0 6px 34px; color: var(--orange-mid); font-size: var(--fs-s); }
  .head span:nth-child(2), .head span:nth-child(3) { padding-left: 12px; }
  .scroll { overflow: auto; min-height: 0; flex: 1; padding-bottom: 20px; }
  .grp { grid-template-columns: 22px 1fr 190px; gap: 12px; width: 100%; }
  .box { width: 18px; height: 18px; border: 1.5px solid var(--accent); color: var(--accent); display: grid; place-items: center; font-weight: 600; line-height: 1; }
  .gname { font-weight: 500; }
  .grp:hover .gname { color: var(--orange-lit); }
  .iss { color: var(--orange-mid); font-size: var(--fs-s); }
  .item { margin-bottom: var(--gap); }
  .item .name { padding-left: 34px; height: var(--row); display: flex; align-items: center; color: var(--options-grey); }
  .item:hover .name { color: var(--accent); }
  .cell { height: var(--row); display: flex; align-items: center; padding: 0 12px; background: var(--cell); color: var(--options-grey); overflow: hidden; white-space: nowrap; text-overflow: ellipsis; }
  .cell.wide { grid-column: span 2; }
  /* An empty slot's "Set" is what the commander clicks to bind it, so it is Orange Mid (4.7:1 on
     Ember Empty), not Dim. An ignored binding is struck through and Dim. */
  .cell.empty { background: var(--cell-empty); color: var(--orange-mid); }
  .cell.bad { text-decoration: line-through; color: var(--orange-dim); }
  .cell:hover { background: var(--cell-hover); }
  .mark { color: var(--accent); font-size: var(--fs-s); padding-left: 10px; font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .item.sel .name, .item.hit .name { color: var(--accent); }
  .item.sel .cell { outline: 1px solid var(--accent); outline-offset: -1px; }
  .lo { color: var(--orange-mid); }
  .warn { color: var(--accent); }
  .pad { padding-left: 34px; }
  .id { margin-top: 24px; font-size: var(--fs-s); }
  @container win (max-width: 1200px) {
    .cols { grid-template-columns: minmax(0, 1fr) 260px; gap: 20px; }
    .row, .head { grid-template-columns: minmax(0, 1fr) 120px 120px 110px; }
    .grp { grid-template-columns: 22px minmax(0, 1fr) 110px; }
  }
</style>
