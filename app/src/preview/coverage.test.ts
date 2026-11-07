import { describe, expect, it } from "vitest";
import generated from "../lib/api/commands.ts?raw";
import { previewCommands, topics } from "./transport";

// Every command the window may send has a fake, so no page of the browser preview (or a test
// behind the transport) meets a call the preview can't answer. The command list is the generated
// command map's (lib/api/commands.ts, from the core's registry); the app's own sources (not
// tests, not the preview) are read for the names it passes to call, coreCall and subscribe.
const sources = import.meta.glob<string>(["../**/*.ts", "../**/*.svelte", "!../**/*.test.ts", "!./**", "!../lib/api/**"], {
  query: "?raw",
  import: "default",
  eager: true,
});

const CALL = /\b(?:call|coreCall|routeCall)(?:<[^()]*?>)?\(\s*(?:true,\s*|false,\s*)?"([a-z_][a-z0-9_-]*)"/g;
const SUBSCRIBE = /\bsubscribe(?:<[^()]*?>)?\(\s*"([a-z_][a-z0-9_-]*)"/g;

function used(re: RegExp): Map<string, string> {
  const found = new Map<string, string>();
  for (const [file, text] of Object.entries(sources)) for (const m of text.matchAll(re)) found.set(m[1]!, file);
  return found;
}

/** The map's commands by exposure. */
const exposed = (exposure: string) => [...generated.matchAll(/^ {2}(\w+): \{ args: .*exposure: "(\w+)"/gm)].filter((m) => m[2] === exposure).map((m) => m[1]!);
const windowCommands = exposed("window");

describe("the browser preview fakes everything the window uses", () => {
  it("answers every command the map lets the window send, and no other", () => {
    // The map is read (it would pass on nothing otherwise).
    expect(windowCommands.length).toBeGreaterThan(100);
    expect(exposed("local")).toEqual(expect.arrayContaining(["quit"]));
    expect(Object.keys(previewCommands()).sort()).toEqual([...windowCommands].sort());
  });

  it("sends only the map's window commands", () => {
    const calls = used(CALL);
    // The scan itself still finds the app's calls.
    expect(calls.size).toBeGreaterThan(60);
    const unknown = [...calls].filter(([cmd]) => !windowCommands.includes(cmd)).map(([cmd, file]) => `${cmd} (${file})`);
    expect(unknown).toEqual([]);
  });

  it("has every topic the app subscribes to", () => {
    const subs = used(SUBSCRIBE);
    expect([...subs.keys()].sort()).toEqual(expect.arrayContaining(["input", "neutron_download", "route"]));
    const missing = [...subs].filter(([topic]) => !(topic in topics)).map(([topic, file]) => `${topic} (${file})`);
    expect(missing).toEqual([]);
  });
});
