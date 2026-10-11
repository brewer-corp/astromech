<script lang="ts">
  // CONCEPT ONLY (V1-DESIGN.md §3.4): Docking & landing as a ship part diagram. Our own generic
  // hull, line art. Part anchors are in the SVG's 1000 x 520 space; labels are spread per side.
  import Frame from "./Frame.svelte";

  type Part = { name: string; action: string; a: [number, number]; p: string; s?: string; side: "l" | "r"; missing?: boolean };
  const parts: Part[] = [
    { name: "Cockpit · left panel", action: "FocusLeftPanel", a: [318, 196], p: "L · JOY 6", s: "1", side: "l" },
    { name: "Ship lights", action: "ShipSpotLightToggle", a: [150, 262], p: "Insert", side: "l" },
    { name: "Night vision", action: "NightVisionToggle", a: [300, 214], p: "", side: "l", missing: true },
    { name: "Landing gear", action: "LandingGearToggle", a: [352, 352], p: "L · JOY 3", s: "L", side: "l" },
    { name: "Throttle · all stop", action: "SetSpeedZero", a: [470, 266], p: "L · JOY 18", s: "X", side: "l" },
    { name: "Top thrusters · down", action: "DownThrustButton", a: [560, 168], p: "R · POV1 DOWN", s: "F", side: "r" },
    { name: "Belly thrusters · up", action: "UpThrustButton", a: [560, 334], p: "R · POV1 UP", s: "R", side: "r" },
    { name: "Side thrusters · left / right", action: "LeftThrustButton / RightThrustButton", a: [690, 262], p: "R · POV1 ◀ ▶", s: "Q / E", side: "r" },
    { name: "Main thrusters · forward / back", action: "ForwardThrustButton / BackwardThrustButton", a: [890, 252], p: "", side: "r", missing: true },
    { name: "Rear landing gear", action: "LandingGearToggle", a: [742, 362], p: "L · JOY 3", s: "L", side: "r" },
  ];
  const W = 1000, ROW = 52, BOXW = 250, Z = 1.35;
  const T = (x: number, y: number): [number, number] => [500 + (x - 500) * Z, 265 + (y - 265) * Z];
  for (const p of parts) p.a = T(p.a[0], p.a[1]);
  function spread(side: "l" | "r") {
    const ps = parts.filter((p) => p.side === side).sort((x, y) => x.a[1] - y.a[1]);
    let y = 90;
    return ps.map((p) => { y = Math.max(y, p.a[1] - ROW / 2); const out = { p, y }; y += ROW + 8; return out; });
  }
  const labels = [...spread("l"), ...spread("r")];
  const hull = "M120,262 L250,214 L290,190 L360,180 L560,166 L840,186 L892,220 L892,290 L840,318 L560,338 L330,334 L200,300 Z";
</script>

<Frame active="activities">
  <div class="acts">
    {#each ["Flight basics", "Supercruise & jumps", "Exploration", "Trading", "Mining", "Combat", "Docking & landing", "SRV", "On foot"] as a (a)}<span class:on={a === "Docking & landing"}>{a}</span>{/each}
    <span class="sp"></span><span class="view">View: <span>Sticks</span> · <span class="on">Ship</span></span>
  </div>
  <div class="concept">CONCEPT SKETCH · after v1 · example bindings</div>
  <svg viewBox="-400 70 {W + 800} 400" class="diag">
    <g transform="translate({500 - 500 * Z},{265 - 265 * Z}) scale({Z})">
    <path d={hull} class="hull" />
    <path d="M290,190 L330,214 L250,214" class="detail" />
    <path d="M360,180 L380,250 L840,250 M560,166 L560,338 M892,236 L910,236 M892,274 L910,274" class="detail" />
    <path d="M352,334 L352,352 M330,352 L374,352 M742,326 L742,362 M720,362 L764,362" class="detail" />
    </g>
    {#each labels as { p, y } (p.name)}
      {@const bx = p.side === "l" ? -390 : W + 140}
      {@const ex = p.side === "l" ? bx + BOXW : bx}
      {@const ey = y + ROW / 2}
      <polyline points="{p.a[0]},{p.a[1]} {p.side === 'l' ? ex + 24 : ex - 24},{ey} {ex},{ey}" class="lead" class:miss={p.missing} />
      <circle cx={p.a[0]} cy={p.a[1]} r="4" class="dot" class:miss={p.missing} />
      <g transform="translate({bx},{y})">
        <text x="0" y="14" class="pn">{p.name.toUpperCase()}</text>
        <rect x="0" y="20" width={BOXW / 2 - 1} height="28" class={p.missing ? "c miss" : p.p ? "c" : "c e"} />
        <rect x={BOXW / 2 + 1} y="20" width={BOXW / 2 - 1} height="28" class={p.s ? "c" : "c e"} />
        <text x="8" y="39" class="v">{p.missing ? "NOT BOUND · BIND THIS" : p.p}</text>
        <text x={BOXW / 2 + 9} y="39" class="v">{p.s ?? ""}</text>
      </g>
    {/each}
  </svg>
  {#snippet footer()}<span>Show all activities</span><span class="note">Concept for after v1 · our own generic hull, not a Frontier asset</span>{/snippet}
</Frame>

<style>
  .acts { display: flex; gap: 26px; padding: 14px 0 10px; text-transform: uppercase; letter-spacing: var(--caps); color: var(--text-lo); }
  .acts .on, .view .on { color: var(--accent); }
  .sp { flex: 1; }
  .view { text-transform: none; letter-spacing: 0; }
  .concept { color: var(--text-lo); font-size: var(--fs-s); letter-spacing: var(--caps); margin: 4px 0 6px; }
  .diag { width: 100%; height: 680px; display: block; }
  .hull { fill: rgba(250, 110, 0, 0.06); stroke: var(--accent); stroke-width: 1.5; }
  .detail { fill: none; stroke: var(--accent-dim); stroke-width: 1.2; }
  .lead { fill: none; stroke: var(--text-lo); stroke-width: 1; }
  .dot { fill: var(--accent); }
  .lead.miss { stroke: var(--accent); stroke-dasharray: 4 3; }
  .dot.miss { fill: none; stroke: var(--accent); stroke-width: 1.5; }
  .pn { fill: var(--text-hi); font: 500 13px var(--font); letter-spacing: 0.06em; }
  .c { fill: var(--cell); }
  .c.e { fill: var(--cell-empty); }
  .c.miss { fill: var(--cell-empty); stroke: var(--accent); stroke-width: 1; }
  .v { fill: var(--text); font: 400 14px var(--font); }
  .note { margin-left: auto; color: var(--text-lo); font-weight: 400; text-transform: none; letter-spacing: 0; }
</style>
