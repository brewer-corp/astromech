<script lang="ts">
  import type { Snippet } from "svelte";
  let { title, sub = "", right }: { title: string; sub?: string; right?: Snippet } = $props();
</script>

<header>
  <div>
    <h1>{title}</h1>
    {#if sub}<p class="am-caps">{sub}</p>{/if}
  </div>
  <div class="r">{@render right?.()}</div>
</header>

<style>
  header { display: flex; align-items: flex-end; justify-content: space-between; border-bottom: 1px solid var(--am-orange-line); padding-bottom: 12px; }
  h1 { margin: 0; font: 400 var(--am-size-xl) var(--am-font-display); color: var(--am-orange); letter-spacing: 0.08em; text-transform: uppercase; text-shadow: var(--am-orange-glow); }
  p { margin: 4px 0 0; font: 500 var(--am-size-xs) var(--am-font-ui); color: var(--am-muted); }
  .r { display: flex; gap: 10px; align-items: center; }
</style>
