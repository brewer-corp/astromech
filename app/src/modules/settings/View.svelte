<script lang="ts">
  // Settings (docs/V1-DESIGN.md §2.3): one list, label left, value right, actions on the row.
  // Everything the tray used to carry lives here now. Push-to-talk, the kill switch and the mic
  // are picked in setup (onboarding); calibration's "Run again" opens just that step.
  import { coreCall, coreStatus, openUrl, type CoreStatus, type CoreCmd, type MicList, type VoiceSettings, type Colour } from "../../lib/core";
  import { control } from "../../lib/format";
  import Calibration from "../../lib/ui/Calibration.svelte";

  let status = $state<CoreStatus | null>(null);
  let note = $state("");
  let mics = $state<MicList | null>(null);
  let calibrating = $state(false);
  let voice = $state<VoiceSettings | null>(null);
  let voiceNote = $state("");

  async function loadVoice() {
    const r = await coreCall("voice_settings");
    voice = r.ok ? (r as unknown as VoiceSettings) : null;
  }
  $effect(() => {
    void loadVoice();
  });

  async function setVoice(change: { speaker?: string; speed?: number; colour?: Colour }) {
    const r = await coreCall("set_voice", change);
    voiceNote = r.ok ? "Saved. The next reply uses it." : String(r.error);
    await loadVoice();
  }

  async function preview() {
    voiceNote = "Speaking…";
    const r = await coreCall("voice_preview");
    voiceNote = r.ok ? `“${String(r.line)}”` : String(r.error);
  }

  const COLOURS: { id: Colour; label: string }[] = [
    { id: "natural", label: "Natural" },
    { id: "cockpit", label: "Cockpit" },
    { id: "harsh", label: "Harsh" },
  ];
  const speeds = $derived.by(() => {
    if (!voice) return [];
    const [lo, hi] = voice.speeds;
    const out: number[] = [];
    for (let x = Math.round(lo * 10); x <= Math.round(hi * 10); x++) out.push(x / 10);
    return out;
  });

  async function loadMics() {
    const r = await coreCall("mic_list");
    mics = r.ok ? (r as unknown as MicList) : null;
  }
  $effect(() => {
    void loadMics();
  });

  const button = (s: string | null | undefined) => {
    const [d, k] = (s ?? "").split(" ");
    return d && k ? control(d, k) : null;
  };
  const gate = $derived(mics?.calibration.min_level_dbfs);

  async function refresh() {
    status = await coreStatus();
  }
  $effect(() => {
    void refresh();
    const t = setInterval(() => void refresh(), 1000);
    return () => clearInterval(t);
  });

  async function run(cmd: CoreCmd, doing = "") {
    note = doing;
    const r = await coreCall(cmd);
    note = r.ok ? "" : String(r.error);
    await refresh();
  }

  const u = $derived(status?.update);
  const busy = $derived(u?.status === "checking" || u?.status === "downloading" || u?.status === "applying");

  function updateLine(): string {
    if (!u) return "—";
    if (u.status === "checking") return "Checking…";
    if (u.status === "downloading") return `Downloading ${u.latest ?? "update"}…`;
    if (u.status === "applying") return `Installing ${u.latest ?? "update"}; astromech will reopen`;
    if (u.status === "error") return u.error ?? "Check failed";
    if (u.ready && u.latest) return `${u.latest} is ready to install`;
    if (u.status === "available" && u.latest) return `${u.latest} is available`;
    return u.last_check ? "Up to date" : "Not checked yet";
  }
</script>

