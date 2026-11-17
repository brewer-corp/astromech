<script lang="ts">
  // Diagnostics (docs/V1-DESIGN.md §2.2): what astromech sees right now, and Save diagnostics for
  // a bug report. Settings live in Settings; the key tester is under Controls.
  import Button from "../../lib/ui/Button.svelte";
  import { coreCall, coreStatus, openUrl, WINDOW_VERSION as shell, type CoreStatus } from "../../lib/core";
  import { pollFailure } from "../../lib/ui/errors";
  import Failure from "../../lib/ui/Failure.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
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
        <span class="when">{when(e.at)}</span>
        <span>{schemaName(e.schema_ref)}</span>
        <span>{outcome[e.outcome] ?? e.outcome}</span>
        <span class="detail">{e.detail}</span>
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
    <dt>Process</dt><dd>{status ? `pid ${status.pid}` : "—"}</dd>
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
  section { padding: 1.75rem 0; max-width: 56.25rem; }
  dl { display: grid; grid-template-columns: 13.125rem 1fr; gap: var(--gap); margin: 0 0 var(--sp-6); }
  dt, dd { margin: 0; padding: 0 var(--sp-3); min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--options-grey); }
  /* Read-only facts, outside a cell (DESIGN.md Settings row). */
  dd { color: var(--prose); }
  .actions { display: flex; gap: var(--sp-4); align-items: center; }
  /* A warning: an orange "!" and plain words, never a filled block (DESIGN.md States). */
  .crash { margin: 0 0 1.25rem; }
  .crash b { font-weight: 600; }
  h2 { margin: var(--sp-8) 0 var(--sp-3); }
  h3 { margin: 1.25rem 0 0; font-weight: 500; font-size: var(--fs); }
  .eddn { list-style: none; margin: var(--sp-3) 0 0; padding: 0; display: grid; gap: var(--gap); }
  .eddn li { display: grid; grid-template-columns: 10.3125rem 11.25rem 6.5625rem 1fr; align-items: center; min-height: var(--row); outline: 1px solid var(--line); outline-offset: -1px; } /* read-only log rows: a Seam outline (Hull is the frame's) */
  .eddn li > span { padding: 0 var(--sp-3); }
  .eddn :is(.when, .detail) { font-size: var(--fs-s); }
  .eddn .detail { color: var(--label-grey); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  /* A send that went nowhere steps down a level; its outcome word says why. */
  .eddn li:is(.rejected, .gave_up, .discarded, .dropped) { color: var(--label-grey); }
  .note { color: var(--label-grey); font-size: var(--fs-s); }
  .note.err { color: var(--alarm); }
  .note.err::before { content: "! "; font-weight: 600; }
</style>
