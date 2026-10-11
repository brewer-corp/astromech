<script lang="ts">
  import Frame from "./Frame.svelte";
  import Sheet, { type Cell } from "./Sheet.svelte";
  import left from "../../../data/devices/vkb/gladiator-evo-ot-l.231D3201.json";
  import right from "../../../data/devices/vkb/gladiator-evo-r.231D0200.json";

  const activities = ["Flight basics", "Supercruise & jumps", "Exploration", "Trading", "Mining", "Combat", "Docking & landing", "SRV", "On foot"];

  // Example bindings in the shape of his DualVKB preset. Real from his files: L Joy_1
  // Frame shift drive, L Joy_16 Throttle 100 %, L Joy_10 push-to-talk. The rest is example data.
  const L: Record<string, Cell> = {
    Joy_1: { text: "Frame shift drive", other: "J", hi: true },
    Joy_11: { text: "HUD mode", other: "M", hi: true },
    Joy_15: { text: "Enter FSS", hi: true },
    Joy_POV1Left: { text: "FSS tune left", hi: true },
    Joy_POV1Right: { text: "FSS tune right", hi: true },
    Joy_16: { text: "Throttle 100 %", other: "W" },
    Joy_17: { text: "Throttle 75 %" },
    Joy_18: { text: "All stop", other: "X" },
    Joy_3: { text: "Landing gear", other: "L" },
    Joy_4: { text: "Cargo scoop", other: "Home" },
    Joy_10: { text: "Push-to-talk (astromech)" },
    Joy_5: { text: "Kill switch (astromech)" },
    Joy_YAxis: { text: "Throttle" },
    Joy_RZAxis: { text: "Lateral thrust" },
  };
  const R: Record<string, Cell> = {
    Joy_1: { text: "Discovery scan", other: "+ Fire 1", hi: true },
    Joy_3: { text: "FSS target", hi: true },
    Joy_12: { text: "FSS zoom in", other: "Page Up", hi: true },
    Joy_14: { text: "FSS zoom out", other: "Page Down", hi: true },
    Joy_13: { text: "Exit FSS", hi: true },
    Joy_10: { text: "Galaxy map", other: ",", hi: true },
    Joy_20: { text: "System map", other: ".", hi: true },
    Joy_6: { text: "DSS next signal", hi: true },
    Joy_8: { text: "DSS previous signal", hi: true },
    Joy_9: { text: "Exit DSS", hi: true },
    Joy_2: { text: "Fire 2" },
    Joy_4: { text: "Hardpoints", other: "U" },
    Joy_5: { text: "Boost", other: "Tab" },
    Joy_POV1Up: { text: "Thrust up" },
    Joy_POV1Down: { text: "Thrust down" },
    Joy_POV1Left: { text: "Thrust left" },
    Joy_POV1Right: { text: "Thrust right" },
    Joy_XAxis: { text: "Roll" },
    Joy_YAxis: { text: "Pitch" },
    Joy_RZAxis: { text: "Yaw" },
  };
  const missing = [
    ["Target next system in route", "TargetNextRouteSystem", true],
    ["Toggle planet front/back (DSS)", "ExplorationSAAChangeScannedAreaViewToggle", false],
    ["Codex: go to discovery", "OpenCodexGoToDiscovery", false],
  ] as const;
  const W = 900;
</script>

<Frame active="activities">
  <div class="acts">
    {#each activities as a (a)}<span class:on={a === "Exploration"}>{a}</span>{/each}
    <span class="sp"></span><span class="hint">Press any control to find it</span>
  </div>
  <div class="sheets">
    <div><div class="cap">Left · Gladiator EVO OT L</div><Sheet image="/devices/vkb/VKB-Gladiator-SCG-OTA-LH-v1.0.png" controls={left.controls} cells={L} width={W} dimOthers /></div>
    <div><div class="cap">Right · Gladiator EVO R</div><Sheet image="/devices/vkb/VKB-Gladiator-SCG-RH-v1.0.png" controls={right.controls} cells={R} width={W} dimOthers /></div>
  </div>
  <div class="band">
    <section>
      <h3>Not bound yet</h3>
      {#each missing as [name, id, sel] (id)}
        <div class="row" class:sel><span class="n">{name}</span><span class="bind">Bind this</span></div>
      {/each}
    </section>
    <section>
      <h3>Elsewhere for exploration</h3>
      <div class="row"><span class="n">FSS help</span><span class="v">H</span></div>
      <div class="row"><span class="n">Step zoom FSS in / out</span><span class="v">Mouse wheel</span></div>
      <div class="row"><span class="n">FSS look around</span><span class="v">Mouse</span></div>
    </section>
    <section class="about">
      <h3>Target next system in route</h3>
      <p>Targets the next jump on your plotted route, so you can jump again without opening the galaxy map. You need it on every long trip.</p>
      <p class="lo">Free on your sticks: A4 down on the left (13), C1 push on the left (20). Press one to bind it.</p>
    </section>
  </div>
  {#snippet footer()}<span>Show all activities</span><span class="note">Example data in the shape of your DualVKB preset · VKB sheets © VKB Gaming</span>{/snippet}
</Frame>

<style>
  .acts { display: flex; gap: 26px; padding: 14px 0 10px; text-transform: uppercase; letter-spacing: var(--caps); color: var(--text-lo); }
  .acts .on { color: var(--accent); }
  .sp { flex: 1; }
  .hint { text-transform: none; letter-spacing: 0; font-size: var(--fs-s); }
  .sheets { display: flex; gap: 40px; }
  .cap { text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); color: var(--text-lo); margin-bottom: 6px; }
  .band { display: grid; grid-template-columns: 1fr 1fr 1.2fr; gap: 40px; margin-top: 14px; }
  h3 { margin: 0 0 8px; color: var(--accent); font-weight: 600; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .row { display: flex; justify-content: space-between; align-items: center; height: 30px; margin-bottom: var(--gap); padding: 0 12px; background: var(--cell-empty); }
  .row .n { text-transform: uppercase; letter-spacing: var(--caps); }
  .row.sel { outline: 1px solid var(--accent); outline-offset: -1px; }
  .row.sel .n { color: var(--accent); }
  .bind { color: var(--accent); font-weight: 600; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .v { color: var(--text); }
  .about { border-left: 2px solid var(--accent); padding-left: 24px; }
  .about h3 { color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  .about p { margin: 0 0 8px; line-height: 1.45; }
  .lo { color: var(--text-lo); }
  .note { margin-left: auto; color: var(--text-lo); font-weight: 400; text-transform: none; letter-spacing: 0; }
</style>
