<script lang="ts">
  // Diagnostics (docs/V1-DESIGN.md §2.2): what astromech sees right now, and Save diagnostics for
  // a bug report. Settings live in Settings; the key tester is under Controls.
  import { coreCall, coreStatus, openUrl, shellInfo, type CoreStatus } from "../../lib/core";
  import { eddnLog, schemaName, type EddnLog } from "../../lib/eddn";

  let status = $state<CoreStatus | null>(null);
  let shell = $state("…");
  let note = $state("");
  let eddn = $state<EddnLog | null>(null);

  async function refresh() {
    status = await coreStatus();
    eddn = await eddnLog();
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

  // Date and time: the log keeps entries across days and restarts.
  const time = (at: number) =>
    new Date(at * 1000).toLocaleString([], { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit", second: "2-digit" });
  const outcome: Record<string, string> = { sent: "Sent", rejected: "Rejected", retry: "Will retry", gave_up: "Gave up", dropped: "Not sent" };

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

  <h2>EDDN</h2>
  {#if !eddn}
    <p class="note">—</p>
  {:else}
    <p class="note">{eddn.pending} waiting to send</p>
    {#if eddn.log.length}
      <ol class="eddn">
        {#each eddn.log as e, i (i)}
          <li class={e.outcome}>
            <span class="mono">{time(e.at)}</span>
            <span>{schemaName(e.schema_ref)}</span>
            <span>{outcome[e.outcome] ?? e.outcome}</span>
            <span class="mono detail">{e.detail}</span>
          </li>
        {/each}
      </ol>
    {:else}
      <p class="note">Nothing sent to EDDN yet.</p>
    {/if}
  {/if}
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
  h2 { margin: 32px 0 12px; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); color: var(--text-lo); font-weight: 600; }
  .eddn { list-style: none; margin: 12px 0 0; padding: 0; display: grid; gap: var(--gap); }
  .eddn li { display: grid; grid-template-columns: 11rem 12rem 7rem 1fr; align-items: center; min-height: var(--row); background: var(--cell-opt); color: var(--text); }
  .eddn li > span { padding: 0 12px; }
  .eddn .detail { color: var(--text-lo); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .eddn li.rejected, .eddn li.gave_up, .eddn li.dropped { color: var(--text-lo); }
  .note { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); }
</style>