<section>
  {#if !status}
    <p class="lo">astromech's background process isn't running, so settings can't be shown. It starts with this window; if this stays, see Diagnostics.</p>
  {:else}
    <h3>Startup</h3>
    <dl>
      <dt>Start with Windows</dt>
      <dd>
        <span>{status.autostart ? "On" : "Off"}</span>
        <button onclick={() => run(status?.autostart ? "autostart_off" : "autostart_on")}>{status.autostart ? "Turn off" : "Turn on"}</button>
      </dd>
      <dt>Setup</dt>
      <dd>
        <span>{status.onboarded === false ? "Not finished" : "Done"}</span>
        <a href="#/setup">Run again</a>
      </dd>
    </dl>
    <p class="hint">Starts quietly in the tray and waits for Elite, so voice is ready when you launch the game.</p>

    <h3>Voice</h3>
    <dl>
      <dt>Push-to-talk</dt>
      <dd class:warn={!status.ptt}>
        <span>{button(status.ptt) ?? "Not set"}</span>
        <a href="#/setup/buttons">Change</a>
      </dd>
      <dt>Kill switch</dt>
      <dd>
        <span>{button(status.kill_switch) ?? "Not set"}</span>
        <a href="#/setup/buttons">Change</a>
      </dd>
      <dt>Speech models</dt>
      <dd class:warn={status.models_missing && !status.models_progress}>
        <span>{status.models_progress ?? (status.models_missing ? "Not downloaded (about 800 MB)" : "Installed")}</span>
        {#if status.models_missing && !status.models_progress}
          <button onclick={() => run("download_models")}>Download</button>
        {/if}
      </dd>
      <dt>Voice</dt>
      <dd>
        <span>{status.voice_paused ? "Paused" : "Listening while Elite runs"}</span>
        <button onclick={() => run(status?.voice_paused ? "resume_voice" : "pause_voice")}>{status.voice_paused ? "Resume" : "Pause"}</button>
      </dd>
    </dl>
    {#if !status.ptt}
      <p class="hint">Push-to-talk isn't set, so voice can't listen. <a href="#/setup">Run setup</a> to pick a button.</p>
    {/if}

    {#if voice}
      <h3>Voice responses</h3>
      <dl>
        <dt>Speaker</dt>
        <dd>
          <select aria-label="Speaker" value={voice.speaker} onchange={(e) => setVoice({ speaker: e.currentTarget.value })}>
            {#each ["British", "American"] as accent (accent)}
              <optgroup label={accent}>
                {#each voice.voices.filter((v) => v.accent === accent) as v (v.name)}
                  <option value={v.name}>{v.label} · {v.gender}</option>
                {/each}
              </optgroup>
            {/each}
          </select>
          <button onclick={preview}>Preview</button>
        </dd>
        <dt>Colour</dt>
        <dd>
          <span class="choices">
            {#each COLOURS as c (c.id)}
              <button class:on={voice.colour === c.id} aria-pressed={voice.colour === c.id} onclick={() => setVoice({ colour: c.id })}>{c.label}</button>
            {/each}
          </span>
        </dd>
        <dt>Speed</dt>
        <dd>
          <select aria-label="Speed" value={voice.speed.toFixed(1)} onchange={(e) => setVoice({ speed: Number(e.currentTarget.value) })}>
            {#each speeds as x (x)}
              <option value={x.toFixed(1)}>{x === 1 ? "Normal (the colour's own pace)" : `${x.toFixed(1)}×`}</option>
            {/each}
          </select>
        </dd>
      </dl>
      <p class="hint">{voiceNote || "Cockpit sounds like the ship's PA, close to Elite's own voice. Changes apply to the next reply."}</p>
    {/if}

    <h3>Microphone</h3>
    <dl>
      <dt>Device</dt>
      <dd class:warn={!!mics && !mics.names.length}>
        <span>{mics ? (mics.setting ? (mics.chosen ?? `${mics.setting} (not connected)`) : `Windows default${mics.default ? ` · ${mics.default}` : ""}`) : "—"}</span>
        <a href="#/setup/mic">Change</a>
      </dd>
      <dt>Calibration</dt>
      <dd class:warn={gate == null}>
        <span>{gate != null ? `Ignores clips quieter than ${gate.toFixed(0)} dBFS` : "Not calibrated"}</span>
        <button onclick={() => (calibrating = true)}>{gate != null ? "Run again" : "Calibrate"}</button>
      </dd>
    </dl>
    <p class="hint">Calibration measures the room and your voice on this mic, so quiet speech isn't dropped and background noise isn't heard as words.</p>

    <h3>Updates</h3>
    <dl>
      <dt>Version</dt>
      <dd>
        <span>{status.version}</span>
        <button onclick={() => openUrl("https://github.com/spadaval/astromech/releases")}>Release notes</button>
      </dd>
      <dt>Updates</dt>
      <dd class:warn={u?.status === "error"}>
        <span>{updateLine()}</span>
        {#if u?.ready && u.status !== "applying"}
          <button onclick={() => run("install_update", "Installing… astromech will close and reopen.")}>Install and restart</button>
        {:else}
          <button disabled={busy} onclick={() => run("check_update")}>Check now</button>
        {/if}
      </dd>
    </dl>
    {#if u?.apply_error}
      <p class="hint warn-text">The last update didn't install: {u.apply_error}. Try again, or run the setup.exe from the release page.</p>
    {:else}
      <p class="hint">Installing closes astromech for a few seconds and reopens it. Elite can keep running.</p>
    {/if}

    <h3>Files</h3>
    <dl>
      <dt>Config file</dt>
      <dd><span class="mono path">{status.config}</span><button onclick={() => run("open_config")}>Open</button></dd>
      <dt>Logs</dt>
      <dd><span class="mono path">{status.log_dir}</span><button onclick={() => status && openUrl(status.log_dir)}>Open folder</button></dd>
    </dl>
    {#if status.config_note}<p class="hint warn-text">{status.config_note}</p>{/if}
  {/if}
  {#if note}<p class="note">{note}</p>{/if}
</section>

{#if calibrating}
  <div class="shade" role="presentation">
    <div class="dlg" role="dialog" aria-modal="true" aria-label="Calibrate the microphone">
      <div class="lbl">Microphone · Calibration</div>
      <h2>{mics?.calibration.device ?? "Microphone"}</h2>
      <Calibration ondone={() => void loadMics()} />
      <div class="close"><button onclick={() => (calibrating = false)}>Close</button></div>
    </div>
  </div>
{/if}

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
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .path { overflow-wrap: anywhere; }
  .hint { color: var(--text-lo); margin: 8px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); line-height: 1.6; }
  .warn-text { color: var(--accent); }
  .lo { color: var(--text-lo); }
  select { all: unset; flex: 1; cursor: pointer; color: var(--text); }
  select:focus-visible { color: var(--text-hi); }
  option, optgroup { background: var(--bg); color: var(--text); }
  .choices { display: flex; gap: 24px; }
  .choices button { color: var(--text-lo); }
  .choices button.on { color: var(--accent); }
  .shade { position: fixed; inset: 0; background: rgb(0 0 0 / 0.7); display: grid; place-items: center; z-index: 10; }
  .dlg { width: 600px; background: var(--bg); border-top: 2px solid var(--accent); border-bottom: 2px solid var(--accent); padding: 22px 28px; }
  .lbl { color: var(--accent); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  h2 { margin: 6px 0 16px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-l); }
  .close { margin-top: 20px; }
  a { color: var(--accent); text-decoration: none; text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); flex: none; }
  .hint a { text-transform: none; letter-spacing: 0; font-size: inherit; }
  a:hover { color: var(--text-hi); }
  button { all: unset; cursor: pointer; flex: none; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
  .note { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); margin-top: 20px; }
</style>
