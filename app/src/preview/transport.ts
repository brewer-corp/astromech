/**
 * The browser preview's transport (lib/transport.ts picks it outside Tauri in `pnpm dev`, the
 * tests and a preview build): every command the window sends and every topic it subscribes to,
 * answered without Tauri, a core or the network. The fake core is fakecore.ts, the command
 * layer's own commands (astromech-api's) are shell.ts and binds.ts, and the URL's options are
 * query.ts. The handlers are typed against the generated command map (recorded.ts).
 */
import { usePreview, type Transport } from "../lib/transport";
import { bindsCommands, onPreviewPress } from "./binds";
import { coreCommands, fakeScenario } from "./fakecore";
import engineeringSample from "./fixtures/engineering.json";
import odysseySample from "./fixtures/odyssey.json";
import { previewQuery } from "./query";
import type { Handlers } from "./recorded";
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

let handlers: Handlers | null = null;

/** Every command the window may send (the generated command map's `window` ones): the command
 * layer's (shell.ts, binds.ts) and the core's own, which the fake core answers (fakecore.ts).
 * Made on first use, so a release build, which never picks this transport, drops the fake. */
export function previewCommands(): Handlers {
  return (handlers ??= { ...shellCommands(publish), ...bindsCommands, ...coreCommands() });
}

/** The topics the window subscribes to. */
export const topics: Record<string, (handler: (payload: unknown) => void) => () => void> = {
  input: onPreviewPress as (handler: (payload: unknown) => void) => () => void,
  // The tray's page requests don't happen in a browser.
  route: quiet,
  neutron_download: pushed("neutron_download"),
};

/** What a remembered choice starts as in the preview, by its localStorage key. */
function storedDefault(name: string): string | null {
  if (name === "astromech.odyssey.picks") return fakeScenario() === "empty" ? null : JSON.stringify(odysseySample.picks);
  if (name === `astromech.engineering.plan.${engineeringSample.plan_input.ship_id}`) return JSON.stringify(engineeringSample.plan_input);
  return null;
}

export const previewTransport: Transport = {
  call(cmd, args) {
    // The wire's command name and args, unchecked: an unknown name is rejected as the core rejects it.
    const f = (previewCommands() as Partial<Record<string, (args: unknown) => unknown>>)[cmd];
    if (!f) return Promise.reject(new Error(`preview: no fake for command ${cmd}`));
    try {
      return Promise.resolve(f(args ?? null));
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

usePreview(previewTransport);
