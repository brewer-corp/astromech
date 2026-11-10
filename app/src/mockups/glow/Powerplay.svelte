<script lang="ts">
  // Glow-up mockup: Powerplay on one screen (Board). Three answers with one large value each
  // (merits to the next rank, time left in the cycle, merits this cycle); then this system's
  // control as a tug of war, what earns merits here as names with badges (the rules sit in a
  // Tip, shown open), and where to work as a short result list ranked by jumps.
  import Frame from "./Frame.svelte";
  import Copy from "./Copy.svelte";
  import Icon from "./Icon.svelte";
  import Tip from "./Tip.svelte";

  const earn: [string, string[]][] = [
    ["Deliver Aisling Sealed Contracts", ["Best here"]],
    ["Sell exploration data", []],
    ["Bounty hunting", []],
    ["Sell rare goods", []],
    ["Scan data links", []],
    ["Hand in settlement data", ["On foot"]],
    ["Missions for factions here", []],
  ];
  const work = [
    { jumps: "2", system: "HIP 19934", state: "Fortified · under attack", task: "Reinforce" },
    { jumps: "3", system: "Kurumanit", state: "Expansion", task: "Acquire" },
    { jumps: "4", system: "Col 285 Sector SE-T b18-1", state: "Exploited by Felicia Winters", task: "Undermine" },
    { jumps: "5", system: "Eva's Hush", state: "Contested", task: "Acquire" },
    { jumps: "7", system: "LTT 4487", state: "Stronghold", task: "Reinforce" },
  ];
</script>

