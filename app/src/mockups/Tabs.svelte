<script lang="ts">
  let { tabs, active }: { tabs: string[]; active: string } = $props();
</script>

<div class="tabs">
  {#each tabs as t (t)}<span class="t am-caps" class:on={t === active}>{t}</span>{/each}
</div>

<style>
  .tabs { display: flex; gap: 2px; }
  .t { font: 600 var(--am-size-s) var(--am-font-ui); color: var(--am-muted); padding: 7px 18px; border: 1px solid var(--am-orange-line); border-bottom: none; background: var(--am-bg-2); }
  .t.on { color: var(--am-bg); background: var(--am-orange); border-color: var(--am-orange); }
</style>
