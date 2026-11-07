/**
 * The neutron star file (astromech_galaxy::neutron): every known neutron star, downloaded once
 * into the data folder from the `neutron` pre-release when the commander presses Download (the
 * core's `neutron_file`, `neutron_download`, `neutron_check`; the download's progress is
 * published on the "neutron_download" topic). The browser preview pretends: no file until Download, then a short
 * fake download (app/src/preview/shell.ts).
 */
import type { NeutronCheck } from "./api/types/NeutronCheck";
import type { NeutronFile } from "./api/types/NeutronFile";
import { call, subscribe, type Progress } from "./transport";

export type { Header as NeutronHeader } from "./api/types/Header";
export type { NeutronFile } from "./api/types/NeutronFile";

/** `done` bytes of `total` (null when the server didn't say). */
export type NeutronProgress = Progress<"neutron_download">;

/** The published file's size, for the copy before a download (22 Oct 2026 build). */
export const DOWNLOAD_MB = 71;
export const DISK_MB = 184;

export async function neutronFile(): Promise<NeutronFile> {
  return await call("neutron_file");
}

/** Whether a different file has been published since this one was downloaded. */
export async function checkNeutron(): Promise<NeutronCheck> {
  return await call("neutron_check");
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
  const stop = subscribe<NeutronProgress>("neutron_download", report);
  try {
    return await call("neutron_download");
  } finally {
    stop();
  }
}

/** Forget the download in flight (tests). */
export function resetNeutronDownload(): void {
  running = null;
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
    const r: Running = { promise: Promise.resolve({ file: null, error: null }), progress: null, listeners: new Set() };
    running = r;
    r.promise = fetchFile().finally(() => {
      if (running === r) running = null;
    });
  }
  return watchDownload(onProgress) as Watch;
}
