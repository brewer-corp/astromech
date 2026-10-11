<script lang="ts">
  // Status chip: always icon + word, so it never depends on colour alone.
  let { level, text }: { level: "ok" | "warn" | "error" | "info" | "idle"; text: string } = $props();
  const icon = { ok: "✓", warn: "!", error: "✕", info: "i", idle: "–" };
</script>

<span class="chip {level} am-caps"><b>{icon[level]}</b>{text}</span>

<style>
  .chip { display: inline-flex; align-items: center; gap: 6px; font: 600 var(--am-size-xs) var(--am-font-ui); padding: 2px 8px 2px 4px; border: 1px solid currentColor; }
  b { display: inline-grid; place-items: center; width: 14px; height: 14px; background: var(--c); color: var(--am-bg); font-size: 10px; }
  .chip { color: var(--c); }
  .ok { --c: var(--am-green); }
  .warn { --c: var(--am-amber); }
  .error { --c: var(--am-red); }
  .info { --c: var(--am-blue); }
  .idle { --c: var(--am-muted); }
</style>
