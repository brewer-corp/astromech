<script lang="ts">
  // Glow-up mockup: NOW, the home page, docked variant. It follows the game: docked, it leads
  // with where you are and the station's services board; then the four answers a commander
  // wants between jumps, one large value each. What needs you is one orange "!" line.
  import Frame from "./Frame.svelte";
  import Copy from "./Copy.svelte";
  import Icon from "./Icon.svelte";

  // The station services board, in the game's order. false: this station has none.
  const services: [string, boolean][] = [
    ["Commodities", true], ["Refuel", true], ["Repair", true], ["Restock", true],
    ["Outfitting", true], ["Shipyard", true], ["Cartographics", true], ["Vista Genomics", true],
    ["Material trader", false], ["Technology broker", false], ["Interstellar Factors", true], ["Black market", false],
    ["Search and rescue", true], ["Redemption office", true], ["Pioneer Supplies", false], ["Frontline Solutions", false],
    ["Bartender", true], ["Crew lounge", true], ["Apex Interstellar", true], ["Missions", true],
  ];
</script>

<Frame top="Now" strip={{ next: "Wregoe OI-C b54-6", of: "4 of 10", flags: ["Scoopable"], dest: "Synuefe GW-Z a54-3", left: "6 jumps left" }}>
  <div class="now">
    <section class="where">
      <div class="place">
        <span class="kind"><Icon name="orbis" size={24} label="Orbis starport" /></span>
        <div>
          <h1 class="title">Ehrlich City</h1>
          <p class="sub"><span class="label">Docked at pad 14 · Orbis starport in</span><span class="sys">Liaedin</span><Copy /><span class="label">1,210 ls from arrival</span></p>
        </div>
      </div>
      <p class="needs"><span class="bang">!</span>Refuel here before you undock. <span class="mid">3.9 t aboard; the next jump needs 4.6 t.</span></p>
    </section>


    <section class="answers">
      <div class="answer">
        <h2 class="section">Sell your cargo</h2>
        <p class="hero"><span class="vxl">1,169,920</span><span class="unit">cr profit</span></p>
        <p class="line"><span class="num">128</span> t Palladium at <span class="name">Tasman Ring</span></p>
        <p class="label">HIP 19934 · 4 jumps · 58,960 cr/t, here 51,200</p>
      </div>
      <div class="answer">
        <h2 class="section">This session</h2>
        <p class="hero"><span class="vxl">6.24M</span><span class="unit">cr per hour</span></p>
        <p class="line"><span class="num">14.1M</span> cr earned in 2 h 16 m</p>
        <p class="label">22 jumps · 3 sales · 1 mission</p>
      </div>
      <div class="answer">
        <h2 class="section">Next stop</h2>
        <p class="hero"><span class="next">Wregoe OI-C b54-6</span><Copy /></p>
        <p class="line icon-line"><Icon name="star" /><span>K star, scoopable</span></p>
        <p class="label">18.2 ly · stop 4 of 10 · then 6 jumps to Synuefe GW-Z a54-3</p>
      </div>
      <div class="answer">
        <h2 class="section">Powerplay</h2>
        <p class="hero"><span class="vxl">1,240</span><span class="unit">merits this week</span></p>
        <p class="line icon-line"><Icon name="clock" /><span>2 d 14 h left in the cycle</span></p>
        <span class="meter cycle"><span style="width: 63%"></span></span>
        <p class="label">Aisling Duval · rank 38 · 1,860 merits to rank 39</p>
      </div>
    </section>

    <section class="services">
      <h2 class="section">Services</h2>
      <div class="board">
        {#each services as [name, has] (name)}
          <span class="svc" class:none={!has}>{#if name === "Refuel"}<span class="bang">!</span>{/if}{name}</span>
        {/each}
      </div>
    </section>
  </div>
</Frame>

<style>
  /* Three bands: where you are and what needs you; the four answers as one row of large values to
     scan; then the station's services board, which is reference, so it comes last. */
  .now { display: flex; flex-direction: column; gap: 56px; max-width: 1760px; }
  .place { display: flex; align-items: flex-start; gap: var(--sp-3); }
  .kind { color: var(--accent); padding-top: 4px; }
  .sub { display: flex; align-items: baseline; gap: var(--sp-2); margin-top: var(--sp-1); white-space: nowrap; }
  .sub :global(.copy) { align-self: center; margin-right: var(--sp-2); }
  .sys { color: var(--accent); }
  .needs { margin-top: var(--sp-6); font-size: var(--fs-l); color: var(--accent); }
  .needs .mid { font-size: var(--fs); }

  .section { margin-bottom: var(--sp-3); }
  .board { display: grid; grid-template-columns: repeat(5, 1fr); gap: var(--gap); }
  .svc { display: flex; align-items: center; height: 60px; padding: 0 var(--sp-4); background: var(--surface); color: var(--accent); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .svc.none { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px; color: var(--orange-dim); }

  .answers { display: grid; grid-template-columns: repeat(4, 1fr); column-gap: 48px; row-gap: var(--sp-8); }
  .hero { display: flex; align-items: baseline; margin: var(--sp-4) 0 var(--sp-3); white-space: nowrap; }
  .hero :global(.copy) { align-self: center; margin-left: var(--sp-1); }
  .next { font-size: var(--fs-xl); font-weight: 500; line-height: 1; color: var(--orange-lit); }
  .line { color: var(--accent); }
  .name { color: var(--text-hi); }
  .icon-line { display: flex; align-items: center; gap: var(--sp-2); }
  .icon-line :global(.icon) { color: var(--orange-mid); }
  .label { margin-top: 2px; }
  .cycle { width: 220px; margin: var(--sp-2) 0 var(--sp-2) 24px; }

  /* Narrow windows (the 1100 px minimum): two answers a row, and four services a row. */
  @container (max-width: 1400px) {
    .now { gap: 40px; }
    .answers { grid-template-columns: 1fr 1fr; }
    .board { grid-template-columns: repeat(4, 1fr); }
    .svc { height: 48px; }
  }
</style>
