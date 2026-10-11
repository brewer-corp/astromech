<script lang="ts">
  import Shell from "./Shell.svelte";
  import Header from "./Header.svelte";
  import Tabs from "./Tabs.svelte";
  import Panel from "../design/Panel.svelte";
  import Button from "../design/Button.svelte";
  import Status from "../design/Status.svelte";
  import KeyCap from "../design/KeyCap.svelte";
  import { rows } from "./data";
  const filters = [["Ship", true], ["SRV", false], ["On foot", false], ["Conflicts", false], ["No keyboard key", true], ["Unbound", false]] as const;
  const cap = (s: string) => (s === "—" ? "missing" : s.startsWith("Joy") ? "joy" : "key");
  const sel = rows[2];
</script>

<Shell active="keybinds">
  <Header title="Keybinds" sub="Profile DualVKB · 412 actions · 186 bound">
    {#snippet right()}<Button>Export</Button><Button kind="primary">Back up</Button>{/snippet}
  </Header>
  <Tabs tabs={["Controllers", "Ship", "Search", "Key tester", "Profiles"]} active="Search" />
  <div class="search am-chamfer">
    <span class="mag">⌕</span>
    <span class="q">night<span class="caret"></span></span>
    <span class="or am-caps">or press a button to find it</span>
  </div>
  <div class="chips">
    {#each filters as [f, on] (f)}<span class="chip am-caps" class:on>{on ? "✓ " : ""}{f}</span>{/each}
    <span class="count am-caps">10 of 412</span>
  </div>
  <div class="split">
    <div class="table am-chamfer">
      <div class="tr th am-caps"><span>Action</span><span>Category</span><span>Primary</span><span>Secondary</span><span>Status</span></div>
      {#each rows as r (r.action)}
        <div class="tr" class:sel={r === sel}>
          <span class="nm">{r.name}<small>{r.action}</small></span>
          <span class="cat">{r.cat}</span>
          <span><KeyCap kind={cap(r.primary)} label={r.primary} /></span>
          <span><KeyCap kind={cap(r.secondary)} label={r.secondary} /></span>
          <span><Status level={r.status} text={r.status === "ok" ? "OK" : r.status === "warn" ? "Check" : r.status === "error" ? "Problem" : "Note"} /></span>
        </div>
      {/each}
    </div>
    <Panel title="Night Vision" tag="NightVisionToggle">
      <div class="detail">
        <div class="kv"><span class="am-caps">Primary</span><KeyCap kind="joy" label="Joy_12 · VKB OT L · A1" /></div>
        <div class="kv"><span class="am-caps">Secondary</span><KeyCap kind="missing" label="not set" /></div>
        <div class="kv"><span class="am-caps">Contexts</span><span>Ship, SRV, On foot</span></div>
        <div class="callout warn">
          <b class="am-caps">! Voice can't press this</b>
          <p>astromech sends keyboard keys only. Give Night Vision a keyboard key as its secondary binding.</p>
          <div class="sugg">Free keys nearby: <KeyCap label="Key_N" /> <KeyCap label="Key_F7" /> <KeyCap label="Key_Numpad_9" /></div>
        </div>
        <div class="btns">
          <Button kind="primary">Bind Key_N in a copy</Button>
          <Button>Press to bind…</Button>
        </div>
        <p class="fine">Edits go to a copy of your .binds file. The original is backed up first and Elite must be restarted, or the preset re-selected, to load it.</p>
      </div>
    </Panel>
  </div>
</Shell>

<style>
  .search {
    display: flex; align-items: center; gap: 14px; padding: 12px 18px;
    border: 1px solid var(--am-orange); background: var(--am-bg-2);
    font: 500 var(--am-size-l) var(--am-font-ui); color: var(--am-text-strong);
  }
  .mag { color: var(--am-orange); font-size: 22px; }
  .q { flex: 1; }
  .caret { display: inline-block; width: 9px; height: 20px; margin-left: 2px; vertical-align: -3px; background: var(--am-orange); }
  .or { font: 500 var(--am-size-xs) var(--am-font-ui); color: var(--am-blue); }
  .chips { display: flex; gap: 8px; align-items: center; }
  .chip { font: 600 var(--am-size-xs) var(--am-font-ui); padding: 4px 10px; border: 1px solid var(--am-orange-line); color: var(--am-muted); }
  .chip.on { color: var(--am-orange); border-color: var(--am-orange); background: var(--am-orange-faint); }
  .count { margin-left: auto; font: 500 var(--am-size-xs) var(--am-font-ui); color: var(--am-muted); }
  .split { display: grid; grid-template-columns: 1fr 380px; gap: 16px; min-height: 0; flex: 1; }
  .table { border: var(--am-border); background: var(--am-panel); overflow: hidden; }
  .tr { display: grid; grid-template-columns: 1.4fr 1.2fr 1fr 1fr 0.7fr; align-items: center; gap: 10px; padding: 8px 16px; border-bottom: 1px solid rgb(255 122 0 / 0.12); font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-text); }
  .th { color: var(--am-orange); font-size: var(--am-size-xs); background: var(--am-orange-faint); }
  .tr.sel { background: var(--am-orange-faint); box-shadow: inset 3px 0 0 var(--am-orange); }
  .nm { display: flex; flex-direction: column; color: var(--am-text-strong); font-size: var(--am-size-m); }
  small { font: 400 var(--am-size-xs) var(--am-font-mono); color: var(--am-muted); }
  .cat { color: var(--am-muted); }
  .detail { display: flex; flex-direction: column; gap: 12px; font: 500 var(--am-size-m) var(--am-font-ui); color: var(--am-text); }
  .kv { display: flex; justify-content: space-between; align-items: center; }
  .kv > span:first-child { font-size: var(--am-size-xs); color: var(--am-muted); }
  .callout { border: 1px solid var(--am-amber); background: rgb(255 210 63 / 0.08); padding: 10px 12px; }
  .callout b { color: var(--am-amber); font-size: var(--am-size-xs); }
  .callout p { margin: 6px 0; font-size: var(--am-size-s); }
  .sugg { font-size: var(--am-size-s); color: var(--am-muted); display: flex; gap: 6px; flex-wrap: wrap; align-items: center; }
  .btns { display: flex; gap: 8px; flex-wrap: wrap; }
  .fine { margin: 0; font-size: var(--am-size-xs); color: var(--am-muted); }
</style>
