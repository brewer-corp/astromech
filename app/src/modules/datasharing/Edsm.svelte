<script lang="ts">
  // Settings → Data sharing, EDSM (astromech-ah0i): your journal to your EDSM account, so its
  // flight log and map keep up. The API key goes to the OS keyring; it's never shown again.
  import { edsmStatus, flightLog, setEdsm, type EdsmStatus, type Visit } from "../../lib/edsm";
  import { when } from "../../lib/eddn";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";

  let st = $state<EdsmStatus | null>(null);
  let commander = $state("");
  let key = $state("");
  let note = $state("");
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
  <h3>EDSM</h3>
  <Stale error={readErr} />
  <dl>
    <dt>Upload to EDSM</dt>
    <dd>
      <span>{label(st)}</span>
      <button onclick={toggle} disabled={busy}>{st.enabled ? "Turn off" : "Turn on"}</button>
    </dd>
    <dt><label for="edsm-cmdr">Commander name</label></dt>
    <dd>
      <input id="edsm-cmdr" bind:value={commander} placeholder="as registered on EDSM" autocomplete="off" spellcheck="false" />
      <button onclick={saveName} disabled={busy || commander.trim() === st.commander} aria-label="Save commander name">Save</button>
    </dd>
    <dt><label for="edsm-key">API key</label></dt>
    <dd>
      <input id="edsm-key" type="password" bind:value={key} placeholder={st.has_key ? "saved (enter a new one to replace it)" : "from EDSM: Settings > My API Key"} autocomplete="off" />
      {#if key.trim()}
        <button onclick={saveKey} disabled={busy} aria-label="Save API key">Save</button>
      {:else if st.has_key}
        <button onclick={forgetKey} disabled={busy} aria-label="Remove API key">Remove</button>
      {/if}
    </dd>
    {#if st.key_error}
      <dt>Keyring</dt>
      <dd class="warn"><span class="err">{st.key_error}</span></dd>
    {/if}
    <dt>Waiting to send</dt>
    <dd><span>{st.pending}</span></dd>
    <dt>Last sent</dt>
    <dd><span>{st.last_sent ? `${when(st.last_sent.at)} · ${st.last_sent.detail}` : "Nothing yet"}</span></dd>
    <dt>Last error</dt>
    <dd class:warn={!!st.last_error}>
      <span class="err">{st.last_error ? `${when(st.last_error.at)} · ${st.last_error.detail}` : "None"}</span>
    </dd>
    <dt>Flight log</dt>
    <dd>
      <span>{visits ? `${visits.length} systems in the last week` : logError || "On EDSM"}</span>
      <button onclick={showLog} disabled={busy || !st.has_key || !st.commander} aria-label="Show flight log">Show</button>
    </dd>
  </dl>
  {#if note}<p class="hint">{note}</p>{/if}
  {#if visits && visits.length}
    <table>
      <thead><tr><th>Arrived (UTC)</th><th>System</th><th></th></tr></thead>
      <tbody>
        {#each visits as v (v.date + v.system)}
          <tr><td>{v.date}</td><td>{v.system}</td><td>{v.first_discover ? "First to report" : ""}</td></tr>
        {/each}
      </tbody>
    </table>
  {/if}
  <div class="text">
    <p>
      With upload on, astromech sends your journal to your own EDSM account as you play, the way EDMC did, so your
      EDSM flight log, map and profile stay up to date. Events EDSM doesn't use (chat, music, the game's own status)
      stay on your PC. Only live play is sent, never the game's beta. Your API key is kept in Windows Credential
      Manager, not in astromech's settings file.
    </p>
  </div>
{:else if readErr}
  <p class="lo">{readErr}</p>
{/if}

<style>
  h3 { margin: 28px 0 10px; color: var(--accent); font-weight: 600; font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--text-lo); }
  dd { background: var(--cell-opt); color: var(--text); gap: 24px; justify-content: space-between; }
  dd.warn { background: var(--cell); color: var(--text-hi); }
  dd > span { min-width: 0; }
  .err { overflow-wrap: anywhere; }
  input { all: unset; flex: 1; min-width: 0; color: var(--text-hi); }
  input::placeholder { color: var(--text-lo); }
  .hint { color: var(--text-lo); margin: 8px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); line-height: 1.6; }
  .text { color: var(--text); line-height: 1.6; max-width: 46rem; margin-top: 16px; }
  .text p { margin: 0 0 10px; }
  table { border-collapse: collapse; margin: 12px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); }
  th { color: var(--text-lo); font-weight: 400; text-align: left; padding: 4px 16px 4px 0; }
  td { color: var(--text); padding: 4px 16px 4px 0; }
  button { all: unset; cursor: pointer; flex: none; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
</style>
