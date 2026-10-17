<script lang="ts">
  // Press-to-bind (docs/V1-DESIGN.md §3.3 "Bind this"): wait for one press, show what else uses
  // that control, then write it into the slot. Built-in presets are never edited: the first edit
  // makes a named copy in your Bindings folder. Every write makes a backup first.
  import { api, isWindowClick, type Press, type Row, type Slot } from "../binds";
  import { binds } from "../store.svelte";
  import { control, slotText } from "../format";
  import { controlFor } from "../devices";

  let { row, slot, onclose }: { row: Row; slot: string; onclose: () => void } = $props();
  let got = $state<Press | null>(null);
  let copyName = $state(`${binds.view?.name ?? "My preset"} astromech`);
  let busy = $state(false);
  let error = $state<string | null>(null);
  let done = $state<string | null>(null);
  const seen = binds.presses;
  const editable = $derived(binds.view?.editable ?? false);
  const current = $derived(row.slots.find((s) => s.slot === slot));
  const others = $derived(got ? binds.rowsFor(got.device, got.key).filter((r) => r.id !== row.id) : []);
  const ctl = $derived(got && binds.state ? controlFor(got.device, got.key, binds.state.overrides) : undefined);

  $effect(() => {
    binds.capturing = true;
    return () => (binds.capturing = false);
  });
  $effect(() => {
    const p = binds.last;
    if (binds.presses === seen || !p || got) return;
    // A click in this dialog arrives as Mouse_1; it is not the binding.
    if (isWindowClick(p)) return;
    // Axes only for axis rows; buttons and keys for everything else.
    if ((p.kind === "axis") !== row.analogue) return;
    got = p;
  });

  async function save(clear = false) {
    if (!binds.view) return;
    busy = true;
    error = null;
    try {
      let path = binds.view.path;
      if (!editable) {
        path = await api.copy(path, copyName);
        const ctx = (binds.state?.active ?? []).flatMap((n, i) => (n === binds.view?.name ? [i] : []));
        if (ctx.length) await api.setActive(copyName.trim(), ctx);
      }
      const binding: Slot | null = clear || !got ? null : { slot, device: got.device, key: got.key, modifiers: [] };
      const r = await api.setSlot({ path, action: row.id, slot, analogue: row.analogue, binding });
      done = `Saved to ${path.split("\\").pop()}. Backup: ${r.backup.split("\\").pop()}.`;
      await binds.show(path);
    } catch (e) {
      error = String(e);
    }
    busy = false;
  }
</script>

<div class="shade" role="presentation">
  <div class="dlg" role="dialog" aria-modal="true" aria-label="Bind {row.name}">
    <div class="lbl">{row.analogue ? "Bind an axis" : `Bind · ${slot.toLowerCase()} slot`}</div>
    <h2>{row.name}</h2>
    <p class="lo">Now: {slotText(current) || "not bound"}</p>
    {#if done}
      <p>{done}</p>
      <p class="lo">Elite reads bindings when it starts. If it's running, open Options → Controls and pick the preset again, or restart it.</p>
      <div class="acts"><button onclick={onclose}>Close</button></div>
    {:else}
      <div class="press" class:got>
        {#if got}
          {control(got.device, got.key)}{#if ctl}<span class="lo"> · {ctl.control}</span>{/if}
        {:else}
          {row.analogue ? "Move an axis on a stick…" : "Press a button, hat or key…"}
        {/if}
      </div>
      {#if others.length}
        <p class="warn">Also used by {others.map((o) => o.name).join(", ")}. Both will respond to it.</p>
      {/if}
      {#if !editable}
        <label class="copy">
          <span>This preset can't be edited. Save into a copy named</span>
          <input bind:value={copyName} maxlength="40" />
          <span class="lo">The copy goes in your Bindings folder and replaces this preset where it's active.</span>
        </label>
      {:else}
        <p class="lo">Writes to {binds.view?.path.split("\\").pop()} after making a backup.</p>
      {/if}
      {#if error}<p class="warn">{error}</p>{/if}
      <div class="acts">
        <button disabled={!got || busy} onclick={() => save()}>Save</button>
        {#if got}<button onclick={() => (got = null)}>Press again</button>{/if}
        {#if current?.device}<button disabled={busy} onclick={() => save(true)}>Clear this slot</button>{/if}
        <button onclick={onclose}>Cancel</button>
      </div>
    {/if}
  </div>
</div>

<style>
  .shade { position: fixed; inset: 0; background: rgb(0 0 0 / 0.7); display: grid; place-items: center; z-index: 10; }
  .dlg { width: 560px; background: var(--bg); border-top: 2px solid var(--accent); border-bottom: 2px solid var(--accent); padding: 22px 28px; }
  .lbl { color: var(--accent); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  h2 { margin: 6px 0 4px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-xl); }
  p { margin: 8px 0; line-height: 1.45; }
  .lo { color: var(--text-lo); }
  .warn { color: var(--accent); }
  .press { margin: 16px 0; height: 48px; display: flex; align-items: center; padding: 0 14px; background: var(--cell-empty); color: var(--text-lo); font-size: var(--fs-l); }
  .press.got { background: var(--cell); color: var(--text-hi); }
  .copy { display: grid; gap: 6px; margin: 12px 0; }
  input { all: unset; height: var(--row); padding: 0 12px; background: var(--cell-opt); color: var(--text-hi); border-bottom: 2px solid var(--accent); }
  .acts { display: flex; gap: 28px; margin-top: 18px; }
  button { all: unset; cursor: pointer; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover, button:focus-visible { color: var(--text-hi); }
</style>
