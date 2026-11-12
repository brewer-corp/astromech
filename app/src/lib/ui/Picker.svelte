<script lang="ts" module>
  export type PickerOption<V> = { value: V; label: string; hint?: string; disabled?: boolean };
</script>

<script lang="ts" generics="T extends string">
  // DESIGN.md's dropdown where the open list needs more than a native <select> holds: each option
  // a name with a Label line under it, and `extra` under the options (the neutron star file's
  // status and download, under Worked out). The cell is an ember cell: `label` in Label, the
  // current option in Orange, the chevron. The open list is the popover: Hull Raised, Seam
  // border, the current option filled. Arrows move between options, Enter or a click picks,
  // Escape or a click outside closes it. `inline` opens the list in the flow under the cell
  // instead of over the page, for a container that scrolls and would clip it (the header's plot
  // panel).
  import type { Snippet } from "svelte";
  import Glyph from "./Glyph.svelte";

  let {
    label,
    value,
    options,
    onchange,
    extra,
    shown,
    inline = false,
  }: {
    label: string;
    value: T;
    options: PickerOption<T>[];
    onchange: (v: T) => void;
    extra?: Snippet;
    /** What the cell says, when it isn't the current option's label. */
    shown?: string;
    inline?: boolean;
  } = $props();

  let open = $state(false);
  let box = $state<HTMLElement>();
  let cell = $state<HTMLButtonElement>();
  let list = $state<HTMLElement>();
  const id = `picker-${Math.random().toString(36).slice(2, 8)}`;
  const current = $derived(options.find((o) => o.value === value));

  const items = () => [...(list?.querySelectorAll<HTMLButtonElement>('[role="option"]:not([aria-disabled="true"])') ?? [])];

  function toggle() {
    open = !open;
    if (open) queueMicrotask(() => (items().find((b) => b.getAttribute("aria-selected") === "true") ?? items()[0])?.focus());
  }

  function pick(o: PickerOption<T>) {
    if (o.disabled) return;
    open = false;
    cell?.focus();
    if (o.value !== value) onchange(o.value);
  }

  /** The arrows move between options only from the cell or an option: a control under the
   * options (the Efficiency field) keeps its own. */
  const onList = (t: EventTarget | null) => t === cell || (t instanceof Element && t.getAttribute("role") === "option");

  function key(e: KeyboardEvent) {
    if (e.key === "Escape" && open) {
      open = false;
      cell?.focus();
    } else if ((e.key === "ArrowDown" || e.key === "ArrowUp") && open && onList(e.target)) {
      const all = items();
      const i = all.indexOf(document.activeElement as HTMLButtonElement);
      const n = all.length;
      if (!n) return;
      all[(i + (e.key === "ArrowDown" ? 1 : n - 1) + (i < 0 ? 1 : 0)) % n]?.focus();
    } else if (e.key === "ArrowDown" && !open && document.activeElement === cell) toggle();
    else return;
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
<span class="picker" class:inline bind:this={box} onkeydown={key}>
  <button bind:this={cell} type="button" class="cell" aria-haspopup="listbox" aria-expanded={open} aria-controls={id} onclick={toggle}>
    <span class="type-label">{label}</span>
    <span class="val">{shown ?? current?.label ?? ""}</span>
    <Glyph name="chevron" />
  </button>
  {#if open}
    <div class="pop" {id}>
      <div bind:this={list} role="listbox" aria-label={label}>
        {#each options as o (o.value)}
          <button type="button" role="option" class="opt" class:on={o.value === value} aria-selected={o.value === value} aria-disabled={o.disabled} tabindex="-1" onclick={() => pick(o)}>
            <span>{o.label}</span>
            {#if o.hint}<span class="type-label hint">{o.hint}</span>{/if}
          </button>
        {/each}
      </div>
      {#if extra}<div class="extra">{@render extra()}</div>{/if}
    </div>
  {/if}
</span>

<style>
  .picker { position: relative; display: inline-flex; }
  .cell { all: unset; box-sizing: border-box; cursor: pointer; display: inline-flex; align-items: center; gap: var(--sp-3); height: var(--row); padding: 0 var(--sp-3); background: var(--cell); color: var(--accent); white-space: nowrap; }
  .cell:hover { background: var(--cell-hover); }
  .cell:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .cell :global(.glyph) { color: var(--accent); }
  .pop { position: absolute; z-index: 20; left: 0; top: 100%; margin-top: var(--gap); min-width: 100%; width: max-content; max-width: min(28rem, calc(var(--win-w) * 100 - 32px)); background: var(--surface-hi); border: 1px solid var(--line); }
  .opt { all: unset; box-sizing: border-box; cursor: pointer; display: flex; flex-direction: column; justify-content: center; width: 100%; min-height: var(--row); padding: var(--sp-2) var(--sp-3); color: var(--accent); }
  .opt:hover { background: var(--row-hover); }
  .opt:focus-visible { outline: 1px solid var(--accent); outline-offset: -2px; }
  .opt.on { background: var(--accent); color: var(--on-accent); }
  .opt.on .hint { color: var(--on-accent); }
  .opt[aria-disabled="true"] { cursor: default; color: var(--orange-dim); }
  .opt[aria-disabled="true"] .hint { color: var(--orange-dim); }
  .hint { margin-top: 2px; }
  .inline { display: flex; flex-direction: column; align-items: flex-start; }
  .inline .pop { position: static; width: auto; max-width: none; align-self: stretch; }
  .extra { padding: var(--sp-2) var(--sp-3) var(--sp-3); border-top: 1px solid var(--line); }
</style>
