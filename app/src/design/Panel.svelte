<script lang="ts">
  import type { Snippet } from "svelte";
  let { title = "", tag = "", children, actions }: { title?: string; tag?: string; children: Snippet; actions?: Snippet } =
    $props();
</script>

<section class="panel am-chamfer">
  {#if title}
    <header>
      <span class="tick"></span>
      <h3 class="am-caps">{title}</h3>
      {#if tag}<span class="tag am-caps">{tag}</span>{/if}
      <span class="fill"></span>
      {@render actions?.()}
    </header>
  {/if}
  <div class="body">{@render children()}</div>
</section>

<style>
  .panel {
    position: relative;
    background: linear-gradient(180deg, var(--am-panel-2), var(--am-panel)), var(--am-panel);
    border: var(--am-border);
    box-shadow: inset 0 0 0 1px rgb(0 0 0 / 0.6);
  }
  .panel::after {
    content: "";
    position: absolute;
    inset: 0;
    background: var(--am-scan);
    pointer-events: none;
  }
  header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px var(--am-pad);
    border-bottom: 1px solid var(--am-orange-line);
    background: var(--am-orange-faint);
  }
  .tick { width: 4px; height: 14px; background: var(--am-orange); box-shadow: var(--am-orange-glow); }
  h3 { margin: 0; font: 400 var(--am-size-s) var(--am-font-display); color: var(--am-orange); }
  .tag { font: 500 var(--am-size-xs) var(--am-font-ui); color: var(--am-muted); }
  .fill { flex: 1; }
  .body { position: relative; z-index: 1; padding: var(--am-pad); }
</style>
