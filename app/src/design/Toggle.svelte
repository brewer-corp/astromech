<script lang="ts">
  let { on = false, label }: { on?: boolean; label: string } = $props();
</script>

<div class="toggle" role="switch" aria-checked={on} aria-label={label} tabindex="0">
  <span class="track" class:on><span class="knob"></span></span>
  <span class="lbl">{label}</span>
  <span class="state am-caps">{on ? "On" : "Off"}</span>
</div>

<style>
  .toggle { display: flex; align-items: center; gap: 10px; font: 500 var(--am-size-m) var(--am-font-ui); color: var(--am-text); }
  .track { width: 34px; height: 14px; border: 1px solid var(--am-orange-line); position: relative; background: var(--am-bg-2); }
  .knob { position: absolute; top: 2px; left: 2px; width: 12px; height: 8px; background: var(--am-muted); transition: left var(--am-fast); }
  .on .knob { left: 18px; background: var(--am-orange); box-shadow: var(--am-orange-glow); }
  .lbl { flex: 1; }
  .state { font-size: var(--am-size-xs); color: var(--am-muted); width: 3em; text-align: right; }
</style>
