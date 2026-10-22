<script lang="ts">
  // The neutron star file under Plot with Spansh → Neutron (docs/guide/routes.md): whether it is
  // in the data folder and its build date, Download with progress, and Check for a newer file.
  // Nothing downloads until the commander presses Download.
  import { DISK_MB, DOWNLOAD_MB, checkNeutron, downloadNeutron, neutronFile, watchDownload, type NeutronFile, type NeutronProgress, type Watch } from "../../lib/neutron";

  let status = $state<NeutronFile | null>(null);
  let downloading = $state(false);
  let progress = $state<NeutronProgress | null>(null);
  let err = $state<string | null>(null);
  let checking = $state(false);
  let newer = $state<boolean | null>(null);
  let stopWatching: (() => void) | null = null;

  const message = (e: unknown) => String(e instanceof Error ? e.message : e);

  async function follow(w: Watch) {
    stopWatching?.();
    stopWatching = w.stop;
    downloading = true;
    progress = w.progress;
    try {
      status = await w.promise;
      newer = null;
    } catch (e) {
      err = message(e);
    } finally {
      w.stop();
      downloading = false;
      progress = null;
    }
  }

  $effect(() => {
    neutronFile()
      .then((s) => (status = s))
      .catch((e) => (err = message(e)));
    // A download started before this page was opened is still running in the shell.
    const w = watchDownload((p) => (progress = p));
    if (w) void follow(w);
    return () => stopWatching?.();
  });

  function download() {
    err = null;
    void follow(downloadNeutron((p) => (progress = p)));
  }

  async function check() {
    checking = true;
    err = null;
    try {
      newer = (await checkNeutron()).newer;
    } catch (e) {
      err = message(e);
    } finally {
      checking = false;
    }
  }

  const mb = (n: number) => Math.round(n / 1_000_000).toLocaleString("en-GB");
  const date = (t: number) => new Date(t * 1000).toLocaleDateString("en-GB", { day: "numeric", month: "short", year: "numeric", timeZone: "UTC" });
  const file = $derived(status?.file ?? null);
</script>

<div class="neutron" aria-label="Neutron star file">
  {#if downloading}
    <p>
      Downloading the neutron star file…
      {#if progress?.total}{mb(progress.done)} of {mb(progress.total)} MB{:else if progress}{mb(progress.done)} MB{/if}
    </p>
    <progress max={progress?.total ?? undefined} value={progress?.total ? progress.done : undefined}></progress>
  {:else if file}
    <p>Neutron star file: {file.stars.toLocaleString("en-GB")} neutron stars, built {date(file.built)}.</p>
    {#if newer === true}
      <p class="lo">A newer file is available.</p>
      <button onclick={download}>Download the newer file</button>
    {:else}
      {#if newer === false}<p class="lo">This is the latest file.</p>{/if}
      <button onclick={() => void check()} disabled={checking}>{checking ? "Checking…" : "Check for a newer file"}</button>
    {/if}
  {:else if status}
    <p>
      Neutron star file: not downloaded. It lists every known neutron star, for plotting neutron
      routes on this PC: about {DOWNLOAD_MB} MB to download, {DISK_MB} MB in astromech's data folder.
    </p>
    {#if status.error}<p class="lo">The file there can't be read: {status.error}</p>{/if}
    <button onclick={download}>Download</button>
  {/if}
  {#if status || downloading}<p class="lo">Neutron routes still come from Spansh for now.</p>{/if}
  {#if err}<p class="err">{err}</p>{/if}
</div>

<style>
  .neutron { display: flex; flex-direction: column; gap: 4px; margin: 4px 0 6px; font-size: var(--fs-s); }
  p { margin: 0; }
  .lo { color: var(--text-lo); }
  button { all: unset; cursor: pointer; align-self: flex-start; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
  progress { width: 100%; max-width: 24rem; height: 6px; accent-color: var(--accent); }
  .err { color: var(--text-hi); background: var(--accent); padding: 8px 12px; }
</style>
