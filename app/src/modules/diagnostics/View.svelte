<script lang="ts">
  import KeyTester from "./KeyTester.svelte";
  import { coreCall, coreStatus, openUrl, shellInfo, type CoreStatus } from "../../lib/core";

  const RELEASES = "https://github.com/spadaval/astromech/releases/latest";
  let status = $state<CoreStatus | null>(null);
  let shell = $state("…");
  let note = $state("");

  async function refresh() {
    status = await coreStatus();
  }
  $effect(() => {
    void refresh();
    void shellInfo().then((s) => (shell = s.version));
    const t = setInterval(() => void refresh(), 2000);
    return () => clearInterval(t);
  });

  async function save() {
    note = "Saving…";
    const r = await coreCall("save_diagnostics");
    note = r.ok ? `Saved ${String(r.path)}` : `Couldn't save: ${String(r.error)}`;
    await refresh();
  }

  async function checkUpdate() {
    note = "Checking…";
    const r = await coreCall("check_update");
    if (!r.ok) {
      note = `Couldn't check: ${String(r.error)}`;
      return;
    }
    for (let i = 0; i < 40; i++) {
      await new Promise((ok) => setTimeout(ok, 500));
      await refresh();
      const st = status?.update?.status;
      if (st && st !== "checking" && st !== "downloading") break;
    }
    const u = status?.update;
    if (!u) note = "No update status from the core";
    else if (u.status === "error") note = u.error ?? "Update check failed";
    else if (u.ready && u.latest) note = `Update ${u.latest} is ready to install`;
    else if (u.status === "available" && u.latest) note = `Update ${u.latest} is available`;
    else note = `Installed ${u.installed}`;
  }

  async function installUpdate() {
    note = "Installing… astromech will restart.";
    const r = await coreCall("install_update");
    note = r.ok ? "Restarting to apply the update…" : `Couldn't install: ${String(r.error)}`;
  }

  const game = (g: CoreStatus["game"]) =>
    g === "running" ? "Running" : g === "launcher" ? "Launcher open" : "Not running";

  function updateLine(s: CoreStatus | null): string {
    const u = s?.update;
    if (!u) return "—";
    if (u.status === "checking") return "Checking…";
    if (u.status === "downloading") return `Downloading ${u.latest ?? "update"}…`;
    if (u.status === "applying") return `Installing ${u.latest ?? "update"}…`;
    if (u.status === "error") return u.error ?? "Check failed";
    if (u.ready && u.latest) return `${u.latest} ready (installed ${u.installed})`;
    if (u.status === "available" && u.latest) return `${u.latest} available (installed ${u.installed})`;
    return `Installed ${u.installed}`;
  }
</script>

<section>
  {#if status?.crash_pending}
    <p class="crash">astromech crashed last time. Save diagnostics and attach the zip to a GitHub issue.</p>
  {/if}
  <dl>
    <dt>App window</dt><dd>{shell}</dd>
    <dt>Core</dt><dd>{status ? `${status.version} (pid ${status.pid})` : "Not running"}</dd>
    <dt>Elite</dt><dd>{status ? game(status.game) : "—"}</dd>
    <dt>Core state</dt><dd>{status ? (status.starting ? "Starting" : status.active ? "Active" : "Idle") : "—"}</dd>
    <dt>Voice</dt><dd>{status?.voice ?? "—"}{status?.voice_paused ? " (paused)" : ""}</dd>
    <dt>Kill switch</dt><dd>{status ? (status.killed ? "On" : "Off") : "—"}</dd>
    <dt>Start with Windows</dt><dd>{status ? (status.autostart ? "On" : "Off") : "—"}</dd>
    <dt>Config file</dt><dd class="path">{status?.config ?? "—"}{status?.config_note ? ` · ${status.config_note}` : ""}</dd>
    <dt>Logs</dt><dd class="path">{status?.log_dir ?? "—"}</dd>
    <dt>Update</dt><dd>{updateLine(status)}</dd>
  </dl>
  <div class="actions">
    <button onclick={save} disabled={!status}>Save diagnostics</button>
    <button onclick={() => status && openUrl(status.log_dir)} disabled={!status}>Open log folder</button>
    <button onclick={checkUpdate} disabled={!status}>Check for updates</button>
    {#if status?.update?.ready}
      <button onclick={installUpdate}>Install update {status.update.latest ?? ""}</button>
    {/if}
    <button onclick={() => openUrl(RELEASES)}>Release notes</button>
  </div>
  {#if note}<p class="note">{note}</p>{/if}
</section>
<KeyTester />

<style>
  section { padding: 28px 0; max-width: 60rem; }
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0 0 24px; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--text-lo); }
  dd { background: var(--cell-opt); color: var(--text); }
  .path { font-family: var(--mono); font-size: var(--fs-s); overflow-wrap: anywhere; }
  .actions { display: flex; gap: 28px; }
  button { all: unset; cursor: pointer; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
  .crash { background: var(--accent); color: var(--on-accent); padding: 10px 12px; margin: 0 0 20px; font-weight: 600; }
  .note { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); }
</style>
