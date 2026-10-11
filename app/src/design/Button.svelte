<script lang="ts">
  import type { Snippet } from "svelte";
  let {
    kind = "normal",
    small = false,
    children,
    onclick,
  }: { kind?: "normal" | "primary" | "danger" | "ghost"; small?: boolean; children: Snippet; onclick?: () => void } =
    $props();
</script>

<button class="am-caps {kind}" class:small {onclick}>{@render children()}</button>

<style>
  button {
    font: 600 var(--am-size-s) var(--am-font-ui);
    color: var(--am-orange);
    background: transparent;
    border: 1px solid var(--am-orange-line);
    padding: 7px 16px;
    cursor: pointer;
    clip-path: polygon(var(--am-chamfer-s) 0, 100% 0, 100% calc(100% - var(--am-chamfer-s)),
      calc(100% - var(--am-chamfer-s)) 100%, 0 100%, 0 var(--am-chamfer-s));
    transition: background var(--am-fast) var(--am-ease), color var(--am-fast) var(--am-ease);
  }
  button:hover, button:focus-visible { background: var(--am-orange); color: var(--am-bg); outline: none; }
  .primary { background: var(--am-orange); color: var(--am-bg); }
  .primary:hover { background: var(--am-orange-bright); }
  .danger { color: var(--am-red); border-color: var(--am-red); }
  .danger:hover { background: var(--am-red); color: var(--am-bg); }
  .ghost { border-color: transparent; }
  .small { padding: 4px 10px; font-size: var(--am-size-xs); }
</style>
