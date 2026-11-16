<script lang="ts">
  // The route collapsed: today's 36 px strip, with the open-route control at its left end where the
  // sidebar would be. `open` draws that control pressed (the route opened over the page).
  import Glyph from "../../lib/ui/Glyph.svelte";
  import { NEXT, ROUTE } from "./route";
  let { open = false }: { open?: boolean } = $props();
</script>

<div class="strip">
  <span class="ibtn side" class:on={open} aria-label={open ? "Close the route" : "Open the route"} aria-expanded={open}><span class="chev"><Glyph name="chevron" /></span></span>
  <span class="label">Next</span>
  <span class="sys">{NEXT}</span>
  <span class="ibtn copy" aria-label="Copy {NEXT}"><Glyph name="copy" /></span>
  <span class="of">{ROUTE.n} of {ROUTE.of}</span>
  <span class="badge">Fuel</span>
  <span class="grow"></span>
  <span class="label">To</span><span class="dest">{ROUTE.to}</span>
  <span class="of">{ROUTE.left}<span class="unit">jumps left</span></span>
</div>

<style>
  .strip { display: flex; align-items: center; gap: var(--sp-3); height: var(--strip); padding: 0 var(--side) 0 4px; background: var(--surface); white-space: nowrap; font-size: var(--fs); }
  .side { color: var(--accent); }
  .side.on { background: var(--accent); color: var(--on-accent); }
  .chev { display: grid; transform: rotate(-90deg); }
  .side.on .chev { transform: rotate(90deg); }
  .sys { color: var(--orange-lit); font-weight: 600; }
  .copy { margin-left: -8px; }
  .dest { color: var(--accent); }
  .of { color: var(--orange-mid); }
  .grow { flex: 1; }
</style>
