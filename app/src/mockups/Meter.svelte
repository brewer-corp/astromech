<script lang="ts">
  // Mic level meter in dBFS (-60..0) with the target speech band and markers.
  let { level, peak, before = null }: { level: number; peak: number; before?: number | null } = $props();
  const x = (db: number) => `${Math.max(0, Math.min(100, ((db + 60) / 60) * 100))}%`;
  const ticks = [-60, -50, -40, -30, -20, -10, 0];
</script>

<div class="meter">
  <div class="bar">
    <div class="band" style="left:{x(-30)};width:calc({x(-12)} - {x(-30)})"></div>
    <div class="fill" style="width:{x(level)}"></div>
    <div class="peak" style="left:{x(peak)}"></div>
    {#if before !== null}<div class="before" style="left:{x(before)}"><span>VR raw {before} dB</span></div>{/if}
  </div>
  <div class="ticks">{#each ticks as t (t)}<span style="left:{x(t)}">{t}</span>{/each}</div>
</div>

<style>
  .meter { position: relative; padding-bottom: 18px; }
  .bar { position: relative; height: 18px; border: 1px solid var(--am-orange-line); background: var(--am-bg-2); }
  .band { position: absolute; top: 0; bottom: 0; background: rgb(123 216 143 / 0.12); border-left: 1px dashed var(--am-green); border-right: 1px dashed var(--am-green); }
  .fill { position: absolute; top: 3px; bottom: 3px; left: 0;
    background: repeating-linear-gradient(90deg, var(--am-orange) 0 6px, transparent 6px 8px); box-shadow: var(--am-orange-glow); }
  .peak { position: absolute; top: -3px; bottom: -3px; width: 2px; background: var(--am-orange-bright); }
  .before { position: absolute; top: -3px; bottom: -3px; width: 2px; background: var(--am-amber); }
  .before span { position: absolute; top: -18px; left: 4px; white-space: nowrap; font: 600 var(--am-size-xs) var(--am-font-ui); color: var(--am-amber); }
  .ticks { position: absolute; left: 0; right: 0; bottom: 0; height: 14px; }
  .ticks span { position: absolute; transform: translateX(-50%); font: 400 10px var(--am-font-mono); color: var(--am-muted); }
</style>
