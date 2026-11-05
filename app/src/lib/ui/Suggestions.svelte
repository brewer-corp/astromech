<script lang="ts" generics="T">
  // The popover under an Input that suggests values as you type (system names, commodities,
  // what to find). It is DESIGN.md's open dropdown list: Hull Raised, a Seam border, 34 px rows in
  // Orange, Row Hover under the pointer, the highlighted one filled orange. Put it in the Input's
  // children so it hangs from the cell. A `.sub` span in `item` is a Label-size aside (a category).
  import type { Snippet } from "svelte";

  let {
    items,
    sel,
    key,
    onpick,
    item,
    id,
  }: {
    items: T[];
    /** The highlighted suggestion, which Enter takes. */
    sel: number;
    key: (t: T, i: number) => string;
    onpick: (t: T) => void;
    item: Snippet<[T]>;
    /** The listbox id; option i is `${id}-${i}`, for aria-activedescendant. */
    id?: string;
  } = $props();
</script>

<ul {id} class="hits" role="listbox">
  {#each items as h, i (key(h, i))}
    <li>
      <button
        type="button"
        id={id ? `${id}-${i}` : undefined}
        role="option"
        aria-selected={i === sel}
        class:on={i === sel}
        onmousedown={(e) => e.preventDefault()}
        onclick={() => onpick(h)}>{@render item(h)}</button
      >
    </li>
  {/each}
</ul>

<style>
  .hits { position: absolute; z-index: 20; top: 100%; left: 0; right: 0; margin: var(--gap) 0 0; padding: 0; list-style: none; background: var(--surface-hi); border: 1px solid var(--line); }
  button { all: unset; box-sizing: border-box; cursor: pointer; display: flex; align-items: center; width: 100%; min-height: var(--row); padding: 0 var(--sp-3); color: var(--accent); font-size: var(--fs); }
  button:hover { background: var(--row-hover); }
  button.on { background: var(--accent); color: var(--on-accent); }
  button :global(.sub) { margin-left: var(--sp-2); font-size: var(--fs-s); color: var(--orange-mid); }
  button.on :global(.sub) { color: var(--on-accent); }
</style>
