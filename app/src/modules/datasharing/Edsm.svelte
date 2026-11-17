<script lang="ts">
  // Settings → Data sharing, EDSM (astromech-ah0i): your journal to your EDSM account, so its
  // flight log and map keep up. The API key goes to the OS keyring; it's never shown again.
  import Button from "../../lib/ui/Button.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import { edsmStatus, flightLog, setEdsm, type EdsmStatus, type Visit } from "../../lib/edsm";
  import { when } from "../../lib/eddn";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";

  let st = $state<EdsmStatus | null>(null);
  let commander = $state("");
  let key = $state("");
  let note = $state("");
  let noteErr = $state(false);
  let busy = $state(false);
  let visits = $state<Visit[] | null>(null);
  let logError = $state("");

  let readErr = $state<string | null>(null);
  /** The name field takes the saved commander once, from the first read that works. */
  let seeded = false;
  async function refresh() {
    try {
      st = await edsmStatus();
      readErr = null;
      if (!seeded) {
        seeded = true;
        commander = st.commander;
      }
    } catch (e) {
      readErr = pollFailure("read the EDSM status", e, st !== null);
    }
  }
  $effect(() => {
    void refresh();
    const t = setInterval(() => void refresh(), 2000);
    return () => clearInterval(t);
  });

  async function save(change: Parameters<typeof setEdsm>[0], ok: string) {
    busy = true;
    const r = await setEdsm(change);
    busy = false;
    noteErr = !r.ok;
    note = r.ok ? ok : `Couldn't save: ${String(r.error)}`;
    // A saved key leaves the page at once.
    if (r.ok && change.api_key !== undefined) key = "";
    await refresh();
  }

  const toggle = () =>
    st && save({ enabled: !st.enabled }, st.enabled ? "EDSM upload is off. Anything waiting is discarded." : "EDSM upload is on.");
  const saveName = () => save({ commander }, "Commander name saved.");
  const saveKey = () => save({ api_key: key }, "API key saved in Windows Credential Manager.");
  const forgetKey = () => save({ api_key: "" }, "API key removed.");

  async function showLog() {
    logError = "";
    const r = await flightLog();
    if ("error" in r) {
      visits = null;
      logError = r.error;
    } else visits = r.visits;
  }

  function label(s: EdsmStatus): string {
    if (!s.enabled) return "Off";
    if (s.uploading) return "On";
    return !s.commander ? "On, but no commander name" : "On, but no API key";
  }
</script>

{#if st}
  <h3 class="section">EDSM</h3>
  <Stale error={readErr} />
  <dl>
    <dt class="type-name">Upload to EDSM</dt>
    <dd class="value">
      <span>{label(st)}</span>
      <Button onclick={toggle} disabled={busy}>{st.enabled ? "Turn off" : "Turn on"}</Button>
    </dd>
    <dt class="type-name"><label for="edsm-cmdr">Commander name</label></dt>
    <dd class="bare">
      <Input class="grow" id="edsm-cmdr" bind:value={commander} placeholder="As registered on EDSM" autocomplete="off" spellcheck="false">
        {#snippet end()}<Button onclick={saveName} disabled={busy || commander.trim() === st?.commander} aria-label="Save commander name">Save</Button>{/snippet}
      </Input>
    </dd>
    <dt class="type-name"><label for="edsm-key">API key</label></dt>
    <dd class="bare">
      <Input class="grow" id="edsm-key" type="password" bind:value={key} placeholder={st.has_key ? "Saved (enter a new one to replace it)" : "From EDSM: Settings > My API Key"} autocomplete="off">
        {#snippet end()}
          {#if key.trim()}
            <Button onclick={saveKey} disabled={busy} aria-label="Save API key">Save</Button>
          {:else if st?.has_key}
            <Button onclick={forgetKey} disabled={busy} aria-label="Remove API key">Remove</Button>
          {/if}
        {/snippet}
      </Input>
    </dd>
    {#if st.key_error}
      <dt class="type-name">Keyring</dt>
      <dd class="warn failed" role="alert"><span class="err">{st.key_error}</span></dd>
    {/if}
    <dt class="type-name">Waiting to send</dt>
    <dd><span>{st.pending}</span></dd>
    <dt class="type-name">Last sent</dt>
    <dd><span>{st.last_sent ? `${when(st.last_sent.at)} · ${st.last_sent.detail}` : "Nothing yet"}</span></dd>
    <dt class="type-name">Last error</dt>
    <dd class:warn={!!st.last_error}>
      <span class="err">{st.last_error ? `${when(st.last_error.at)} · ${st.last_error.detail}` : "None"}</span>
    </dd>
    <dt class="type-name">Flight log</dt>
    <dd>
      <span>{visits ? `${visits.length} systems in the last week` : logError || "On EDSM"}</span>
      <Button onclick={showLog} disabled={busy || !st.has_key || !st.commander} aria-label="Show flight log">Show</Button>
    </dd>
  </dl>
  {#if note}<p class="hint type-label" class:err={noteErr} role={noteErr ? "alert" : undefined}>{note}</p>{/if}
  {#if visits && visits.length}
    <table class="type-figures">
      <thead><tr class="type-label"><th>Arrived (UTC)</th><th>System</th><th></th></tr></thead>
      <tbody>
        {#each visits as v (v.date + v.system)}
          <tr><td>{v.date}</td><td>{v.system}</td><td>{v.first_discover ? "First to report" : ""}</td></tr>
        {/each}
      </tbody>
    </table>
  {/if}
  <div class="text type-body">
    <p>
      With upload on, astromech sends your journal to your own EDSM account as you play, the way EDMC did, so your
      EDSM flight log, map and profile stay up to date. Events EDSM doesn't use (chat, music, the game's own status)
      stay on your PC. Only live play is sent, never the game's beta. Your API key is kept in Windows Credential
      Manager, not in astromech's settings file.
    </p>
  </div>
{:else if readErr}
  <p class="type-body">{readErr}</p>
{/if}

<style>
  h3 { margin: var(--sp-8) 0 var(--sp-2); }
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
  .hint.err { color: var(--alarm); }
  .hint.err::before { content: "! "; font-weight: 600; }
  /* A row whose value is a box of its own has no cell around it. */
  dd.bare { padding: 0; background: none; }
  dd.bare > :global(.grow) { flex: 1; }
  .hint { color: var(--prose); margin: var(--sp-2) 0 0 calc(var(--form-label) + var(--gap)); }
  .text { max-width: var(--measure); margin-top: var(--sp-4); }
  .text p { margin: 0 0 var(--sp-3); }
  table { border-collapse: collapse; margin: var(--sp-3) 0 0 calc(var(--form-label) + var(--gap)); }
  th { color: var(--label-grey); font-weight: 400; text-align: left; padding: var(--sp-1) var(--sp-4) var(--sp-1) 0; }
  td { padding: var(--sp-1) var(--sp-4) var(--sp-1) 0; }
</style>
