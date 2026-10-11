<script lang="ts">
  import Frame from "./Frame.svelte";
  import About from "./About.svelte";
  import { OTA_LH, capture, type Rect } from "./sheet";
  const H = 612;
  const sel = "A1_A";
  const t = (rect: Rect) => ({ x: rect[0], y: H - rect[3], w: rect[2] - rect[0], h: rect[3] - rect[1] });
</script>

<Frame active="controller">
  <div class="cols">
    <section class="main">
      <div class="devices">
        <span class="on">Left · Gladiator EVO OT L · 231D3201</span>
        <span>Right · Gladiator EVO R · 231D0200</span>
      </div>
      <div class="src">
        <span>Sheet: VKB Gladiator SCG OTA (LH) v1.0 · from your copy</span>
        <span class="sp"></span>
        <span class="a">Capture buttons…</span><span class="a">Choose sheet…</span>
      </div>
      <svg viewBox="0 0 792 {H}" class="sheet" role="img" aria-label="VKB reference sheet with bindings">
        <rect width="792" height={H} fill="#f2f2f2" />
        <!-- Placeholder for VKB's artwork: the real PDF page renders here from the user's file. -->
        <rect x="268" y="118" width="262" height="420" fill="#e2e2e2" stroke="#b5b5b5" stroke-dasharray="6 4" />
        <text x="399" y="300" text-anchor="middle" class="ph">VKB SHEET RENDERS HERE</text>
        <text x="399" y="318" text-anchor="middle" class="ph2">Placeholder. The photo and layout are VKB's;</text>
        <text x="399" y="331" text-anchor="middle" class="ph2">astromech draws them from your downloaded PDF.</text>
        <text x="399" y="344" text-anchor="middle" class="ph2">Field positions below are VKB's form fields.</text>
        <text x="56" y="48" class="title">VKB Gladiator SCG OTA (LH)</text>
        {#each OTA_LH as s (s.title)}
          {@const first = s.rows[0]}
          {#if first}
            <rect x={s.x} y={H - s.top - 2} width={first.txt[2] - s.x} height="14" fill="#c8c8c8" />
            <text x={(s.x + first.txt[2]) / 2} y={H - s.top + 9} text-anchor="middle" class="hdr">{s.title}</text>
          {/if}
          {#each s.rows as row (row.id)}
            {@const n = t(row.num)}
            {@const x = t(row.txt)}
            {@const v = capture[row.id] ?? ["", ""]}
            {@const on = row.id === sel}
            <text x={s.x + 1} y={n.y + n.h - 4} class="sym">{row.sym === "Push" ? "●" : row.sym === "Up" ? "▲" : row.sym === "Down" ? "▼" : row.sym === "Left" ? "◀" : row.sym === "Right" ? "▶" : row.sym}</text>
            <rect x={n.x} y={n.y} width={n.w} height={n.h - 1} fill="#d6d6d6" />
            <rect x={x.x} y={x.y} width={x.w} height={x.h - 1} fill={on ? "#fa6e00" : "#e4e4e4"} />
            <text x={n.x + n.w / 2} y={n.y + n.h - 4.5} text-anchor="middle" class="num">{v[0]}</text>
            <text x={x.x + 5} y={x.y + x.h - 4.5} class="act" class:on>{v[1]}</text>
          {/each}
        {/each}
      </svg>
    </section>
    <About label="Selected control" title="A1 ministick push">
      <p>Button 12 on the left stick. Bound to <b>Night vision</b> (secondary slot).</p>
      <div class="k">! No keyboard key</div>
      <p>Voice can’t press Night vision. Give it a keyboard key in the keybinds list.</p>
      <span class="act">Show in keybinds</span>
      <div class="k">Numbering</div>
      <p>Button numbers come from your firmware profile, so astromech records them by asking you to press each control once. Captured 10 Oct, 23 of 41 controls.</p>
      <div class="k">Press any control</div>
      <p>to select it here.</p>
    </About>
  </div>
  {#snippet footer()}<span>Print</span><span class="note">Sheet © VKB, used from your own download; not included with astromech.</span>{/snippet}
</Frame>

<style>
  .cols { display: grid; grid-template-columns: 1fr 340px; gap: 28px; height: 100%; padding-top: 18px; }
  .main { display: flex; flex-direction: column; min-height: 0; }
  .devices { display: flex; gap: 32px; text-transform: uppercase; letter-spacing: var(--caps); color: var(--text-lo); }
  .devices .on { color: var(--accent); }
  .src { display: flex; gap: 24px; margin: 10px 0 12px; color: var(--text-lo); font-size: var(--fs-s); }
  .sp { flex: 1; }
  .a { color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; }
  .sheet { width: 100%; max-height: calc(100vh - 230px); display: block; }
  .ph { font: 600 15px var(--font); fill: #6a6a6a; letter-spacing: 0.08em; }
  .ph2 { font: 400 10px var(--font); fill: #7a7a7a; }
  .title { font: 600 26px var(--font); fill: #111; }
  .hdr { font: 600 10px var(--font); fill: #111; }
  .sym { font: 600 8px var(--font); fill: #333; }
  .num { font: 600 10px var(--font); fill: #000; }
  .act { font: 500 10px var(--font); fill: #1a1a1a; }
  .act.on { fill: #000; font-weight: 600; }
  .note { margin-left: auto; color: var(--text-lo); font-weight: 400; text-transform: none; letter-spacing: 0; }
</style>
