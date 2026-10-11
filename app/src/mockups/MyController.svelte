<script lang="ts">
  import Frame from "./Frame.svelte";
  import About from "./About.svelte";
  import Sheet, { type Cell } from "./Sheet.svelte";
  import left from "../../../data/devices/vkb/gladiator-evo-ot-l.231D3201.json";

  // Everything bound on the left stick, in every context. Example data in the shape of his preset.
  const cells: Record<string, Cell> = {
    Joy_1: { text: "Frame shift drive", other: "J" },
    Joy_3: { text: "Landing gear", other: "L" },
    Joy_4: { text: "Cargo scoop", other: "Home" },
    Joy_5: { text: "Kill switch (astromech)" },
    Joy_10: { text: "Push-to-talk (astromech)", hi: true, sel: true },
    Joy_11: { text: "HUD mode", other: "M" },
    Joy_15: { text: "Enter FSS" },
    Joy_16: { text: "Throttle 100 %", other: "W" },
    Joy_17: { text: "Throttle 75 %" },
    Joy_18: { text: "All stop", other: "X" },
    Joy_POV1Left: { text: "FSS tune left" },
    Joy_POV1Right: { text: "FSS tune right" },
    Joy_YAxis: { text: "Throttle" },
    Joy_RZAxis: { text: "Lateral thrust" },
  };
</script>

<Frame active="controller">
  <div class="cols">
    <section>
      <div class="devices"><span class="on">Left · Gladiator EVO OT L · 231D3201</span><span>Right · Gladiator EVO R · 231D0200</span></div>
      <Sheet image="/devices/vkb/VKB-Gladiator-SCG-OTA-LH-v1.0.png" controls={left.controls} cells={cells} width={980} />
    </section>
    <About label="You pressed" title="A3 centre hat, push">
      <p>Button 10 on the left stick. astromech uses it for push-to-talk. Nothing in Elite is bound to it.</p>
      <div class="k">Button numbers</div>
      <p>From VKB's factory profile. If you changed the profile and a press lands somewhere else, astromech asks you about that one control.</p>
      <span class="act">Show in all keybinds</span>
    </About>
  </div>
  {#snippet footer()}<span>Print</span><span class="note">Example data · VKB sheet © VKB Gaming, design by UntoldForce</span>{/snippet}
</Frame>

<style>
  .cols { display: grid; grid-template-columns: 1fr 300px; gap: 28px; height: 100%; padding-top: 14px; }
  .devices { display: flex; gap: 32px; text-transform: uppercase; letter-spacing: var(--caps); color: var(--text-lo); margin-bottom: 10px; }
  .devices .on { color: var(--accent); }
  .note { margin-left: auto; color: var(--text-lo); font-weight: 400; text-transform: none; letter-spacing: 0; }
</style>
