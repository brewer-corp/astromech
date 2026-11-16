<script lang="ts">
  // ENGINEERING > Engineers, revised (tone 5). Ranks and grades are Elite's hexagons (Hex.svelte).
  // In the tree, brightness and one colour say where each engineer stands: unlocked white with
  // their rank; one you can work on now in Orange Mid with its next step; not introduced dim; barred
  // red-low. The "laser" answer groups each engineer's facts into three: what they reach (best
  // grade as orange hexagons, the per-laser grades under it), who they are to you (name, rank),
  // and where (system, distance), then the actions.
  import Frame from "./Frame.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import Hex from "./Hex.svelte";

  let { query = "" }: { query?: string } = $props();

  type Eng = { name: string; state: "unlocked" | "known" | "hidden" | "barred"; rank?: number; at?: string; step?: string; kids?: Eng[]; also?: string[] };
  const ship: Eng[] = [
    { name: "Tod McQuinn", state: "unlocked", rank: 5, at: "Wolf 397", kids: [
      { name: "Selene Jean", state: "unlocked", rank: 5, at: "Kuk", kids: [
        { name: "Didi Vatermann", state: "unlocked", rank: 2, at: "Leesti" },
        { name: "Bill Turner", state: "known", step: "Unlock: 50 Bromellite" },
      ] },
      { name: "Petra Olmanova", state: "known", step: "Invite: Expert combat rank" },
    ] },
    { name: "Felicity Farseer", state: "unlocked", rank: 5, at: "Deciat", kids: [
      { name: "Juri Ishmaak", state: "unlocked", rank: 4, at: "Giryak", kids: [
        { name: "Colonel Bris Dekker", state: "known", step: "Invite: Friendly, Federation" },
        { name: "The Sarge", state: "barred", step: "Barred: Federation hostile" },
      ] },
    ] },
    { name: "Elvira Martuuk", state: "unlocked", rank: 5, at: "Khun", kids: [
      { name: "Marco Qwent", state: "known", step: "Unlock: 25 Modular terminals", kids: [
        { name: "Professor Palin", state: "hidden" },
        { name: "Chloe Sedesi", state: "hidden" },
        { name: "Lori Jameson", state: "hidden" },
      ] },
      { name: "Zacariah Nemo", state: "known", step: "Unlock: 25 Xihe companions" },
      { name: "Mel Brandon", state: "known", step: "Invite: Friendly, Colonia" },
    ] },
    { name: "Liz Ryder", state: "unlocked", rank: 4, at: "Eurybia", kids: [
      { name: "Hera Tani", state: "unlocked", rank: 5, at: "Kuwemaki", kids: [
        { name: "Broo Tarquin", state: "unlocked", rank: 3, at: "Muang" },
        { name: "Tiana Fortune", state: "known", step: "Invite: Friendly, Empire" },
      ] },
      { name: "Etienne Dorn", state: "known", step: "Invite: Dealer trade rank" },
    ] },
    { name: "The Dweller", state: "unlocked", rank: 5, at: "Wyrd", kids: [
      { name: "Lei Cheung", state: "known", step: "Invite: trade at 50 markets", kids: [{ name: "Ram Tah", state: "hidden" }] },
      { name: "Marsha Hicks", state: "known", step: "Invite: Surveyor explorer rank" },
    ] },
  ];
  const suit: Eng[] = [
    { name: "Jude Navarro", state: "unlocked", at: "Aurai", kids: [
      { name: "Terra Velasquez", state: "unlocked", at: "Shou Xing", kids: [{ name: "Oden Geiger", state: "unlocked", at: "Candiaei" }] },
    ] },
    { name: "Domino Green", state: "unlocked", at: "Orishis", kids: [
      { name: "Kit Fowler", state: "known", step: "Invite: sell 5 Opinion Polls", kids: [{ name: "Yarden Bond", state: "hidden" }] },
    ] },
    { name: "Hero Ferrari", state: "known", step: "Invite: 10 conflict zones", kids: [
      { name: "Wellington Beck", state: "hidden", kids: [{ name: "Uma Laszlo", state: "hidden" }] },
    ] },
    { name: "Baltanos", state: "known", step: "Invite: Friendly, Colonia" },
    { name: "Eleanor Bresa", state: "known", step: "Invite: visit 5 settlements" },
    { name: "Rosa Dayette", state: "known", step: "Invite: sell 10 recipes", kids: [{ name: "Yi Shen", state: "hidden", also: ["Baltanos", "Eleanor Bresa"] }] },
  ];

  const W = 200, H = 50, GX = 28, PITCH = 56;
  type Node = Eng & { x: number; y: number };
  function layout(roots: Eng[]) {
    const nodes: Node[] = [];
    const links: { from: Node; to: Node }[] = [];
    let row = 0;
    function place(e: Eng, depth: number): Node {
      const kids = (e.kids ?? []).map((k) => place(k, depth + 1));
      const y = kids.length ? (kids[0]!.y + kids[kids.length - 1]!.y) / 2 : row++ * PITCH;
      const n = { ...e, x: depth * (W + GX), y };
      nodes.push(n);
      for (const k of kids) links.push({ from: n, to: k });
      return n;
    }
    roots.forEach((r) => place(r, 0));
    for (const n of nodes) for (const a of n.also ?? []) { const f = nodes.find((m) => m.name === a); if (f) links.push({ from: f, to: n }); }
    return { nodes, links, height: row * PITCH - (PITCH - H), width: 3 * W + 2 * GX };
  }
  const trees = [{ title: "Ship engineers", ...layout(ship) }, { title: "Suit engineers", ...layout(suit) }];
  const path = (a: Node, b: Node) => {
    const x1 = a.x + W, y1 = a.y + H / 2 + 0.5, x2 = b.x, y2 = b.y + H / 2 + 0.5, mx = x1 + GX / 2 + 0.5;
    return `M${x1} ${y1}H${mx}V${y2}H${x2}`;
  };
  const count = (t: Eng[], s: string): number => t.reduce((c, e) => c + (e.state === s ? 1 : 0) + count(e.kids ?? [], s), 0);

  const hits = [
    { name: "Broo Tarquin", grades: [5, 5, 5], state: "unlocked", rank: 3, at: "Muang", ly: "174" },
    { name: "The Dweller", grades: [4, 3, 3], state: "unlocked", rank: 5, at: "Wyrd", ly: "34" },
    { name: "Mel Brandon", grades: [5, 5, 5], state: "known", rank: 0, at: "Luchtaine", ly: "22,014" },
  ];
  const kinds = ["Pulse", "Burst", "Beam"];
  /** "Pulse, burst and beam 5" when equal, else each: "Pulse 4 · burst 3 · beam 3". */
  const gradeLine = (g: number[]) => (g.every((x) => x === g[0]) ? `Grade ${g[0]} on all three` : kinds.map((k, i) => `${i ? k.toLowerCase() : k} ${g[i]}`).join(" · "));
