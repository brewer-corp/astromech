<script lang="ts">
  // Glow-up mockup: Exploration > System as a system tree (List with a detail panel). Stars,
  // planets and moons hang off a spine, one body-type icon each. The one main value is what the
  // body is worth mapped, first in the row as in every lookup row. Bodies worth mapping are lit,
  // mapped ones step down to label orange; badges only where they change what you do.
  import Frame from "./Frame.svelte";
  import Copy from "./Copy.svelte";
  import Icon, { type IconName } from "./Icon.svelte";

  type Body = { d: number; icon: IconName; name: string; type: string; ls: string; value: string; state: "star" | "worth" | "plain" | "mapped"; badges?: string[] };
  const bodies: Body[] = [
    { d: 0, icon: "star", name: "A", type: "K star", ls: "0", value: "4,820", state: "star", badges: ["Scoopable"] },
    { d: 1, icon: "metal-rich", name: "A 1", type: "Metal-rich body", ls: "8", value: "31,200", state: "plain", badges: ["Landable"] },
    { d: 1, icon: "hmc", name: "A 2", type: "High metal content world", ls: "412", value: "68,400", state: "plain", badges: ["Landable"] },
    { d: 1, icon: "water", name: "A 3", type: "Water world", ls: "690", value: "1,248,300", state: "mapped" },
    { d: 1, icon: "hmc", name: "A 4", type: "High metal content world", ls: "1,204", value: "1,612,400", state: "worth", badges: ["Terraformable", "Landable", "Bio 2"] },
    { d: 2, icon: "icy", name: "A 4 a", type: "Icy body", ls: "1,204", value: "2,900", state: "plain", badges: ["Landable"] },
    { d: 1, icon: "gas-giant", name: "A 5", type: "Gas giant with water-based life", ls: "1,890", value: "41,800", state: "plain" },
    { d: 2, icon: "rocky-ice", name: "A 5 a", type: "Rocky ice world", ls: "1,890", value: "9,700", state: "plain", badges: ["Landable", "Bio 1"] },
    { d: 2, icon: "icy", name: "A 5 b", type: "Icy body", ls: "1,891", value: "3,100", state: "plain" },
    { d: 1, icon: "earth-like", name: "A 6", type: "Earth-like world", ls: "3,050", value: "4,912,300", state: "worth", badges: ["First discovery"] },
    { d: 2, icon: "rocky", name: "A 6 a", type: "Rocky body", ls: "3,051", value: "22,300", state: "plain", badges: ["Landable"] },
    { d: 0, icon: "star", name: "B", type: "M red dwarf", ls: "18,320", value: "2,950", state: "star" },
    { d: 1, icon: "icy", name: "B 1", type: "Icy body", ls: "18,330", value: "2,700", state: "plain" },
    { d: 1, icon: "ammonia", name: "B 2", type: "Ammonia world", ls: "18,345", value: "2,887,300", state: "worth", badges: ["First discovery"] },
    { d: 1, icon: "gas-giant", name: "B 3", type: "Class I gas giant", ls: "18,400", value: "38,200", state: "plain" },
    { d: 0, icon: "star-dwarf", name: "C", type: "T brown dwarf", ls: "41,200", value: "1,820", state: "star" },
  ];
  const selected = "A 4";

  // The spine: for each row, which ancestor levels still have a sibling below (a line passes
  // through), and whether this body is the last child of its parent (its line stops here).
  function lastChild(i: number): boolean {
    const d = bodies[i]!.d;
    for (let j = i + 1; j < bodies.length; j++) {
      if (bodies[j]!.d < d) return true;
      if (bodies[j]!.d === d) return false;
    }
    return true;
  }
  const rows = bodies.map((b, i) => ({
    ...b,
    last: lastChild(i),
    pass: Array.from({ length: Math.max(0, b.d - 1) }, (_, k) => {
      const lvl = k + 1;
      for (let j = i + 1; j < bodies.length; j++) {
        if (bodies[j]!.d < lvl) return false;
        if (bodies[j]!.d === lvl) return true;
      }
      return false;
    }),
  }));
</script>

