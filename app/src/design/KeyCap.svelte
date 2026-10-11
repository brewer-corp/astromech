<script lang="ts">
  let { label, kind = "key" }: { label: string; kind?: "key" | "joy" | "missing" } = $props();
</script>

<span class="cap {kind}">{label}</span>

<style>
  .cap {
    display: inline-block;
    font: 400 var(--am-size-s) var(--am-font-mono);
    padding: 1px 7px;
    border: 1px solid var(--am-orange-line);
    color: var(--am-text-strong);
    background: var(--am-bg-2);
    white-space: nowrap;
  }
  .joy { border-color: var(--am-blue); color: var(--am-blue); background: var(--am-blue-faint); }
  .missing { border-style: dashed; color: var(--am-muted); }
</style>
