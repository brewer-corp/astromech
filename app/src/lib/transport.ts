/**
 * The window's one way to its backend. `call` sends a command and `subscribe` takes what the
 * backend pushes. In the app (Tauri) every command goes through the window's one Tauri command,
 * `call`, into the core's command registry in the same process, and pushes are Tauri events
 * ("input", "route") or a command's progress, published on the command's own name
 * ("neutron_download"). Outside Tauri, in `pnpm dev`, the tests and a
 * preview build (scripts/preview-publish sets VITE_PREVIEW), the browser preview's fake answers
 * (app/src/preview/): main.ts and the tests' setup files load it, and it registers itself here
 * (`usePreview`). This module never imports it: the preview imports the app's lib modules, so
 * that would be an import cycle, and a module imported while the cycle loaded could be handed
 * this one half-loaded. A plain `pnpm build` loads no preview, so the fake is dropped.
 */
import type { Commands } from "./api/commands";

/** The commands the window may send (exposure `window`); a `local` one (`quit`) doesn't typecheck. */
export type WindowCommand = { [K in keyof Commands]: Commands[K]["exposure"] extends "window" ? K : never }[keyof Commands];
/** Command `K`'s arguments (`null`: none). A string is the request's text (the voice tester's
 * phrase), which the shell's proxy sends as such. */
export type Args<K extends keyof Commands> = Commands[K]["args"];
/** Command `K`'s reply (the envelope's `value`). */
export type Reply<K extends keyof Commands> = Commands[K]["reply"];
/** `call`'s arguments after the command: left out only for a command that takes none (args
 * `null`); a command whose args may be null (`set_ptt` clears with `null`) still needs them. */
export type ArgList<K extends keyof Commands> = [Args<K>] extends [null] ? [args?: null] : [args: Args<K>];

/** A reply as the window shows it: the browser preview labels its samples ("Sample journal
 * history"), which the page shows beside them; the core never sends `source` for these. */
export type Sampled<T> = T & { source?: string };

/** The wire as each transport carries it; `call` below gives it the command map's types. */
export interface Transport {
  call(cmd: string, args?: unknown): Promise<unknown>;
  /** Returns the unsubscribe. */
  subscribe(topic: string, handler: (payload: unknown) => void): () => void;
  /** A browser preview's page option (`?q=`, `?device=`) or default for a remembered choice;
   * always null in the app. */
  option(name: string): string | null;
}

/** What command `C` reports before its reply (the generated map's `progress`), published on the
 * topic named after the command: `subscribe<Progress<"neutron_download">>("neutron_download", …)`. */
export type Progress<C extends keyof Commands> = Commands[C] extends { progress: infer P } ? P : never;

const local = new Map<string, Set<(payload: unknown) => void>>();
const publish = (topic: string, payload: unknown) => {
  for (const h of local.get(topic) ?? []) h(payload);
};

// Tauri's API loads on the first call or subscription, not with this module: the tests' setup
// loads this module before a test that fakes Tauri replaces that API (vi.mock).
const tauri: Transport = {
  async call(cmd, args) {
    const { Channel, invoke } = await import("@tauri-apps/api/core");
    const progress = new Channel<unknown>((p) => publish(cmd, p));
    return invoke("call", { cmd, args: args ?? null, progress });
  },
  subscribe(topic, handler) {
    // A command's progress (published above) or one of the shell's events.
    const set = local.get(topic) ?? new Set();
    local.set(topic, set.add(handler));
    let un: (() => void) | undefined;
    let dead = false;
    void import("@tauri-apps/api/event")
      .then(({ listen }) => listen(topic, (e) => handler(e.payload)))
      .then((u) => (dead ? u() : (un = u)));
    return () => {
      dead = true;
      set.delete(handler);
      un?.();
    };
  },
  option: () => null,
};

/** The window runs in the app (Tauri), not the browser preview. */
export const inTauri = (): boolean => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

let current: Transport | null = null;
let preview: Transport | null = null;
const transport = () => current ?? (!inTauri() && preview ? preview : tauri);

/** The browser preview's fake answers outside Tauri (app/src/preview/transport.ts, when it loads). */
export function usePreview(t: Transport): void {
  preview = t;
}

/** Answer the window's calls with `t` from now on (tests). */
export function setTransport(t: Transport): void {
  current = t;
}

/** Send `cmd` with its arguments; the reply is the core's, typed by the generated command map. */
export function call<K extends WindowCommand>(cmd: K, ...[args]: ArgList<K>): Promise<Reply<K>> {
  return transport().call(cmd, args) as Promise<Reply<K>>;
}

export function subscribe<T>(topic: string, handler: (payload: T) => void): () => void {
  return transport().subscribe(topic, handler as (payload: unknown) => void);
}

/** See {@link Transport.option}. */
export function previewOption(name: string): string | null {
  return transport().option(name);
}
