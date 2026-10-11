<script lang="ts">
  import Frame from "./Frame.svelte";
  import About from "./About.svelte";
  import { groups } from "./binds";
</script>

<Frame active="keybinds">
  <div class="cols">
    <section class="list">
      <div class="search"><span class="q">Search actions, or press any button…</span><span class="hint">Ctrl+F</span></div>
      <div class="row preset">
        <span class="lbl">Preset</span>
        <span class="dd">DualVKB <i>· ship, SRV, on foot, general</i><b>▼</b></span>
      </div>
      <div class="filters">
        <span class="on">Ship</span><span>SRV</span><span>On foot</span><span>General</span>
        <span class="sp"></span>
        <span class="on">All</span><span>Issues 3</span><span>Unbound</span>
      </div>
      <div class="head"><span>Action</span><span>Primary</span><span>Secondary</span><span></span></div>
      <div class="scroll">
        {#each groups as g (g.name)}
          <div class="row grp" class:open={g.open}>
            <span class="box">{g.open ? "−" : "+"}</span>
            <span class="gname">{g.name}</span>
            <span class="iss">{g.issues ? `${g.issues} issue${g.issues > 1 ? "s" : ""}` : ""}</span>
          </div>
          {#if g.open}
            {#each g.rows ?? [] as r (r.action)}
              <div class="row item" class:sel={r.sel}>
                <span class="name">{r.name}</span>
                <span class="cell" class:empty={!r.p}>{r.p ?? ""}</span>
                <span class="cell" class:empty={!r.s} class:bad={r.mark?.startsWith("Joy 33")}>{r.s ?? ""}</span>
                <span class="mark">{r.mark ? `! ${r.mark}` : ""}</span>
              </div>
            {/each}
          {/if}
        {/each}
      </div>
    </section>
    <About title="Night vision">
      <p>Toggles night vision in the ship, SRV and on foot.</p>
      <div class="k">Bound to</div>
      <p>Secondary: button 12 on the left stick (Gladiator EVO OT L), the A1 ministick push.</p>
      <div class="k">! No keyboard key</div>
      <p>Voice commands press keyboard keys, so “night vision on” can’t work until this action has a key in its primary slot.</p>
      <span class="act">Assign a key…</span>
      <span class="act">Show on my controller</span>
      <div class="k">Free keys nearby</div>
      <p>N, F7, Numpad 9</p>
    </About>
  </div>
  {#snippet footer()}
    <span>Undo</span><span>Save to a copy…</span>
    <span class="path">DualVKB.4.2.binds · last backup 21:40</span>
  {/snippet}
</Frame>

<style>
  .cols { display: grid; grid-template-columns: 1fr 380px; gap: 28px; height: 100%; padding-top: 18px; }
  .list { display: flex; flex-direction: column; min-height: 0; }
  .search { display: flex; justify-content: space-between; align-items: center; height: 40px; padding: 0 14px; background: var(--cell-opt); color: var(--text-lo); font-size: var(--fs-l); border-bottom: 2px solid var(--accent); }
  .hint { font-size: var(--fs-s); }
  .row { display: grid; grid-template-columns: 1fr 210px 210px 170px; gap: var(--gap); align-items: center; min-height: var(--row); }
  .preset { grid-template-columns: 1fr 424px 170px; margin-top: 14px; }
  .lbl, .name, .gname { text-transform: uppercase; letter-spacing: var(--caps); }
  .lbl { padding-left: 34px; }
  .dd { display: flex; align-items: center; height: var(--row); padding: 0 12px; background: var(--cell); color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); }
  .dd i { font-style: normal; color: var(--text-lo); text-transform: none; letter-spacing: 0; margin-left: 8px; flex: 1; }
  .dd b { font-size: 11px; }
  .filters { display: flex; gap: 22px; margin: 14px 0 6px; padding-left: 34px; color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .filters .on { color: var(--accent); }
  .sp { flex: 1; }
  .head { display: grid; grid-template-columns: 1fr 210px 210px 170px; gap: var(--gap); padding: 6px 0 6px 34px; color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .head span:nth-child(2), .head span:nth-child(3) { padding-left: 12px; }
  .scroll { overflow: hidden; }
  .grp { grid-template-columns: 22px 1fr 170px; gap: 12px; }
  .box { width: 18px; height: 18px; border: 1.5px solid var(--accent); color: var(--accent); display: grid; place-items: center; font-weight: 600; line-height: 1; }
  .grp.open .gname { color: var(--accent); }
  .iss { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .item { margin-bottom: var(--gap); }
  .item .name { padding-left: 34px; }
  .cell { height: var(--row); display: flex; align-items: center; padding: 0 12px; background: var(--cell); color: var(--text); font-family: var(--font); letter-spacing: 0.02em; }
  .cell.empty { background: var(--cell-empty); }
  .cell.bad { text-decoration: line-through; color: var(--text-lo); }
  .mark { color: var(--accent); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); padding-left: 10px; font-weight: 600; }
  .item.sel .name { color: var(--accent); }
  .item.sel .cell { outline: 1px solid var(--accent); outline-offset: -1px; }
  .path { margin-left: auto; color: var(--text-lo); font-weight: 400; text-transform: none; letter-spacing: 0; }
</style>
