<script lang="ts">
  // Settings (docs/V1-DESIGN.md §2.3): one list, label left, value right, actions on the row.
  // Everything the tray used to carry lives here now. Push-to-talk, the kill switch and the mic
  // are picked in setup (onboarding); calibration's "Run again" opens just that step.
  import { coreCall, coreStatus, openUrl, type BareCoreCmd, type CoreStatus, type MicList, type VoiceSettings, type Colour } from "../../lib/core";
  import { control } from "../../lib/format";
  import Button from "../../lib/ui/Button.svelte";
  import Calibration from "../../lib/ui/Calibration.svelte";
  import Choice from "../../lib/ui/Choice.svelte";
  import Dropdown from "../../lib/ui/Dropdown.svelte";
  import ProspectorThresholds from "./ProspectorThresholds.svelte";
  import Callouts from "./Callouts.svelte";
  import Stale from "../../lib/ui/Stale.svelte";
  import { pollFailure } from "../../lib/ui/errors";

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
  // A hand-edited speed (0.94, 2.0) shows as the nearest step the picker offers.
  const shownSpeed = $derived(voice ? Math.min(voice.speeds[1], Math.max(voice.speeds[0], Math.round(voice.speed * 10) / 10)).toFixed(1) : "");
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

  /** A status read failed after a good one: the last status stays, with a line saying so. A failed
   * read is null (coreStatus), so the window can't tell a stopped core from one that didn't answer. */
  let statusErr = $state<string | null>(null);
  async function refresh() {
    const s = await coreStatus();
    if (s) {
      status = s;
      statusErr = null;
    } else if (status) statusErr = pollFailure("read the status", "the core did not answer", true);
  }
  $effect(() => {
    void refresh();
    const t = setInterval(() => void refresh(), 1000);
    return () => clearInterval(t);
  });

  async function run(cmd: BareCoreCmd, doing = "") {
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
    <p class="lo">astromech didn't answer, so settings can't be shown. If this stays, see Diagnostics.</p>
  {:else}
    <Stale error={statusErr} />
    <h3 class="section">Startup</h3>
    <dl>
      <dt>Start with Windows</dt>
      <dd class="bare">
        <Choice
          label="Start with Windows"
          value={status.autostart ? "on" : "off"}
          options={[{ value: "on", label: "On" }, { value: "off", label: "Off" }]}
          onchange={(v) => {
            if ((v === "on") !== status?.autostart) void run(v === "on" ? "autostart_on" : "autostart_off");
          }}
        />
      </dd>
      <dt>Setup</dt>
      <dd class="value">
        <span>{status.onboarded === false ? "Not finished" : "Done"}</span>
        <Button onclick={() => (location.hash = "#/setup")}>Run again</Button>
      </dd>
    </dl>
    <p class="hint">Starts quietly in the tray and waits for Elite, so voice is ready when you launch the game.</p>

    <h3 class="section">Voice</h3>
    <dl>
      <dt>Push-to-talk</dt>
      <dd class="value" class:warn={!status.ptt}>
        <span>{button(status.ptt) ?? "Not set"}</span>
        <Button onclick={() => (location.hash = "#/setup/buttons")}>Change</Button>
      </dd>
      <dt>Kill switch</dt>
      <dd class="value">
        <span>{button(status.kill_switch) ?? "Not set"}</span>
        <Button onclick={() => (location.hash = "#/setup/buttons")}>Change</Button>
      </dd>
      <dt>Speech models</dt>
      <dd class:value={status.models_missing && !status.models_progress} class:warn={status.models_missing && !status.models_progress}>
        <span>{status.models_progress ?? (status.models_missing ? "Not downloaded (about 800 MB)" : "Installed")}</span>
        {#if status.models_missing && !status.models_progress}
          <Button onclick={() => run("download_models")}>Download</Button>
        {/if}
      </dd>
      <dt>Voice</dt>
      <dd class="value">
        <span>{status.voice_paused ? "Paused" : "Listening while Elite runs"}</span>
        <Button onclick={() => run(status?.voice_paused ? "resume_voice" : "pause_voice")}>{status.voice_paused ? "Resume" : "Pause"}</Button>
      </dd>
    </dl>
    {#if !status.ptt}
      <p class="hint">Push-to-talk isn't set, so voice can't listen. <a href="#/setup">Run setup</a> to pick a button.</p>
    {/if}

    {#if voice}
      <h3 class="section">Voice responses</h3>
      <dl>
        <dt>Speaker</dt>
        <dd class="bare">
          <Dropdown class="grow" aria-label="Speaker" value={voice.speaker} onchange={(e) => setVoice({ speaker: e.currentTarget.value })}>
            {#each ["British", "American"] as accent (accent)}
              <optgroup label={accent}>
                {#each voice.voices.filter((v) => v.accent === accent) as v (v.name)}
                  <option value={v.name}>{v.label} · {v.gender}</option>
                {/each}
              </optgroup>
            {/each}
            {#snippet end()}<Button onclick={preview}>Preview</Button>{/snippet}
          </Dropdown>
        </dd>
        <dt>Colour</dt>
        <dd class="bare">
          <Choice label="Colour" value={voice.colour} options={COLOURS.map((c) => ({ value: c.id, label: c.label }))} onchange={(v) => setVoice({ colour: v })} />
        </dd>
        <dt>Speed</dt>
        <dd class="bare">
          <Dropdown class="grow" aria-label="Speed" value={shownSpeed} onchange={(e) => setVoice({ speed: Number(e.currentTarget.value) })}>
            {#each speeds as x (x)}
              <option value={x.toFixed(1)}>{x === 1 ? "Normal (the colour's own pace)" : `${x.toFixed(1)}×`}</option>
            {/each}
          </Dropdown>
        </dd>
      </dl>
      <p class="hint">{voiceNote || "Cockpit sounds like the ship's PA, close to Elite's own voice. Changes apply to the next reply."}</p>
    {/if}

    <h3 class="section">Microphone</h3>
    <dl>
      <dt>Device</dt>
      <dd class="value" class:warn={!!mics && !mics.names.length}>
        <span>{mics ? (mics.setting ? (mics.chosen ?? `${mics.setting} (not connected)`) : `Windows default${mics.default ? ` · ${mics.default}` : ""}`) : "—"}</span>
        <Button onclick={() => (location.hash = "#/setup/mic")}>Change</Button>
      </dd>
      <dt>Calibration</dt>
      <dd class="value" class:warn={gate == null}>
        <span>{gate != null ? `Ignores clips quieter than ${gate.toFixed(0)} dBFS` : "Not calibrated"}</span>
        <Button onclick={() => (calibrating = true)}>{gate != null ? "Run again" : "Calibrate"}</Button>
      </dd>
    </dl>
    <p class="hint">Calibration measures the room and your voice on this mic, so quiet speech isn't dropped and background noise isn't heard as words.</p>
    <h3 class="section">Callouts</h3>
    <Callouts />
    <p class="hint">Spoken lines astromech says on its own as you play. Off means never; the cooldown is how long one stays silent after it speaks. Changes apply from the next session.</p>

    <h3 class="section">Prospector callouts</h3>
    <ProspectorThresholds />
    <p class="hint">A prospected rock with at least this much of an ore is read out ("Platinum 34%."). Leave a box empty to turn that ore off. A motherlode is always read out.</p>

    <h3 class="section">Updates</h3>
    <dl>
      <dt>Version</dt>
      <dd>
        <span>{status.version}</span>
        <Button onclick={() => openUrl("https://github.com/brewer-corp/astromech/releases")}>Release notes</Button>
      </dd>
      <dt>Updates</dt>
      <dd class="value" class:warn={u?.status === "error"}>
        <span>{updateLine()}</span>
        {#if u?.ready && u.status !== "applying"}
          <Button onclick={() => run("install_update", "Installing… astromech will close and reopen.")}>Install and restart</Button>
        {:else}
          <Button disabled={busy} onclick={() => run("check_update")}>Check now</Button>
        {/if}
      </dd>
    </dl>
    {#if u?.apply_error}
      <p class="hint warn-text">The last update didn't install: {u.apply_error}. Try again, or run the setup.exe from the release page.</p>
    {:else}
      <p class="hint">Installing closes astromech for a few seconds and reopens it. Elite can keep running.</p>
    {/if}

    <h3 class="section">Files</h3>
    <dl>
      <dt>Config file</dt>
      <dd><span class="path">{status.config}</span><Button onclick={() => run("open_config")}>Open</Button></dd>
      <dt>Logs</dt>
      <dd><span class="path">{status.log_dir}</span><Button onclick={() => status && openUrl(status.log_dir)}>Open folder</Button></dd>
    </dl>
    {#if status.config_note}<p class="hint warn-text">{status.config_note}</p>{/if}
  {/if}
  {#if note}<p class="note err" role="alert">{note}</p>{/if}
</section>

{#if calibrating}
  <div class="shade" role="presentation">
    <div class="dlg" role="dialog" aria-modal="true" aria-label="Calibrate the microphone">
      <div class="lbl">Microphone · Calibration</div>
      <h2>{mics?.calibration.device ?? "Microphone"}</h2>
      <Calibration primary ondone={() => void loadMics()} />
      <div class="close"><Button onclick={() => (calibrating = false)}>Close</Button></div>
    </div>
  </div>
{/if}

<style>
  section { padding: 28px 0; max-width: 60rem; }
  h3 { margin: 28px 0 10px; }
  h3:first-child { margin-top: 0; }
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--options-grey); }
  /* DESIGN.md Settings row: a value the commander changes with its action word sits in an ember
     cell (.value); a status with no such action is a fact, outside a cell. */
  dd { color: var(--accent); gap: 24px; justify-content: space-between; }
  dd.value { background: var(--cell); }
  /* A row that needs the commander: an orange "!" before its words (DESIGN.md States). */
  dd.warn > :first-child::before { content: "! "; font-weight: 600; }
  dd > span { min-width: 0; }
  .path { font-size: var(--fs-s); overflow-wrap: anywhere; }
  .hint { color: var(--prose); margin: 8px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); line-height: 1.6; }
  .warn-text { color: var(--accent); }
  .warn-text::before { content: "! "; font-weight: 600; }
  .lo { color: var(--prose); }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }
  /* A row whose value is its own control (a choice or a dropdown) has no cell around it. */
  dd.bare { padding: 0; background: none; }
  dd.bare > :global(.grow) { flex: 1; }
  .shade { position: fixed; inset: 0; background: rgb(0 0 0 / 0.7); display: grid; place-items: center; z-index: 10; }
  .dlg { width: 600px; background: var(--bg); border-top: 2px solid var(--accent); border-bottom: 2px solid var(--accent); padding: 22px 28px; }
  .lbl { color: var(--orange-mid); font-size: var(--fs-s); }
  h2 { margin: 6px 0 16px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-l); }
  .close { margin-top: 20px; }
  .hint a { color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .note { font-size: var(--fs-s); margin-top: 20px; }
</style>
