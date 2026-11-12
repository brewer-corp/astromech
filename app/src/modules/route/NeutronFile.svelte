<script lang="ts">
  // The neutron star file, under Worked out in the plot setup (PlotSetup.svelte,
  // docs/guide/routes.md): whether it is in the data folder and its build date, Download with
  // progress, and Check for a newer file. Nothing downloads until the commander presses Download.
  // Its status lives in the shared plot setup (lib/plot.svelte.ts), so On this PC can be picked
  // once it is there.
  import Button from "../../lib/ui/Button.svelte";
  import { DISK_MB, DOWNLOAD_MB, checkNeutron, downloadNeutron, watchDownload, type NeutronProgress, type Watch } from "../../lib/neutron";
  import { setup } from "../../lib/plot.svelte";

  let downloading = $state(false);
  let progress = $state<NeutronProgress | null>(null);
  let err = $state<string | null>(null);
  let checking = $state(false);
  let newer = $state<boolean | null>(null);
  let stopWatching: (() => void) | null = null;

  const message = (e: unknown) => String(e instanceof Error ? e.message : e);
  const status = $derived(setup.file);

  async function follow(w: Watch) {
    stopWatching?.();
    stopWatching = w.stop;
    downloading = true;
    progress = w.progress;
    try {
      setup.file = await w.promise;
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
    void setup.readFile();
  });

  $effect(() => {
    // A download started before this was opened is still running in the core.
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

<div class="neutron type-label" aria-label="Neutron star file">
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
      <Button class="act" onclick={download}>Download the newer file</Button>
    {:else}
      {#if newer === false}<p class="lo">This is the latest file.</p>{/if}
      <Button class="act" onclick={() => void check()} disabled={checking}>{checking ? "Checking…" : "Check for a newer file"}</Button>
    {/if}
  {:else if status}
    <p>
      Neutron star file: not downloaded. It lists every known neutron star, for plotting neutron
      routes on this PC: about {DOWNLOAD_MB} MB to download, {DISK_MB} MB in astromech's data folder.
    </p>
    {#if status.error}<p class="lo">The file there can't be read: {status.error}</p>{/if}
    <Button class="act" onclick={download}>Download</Button>
  {/if}
  {#if !file && (status || downloading)}
    <p class="lo">Until it is downloaded, Spansh plots neutron routes.</p>
  {/if}
  {#if err ?? setup.fileError}<p class="err" role="alert"><b>!</b> {err ?? setup.fileError}</p>{/if}
</div>

<style>
  /* The Label role sets the size; the text keeps the dropdown's colour. */
  .neutron { display: flex; flex-direction: column; gap: var(--sp-1); color: inherit; }
  p { margin: 0; }
  .lo { color: var(--prose); }
  .neutron > :global(.act) { align-self: flex-start; margin-left: calc(-1 * var(--sp-2)); }
  progress { width: 100%; height: 6px; accent-color: var(--accent); }
  .err { color: var(--alarm); }
  .err b { font-weight: 600; }
</style>
