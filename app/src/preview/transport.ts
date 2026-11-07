/**
 * The browser preview's transport (lib/transport.ts picks it outside Tauri in `pnpm dev`, the
 * tests and a preview build): every command the window sends and every topic it subscribes to,
 * answered without Tauri, a core or the network. The fake core is fakecore.ts, the shell's
 * commands are shell.ts and binds.ts, and the URL's options are query.ts.
 */
import { CORE_COMMANDS, type Args, type CoreCmd, type Transport } from "../lib/transport";
import { bindsCommands, onPreviewPress } from "./binds";
import { coreReply, fakeCoreStatus, fakeScenario } from "./fakecore";
import engineeringSample from "./fixtures/engineering.json";
import odysseySample from "./fixtures/odyssey.json";
import { previewQuery } from "./query";
import { shellCommands } from "./shell";

const local = new Map<string, Set<(payload: unknown) => void>>();
const publish = (topic: string, payload: unknown) => {
  for (const h of local.get(topic) ?? []) h(payload);
};
const pushed = (topic: string) => (handler: (payload: unknown) => void) => {
  const set = local.get(topic) ?? new Set();
  local.set(topic, set.add(handler));
  return () => void set.delete(handler);
};
/** A topic the preview never pushes on. */
const quiet = () => () => {};

/** The shell's commands by name (core commands go to the fake core). */
export const commands: Record<string, (args: Record<string, unknown>) => unknown> = {
  core_status: () => fakeCoreStatus(),
  ...shellCommands(publish),
  ...bindsCommands,
};

/** The topics the window subscribes to. */
export const topics: Record<string, (handler: (payload: unknown) => void) => () => void> = {
  input: onPreviewPress as (handler: (payload: unknown) => void) => () => void,
  // The tray's page requests and the shell restarting the core don't happen in a browser.
  route: quiet,
  "core-restarted": quiet,
  neutron_progress: pushed("neutron_progress"),
};

/** What a remembered choice starts as in the preview, by its localStorage key. */
function storedDefault(name: string): string | null {
  if (name === "astromech.odyssey.picks") return fakeScenario() === "empty" ? null : JSON.stringify(odysseySample.picks);
  if (name === `astromech.engineering.plan.${engineeringSample.plan_input.ship_id}`) return JSON.stringify(engineeringSample.plan_input);
  return null;
}

export const previewTransport: Transport = {
  call(cmd: string, args?: Args) {
    // Not a set made at load: this module and lib/transport.ts import each other.
    if ((CORE_COMMANDS as readonly string[]).includes(cmd)) return coreReply(cmd as CoreCmd, args);
    const f = commands[cmd];
    if (!f) return Promise.reject(new Error(`preview: no fake for shell command ${cmd}`));
    try {
      return Promise.resolve(f((args ?? {}) as Record<string, unknown>));
    } catch (e) {
      return Promise.reject(e);
    }
  },
  subscribe(topic, handler) {
    const f = topics[topic];
    if (!f) throw new Error(`preview: no fake for topic ${topic}`);
    return f(handler);
  },
  option: (name) => previewQuery().get(name) ?? storedDefault(name),
};
