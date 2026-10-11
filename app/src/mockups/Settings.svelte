<script lang="ts">
  import Frame from "./Frame.svelte";
  import About from "./About.svelte";
  type R = { label: string; value: string; kind?: "bind" | "opt" | "meter" | "link"; sel?: boolean; listening?: boolean };
  const groups: [string, R[]][] = [
    ["Push-to-talk", [
      { label: "Button", value: "Press a button…   Esc to cancel", kind: "bind", sel: true, listening: true },
    ]],
    ["Kill switch", [
      { label: "Button", value: "L · JOY 5", kind: "bind" },
      { label: "Hold to re-arm", value: "1 s", kind: "opt" },
    ]],
    ["Microphone", [
      { label: "Device", value: "BlackShark V3 Pro", kind: "opt" },
      { label: "Input level", value: "-18", kind: "meter" },
      { label: "Level correction", value: "Automatic (+12 dB)", kind: "opt" },
      { label: "Calibration", value: "Done 10 Oct · Run again", kind: "link" },
    ]],
    ["Voice responses", [
      { label: "Spoken confirmations", value: "On", kind: "opt" },
      { label: "Voice", value: "Kokoro · af_heart", kind: "opt" },
      { label: "Volume", value: "80 %", kind: "opt" },
      { label: "Game event callouts", value: "Off", kind: "opt" },
    ]],
    ["Debug", [
      { label: "Save push-to-talk clips", value: "Off", kind: "opt" },
      { label: "Log every input event", value: "Off", kind: "opt" },
      { label: "Ignore button bounces under", value: "150 ms", kind: "opt" },
      { label: "Ignore clips shorter than", value: "300 ms", kind: "opt" },
    ]],
  ];
</script>

<Frame active="settings">
  <div class="cols">
    <section class="list">
      {#each groups as [g, rows] (g)}
        <div class="g">{g}</div>
        {#each rows as r (g + r.label)}
          <div class="row" class:sel={r.sel}>
            <span class="lbl">{r.label}</span>
            {#if r.kind === "meter"}
              <span class="val meter"><span class="bar"><span class="fill" style="width:70%"></span><span class="tgt"></span></span><span class="db">{r.value} dBFS</span></span>
            {:else}
              <span class="val {r.kind}" class:listening={r.listening}>{r.value}</span>
            {/if}
          </div>
        {/each}
      {/each}
    </section>
    <About title="Push-to-talk button">
      <p>Hold this button while you speak a command. Release it to send.</p>
      <p>Press the button you want on any controller now. astromech reads controllers without taking them from the game.</p>
      <div class="k">Current</div>
      <p>L · Joy 10 (Gladiator EVO OT L)</p>
      <div class="k">Good to know</div>
      <p>Use a button that isn’t bound in Elite, or the game will act on it too. astromech warns you if it is.</p>
    </About>
  </div>
  {#snippet footer()}<span>Reset to defaults</span><span class="note">Changes apply immediately</span>{/snippet}
</Frame>

<style>
  .cols { display: grid; grid-template-columns: 1fr 380px; gap: 28px; height: 100%; padding-top: 18px; }
  .list { overflow: hidden; max-width: 820px; }
  .g { color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; margin: 18px 0 6px; }
  .g:first-child { margin-top: 4px; }
  .row { display: grid; grid-template-columns: 1fr 380px; gap: var(--gap); align-items: center; margin-bottom: var(--gap); }
  .lbl { text-transform: uppercase; letter-spacing: var(--caps); padding-left: 16px; }
  .val { height: var(--row); display: flex; align-items: center; padding: 0 12px; background: var(--cell-opt); }
  .val.bind { background: var(--cell); }
  .val.link { background: none; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  .row.sel .lbl { color: var(--accent); }
  .val.listening { background: var(--accent); color: var(--on-accent); font-weight: 600; text-transform: uppercase; letter-spacing: var(--caps); white-space: pre; }
  .meter { gap: 12px; }
  .bar { position: relative; flex: 1; height: 8px; background: #111; }
  .fill { position: absolute; inset: 0 auto 0 0; background: var(--accent); }
  .tgt { position: absolute; left: 75%; top: -4px; bottom: -4px; width: 2px; background: var(--text-hi); }
  .db { font-family: var(--mono); color: var(--text-lo); font-size: var(--fs-s); width: 72px; text-align: right; }
  .note { margin-left: auto; color: var(--text-lo); font-weight: 400; text-transform: none; letter-spacing: 0; }
</style>
