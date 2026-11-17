<script lang="ts">
  // Settings → Data sharing (astromech-09s3): one section per service, EDDN then EDSM
  // (astromech-ah0i). Inara joins when its upload exists.
  import Button from "../../lib/ui/Button.svelte";
  import { eddnLog, schemaName, setEddn, when, type EddnLog } from "../../lib/eddn";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import Form from "../../lib/ui/layout/Form.svelte";
  import Edsm from "./Edsm.svelte";

  let eddn = $state<EddnLog | null>(null);
  let loaded = $state(false);
  let note = $state("");
  let noteErr = $state(false);
  let busy = $state(false);
  let failed = $state<string | null>(null);

  async function refresh() {
    try {
      eddn = await eddnLog();
      failed = null;
    } catch (e) {
      failed = pollFailure("read the sharing log", e, eddn !== null);
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
    noteErr = !r.ok;
    note = r.ok ? (on ? "Sharing is on. Live game data goes to EDDN from now on." : "Sharing is off. Nothing more is sent, and anything waiting is discarded.") : `Couldn't save: ${String(r.error)}`;
    await refresh();
  }
</script>

<Form>
  <Stale error={eddn ? failed : null} />
  {#if !loaded}
    <p class="type-body">Reading the sharing log…</p>
  {:else if !eddn}
    <p class="type-body">{failed}</p>
  {:else}
    <h3 class="section">EDDN</h3>
    <dl>
      <dt class="type-name">Share with EDDN</dt>
      <dd class="value">
        <span>{eddn.enabled ? (eddn.test ? "On (test schemas)" : "On") : "Off"}</span>
        <Button onclick={toggle} disabled={busy}>{eddn.enabled ? "Turn off" : "Turn on"}</Button>
      </dd>
      <dt class="type-name">Waiting to send</dt>
      <dd><span>{eddn.pending}</span></dd>
      <dt class="type-name">Last sent</dt>
      <dd><span>{eddn.last_sent ? `${when(eddn.last_sent.at)} · ${schemaName(eddn.last_sent.schema_ref)}` : "Nothing yet"}</span></dd>
      <dt class="type-name">Last error</dt>
      <dd class:warn={!!eddn.last_error}>
        <span class="err">{eddn.last_error ? `${when(eddn.last_error.at)} · ${schemaName(eddn.last_error.schema_ref)} · ${eddn.last_error.detail}` : "None"}</span>
      </dd>
    </dl>
    {#if note}<p class="hint type-label" class:err={noteErr} role={noteErr ? "alert" : undefined}>{note}</p>{/if}

    <h3 class="section">What gets sent</h3>
    <div class="text type-body">
      <p>
        The Elite Dangerous Data Network collects what players see in game, so tools like Inara, Spansh and EDSM stay
        current. With sharing on, astromech sends it as you play:
      </p>
      <ul>
        <li>station markets, outfitting, shipyards and carrier materials when you open them;</li>
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
</Form>

<style>
  h3 { margin: var(--sp-6) 0 var(--sp-2); }
  h3:first-child { margin-top: 0; }
  dl { display: grid; grid-template-columns: var(--form-label) minmax(0, 1fr); gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 var(--sp-3); min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--options-grey); }
  /* DESIGN.md Settings row: a value the commander changes with its action word sits in an ember
     cell (.value); a status with no such action is a fact, outside a cell. */
  dd { color: var(--prose); gap: var(--sp-6); justify-content: space-between; }
  dd.value { background: var(--cell); color: var(--accent); font-weight: 500; }
  dd.warn { color: var(--accent); font-weight: 500; }
  /* A row that needs the commander: an orange "!" before its words (DESIGN.md States). */
  dd.warn > :first-child::before { content: "! "; font-weight: 600; }
  dd > span { min-width: 0; }
  .err { overflow-wrap: anywhere; }
  .hint { color: var(--prose); margin: var(--sp-2) 0 0 calc(var(--form-label) + var(--gap)); }
  .hint.err { color: var(--alarm); }
  .hint.err::before { content: "! "; font-weight: 600; }
  /* What sharing sends: the group's description, kept in full since it is what the commander agrees
     to by turning sharing on. */
  .text { max-width: var(--measure); }
  .text p, .text ul { margin: 0 0 var(--sp-3); }
  .text ul { padding-left: var(--sp-6); }
</style>
