<script lang="ts">
  import Shell from "./Shell.svelte";
  import Header from "./Header.svelte";
  import Panel from "../design/Panel.svelte";
  import Button from "../design/Button.svelte";
  import Status from "../design/Status.svelte";
  import KeyCap from "../design/KeyCap.svelte";
  import Toggle from "../design/Toggle.svelte";
  import Meter from "./Meter.svelte";
</script>

<Shell active="voice">
  <Header title="Voice & controls" sub="Push-to-talk · kill switch · microphone · recognition">
    {#snippet right()}<Status level="ok" text="Voice ready" /><Status level="info" text="VR mode" />{/snippet}
  </Header>
  <div class="grid">
    <Panel title="Buttons" tag="read with Raw Input · never sent to Elite">
      <div class="bind listening">
        <div class="what"><b class="am-caps">Push-to-talk</b><span>Hold while speaking</span></div>
        <div class="val"><span class="wait am-caps"><span class="pulse"></span>Press a button on any device…</span></div>
        <Button small kind="ghost">Cancel</Button>
      </div>
      <div class="bind">
        <div class="what"><b class="am-caps">Kill switch</b><span>Press: stop all actions · hold 1 s: re-arm</span></div>
        <div class="val"><KeyCap kind="joy" label="Joy_20" /> <span class="dev">VKB OT L · 231D3201</span></div>
        <Button small>Change</Button>
      </div>
      <div class="note"><Status level="ok" text="No conflicts" /> Neither button is bound in your Elite preset.</div>
      <div class="sub am-caps">Recent presses on VKB OT L</div>
      <div class="presses">
        <span><KeyCap kind="joy" label="Joy_10" /> 812 ms</span>
        <span><KeyCap kind="joy" label="Joy_10" /> 1 040 ms</span>
        <span class="bounce">bounce 37 ms · ignored</span>
      </div>
    </Panel>

    <Panel title="Microphone" tag="WASAPI shared · 48 kHz">
      <div class="dev-row">
        <div class="select am-chamfer"><span>Headset Microphone (Valve Index)</span><span class="chev">▾</span></div>
        <Button small>Test</Button>
      </div>
      <div class="callout warn">
        <b class="am-caps">! Default mic changed by SteamVR</b>
        <p>Desk mic: BlackShark V3. In VR, Windows switched to the headset mic. astromech follows the device you pick here, not the Windows default.</p>
      </div>
      <div class="sub am-caps">Level · say "gear down"</div>
      <Meter level={-22} peak={-14} before={-39} />
      <div class="stats">
        <div><span class="am-caps">Raw loudest frame</span><b>−39 dBFS</b></div>
        <div><span class="am-caps">After gain</span><b>−14 dBFS</b></div>
        <div><span class="am-caps">Gain</span><b>+25 dB</b></div>
        <div><span class="am-caps">Noise floor</span><b>−63 dBFS</b></div>
      </div>
      <Toggle on label="Normalise level before recognition" />
      <Toggle on label="Ignore clips shorter than 300 ms" />
    </Panel>

    <Panel title="Calibration" tag="3 phrases · 20 s">
      <ol class="cal">
        <li class="done"><span class="n">1</span><span class="ph">"Gear down"</span><span class="heard">heard "gear down" · 0.98</span><Status level="ok" text="OK" /></li>
        <li class="done"><span class="n">2</span><span class="ph">"Deploy hardpoints"</span><span class="heard">heard "deploy hard points" · 0.94</span><Status level="ok" text="OK" /></li>
        <li class="cur"><span class="n">3</span><span class="ph">"Request docking"</span><span class="heard">hold PTT and say it…</span><Status level="idle" text="Waiting" /></li>
      </ol>
      <p class="fine">Sets your gain and silence threshold for this mic. Recordings stay on this PC and are deleted afterwards.</p>
    </Panel>

    <Panel title="Recognition">
      <div class="rows">
        <div class="kv"><span class="am-caps">Model</span><span>Parakeet TDT 0.6B · int8 · local</span></div>
        <div class="kv"><span class="am-caps">Execute at</span><span>≥ 0.85 · ask "Did you mean…?" from 0.60</span></div>
        <div class="kv"><span class="am-caps">Last</span><span>"You're down." → <i>Did you mean gear down?</i> 0.80</span></div>
      </div>
      <Toggle on label="Spoken confirmations (Kokoro)" />
      <Toggle label="Second pass with the larger model when unsure" />
      <Toggle on label="Game callouts (low fuel, interdiction…)" />
    </Panel>
  </div>
</Shell>

<style>
  .grid { display: grid; grid-template-columns: 1fr 1fr; grid-auto-rows: min-content; gap: 16px; overflow: hidden; }
  .bind { display: grid; grid-template-columns: 210px 1fr auto; align-items: center; gap: 12px; padding: 10px 12px; border: 1px solid var(--am-orange-line); margin-bottom: 8px; background: var(--am-bg-2); }
  .bind.listening { border-color: var(--am-blue); background: var(--am-blue-faint); }
  .what { display: flex; flex-direction: column; font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-muted); }
  .what b { color: var(--am-text-strong); font-size: var(--am-size-s); }
  .val { display: flex; align-items: center; gap: 8px; font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-muted); }
  .wait { display: flex; align-items: center; gap: 8px; color: var(--am-blue); font-size: var(--am-size-xs); font-weight: 600; }
  .pulse { width: 8px; height: 8px; background: var(--am-blue); box-shadow: 0 0 10px var(--am-blue); }
  .note { display: flex; gap: 10px; align-items: center; font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-text); margin: 6px 0 12px; }
  .sub { font: 600 var(--am-size-xs) var(--am-font-ui); color: var(--am-orange); margin: 12px 0 18px; }
  .presses { display: flex; gap: 16px; font: 400 var(--am-size-s) var(--am-font-mono); color: var(--am-muted); }
  .bounce { color: var(--am-amber); }
  .dev-row { display: flex; gap: 10px; align-items: center; }
  .select { flex: 1; display: flex; justify-content: space-between; padding: 8px 12px; border: 1px solid var(--am-orange); font: 500 var(--am-size-m) var(--am-font-ui); color: var(--am-text-strong); background: var(--am-bg-2); }
  .chev { color: var(--am-orange); }
  .callout { border: 1px solid var(--am-amber); background: rgb(255 210 63 / 0.08); padding: 8px 12px; margin: 12px 0 0; font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-text); }
  .callout b { color: var(--am-amber); font-size: var(--am-size-xs); }
  .callout p { margin: 4px 0 0; }
  .stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: 8px; margin: 10px 0 14px; }
  .stats div { display: flex; flex-direction: column; border-left: 2px solid var(--am-orange-line); padding-left: 8px; }
  .stats span { font: 500 10px var(--am-font-ui); color: var(--am-muted); }
  .stats b { font: 400 var(--am-size-m) var(--am-font-mono); color: var(--am-text-strong); }
  :global(.grid .toggle) { margin-top: 8px; }
  .cal { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 8px; }
  .cal li { display: grid; grid-template-columns: 28px 180px 1fr auto; align-items: center; gap: 10px; padding: 8px 10px; border: 1px solid var(--am-orange-line); font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-text); }
  .cal li.cur { border-color: var(--am-orange); background: var(--am-orange-faint); }
  .n { font: 400 var(--am-size-m) var(--am-font-display); color: var(--am-orange); }
  .ph { color: var(--am-text-strong); font-size: var(--am-size-m); }
  .heard { color: var(--am-muted); font-family: var(--am-font-mono); }
  .fine { margin: 10px 0 0; font: 500 var(--am-size-xs) var(--am-font-ui); color: var(--am-muted); }
  .rows { display: flex; flex-direction: column; gap: 8px; margin-bottom: 6px; }
  .kv { display: flex; justify-content: space-between; font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-text); }
  .kv > span:first-child { color: var(--am-muted); font-size: var(--am-size-xs); }
  i { color: var(--am-orange); font-style: normal; }
</style>