<Frame top="Powerplay" strip={{ next: "Wregoe OI-C b54-6", of: "4 of 10", flags: ["Scoopable"], dest: "Synuefe GW-Z a54-3", left: "6 jumps left" }}>
  <div class="page">
    <div class="power">
      <span class="mark"><Icon name="aisling" size={24} label="Empire" /></span>
      <h1 class="title">Aisling Duval</h1>
      <span class="label">Pledged 14 weeks · rank 38</span>
    </div>

    <section class="heroes">
      <div>
        <h2 class="section">Next rank</h2>
        <p class="hero"><span class="vxl">1,860</span><span class="unit">merits to rank 39</span></p>
        <span class="meter"><span style="width: 63%"></span></span>
        <p class="label">3,140 of 5,000 merits into rank 38</p>
      </div>
      <div>
        <h2 class="section">This cycle</h2>
        <p class="hero"><span class="vxl">2 d 14 h</span><span class="unit">left</span></p>
        <span class="meter"><span style="width: 63%"></span></span>
        <p class="label">Ends Thursday 07:00 UTC</p>
      </div>
      <div>
        <h2 class="section">Your merits</h2>
        <p class="hero"><span class="vxl">1,240</span><span class="unit">this cycle</span></p>
        <p class="label below">4,410 last cycle · 38,920 since you pledged</p>
      </div>
    </section>

    <section class="cols">
      <div>
        <h2 class="section">This system</h2>
        <p class="sysline"><span class="sys">Liaedin</span><Copy /><span class="label">Fortified by Aisling Duval</span></p>
        <div class="tug" aria-label="Reinforcement 48,210 merits against undermining 31,900">
          <span class="ours" style="width: 60%"></span><span class="theirs" style="width: 40%"></span><span class="mid-tick"></span>
        </div>
        <div class="tug-legend">
          <span><span class="lit fig">48,210</span><span class="unit">reinforcement</span></span>
          <span class="r"><span class="fig">31,900</span><span class="unit">undermining</span></span>
        </div>
        <p class="state">Holding: reinforcement leads by <span class="lit">16,310</span> merits this cycle.</p>
        <p class="label">Undermined by Felicia Winters and Zemina Torval</p>
      </div>

      <div>
        <h2 class="section">What earns merits here</h2>
        <ul class="earn">
          {#each earn as [what, badges], i (what)}
            <li class:hover={i === 0}>
              <span class="what">{what}</span>
              {#each badges as b (b)}<span class="badge">{b}</span>{/each}
              {#if i === 0}
                <Tip side="below" width={420} pos="left: 0; height: 128px">
                  <b>Deliver Aisling Sealed Contracts.</b> Collect up to 25 t at a Fortified or Stronghold system's
                  Powerplay contact every 30 minutes, then deliver them to any station here. Each tonne earns
                  merits; contracts you still carry when the cycle ends are lost.
                </Tip>
              {/if}
            </li>
          {/each}
        </ul>
      </div>

      <div>
        <h2 class="section">Where to work</h2>
        <div class="work">
          <span class="label r">Jumps</span><span class="label">System</span><span class="label">Do</span><span></span>
          {#each work as w (w.system)}
            <span class="r jumps">{w.jumps}</span>
            <span class="place"><span class="wsys">{w.system}</span><Copy /><span class="label st">{w.state}</span></span>
            <span class="task">{w.task}</span>
            <span class="acts">
              <button class="ibtn" aria-label="Add {w.system} as a stop"><Icon name="add-stop" /></button>
              <button class="ibtn" aria-label="Set {w.system} as the destination"><Icon name="destination" /></button>
            </span>
          {/each}
        </div>
      </div>
    </section>
  </div>
</Frame>

<style>
  .page { display: flex; flex-direction: column; gap: 56px; max-width: 1760px; }
  .power { display: flex; align-items: center; gap: var(--sp-3); }
  .mark { color: var(--accent); }
  .power .label { margin-left: var(--sp-3); }

  .heroes, .cols { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); column-gap: 64px; }
  .section { margin-bottom: var(--sp-4); }
  .hero { display: flex; align-items: baseline; margin-bottom: var(--sp-3); white-space: nowrap; }
  .heroes .meter { width: 70%; margin-bottom: var(--sp-2); }
  .heroes .below { margin-top: 13px; }

  .sysline { display: flex; align-items: baseline; gap: var(--sp-2); margin-bottom: var(--sp-4); }
  .sysline :global(.copy) { align-self: center; margin-right: var(--sp-2); }
  .sys { font-size: var(--fs-l); font-weight: 600; color: var(--text-hi); }
  .tug { position: relative; display: flex; height: 12px; gap: 2px; margin-top: var(--sp-6); }
  .ours { background: var(--accent); }
  .theirs { background: var(--orange-dim); }
  .mid-tick { position: absolute; left: 50%; top: -5px; bottom: -5px; border-left: 1px solid var(--options-grey); }
  .tug-legend { display: flex; justify-content: space-between; margin: var(--sp-2) 0 var(--sp-4); }
  .fig { font-size: var(--fs-l); font-weight: 600; color: var(--accent); }
  .state { color: var(--accent); margin-bottom: var(--sp-1); }

  .earn { display: grid; gap: var(--gap); }
  .earn li { position: relative; display: flex; align-items: center; gap: var(--sp-2); height: var(--row-result); padding: 0 var(--sp-3); }
  .earn li.hover { background: var(--row-hover); }
  .what { color: var(--accent); margin-right: var(--sp-1); }
  .earn li:first-child .what { color: var(--orange-lit); }

  .work { display: grid; grid-template-columns: 48px minmax(0, 220px) 96px 64px; align-items: center; column-gap: var(--sp-4); row-gap: var(--sp-4); }
  .jumps { font-size: var(--fs-l); font-weight: 600; color: var(--orange-lit); }
  .place { display: grid; grid-template-columns: auto auto; grid-template-rows: auto auto; justify-content: start; min-width: 0; column-gap: var(--sp-1); }
  .wsys { color: var(--accent); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .place :global(.copy) { margin: -4px 0; }
  .st { grid-column: 1 / 3; margin-top: -2px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .task { color: var(--accent); }
  .acts { display: flex; }
</style>
