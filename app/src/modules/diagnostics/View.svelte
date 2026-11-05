<script lang="ts">
  // Diagnostics (docs/V1-DESIGN.md §2.2): what astromech sees right now, and Save diagnostics for
  // a bug report. Settings live in Settings; the key tester is under Controls.
  import Button from "../../lib/ui/Button.svelte";
  import { coreCall, coreStatus, openUrl, shellInfo, type CoreStatus } from "../../lib/core";
  import { pollFailure } from "../../lib/ui/errors";
  import Failure from "../../lib/ui/Failure.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import { eddnLog, schemaName, when, type EddnLog, type EddnLogEntry } from "../../lib/eddn";

  let status = $state<CoreStatus | null>(null);
  let shell = $state("…");
  let note = $state("");
  let noteErr = $state(false);
  let eddn = $state<EddnLog | null>(null);
  let eddnErr = $state<string | null>(null);

  async function refresh() {
    status = await coreStatus();
    try {
      eddn = await eddnLog();
      eddnErr = null;
    } catch (e) {
      eddnErr = pollFailure("read the EDDN log", e, eddn !== null);
    }
  }
  $effect(() => {
    void refresh();
    void shellInfo().then((s) => (shell = s.version));
    const t = setInterval(() => void refresh(), 2000);
    return () => clearInterval(t);
  });

  async function save() {
    noteErr = false;
    note = "Saving…";
    const r = await coreCall("save_diagnostics");
    noteErr = !r.ok;
    note = r.ok ? `Saved ${String(r.path)}` : `Couldn't save: ${String(r.error)}`;
    await refresh();
  }

  const outcome: Record<string, string> = { sent: "Sent", rejected: "Rejected", retry: "Will retry", gave_up: "Gave up", discarded: "Discarded", dropped: "Not sent" };

  const game = (g: CoreStatus["game"]) =>
    g === "running" ? "Running" : g === "launcher" ? "Launcher open" : "Not running";

  // The window and the background process ship together; only a half-applied update splits them.
  const version = $derived(
    !status ? `${shell} (background process not running)` : status.version === shell ? shell : `window ${shell}, background ${status.version}`,
  );
</script>

{#snippet rows(entries: EddnLogEntry[])}
  <ol class="eddn">
    {#each entries as e, i (i)}
      <li class={e.outcome}>
        <span class="mono">{when(e.at)}</span>
        <span>{schemaName(e.schema_ref)}</span>
        <span>{outcome[e.outcome] ?? e.outcome}</span>
        <span class="mono detail">{e.detail}</span>
      </li>
    {/each}
  </ol>
{/snippet}

<section>
  {#if status?.crash_pending}
    <p class="crash"><b>!</b> astromech crashed last time. Save diagnostics and attach the zip to a GitHub issue.</p>
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
    <Button kind="primary" onclick={save} disabled={!status}>Save diagnostics</Button>
    <Button onclick={() => status && openUrl(status.log_dir)} disabled={!status}>Open log folder</Button>
  </div>
  {#if note}<p class="note" class:err={noteErr} role={noteErr ? "alert" : undefined}>{note}</p>{/if}

  <h2 class="section">EDDN</h2>
  <Stale error={eddn ? eddnErr : null} />
  {#if eddnErr && !eddn}
    <Failure text={eddnErr} onaction={() => void refresh()} />
  {:else if !eddn}
    <p class="note">—</p>
  {:else}
    <p class="note">{eddn.enabled ? "Sharing on" : "Sharing off"} · {eddn.pending} waiting to send</p>
    <h3>Gateway answers</h3>
    {#if eddn.log.length}
      {@render rows(eddn.log)}
    {:else}
      <p class="note">Nothing sent to EDDN yet.</p>
    {/if}
    <h3>Not sent</h3>
    {#if eddn.skipped.length}
      {@render rows(eddn.skipped)}
    {:else}
      <p class="note">Nothing skipped.</p>
    {/if}
  {/if}
</section>

<style>
  section { padding: 28px 0; max-width: 60rem; }
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0 0 24px; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--options-grey); }
  /* Read-only facts, outside a cell (DESIGN.md Settings row). */
  dd { color: var(--accent); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .actions { display: flex; gap: 16px; align-items: center; }
  /* A warning: an orange "!" and plain words, never a filled block (DESIGN.md States). */
  .crash { margin: 0 0 20px; }
  .crash b { font-weight: 600; }
  h2 { margin: 32px 0 12px; }
  h3 { margin: 20px 0 0; font-weight: 500; font-size: var(--fs); }
  .eddn { list-style: none; margin: 12px 0 0; padding: 0; display: grid; gap: var(--gap); }
  .eddn li { display: grid; grid-template-columns: 11rem 12rem 7rem 1fr; align-items: center; min-height: var(--row); background: var(--surface); } /* read-only log rows: hull */
  .eddn li > span { padding: 0 12px; }
  .eddn .detail { color: var(--orange-mid); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  /* A send that went nowhere steps down a level; its outcome word says why. */
  .eddn li:is(.rejected, .gave_up, .discarded, .dropped) { color: var(--orange-mid); }
  .note { color: var(--orange-mid); font-family: var(--mono); font-size: var(--fs-s); }
  .note.err { color: var(--alarm); }
  .note.err::before { content: "! "; font-weight: 600; }
</style>
