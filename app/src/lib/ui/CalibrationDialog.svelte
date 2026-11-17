<script lang="ts">
  // The microphone's calibration over the page, as Settings' RUN AGAIN opens it: a 600 px dialog
  // between two orange rules, with Calibration and a Close.
  import Button from "./Button.svelte";
  import Calibration from "./Calibration.svelte";

  let { device, ondone, onclose }: { device: string; ondone: () => void; onclose: () => void } = $props();
</script>

<div class="shade" role="presentation">
  <div class="dlg" role="dialog" aria-modal="true" aria-label="Calibrate the microphone">
    <div class="lbl">Microphone · Calibration</div>
    <h2>{device}</h2>
    <Calibration primary {ondone} />
    <div class="close"><Button onclick={onclose}>Close</Button></div>
  </div>
</div>

<style>
  .shade { position: fixed; inset: 0; background: var(--scrim); display: grid; place-items: center; z-index: 10; }
  .dlg { width: 37.5rem; background: var(--bg); border-top: 2px solid var(--accent); border-bottom: 2px solid var(--accent); padding: var(--sp-6); }
  .lbl { color: var(--label-grey); font-size: var(--fs-s); }
  h2 { margin: var(--sp-1) 0 var(--sp-4); color: var(--text-hi); font-weight: 500; font-size: var(--fs-l); }
  .close { margin-top: var(--sp-4); }
</style>