</script>

<Frame top="Engineering" tabs={["Materials", "Engineers"]} tab="Engineers" skin="B" tone={5}>
  <div class="page">
    <div class="search">
      <span class="cell input" class:filled={!!query} aria-label="Engineers for a module">
        {#if query}<span>{query}</span><span class="clear"><Glyph name="clear" label="Clear" /></span>
        {:else}<span class="ph">Engineers for a module: laser, FSD, shields…</span>{/if}
      </span>
      {#if query}<span class="sub">Pulse, burst and beam lasers · 3 engineers</span>
      {:else}<span class="sub">{count(ship, "unlocked")} of 25 ship and {count(suit, "unlocked")} of 13 suit engineers unlocked · {count(ship, "known") + count(suit, "known")} to work on</span>{/if}
    </div>

    {#if !query}
      <div class="trees">
        {#each trees as t (t.title)}
          <section>
            <h2 class="section">{t.title}</h2>
            <div class="graph" style="width: {t.width}px; height: {t.height}px; --w: {W}px; --h: {H}px">
              <svg width={t.width} height={t.height} aria-hidden="true">
                {#each t.links as l, i (i)}<path d={path(l.from, l.to)} class:lit={l.to.state !== "hidden"} />{/each}
              </svg>
              {#each t.nodes as n (n.name)}
                <div class="node {n.state}" style="left: {n.x}px; top: {n.y}px">
                  <span class="nm">{n.name}</span>
                  {#if n.state === "unlocked"}
                    <span class="nsub">{#if n.rank}<Hex n={n.rank} size={11} label="Rank {n.rank}" />{/if}<span>{n.at}</span></span>
                  {:else if n.state === "hidden"}
                    <span class="nsub"><span>Not introduced</span></span>
                  {:else}
                    <span class="nsub"><span>{n.step}</span></span>
                  {/if}
                </div>
              {/each}
            </div>
          </section>
        {/each}
      </div>
    {:else}
      {@const best = hits[0]!}
      <div class="answer">
        <div class="best">
          <span class="lead">Nearest to reach grade 5</span>
          <div class="bline">
            <span class="bname">{best.name}</span>
            <span class="rank"><Hex n={best.rank} size={14} label="Rank {best.rank} of 5" /><span>Rank {best.rank}</span></span>
          </div>
          <div class="bline">
            <span class="grades"><Hex n={5} size={14} label="Grade 5" /><span>Grade 5 on pulse, burst and beam lasers</span></span>
          </div>
          <div class="bline">
            <span class="sys">{best.at}</span><span class="ibtn" aria-label="Copy {best.at}"><Glyph name="copy" /></span>
            <span class="dist"><span class="vl">{best.ly}</span><span class="unit">ly</span></span>
            <span class="grow"></span>
            <span class="primary">Set destination</span>
            <span class="ibtn" aria-label="Add {best.at} as a stop"><Glyph name="add-stop" /></span>
          </div>
        </div>

        <div class="list">
          <div class="row heads"><span>Reaches</span><span>Engineer · your rank</span><span>Where</span><span></span></div>
          {#each hits as h (h.name)}
            {@const top = Math.max(...h.grades)}
            <div class="row" class:far={h.state !== "unlocked"}>
              <div class="reach">
                <Hex n={top} size={13} label="Best grade {top}" />
                <span class="gl">{gradeLine(h.grades)}</span>
              </div>
              <div class="who">
                <span class="ename">{h.name}</span>
                {#if h.state === "unlocked"}<span class="rk"><Hex n={h.rank} size={10} label="Rank {h.rank}" /><span>Rank {h.rank}</span></span>
                {:else}<span class="rk known">Known · invite: Friendly, Colonia</span>{/if}
              </div>
              <div class="whr">
                <span class="l1"><span class="wsys">{h.at}</span><span class="ibtn" aria-label="Copy {h.at}"><Glyph name="copy" /></span></span>
                <span class="wly">{h.ly}<span class="unit">ly</span></span>
              </div>
              <span class="acts"><span class="ibtn" aria-label="Add {h.at} as a stop"><Glyph name="add-stop" /></span><span class="ibtn" aria-label="Set {h.at} as the destination"><Glyph name="destination" /></span></span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</Frame>

<style>
  .page { --o-lit: #ffa04a; --o: #ff7100; --o-mid: #d97a34; --g-hi: #e0e0e0; --g: #c4c4c4; --g-lo: #8c8c8c; --red-low: #c86455; color: var(--g); font-weight: 400; }
  .unit { color: var(--g-lo); }
  .search { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-6); margin-bottom: var(--sp-8); }
  .input { width: 440px; box-sizing: border-box; justify-content: space-between; color: var(--o); }
  .input.filled { outline: 1px solid var(--o); outline-offset: 0; }
  .clear { display: grid; color: var(--o); }
  .sub { font-size: var(--fs-s); color: var(--g-lo); }

  .trees { display: flex; flex-wrap: wrap; gap: var(--sp-8) var(--sp-12); align-items: flex-start; }
  .section { margin-bottom: var(--sp-4); color: var(--g); }
  .graph { position: relative; }
  svg { position: absolute; inset: 0; }
  path { fill: none; stroke: #2e2a27; stroke-width: 1; }
  path.lit { stroke: #5a5550; }
  /* Nodes on the window with a Seam outline: Hull is the frame's surface now. */
  .node { position: absolute; width: var(--w); height: var(--h); box-sizing: border-box; padding: 6px var(--sp-3) 0; outline: 1px solid var(--line); outline-offset: -1px; background: var(--bg); display: flex; flex-direction: column; gap: 3px; white-space: nowrap; }
  .nm { font-weight: 500; line-height: 1.2; overflow: hidden; text-overflow: ellipsis; }
  .nsub { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; font-size: var(--fs-s); color: var(--g-lo); }
  .nsub span { overflow: hidden; text-overflow: ellipsis; }
  .node.unlocked .nm { color: var(--g-hi); }
  .node.unlocked :global(.hexes) { color: var(--g-hi); }
  .node.known { outline-color: #5a3a20; }
  .node.known .nm { color: var(--o-mid); }
  .node.known .nsub { color: var(--g); }
  .node.barred .nm, .node.barred .nsub { color: var(--red-low); }
  .node.hidden .nm, .node.hidden .nsub { color: #6a6a6a; }
  .node.hidden { outline-color: #26221f; }

  .answer { width: max-content; max-width: 100%; }
  .best { display: flex; flex-direction: column; gap: var(--sp-2); padding: var(--sp-4) var(--sp-4) var(--sp-4) var(--sp-6); background: var(--surface-hi); margin-bottom: var(--sp-8); }
  .lead { font-size: var(--fs-s); color: var(--g-lo); }
  .bline { display: flex; align-items: center; gap: var(--sp-3); }
  .bname { font-size: var(--fs-xl); font-weight: 600; line-height: 1.2; color: #ffffff; margin-right: var(--sp-3); }
  .rank, .grades, .rk { display: inline-flex; align-items: center; gap: var(--sp-2); font-size: var(--fs-s); color: var(--g); }
  .rank :global(.hexes) { color: var(--g-hi); }
  .grades :global(.hexes) { color: var(--o); }
  .sys { color: var(--g-hi); }
  .bline .ibtn { margin-left: -8px; color: var(--g-lo); }
  .bline .primary + .ibtn { margin-left: 0; color: var(--o-mid); }
  .dist { margin-left: var(--sp-4); display: flex; align-items: baseline; }
  .dist .vl { color: var(--o-lit); }
  .grow { flex: 1; }

  .row { display: grid; grid-template-columns: 240px minmax(220px, 300px) minmax(160px, 220px) 64px; align-items: center; column-gap: var(--sp-6); height: 60px; padding: 0 var(--sp-3); white-space: nowrap; }
  .row.heads { height: 28px; border-bottom: 1px solid var(--line); margin-bottom: var(--sp-1); font-size: var(--fs-s); color: var(--g-lo); }
  .reach, .who, .whr { display: flex; flex-direction: column; justify-content: center; gap: 4px; min-width: 0; }
  .reach :global(.hexes) { color: var(--o); }
  .gl { font-size: var(--fs-s); color: var(--g); }
  .ename { color: var(--g-hi); font-weight: 500; }
  .rk :global(.hexes) { color: var(--g-hi); }
  .rk.known { color: var(--o-mid); }
  .whr { align-items: flex-start; }
  .l1 { display: flex; align-items: center; gap: 2px; }
  .wsys { color: var(--g); }
  .wly { font-size: var(--fs-s); color: var(--g); }
  .ibtn { color: var(--g-lo); }
  .acts { display: flex; justify-content: flex-end; gap: var(--sp-1); }
  .row.far .reach :global(.hexes) { color: var(--o-mid); }
  @container (max-width: 1000px) {
    .trees { flex-direction: column; }
  }
</style>
