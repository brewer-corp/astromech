<script lang="ts">
  // DESIGN.md's dropdown: an ember cell like the input, the value in Orange, an Orange chevron at
  // the right. It is a native <select>, so keyboard use and screen readers work as everywhere;
  // where the engine supports a styled picker (appearance: base-select) the open list is DESIGN's
  // popover (Hull Raised, Seam border, 34 px rows, the current option filled), elsewhere it is the
  // system list. Pass the <option>s as children; `end` sits in the cell after the chevron (a
  // Settings row's action word). Name it with `aria-label` or a <label>.
  import type { Snippet } from "svelte";
  import type { HTMLSelectAttributes } from "svelte/elements";
  import Glyph from "./Glyph.svelte";

  let {
    value = $bindable(),
    children,
    end,
    class: cls = "",
    ...rest
  }: Omit<HTMLSelectAttributes, "value" | "children"> & {
    value?: unknown;
    children: Snippet;
    end?: Snippet;
    class?: string;
  } = $props();
</script>

<span class="dropdown {cls}" class:off={rest.disabled}>
  <span class="pick">
    <select bind:value {...rest}>{@render children()}</select>
    <span class="chev"><Glyph name="chevron" /></span>
  </span>
  {#if end}<span class="end">{@render end()}</span>{/if}
</span>

<style>
  .dropdown { display: inline-flex; align-items: center; min-width: 8rem; background: var(--cell); color: var(--accent); }
  .dropdown:hover { background: var(--cell-hover); }
  .dropdown.off { background: var(--cell-empty); color: var(--orange-dim); }
  .pick { position: relative; display: flex; flex: 1; min-width: 0; }
  select { all: unset; box-sizing: border-box; display: flex; align-items: center; flex: 1; min-width: 0; height: var(--row); padding: 0 calc(var(--sp-3) + 16px + var(--sp-2)) 0 var(--sp-3); color: inherit; font-size: var(--fs); font-variant-numeric: tabular-nums; cursor: pointer; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  select:disabled { cursor: default; }
  select:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .chev { position: absolute; right: var(--sp-3); top: 50%; transform: translateY(-50%); pointer-events: none; }
  .end { display: flex; align-items: center; flex: none; padding-right: var(--sp-1); }
  select :global(:is(option, optgroup)) { background: var(--surface-hi); color: var(--accent); }

  /* The open list as DESIGN.md draws it, where the engine can style it. */
  @supports (appearance: base-select) {
    select, select::picker(select) { appearance: base-select; }
    select::picker-icon { display: none; }
    select::picker(select) { margin: var(--gap) 0 0; padding: 0; border: 1px solid var(--line); background: var(--surface-hi); }
    select :global(option) { display: flex; align-items: center; min-height: var(--row); padding: 0 var(--sp-3); color: var(--accent); font-size: var(--fs); }
    select :global(optgroup) { padding-top: var(--sp-2); color: var(--orange-mid); font-size: var(--fs-s); font-weight: 400; }
    select :global(option::checkmark) { display: none; }
    select :global(option:hover) { background: var(--row-hover); }
    select :global(option:checked) { background: var(--accent); color: var(--on-accent); }
    select :global(option:focus-visible) { outline: 1px solid var(--on-accent); outline-offset: -3px; }
  }
</style>
