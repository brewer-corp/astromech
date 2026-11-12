<script lang="ts" module>
  export type MenuItem = { label: string; hint?: string; disabled?: boolean; onselect: () => void };
</script>

<script lang="ts">
  // An icon button that opens a short menu of one-off actions (the Route page's ⋯: Import, Open
  // on Spansh, Clear route). The menu is DESIGN.md's popover: Hull Raised, Seam border, 34 px
  // rows in Orange, Row Hover under the pointer, a Label line under an item that needs one. It
  // opens under the button at its right edge. Arrows move, Enter or a click runs an item and
  // closes the menu, Escape or a click outside closes it.
  import type { GlyphName } from "./Glyph.svelte";
  import IconButton from "./IconButton.svelte";

  let { icon = "more", label, items }: { icon?: GlyphName; label: string; items: MenuItem[] } = $props();

  let open = $state(false);
  let box = $state<HTMLElement>();
  let list = $state<HTMLElement>();
  const id = `menu-${Math.random().toString(36).slice(2, 8)}`;

  const buttons = () => [...(list?.querySelectorAll<HTMLButtonElement>('[role="menuitem"]:not(:disabled)') ?? [])];
  const trigger = () => box?.querySelector<HTMLButtonElement>(".ibtn");

  function toggle() {
    open = !open;
    if (open) queueMicrotask(() => buttons()[0]?.focus());
  }

  function run(item: MenuItem) {
    open = false;
    trigger()?.focus();
    item.onselect();
  }

  function key(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === "Escape") {
      open = false;
      trigger()?.focus();
    } else if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      const all = buttons();
      const n = all.length;
      if (!n) return;
      const i = all.indexOf(document.activeElement as HTMLButtonElement);
      all[(i + (e.key === "ArrowDown" ? 1 : n - 1) + (i < 0 ? 1 : 0)) % n]?.focus();
    } else return;
    e.preventDefault();
    e.stopPropagation();
  }

  $effect(() => {
    if (!open) return;
    const outside = (e: Event) => {
      if (box && !box.contains(e.target as Node)) open = false;
    };
    window.addEventListener("pointerdown", outside);
    window.addEventListener("focusin", outside);
    return () => {
      window.removeEventListener("pointerdown", outside);
      window.removeEventListener("focusin", outside);
    };
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<span class="menu" bind:this={box} onkeydown={key}>
  <IconButton {icon} {label} aria-haspopup="menu" aria-expanded={open} aria-controls={id} onclick={toggle} />
  {#if open}
    <div class="pop" {id} role="menu" aria-label={label} bind:this={list}>
      {#each items as item (item.label)}
        <button type="button" role="menuitem" class="item" disabled={item.disabled} tabindex="-1" onclick={() => run(item)}>
          <span>{item.label}</span>
          {#if item.hint}<span class="type-label">{item.hint}</span>{/if}
        </button>
      {/each}
    </div>
  {/if}
</span>

<style>
  .menu { position: relative; display: inline-flex; }
  .pop { position: absolute; z-index: 20; right: 0; top: 100%; margin-top: var(--gap); width: max-content; max-width: min(24rem, calc(var(--win-w) * 100 - 32px)); background: var(--surface-hi); border: 1px solid var(--line); }
  .item { all: unset; box-sizing: border-box; cursor: pointer; display: flex; flex-direction: column; justify-content: center; width: 100%; min-height: var(--row); padding: var(--sp-2) var(--sp-3); color: var(--accent); }
  .item:hover:not(:disabled) { background: var(--row-hover); }
  .item:focus-visible { outline: 1px solid var(--accent); outline-offset: -2px; }
  .item:disabled { cursor: default; color: var(--orange-dim); }
  .item:disabled .type-label { color: var(--orange-dim); }
</style>
