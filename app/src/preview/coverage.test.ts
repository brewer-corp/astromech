import { describe, expect, it } from "vitest";
import { CORE_COMMANDS } from "../lib/core";
import { coreHandlers } from "./fakecore";
import { commands, topics } from "./transport";

// Every command and topic the window uses has a fake, so no page of the browser preview (or a
// test behind the transport) meets a call the preview can't answer. The app's own sources (not
// tests, not the preview) are read for the names it passes to call, coreCall and subscribe.
const sources = import.meta.glob<string>(["../**/*.ts", "../**/*.svelte", "!../**/*.test.ts", "!./**"], {
  query: "?raw",
  import: "default",
  eager: true,
});

const CALL = /\b(?:call|coreCall|routeCall)(?:<[^()]*?>)?\(\s*"([a-z_][a-z0-9_-]*)"/g;
const SUBSCRIBE = /\bsubscribe(?:<[^()]*?>)?\(\s*"([a-z_][a-z0-9_-]*)"/g;

function used(re: RegExp): Map<string, string> {
  const found = new Map<string, string>();
  for (const [file, text] of Object.entries(sources)) for (const m of text.matchAll(re)) found.set(m[1]!, file);
  return found;
}

const core = new Set<string>(CORE_COMMANDS);

describe("the browser preview fakes everything the window uses", () => {
  it("answers every command the app calls", () => {
    const calls = used(CALL);
    // The scan itself still finds the app's calls (it would pass on nothing otherwise).
    expect(calls.size).toBeGreaterThan(60);
    const missing = [...calls].filter(([cmd]) => !(core.has(cmd) ? cmd in coreHandlers : cmd in commands)).map(([cmd, file]) => `${cmd} (${file})`);
    expect(missing).toEqual([]);
  });

  it("answers every core command", () => {
    expect(CORE_COMMANDS.filter((cmd) => !(cmd in coreHandlers))).toEqual([]);
  });

  it("has every topic the app subscribes to", () => {
    const subs = used(SUBSCRIBE);
    expect([...subs.keys()].sort()).toEqual(expect.arrayContaining(["input", "neutron_download", "route"]));
    const missing = [...subs].filter(([topic]) => !(topic in topics)).map(([topic, file]) => `${topic} (${file})`);
    expect(missing).toEqual([]);
  });
});
