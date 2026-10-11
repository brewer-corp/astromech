<script lang="ts">
  import Shell from "./Shell.svelte";
  import Header from "./Header.svelte";
  import Tabs from "./Tabs.svelte";
  import Stick from "./Stick.svelte";
  import Panel from "../design/Panel.svelte";
  import Button from "../design/Button.svelte";
  import Status from "../design/Status.svelte";
  import KeyCap from "../design/KeyCap.svelte";
  import { leftStick, rightStick } from "./data";
</script>

<Shell active="keybinds">
  <Header title="Keybinds" sub="Profile DualVKB · Ship preset · DualVKB.4.2.binds">
    {#snippet right()}
      <Status level="warn" text="2 warnings" />
      <Status level="error" text="1 problem" />
      <Button>Health check</Button>
      <Button kind="primary">Back up</Button>
    {/snippet}
  </Header>
  <Tabs tabs={["Controllers", "Ship", "Search", "Key tester", "Profiles"]} active="Controllers" />
  <div class="pair">
    {#each [leftStick, rightStick] as s, i (s.id)}
      <Panel title={s.name} tag="{s.id} · {s.hand}">
        {#snippet actions()}<Status level="ok" text="Connected" />{/snippet}
        <div class="wrap"><Stick callouts={s.callouts} hand={s.hand} active={i === 0 ? "Joy_10" : ""} /></div>
        <div class="axes">
          {#each (i === 0 ? [["Joy_XAxis", "Strafe lateral", 50], ["Joy_YAxis", "Strafe vertical", 62], ["Joy_RZAxis", "Throttle", 78]] : [["Joy_XAxis", "Roll", 47], ["Joy_YAxis", "Pitch", 55], ["Joy_RZAxis", "Yaw", 50]]) as [ax, act, v] (ax)}
            <div class="ax"><span class="an">{ax}</span><span class="aa">{act}</span><span class="bar"><span style="left:{v}%"></span></span></div>
          {/each}
        </div>
      </Panel>
    {/each}
  </div>
  <div class="tester am-chamfer">
    <span class="live am-caps"><span class="pulse"></span>Live</span>
    <span class="msg">Pressed <KeyCap kind="joy" label="Joy_10" /> on <b>VKB OT L</b></span>
    <span class="arrow">→</span>
    <span class="msg">Push-to-talk (astromech) · not bound in Elite</span>
    <span class="fill"></span>
    <span class="hint am-caps">Read-only · nothing is sent to the game</span>
  </div>
</Shell>

<style>
  .pair { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; flex: 1; min-height: 0; }
  .wrap { position: relative; margin-top: 10px; }
  .axes { display: grid; grid-template-columns: repeat(3, 1fr); gap: 14px; margin-top: 4px; padding-top: 12px; border-top: 1px solid rgb(255 122 0 / 0.15); }
  .ax { display: flex; flex-direction: column; gap: 3px; font: 500 var(--am-size-s) var(--am-font-ui); color: var(--am-text-strong); }
  .an { font: 400 var(--am-size-xs) var(--am-font-mono); color: var(--am-orange); }
  .aa { color: var(--am-text-strong); }
  .bar { position: relative; height: 6px; border: 1px solid var(--am-orange-line); background: var(--am-bg-2); }
  .bar span { position: absolute; top: -3px; width: 3px; height: 10px; background: var(--am-orange); box-shadow: var(--am-orange-glow); }
  .tester {
    display: flex; align-items: center; gap: 14px; padding: 10px 18px;
    border: 1px solid var(--am-blue); background: var(--am-blue-faint);
    font: 500 var(--am-size-m) var(--am-font-ui); color: var(--am-text-strong);
  }
  .live { display: flex; gap: 8px; align-items: center; color: var(--am-blue); font: 600 var(--am-size-xs) var(--am-font-ui); }
  .pulse { width: 8px; height: 8px; background: var(--am-blue); box-shadow: 0 0 10px var(--am-blue); }
  .arrow { color: var(--am-blue); }
  .fill { flex: 1; }
  .hint { font: 500 var(--am-size-xs) var(--am-font-ui); color: var(--am-muted); }
</style>
