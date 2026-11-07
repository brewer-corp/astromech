/**
 * The window's one way to its backend. `call` sends a command and `subscribe` takes what the
 * backend pushes. In the app (Tauri) every command goes through the shell's one proxy, `call`,
 * to the core's command registry, and pushes are Tauri events ("input", "route",
 * "core-restarted") or a command's progress, published on the command's own name
 * ("neutron_download"). Outside Tauri, in `pnpm dev`, the tests and a
 * preview build (scripts/preview-publish sets VITE_PREVIEW), the browser preview's fake answers
 * (app/src/preview/). A plain `pnpm build` sets neither, so the fake is dead code and dropped.
 */
import { Channel, invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { previewTransport } from "../preview/transport";
import type { Commands } from "./api/commands";

/** A command's arguments; a string is the request's text (the voice tester's phrase), which the
 * shell's proxy sends as such. */
export type Args = Record<string, unknown> | string | null;

export interface Transport {
  call(cmd: string, args?: Args): Promise<unknown>;
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

const tauri: Transport = {
  call(cmd, args) {
    const progress = new Channel<unknown>((p) => publish(cmd, p));
    return invoke("call", { cmd, args: args ?? null, progress });
  },
  subscribe(topic, handler) {
    // A command's progress (published above) or one of the shell's events.
    const set = local.get(topic) ?? new Set();
    local.set(topic, set.add(handler));
    let un: (() => void) | undefined;
    let dead = false;
    void listen(topic, (e) => handler(e.payload)).then((u) => (dead ? u() : (un = u)));
    return () => {
      dead = true;
      set.delete(handler);
      un?.();
    };
  },
  option: () => null,
};

const inTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

// Picked on first use, not at load: the preview's modules import this one back.
let current: Transport | null = null;
const transport = () => (current ??= !inTauri && (import.meta.env.DEV || import.meta.env.VITE_PREVIEW) ? previewTransport : tauri);

/** Answer the window's calls with `t` from now on (tests). */
export function setTransport(t: Transport): void {
  current = t;
}

export function call<T>(cmd: string, args?: Args): Promise<T> {
  return transport().call(cmd, args) as Promise<T>;
}

export function subscribe<T>(topic: string, handler: (payload: T) => void): () => void {
  return transport().subscribe(topic, handler as (payload: unknown) => void);
}

/** See {@link Transport.option}. */
export function previewOption(name: string): string | null {
  return transport().option(name);
}
