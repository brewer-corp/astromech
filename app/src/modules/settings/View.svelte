<script lang="ts">
  // Settings (docs/V1-DESIGN.md §2.3): one list, label left, value right, actions on the row.
  // Everything the tray used to carry lives here now. Push-to-talk is still set in the config
  // file until the press-to-bind flow lands.
  import { coreCall, coreStatus, openUrl, type CoreStatus, type CoreCmd } from "../../lib/core";

  let status = $state<CoreStatus | null>(null);
  let note = $state("");

  async function refresh() {
    status = await coreStatus();
  }
  $effect(() => {
    void refresh();
    const t = setInterval(() => void refresh(), 1000);
    return () => clearInterval(t);
  });

  async function run(cmd: CoreCmd, doing = "") {
    note = doing;
    const r = await coreCall(cmd);
    note = r.ok ? "" : String(r.error);
    await refresh();
  }

  const u = $derived(status?.update);
  const busy = $derived(u?.status === "checking" || u?.status === "downloading" || u?.status === "applying");

  function updateLine(): string {
    if (!u) return "—";
    if (u.status === "checking") return "Checking…";
    if (u.status === "downloading") return `Downloading ${u.latest ?? "update"}…`;
    if (u.status === "applying") return `Installing ${u.latest ?? "update"}; astromech will reopen`;
    if (u.status === "error") return u.error ?? "Check failed";
    if (u.ready && u.latest) return `${u.latest} is ready to install`;
    if (u.status === "available" && u.latest) return `${u.latest} is available`;
    return u.last_check ? "Up to date" : "Not checked yet";
  }
</script>

<section>
  {#if !status}
    <p class="lo">astromech's background process isn't running, so settings can't be shown. It starts with this window; if this stays, see Diagnostics.</p>
  {:else}
    <h3>Startup</h3>
    <dl>
      <dt>Start with Windows</dt>
      <dd>
        <span>{status.autostart ? "On" : "Off"}</span>
        <button onclick={() => run(status?.autostart ? "autostart_off" : "autostart_on")}>{status.autostart ? "Turn off" : "Turn on"}</button>
      </dd>
    </dl>
    <p class="hint">Starts quietly in the tray and waits for Elite, so voice is ready when you launch the game.</p>

    <h3>Voice</h3>
    <dl>
      <dt>Push-to-talk</dt>
      <dd class:warn={!status.ptt}>
        <span class:mono={!!status.ptt}>{status.ptt ?? "Not set"}</span>
        <button onclick={() => run("open_config")}>Edit config file</button>
      </dd>
      <dt>Speech models</dt>
      <dd class:warn={status.models_missing && !status.models_progress}>
        <span>{status.models_progress ?? (status.models_missing ? "Not downloaded (about 800 MB)" : "Installed")}</span>
        {#if status.models_missing && !status.models_progress}
          <button onclick={() => run("download_models")}>Download</button>
        {/if}
      </dd>
      <dt>Voice</dt>
      <dd>
        <span>{status.voice_paused ? "Paused" : "Listening while Elite runs"}</span>
        <button onclick={() => run(status?.voice_paused ? "resume_voice" : "pause_voice")}>{status.voice_paused ? "Resume" : "Pause"}</button>
      </dd>
    </dl>
    {#if !status.ptt}
      <p class="hint">
        Find your button's name in Controls → Key tester, then set <span class="mono">ptt = &#123; device = "…", key = "…" &#125;</span>
        under <span class="mono">[controls]</span> and restart astromech (tray → Quit, then open it again).
      </p>
    {/if}

    <h3>Updates</h3>
    <dl>
      <dt>Version</dt>
      <dd>
        <span>{status.version}</span>
        <button onclick={() => openUrl("https://github.com/spadaval/astromech/releases")}>Release notes</button>
      </dd>
      <dt>Updates</dt>
      <dd class:warn={u?.status === "error"}>
        <span>{updateLine()}</span>
        {#if u?.ready && u.status !== "applying"}
          <button onclick={() => run("install_update", "Installing… astromech will close and reopen.")}>Install and restart</button>
        {:else}
          <button disabled={busy} onclick={() => run("check_update")}>Check now</button>
        {/if}
      </dd>
    </dl>
    <p class="hint">Installing closes astromech for a few seconds and reopens it. Elite can keep running.</p>

    <h3>Files</h3>
    <dl>
      <dt>Config file</dt>
      <dd><span class="mono path">{status.config}</span><button onclick={() => run("open_config")}>Open</button></dd>
      <dt>Logs</dt>
      <dd><span class="mono path">{status.log_dir}</span><button onclick={() => status && openUrl(status.log_dir)}>Open folder</button></dd>
    </dl>
    {#if status.config_note}<p class="hint warn-text">{status.config_note}</p>{/if}
  {/if}
  {#if note}<p class="note">{note}</p>{/if}
</section>

<style>
  section { padding: 28px 0; max-width: 60rem; }
  h3 { margin: 28px 0 10px; color: var(--accent); font-weight: 600; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  h3:first-child { margin-top: 0; }
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--text-lo); }
  dd { background: var(--cell-opt); color: var(--text); gap: 24px; justify-content: space-between; }
  dd.warn { background: var(--cell); color: var(--text-hi); }
  dd > span { min-width: 0; }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .path { overflow-wrap: anywhere; }
  .hint { color: var(--text-lo); margin: 8px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); line-height: 1.6; }
  .warn-text { color: var(--accent); }
  .lo { color: var(--text-lo); }
  button { all: unset; cursor: pointer; flex: none; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
  .note { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); margin-top: 20px; }
</style>
