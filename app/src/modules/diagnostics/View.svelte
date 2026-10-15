<script lang="ts">
  // Diagnostics (docs/V1-DESIGN.md §2.2): what astromech sees right now, and Save diagnostics for
  // a bug report. Settings live in Settings; the key tester is under Controls.
  import { coreCall, coreStatus, openUrl, shellInfo, type CoreStatus } from "../../lib/core";

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

  const game = (g: CoreStatus["game"]) =>
    g === "running" ? "Running" : g === "launcher" ? "Launcher open" : "Not running";

  // The window and the background process ship together; only a half-applied update splits them.
  const version = $derived(
    !status ? `${shell} (background process not running)` : status.version === shell ? shell : `window ${shell}, background ${status.version}`,
  );
</script>

<section>
  {#if status?.crash_pending}
    <p class="crash">astromech crashed last time. Save diagnostics and attach the zip to a GitHub issue.</p>
  {/if}
  <dl>
    <dt>Version</dt><dd>{version}</dd>
    <dt>Elite</dt><dd>{status ? game(status.game) : "—"}</dd>
    <dt>State</dt><dd>{status ? (status.starting ? "Starting" : status.active ? "Active" : "Idle") : "—"}</dd>
    <dt>Voice</dt><dd>{status?.voice ?? "—"}{status?.voice_paused ? " (paused)" : ""}</dd>
    <dt>Kill switch</dt><dd>{status ? (status.killed ? "On" : "Off") : "—"}</dd>
    <dt>Setup</dt><dd>{status ? (status.setup_needed?.length ? status.setup_needed.join(", ") : "Done") : "—"}</dd>
    {#if status?.config_note}<dt>Config</dt><dd>{status.config_note}</dd>{/if}
    <dt>Process</dt><dd class="mono">{status ? `pid ${status.pid}` : "—"}</dd>
  </dl>
  <div class="actions">
    <button onclick={save} disabled={!status}>Save diagnostics</button>
    <button onclick={() => status && openUrl(status.log_dir)} disabled={!status}>Open log folder</button>
  </div>
  {#if note}<p class="note">{note}</p>{/if}
</section>

<style>
  section { padding: 28px 0; max-width: 60rem; }
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0 0 24px; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--text-lo); }
  dd { background: var(--cell-opt); color: var(--text); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .actions { display: flex; gap: 28px; }
  button { all: unset; cursor: pointer; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
  .crash { background: var(--accent); color: var(--on-accent); padding: 10px 12px; margin: 0 0 20px; font-weight: 600; }
  .note { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); }
</style>
