/**
 * The window's one way to its backend. `call` sends a command and `subscribe` takes what the
 * backend pushes. In the app (Tauri) a shell command is an `invoke`, a core command goes through
 * the shell's `core_call`, and pushes are Tauri events ("input", "route", "core-restarted") or a
 * command's progress channel ("neutron_progress"). Outside Tauri, in `pnpm dev`, the tests and a
 * preview build (scripts/preview-publish sets VITE_PREVIEW), the browser preview's fake answers
 * (app/src/preview/). A plain `pnpm build` sets neither, so the fake is dead code and dropped.
 */
import { Channel, invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { previewTransport } from "../preview/transport";

/** A command's arguments; a string is a core command's text (the voice tester's phrase). */
export type Args = Record<string, unknown> | string | null;

export interface Transport {
  call(cmd: string, args?: Args): Promise<unknown>;
  /** Returns the unsubscribe. */
  subscribe(topic: string, handler: (payload: unknown) => void): () => void;
  /** A browser preview's page option (`?q=`, `?device=`) or default for a remembered choice;
   * always null in the app. */
  option(name: string): string | null;
}

/** Commands the window may send to the core; the shell's `core_call` allows exactly these. */
export const CORE_COMMANDS = [
  "save_diagnostics",
  "ack_crash",
  "check_update",
  "install_update",
  "autostart_on",
  "autostart_off",
  "pause_voice",
  "resume_voice",
  "kill_switch",
  "download_models",
  "open_config",
  "eddn_log",
  "set_eddn",
  "edsm_status",
  "set_edsm",
  "edsm_flight_log",
  "voice_test",
  "voice_test_poll",
  "voice_test_stop",
  "voice_test_live",
  "learned_phrases",
  "learned_forget",
  // Onboarding and Settings → Microphone (core: setup.rs, calibration.rs).
  "find_elite",
  "set_ptt",
  "set_kill_switch",
  "set_mic",
  "set_folders",
  "mic_list",
  "mic_check",
  "calibrate_start",
  "calibrate_record",
  "calibrate_finish",
  "calibrate_cancel",
  "onboarding_done",
  // Settings → Voice responses.
  "set_voice",
  "voice_settings",
  "voice_preview",
  // Settings → Callouts: each callout's switch and cooldown (core: callouts.rs, setup.rs).
  "callouts",
  "set_callout",
  // Settings → Prospector callouts (core: callouts.rs).
  "prospector_thresholds",
  "prospector_set",
  // Engineering → the arrival briefing's shopping list (core: callouts.rs).
  "shopping_set",
  // Exploration → Biology: the system's biology and the sampling in progress (core: biology.rs).
  "exobiology",
  // Galaxy → Route: the route the core follows (core: route.rs, docs/PLOTTING.md).
  "route",
  "route_set",
  "route_add",
  "route_skip",
  "route_back",
  "route_clear",
  "route_copy",
  "route_resume",
] as const;
export type CoreCmd = (typeof CORE_COMMANDS)[number];
const core = new Set<string>(CORE_COMMANDS);

/** Shell commands that report progress on a channel, and the topic it is published as. */
const CHANNELS: Record<string, { arg: string; topic: string }> = {
  neutron_download: { arg: "onProgress", topic: "neutron_progress" },
};

const local = new Map<string, Set<(payload: unknown) => void>>();
const publish = (topic: string, payload: unknown) => {
  for (const h of local.get(topic) ?? []) h(payload);
};

const tauri: Transport = {
  call(cmd, args) {
    if (core.has(cmd)) {
      const text = typeof args === "string" ? args : undefined;
      return invoke("core_call", { cmd, text, args: typeof args === "string" ? null : (args ?? null) });
    }
    const ch = CHANNELS[cmd];
    if (ch) {
      const c = new Channel<unknown>((p) => publish(ch.topic, p));
      return invoke(cmd, { ...(args as Record<string, unknown> | null), [ch.arg]: c });
    }
    return invoke(cmd, args as Record<string, unknown> | undefined);
  },
  subscribe(topic, handler) {
    if (Object.values(CHANNELS).some((c) => c.topic === topic)) {
      const set = local.get(topic) ?? new Set();
      local.set(topic, set.add(handler));
      return () => set.delete(handler);
    }
    let un: (() => void) | undefined;
    let dead = false;
    void listen(topic, (e) => handler(e.payload)).then((u) => (dead ? u() : (un = u)));
    return () => {
      dead = true;
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
