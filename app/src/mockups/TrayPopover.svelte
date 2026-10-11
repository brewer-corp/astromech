<script lang="ts">
  import Status from "../design/Status.svelte";
  import KeyCap from "../design/KeyCap.svelte";
  import Button from "../design/Button.svelte";
  const log = [
    ["21:42:07", "“Gear down”", "confirmed 9 ms", "ok"],
    ["21:41:30", "“You're down.”", "asked: gear down?", "warn"],
    ["21:39:12", "Low fuel callout", "spoken", "info"],
    ["21:38:55", "“Night vision on”", "no keyboard key", "error"],
  ] as const;
</script>

<div class="desk">
  <div class="pop am-chamfer">
    <header>
      <span class="logo">◢◣</span>
      <div class="t"><div class="name">ASTROMECH</div><div class="sub am-caps">Active · Elite running</div></div>
      <Status level="ok" text="Armed" />
    </header>
    <div class="ship">
      <div class="am-caps lbl">Krait Mk II · Dusk Wren</div>
      <div class="sys">Shinrarta Dezhra · Jameson Memorial</div>
      <div class="flags">
        <span class="f on am-caps">Gear ↓</span><span class="f am-caps">Lights</span><span class="f am-caps">Hardpoints</span><span class="f am-caps">Scoop</span><span class="f warn am-caps">Fuel 24 %</span>
      </div>
    </div>
    <div class="rows">
      <div class="row"><span class="am-caps k">Voice</span><span>Hold <KeyCap kind="joy" label="Joy_10" /> OT L</span><Status level="ok" text="Ready" /></div>
      <div class="row"><span class="am-caps k">Mic</span><span class="mini"><span class="lv" style="width:62%"></span></span><span class="v">Index headset</span></div>
      <div class="row"><span class="am-caps k">Binds</span><span>DualVKB · 2 warnings</span><Status level="warn" text="Check" /></div>
      <div class="row"><span class="am-caps k">Update</span><span>1.0.3 ready · installs after Elite exits</span><Status level="info" text="Queued" /></div>
    </div>
    <div class="log">
      {#each log as [t, what, res, lv] (t)}
        <div class="l {lv}"><span class="tm">{t}</span><span class="w">{what}</span><span class="r">{res}</span></div>
      {/each}
    </div>
    <div class="btns">
      <Button kind="primary">Open</Button>
      <Button>Keybinds</Button>
      <Button kind="danger">Kill switch</Button>
    </div>
    <footer class="am-caps"><span>Pause voice</span><span>Settings</span><span>Quit</span></footer>
  </div>
  <div class="tray"><span class="icon">◢◣</span><span class="clock">21:42</span></div>
</div>

<style>
  .desk { width: 100vw; height: 100vh; background: radial-gradient(ellipse at 70% 40%, #1b2735 0%, #0b1017 70%); position: relative; }
  .pop {
    position: absolute; right: 18px; bottom: 58px; width: 392px;
    background: linear-gradient(180deg, var(--am-panel-2), var(--am-bg)); border: 1px solid var(--am-orange);
    box-shadow: 0 0 40px rgb(255 122 0 / 0.18); display: flex; flex-direction: column;
  }
  header { display: flex; align-items: center; gap: 12px; padding: 14px 16px; border-bottom: 1px solid var(--am-orange-line); background: var(--am-orange-faint); }
  .logo { color: var(--am-orange); font-size: 20px; letter-spacing: -4px; text-shadow: var(--am-orange-glow); }
  .t { flex: 1; }
  .name { font: 400 14px var(--am-font-display); color: var(--am-orange); letter-spacing: 0.18em; }
  .sub { font: 600 var(--am-size-xs) var(--am-font-ui); color: var(--am-green); }
  .ship { padding: 12px 16px; border-bottom: 1px solid rgb(255 122 0 / 0.15); }
  .lbl { font: 600 var(--am-size-s) var(--am-font-ui); color: var(--am-text-strong); }
  .sys { font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-muted); margin: 2px 0 8px; }
  .flags { display: flex; gap: 5px; flex-wrap: wrap; }
  .f { font: 600 10px var(--am-font-ui); padding: 2px 7px; border: 1px solid var(--am-orange-line); color: var(--am-disabled); }
  .f.on { color: var(--am-bg); background: var(--am-orange); border-color: var(--am-orange); }
  .f.warn { color: var(--am-amber); border-color: var(--am-amber); }
  .rows { padding: 8px 16px; display: flex; flex-direction: column; gap: 8px; border-bottom: 1px solid rgb(255 122 0 / 0.15); }
  .row { display: grid; grid-template-columns: 58px 1fr auto; align-items: center; gap: 10px; font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-text); }
  .k { font-size: 10px; color: var(--am-muted); font-weight: 600; }
  .mini { height: 8px; border: 1px solid var(--am-orange-line); position: relative; }
  .lv { position: absolute; inset: 1px auto 1px 1px; background: repeating-linear-gradient(90deg, var(--am-orange) 0 4px, transparent 4px 6px); }
  .v { color: var(--am-muted); font-size: var(--am-size-xs); }
  .log { padding: 8px 16px; display: flex; flex-direction: column; gap: 5px; }
  .l { display: grid; grid-template-columns: 58px 1fr auto; gap: 10px; font: 400 var(--am-size-s) var(--am-font-ui); color: var(--am-text); }
  .tm { font-family: var(--am-font-mono); color: var(--am-muted); font-size: var(--am-size-xs); }
  .r { font-size: var(--am-size-xs); }
  .ok .r { color: var(--am-green); } .warn .r { color: var(--am-amber); } .error .r { color: var(--am-red); } .info .r { color: var(--am-blue); }
  .btns { display: flex; gap: 8px; padding: 10px 16px; }
  footer { display: flex; justify-content: space-between; padding: 9px 16px; border-top: 1px solid var(--am-orange-line); font: 600 10px var(--am-font-ui); color: var(--am-muted); }
  .tray { position: absolute; left: 0; right: 0; bottom: 0; height: 44px; background: rgb(20 24 30 / 0.95); border-top: 1px solid #2a313b; display: flex; justify-content: flex-end; align-items: center; gap: 18px; padding: 0 18px; }
  .icon { color: var(--am-orange); letter-spacing: -4px; font-size: 16px; outline: 1px solid rgb(255 122 0 / 0.4); padding: 4px 8px 4px 4px; background: rgb(255 122 0 / 0.12); }
  .clock { color: #d6dbe1; font: 400 13px "Segoe UI", sans-serif; }
</style>
