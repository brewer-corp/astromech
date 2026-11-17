<script lang="ts">
  // Diagnostics (docs/V1-DESIGN.md §2.2): what astromech sees right now, and Save diagnostics for
  // a bug report. Settings live in Settings; the key tester is under Controls.
  import Button from "../../lib/ui/Button.svelte";
  import { coreCall, coreStatus, openUrl, WINDOW_VERSION as shell, type CoreStatus } from "../../lib/core";
  import { pollFailure } from "../../lib/ui/errors";
  import Failure from "../../lib/ui/Failure.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import Form from "../../lib/ui/layout/Form.svelte";
  import { eddnLog, schemaName, when, type EddnLog, type EddnLogEntry } from "../../lib/eddn";

  let status = $state<CoreStatus | null>(null);
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

  // The window is built into the app, so its version is the app's.
  const version = shell;
</script>

{#snippet rows(entries: EddnLogEntry[])}
  <ol class="eddn">
    {#each entries as e, i (i)}
      <li class={e.outcome}>
        <span class="when type-label">{when(e.at)}</span>
        <span>{schemaName(e.schema_ref)}</span>
        <span>{outcome[e.outcome] ?? e.outcome}</span>
        <span class="detail type-label">{e.detail}</span>
      </li>
    {/each}
  </ol>
{/snippet}

<Form>
  {#if status?.crash_pending}
    <p class="crash type-body"><b>!</b> astromech crashed last time. Save diagnostics and attach the zip to a GitHub issue.</p>
  {/if}
  <dl>
    <dt class="type-name">Version</dt><dd>{version}</dd>
    <dt class="type-name">Elite</dt><dd>{status ? game(status.game) : "—"}</dd>
    <dt class="type-name">State</dt><dd>{status ? (status.starting ? "Starting" : status.active ? "Active" : "Idle") : "—"}</dd>
    <dt class="type-name">Voice</dt><dd>{status?.voice ?? "—"}{status?.voice_paused ? " (paused)" : ""}</dd>
    <dt class="type-name">Kill switch</dt><dd>{status ? (status.killed ? "On" : "Off") : "—"}</dd>
    <dt class="type-name">Setup</dt><dd>{status ? (status.setup_needed?.length ? status.setup_needed.join(", ") : "Done") : "—"}</dd>
    {#if status?.config_note}<dt class="type-name">Config</dt><dd>{status.config_note}</dd>{/if}
    <dt class="type-name">Process</dt><dd>{status ? `pid ${status.pid}` : "—"}</dd>
  </dl>
  <div class="actions">
    <Button kind="primary" onclick={save} disabled={!status}>Save diagnostics</Button>
    <Button onclick={() => status && openUrl(status.log_dir)} disabled={!status}>Open log folder</Button>
  </div>
  {#if note}<p class="note type-label" class:err={noteErr} role={noteErr ? "alert" : undefined}>{note}</p>{/if}

  <h2 class="section">EDDN</h2>
  <Stale error={eddn ? eddnErr : null} />
  {#if eddnErr && !eddn}
    <Failure text={eddnErr} onaction={() => void refresh()} />
  {:else if !eddn}
    <p class="note type-label">—</p>
  {:else}
    <p class="note type-label">{eddn.enabled ? "Sharing on" : "Sharing off"} · {eddn.pending} waiting to send</p>
    <h3 class="type-name">Gateway answers</h3>
    {#if eddn.log.length}
      {@render rows(eddn.log)}
    {:else}
      <p class="note type-label">Nothing sent to EDDN yet.</p>
    {/if}
    <h3 class="type-name">Not sent</h3>
    {#if eddn.skipped.length}
      {@render rows(eddn.skipped)}
    {:else}
      <p class="note type-label">Nothing skipped.</p>
    {/if}
  {/if}
</Form>

<style>
  dl { display: grid; grid-template-columns: var(--form-label) minmax(0, 1fr); gap: var(--gap); margin: 0 0 var(--sp-6); }
  dt, dd { margin: 0; padding: 0 var(--sp-3); min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--options-grey); }
  /* Read-only facts, outside a cell (DESIGN.md Settings row). */
  dd { color: var(--prose); }
  .actions { display: flex; gap: var(--sp-4); align-items: center; }
  /* A warning: an orange "!" and plain words, never a filled block (DESIGN.md States). */
  .crash { margin: 0 0 var(--sp-6); color: var(--accent); font-weight: 500; }
  .crash b { font-weight: 600; }
  h2 { margin: var(--sp-8) 0 var(--sp-3); }
  h3 { margin: var(--sp-6) 0 0; }
  .eddn { list-style: none; margin: var(--sp-3) 0 0; padding: 0; display: grid; gap: var(--gap); }
  /* Read-only log rows in a Seam outline (Hull is the frame's): when, what, the outcome, then the
     gateway's words in the rest of the row. */
  .eddn li { display: grid; grid-template-columns: 10.5rem 11.25rem 6.5rem minmax(0, 1fr); align-items: center; min-height: var(--row); outline: 1px solid var(--line); outline-offset: -1px; }
  .eddn li > span { padding: 0 var(--sp-3); }
  .eddn .when { color: var(--prose); }
  .eddn .detail { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  /* A send that went nowhere steps down a level; its outcome word says why. */
  .eddn li:is(.rejected, .gave_up, .discarded, .dropped) { color: var(--label-grey); }
  .note { margin: var(--sp-3) 0 0; }
  .note.err { color: var(--alarm); }
  .note.err::before { content: "! "; font-weight: 600; }
</style>
