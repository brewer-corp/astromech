<script lang="ts">
  // A native Windows tray context menu (rendered by the core via the OS, not a webview).
  // Drawn here only to show contents and order; Windows 11 dark menu metrics.
  const items: ([string, string?] | null)[] = [
    ["Elite running · voice ready", "info"],
    null,
    ["Open astromech", "bold"],
    ["Keybinds"],
    ["Settings"],
    null,
    ["Pause voice"],
    ["Kill switch (stop all actions)"],
    null,
    ["Check for updates…"],
    ["Save diagnostics…"],
    null,
    ["Quit astromech"],
  ];
</script>

<div class="desk">
  <div class="menu">
    {#each items as it, i (i)}
      {#if it === null}<div class="sep"></div>
      {:else}<div class="it {it[1] ?? ''}"><span class="ck">{it[1] === "check" ? "" : ""}</span>{it[0]}</div>{/if}
    {/each}
  </div>
  <div class="bar"><span class="ic"></span><span class="clock">21:42<br />10/10/2026</span></div>
</div>

<style>
  .desk { width: 100vw; height: 100vh; background: #1f2a36; position: relative; font-family: "Segoe UI", system-ui, sans-serif; }
  .menu { position: absolute; right: 70px; bottom: 56px; width: 268px; padding: 4px; background: #2c2c2c; border: 1px solid #454545; border-radius: 8px; box-shadow: 0 8px 24px rgb(0 0 0 / 0.5); }
  .it { display: flex; align-items: center; height: 30px; padding: 0 10px; color: #f3f3f3; font-size: 14px; border-radius: 4px; }
  .it:nth-child(3) { background: #3a3a3a; }
  .it.info { color: #9d9d9d; }
  .it.bold { font-weight: 600; }
  .ck { width: 26px; color: #f3f3f3; }
  .check .ck::before { content: "✓"; }
  .sep { height: 1px; background: #454545; margin: 4px 0; }
  .bar { position: absolute; left: 0; right: 0; bottom: 0; height: 48px; background: #202020; display: flex; justify-content: flex-end; align-items: center; gap: 16px; padding: 0 14px; }
  .ic { width: 16px; height: 16px; background: #fa6e00; clip-path: polygon(50% 0, 100% 100%, 0 100%); outline: 1px solid #555; outline-offset: 6px; border-radius: 2px; margin-right: 6px; }
  .clock { color: #f3f3f3; font-size: 12px; text-align: right; line-height: 1.35; }
</style>
