<script lang="ts">
  // DESIGN.md's buttons, in Action type (13 px, 600, caps, 0.06em tracking), 34 px high.
  // primary: solid orange with On Orange text, at most one per view, for the step forward;
  //   disabled, Ember Empty with Orange Dim text.
  // secondary: an orange caps verb with no fill; Row Hover under the pointer; disabled, Orange Dim.
  // A repeated row action is an IconButton instead.
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  let {
    kind = "secondary",
    type = "button",
    children,
    class: cls = "",
    ref = $bindable(null),
    ...rest
  }: HTMLButtonAttributes & { kind?: "primary" | "secondary"; children: Snippet; class?: string; ref?: HTMLButtonElement | null } = $props();
</script>

<button bind:this={ref} class="btn {kind} {cls}" {type} {...rest}>{@render children()}</button>

<style>
  .btn { all: unset; box-sizing: border-box; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; gap: var(--sp-2); flex: none; height: var(--row); font-size: var(--fs-s); font-weight: 600; letter-spacing: var(--caps); text-transform: uppercase; white-space: nowrap; }
  .btn:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .btn:disabled { cursor: default; }
  .primary { padding: 0 20px; background: var(--accent); color: var(--on-accent); }
  .primary:disabled { background: var(--cell-empty); color: var(--orange-dim); }
  .secondary { padding: 0 var(--sp-2); color: var(--accent); }
  .secondary:hover:not(:disabled) { background: var(--row-hover); }
  .secondary:disabled { color: var(--orange-dim); }
</style>
