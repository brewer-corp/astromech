<script lang="ts">
  // Settings → Data sharing (astromech-09s3): one section per service, EDDN then EDSM
  // (astromech-ah0i). Inara joins when its upload exists.
  import { eddnLog, schemaName, setEddn, when, type EddnLog } from "../../lib/eddn";
  import { failure } from "../../lib/ui/errors";
  import Edsm from "./Edsm.svelte";

  let eddn = $state<EddnLog | null>(null);
  let loaded = $state(false);
  let note = $state("");
  let busy = $state(false);
  let failed = $state<string | null>(null);

  async function refresh() {
    try {
      eddn = await eddnLog();
      failed = null;
    } catch (e) {
      failed = failure("read the sharing log", e);
    }
    loaded = true;
  }
  $effect(() => {
    void refresh();
    const t = setInterval(() => void refresh(), 2000);
    return () => clearInterval(t);
  });

  async function toggle() {
    if (!eddn) return;
    busy = true;
    const on = !eddn.enabled;
    const r = await setEddn(on);
    busy = false;
    note = r.ok ? (on ? "Sharing is on. Live game data goes to EDDN from now on." : "Sharing is off. Nothing more is sent, and anything waiting is discarded.") : `Couldn't save: ${String(r.error)}`;
    await refresh();
  }
</script>

<section>
  {#if !loaded}
    <p class="lo">Reading the sharing log…</p>
  {:else if failed && !eddn}
    <p class="lo">{failed}</p>
  {:else if !eddn}
    <p class="lo">astromech's background process isn't running, so sharing can't be shown. It starts with this window; if this stays, see Diagnostics.</p>
  {:else}
    <h3>EDDN</h3>
    <dl>
      <dt>Share with EDDN</dt>
      <dd>
        <span>{eddn.enabled ? (eddn.test ? "On (test schemas)" : "On") : "Off"}</span>
        <button onclick={toggle} disabled={busy}>{eddn.enabled ? "Turn off" : "Turn on"}</button>
      </dd>
      <dt>Waiting to send</dt>
      <dd><span>{eddn.pending}</span></dd>
      <dt>Last sent</dt>
      <dd><span>{eddn.last_sent ? `${when(eddn.last_sent.at)} · ${schemaName(eddn.last_sent.schema_ref)}` : "Nothing yet"}</span></dd>
      <dt>Last error</dt>
      <dd class:warn={!!eddn.last_error}>
        <span class="err">{eddn.last_error ? `${when(eddn.last_error.at)} · ${schemaName(eddn.last_error.schema_ref)} · ${eddn.last_error.detail}` : "None"}</span>
      </dd>
    </dl>
    {#if note}<p class="hint">{note}</p>{/if}

    <h3>What gets sent</h3>
    <div class="text">
      <p>
        The Elite Dangerous Data Network collects what players see in game, so tools like Inara, Spansh and EDSM stay
        current. With sharing on, astromech sends it as you play:
      </p>
      <ul>
        <li>station markets, shipyards and carrier materials when you open them;</li>
        <li>stations you dock at, where you are when you log in, and systems you jump to;</li>
        <li>bodies you scan, signals and codex finds;</li>
        <li>routes you plot, and docking granted or denied.</li>
      </ul>
      <p>
        Your commander name goes with each message (EDDN asks for it and obfuscates it before passing messages on).
        Your credits, reputation, cargo, missions and
        anything personal are removed first. Only live play is sent: never the game's beta, never old journals when
        astromech starts, and nothing the game wrote more than 5 minutes ago. A message that can't be delivered
        within 30 minutes is dropped.
      </p>
    </div>
    <Edsm />
  {/if}
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
  .err { overflow-wrap: anywhere; }
  .hint { color: var(--text-lo); margin: 8px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); line-height: 1.6; }
  .lo { color: var(--text-lo); }
  .text { color: var(--text); line-height: 1.6; max-width: 46rem; }
  .text p { margin: 0 0 10px; }
  .text ul { margin: 0 0 10px; padding-left: 20px; }
  button { all: unset; cursor: pointer; flex: none; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
</style>
