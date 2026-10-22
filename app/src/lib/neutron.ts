/**
 * The neutron star file (astromech_galaxy::neutron): every known neutron star, downloaded once
 * into the data folder from the `neutron` pre-release when the commander presses Download (the
 * shell's `neutron_file`, `neutron_download`, `neutron_check`). The browser preview pretends: no
 * file until Download, then a short fake download.
 */
import { Channel, invoke } from "@tauri-apps/api/core";
import { inTauri } from "./core";
import { previewGate } from "./fakecore";

/** The file's header. */
export interface NeutronHeader {
  cell_ly: number;
  stars: number;
  cells: number;
  /** Build time, Unix seconds UTC. */
  built: number;
}

export interface NeutronFile {
  /** Null when there is no file (or it can't be read: see `error`). */
  file: NeutronHeader | null;
  error: string | null;
}

export interface NeutronProgress {
  done: number;
  /** Null when the server didn't say. */
  total: number | null;
}

/** The published file's size, for the copy before a download (22 Oct 2026 build). */
export const DOWNLOAD_MB = 71;
export const DISK_MB = 184;

const PREVIEW_FILE: NeutronHeader = { cell_ly: 128, stars: 5045154, cells: 1645100, built: 1792627200 };
let preview: NeutronFile = { file: null, error: null };

/** The browser preview has "downloaded" the file: its neutron routes are plotted "on this PC". */
export function previewHasNeutronFile(): boolean {
  return preview.file != null;
}

/** Back to "not downloaded" (tests). */
export function resetPreviewNeutron(): void {
  preview = { file: null, error: null };
  running = null;
}

export async function neutronFile(): Promise<NeutronFile> {
  if (inTauri) return await invoke<NeutronFile>("neutron_file");
  await previewGate("neutron_file");
  return preview;
}

/** Whether a different file has been published since this one was downloaded. */
export async function checkNeutron(): Promise<{ newer: boolean }> {
  if (inTauri) return await invoke<{ newer: boolean }>("neutron_check");
  await previewGate("neutron_check");
  return { newer: false };
}

/** The download in flight, shared by every view of it so leaving the page doesn't lose it. */
interface Running {
  promise: Promise<NeutronFile>;
  progress: NeutronProgress | null;
  listeners: Set<(p: NeutronProgress) => void>;
}
let running: Running | null = null;

function report(p: NeutronProgress) {
  if (!running) return;
  running.progress = p;
  for (const f of running.listeners) f(p);
}

async function fetchFile(): Promise<NeutronFile> {
  if (inTauri) {
    const onProgress = new Channel<NeutronProgress>(report);
    return await invoke<NeutronFile>("neutron_download", { onProgress });
  }
  await previewGate("neutron_download");
  const total = DOWNLOAD_MB * 1_000_000;
  for (let i = 1; i <= 4; i++) {
    await new Promise((r) => setTimeout(r, 40));
    report({ done: (total * i) / 4, total });
  }
  preview = { file: PREVIEW_FILE, error: null };
  return preview;
}

export interface Watch {
  promise: Promise<NeutronFile>;
  /** Progress so far. */
  progress: NeutronProgress | null;
  /** Stop calling `onProgress`. */
  stop: () => void;
}

/** The download in flight, if any, calling `onProgress` as it goes. */
export function watchDownload(onProgress: (p: NeutronProgress) => void): Watch | null {
  const r = running;
  if (!r) return null;
  r.listeners.add(onProgress);
  return { promise: r.promise, progress: r.progress, stop: () => r.listeners.delete(onProgress) };
}

/** Starts the download, or joins the one already running. */
export function downloadNeutron(onProgress: (p: NeutronProgress) => void): Watch {
  if (!running) {
    const r: Running = { promise: Promise.resolve(preview), progress: null, listeners: new Set() };
    running = r;
    r.promise = fetchFile().finally(() => {
      if (running === r) running = null;
    });
  }
  return watchDownload(onProgress) as Watch;
}