<Frame top="Exploration" tabs={["System", "Exobiology", "Codex"]} tab="System"
  strip={{ next: "Synuefe ZL-J d10-109", of: "1 of 6", flags: ["Scoopable"], dest: "Col 285 Sector RD-S b18-0", left: "6 jumps left" }}
  status={{ ship: "Long Way Round", type: "Krait Mk II", jump: "31.4", fuel: "24.6 / 32", cargo: "0 / 144" }}>
  <div class="page">
    <section class="main">
      <div class="head">
        <div>
          <div class="name-row"><h1 class="title">Synuefe GW-Z a54-3</h1><Copy /></div>
          <p class="label">16 bodies, all scanned · 4 worth mapping</p>
        </div>
        <div class="heroes">
          <p class="hero"><span class="vxl base">1,412,960</span><span class="unit">cr now</span></p>
          <p class="hero"><span class="vxl">10,824,960</span><span class="unit">cr if you map the best 3</span></p>
        </div>
      </div>

      <div class="list">
        <div class="row heads label"><span class="r">Mapped value</span><span class="r">Distance</span><span>Body</span><span></span></div>
        {#each rows as b (b.name)}
          <div class="row {b.state}" class:sel={b.name === selected}>
            <span class="r value">{b.value}<span class="unit">cr</span></span>
            <span class="r ls">{b.ls}<span class="unit">ls</span></span>
            <span class="tree">
              {#each b.pass as p, k (k)}<span class="guide" class:on={p}></span>{/each}
              {#if b.d > 0}<span class="elbow" class:last={b.last}></span>{/if}
              <span class="ico"><Icon name={b.icon} /></span>
              <span class="bname">{b.name}</span>
              <span class="btype">{b.type}</span>
            </span>
            <span class="badges">{#each b.badges ?? [] as t (t)}<span class="badge">{t}</span>{/each}</span>
          </div>
        {/each}
      </div>
    </section>

    <aside>
      <h2 class="title">A 4</h2>
      <p class="label">High metal content world · Synuefe GW-Z a54-3</p>
      <p class="key"><span class="vxl white">1,612,400</span><span class="unit">cr mapped</span></p>
      <p class="label">412,300 cr now · 1,204 ls from arrival</p>
      <dl>
        <dt class="label">Gravity</dt><dd>0.31 g</dd>
        <dt class="label">Temperature</dt><dd>182 K</dd>
        <dt class="label">Atmosphere</dt><dd>Thin carbon dioxide</dd>
        <dt class="label">Signals</dt><dd>2 biological · 3 geological</dd>
      </dl>
      <h3 class="section">What to do</h3>
      <ol class="todo">
        <li><span class="step">1</span><span>Map it: 6 probes for the efficiency bonus.</span></li>
        <li><span class="step">2</span><span>Land for the 2 biological signals. Likely here:</span></li>
      </ol>
      <div class="species">
        <span>Stratum Tectonicas</span><span class="r"><span class="lit">19,010,800</span><span class="unit">cr</span></span>
        <span>Bacterium Aurasus</span><span class="r">1,000,000<span class="unit">cr</span></span>
      </div>
      <div class="acts"><button class="act">Open in Exobiology</button></div>
    </aside>
  </div>
</Frame>

<style>
  .page { display: grid; grid-template-columns: minmax(0, 1fr) var(--panel); gap: var(--sp-12); max-width: 1760px; }
  .head { display: flex; justify-content: space-between; align-items: flex-end; gap: var(--sp-8); margin-bottom: var(--sp-6); }
  .name-row { display: flex; align-items: center; gap: var(--sp-2); }
  .head .label { margin-top: var(--sp-1); }
  .heroes { display: flex; gap: var(--sp-12); }
  .hero { display: flex; align-items: baseline; white-space: nowrap; }
  .vxl.base { color: var(--accent); }

  .row { display: grid; grid-template-columns: 136px 96px 400px minmax(0, 1fr); align-items: center; column-gap: var(--sp-6); height: 40px; padding: 0 var(--sp-3); white-space: nowrap; }
  .row.heads { height: 28px; border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); }
  .value { color: var(--accent); }
  .row.worth .value { font-size: var(--fs-l); font-weight: 600; color: var(--orange-lit); }
  .row.mapped .value, .row.star .value { color: var(--orange-mid); }
  .row.mapped .bname, .row.mapped .btype, .row.mapped .ico { color: var(--orange-mid); }

  .tree { display: flex; align-items: center; height: 100%; min-width: 0; }
  .guide, .elbow { position: relative; flex: none; width: 24px; height: 100%; }
  .guide.on::before, .elbow::before { content: ""; position: absolute; left: 7px; top: 0; bottom: 0; border-left: 1px solid var(--orange-dim); }
  .elbow.last::before { bottom: 50%; }
  .elbow::after { content: ""; position: absolute; left: 7px; right: 4px; top: 50%; border-top: 1px solid var(--orange-dim); }
  .ico { color: var(--accent); margin-right: var(--sp-3); }
  .row.star .ico { color: var(--orange-lit); }
  .bname { font-weight: 600; min-width: 52px; margin-right: var(--sp-3); color: var(--text-hi); }
  .row.plain .bname { color: var(--accent); }
  .btype { color: var(--orange-mid); overflow: hidden; text-overflow: ellipsis; }
  .row.worth .btype { color: var(--accent); }
  .badges { display: flex; gap: var(--sp-1); overflow: hidden; }
  .ls { color: var(--accent); }
  .row.mapped .ls, .row.star .ls { color: var(--orange-mid); }

  .row.sel { background: var(--accent); }
  .row.sel, .row.sel :global(*) { color: var(--on-accent) !important; }
  .row.sel .badge { border-color: var(--on-accent); }
  .row.sel .guide.on::before, .row.sel .elbow::before, .row.sel .elbow::after { border-color: var(--on-accent); }

  aside { border-left: 1px solid var(--accent); padding-left: var(--sp-6); }
  aside > .label { margin-top: var(--sp-1); }
  .key { display: flex; align-items: baseline; margin: var(--sp-6) 0 var(--sp-1); }
  dl { display: grid; grid-template-columns: 112px 1fr; gap: var(--sp-2) var(--sp-3); margin: var(--sp-6) 0 var(--sp-8); }
  dt { align-self: center; }
  .section { margin-bottom: var(--sp-3); }
  .todo { display: grid; gap: var(--sp-2); }
  .todo li { display: flex; gap: var(--sp-3); color: var(--prose); }
  .step { color: var(--accent); font-weight: 600; }
  .species { display: grid; grid-template-columns: 1fr auto; gap: var(--sp-1) var(--sp-3); margin: var(--sp-2) 0 var(--sp-4) 22px; color: var(--accent); }
  .acts { margin-left: calc(-1 * var(--sp-2)); }
</style>
