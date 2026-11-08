import { afterEach, expect, it, vi } from "vitest";

// With the dev bridge's plugin on (it defines VITE_ASTROMECH_BRIDGE), lib/transport.ts sends every
// call to the dev server, which streams the core's lines back: a command's progress is published
// on its name, as the Tauri path does, and the envelope settles the call. The dev server is faked
// at fetch, with lines split across chunks as a stream may deliver them.
function streamed(...chunks: string[]): Response {
  const body = new ReadableStream<Uint8Array>({
    start(c) {
      for (const s of chunks) c.enqueue(new TextEncoder().encode(s));
      c.close();
    },
  });
  return new Response(body, { status: 200, headers: { "content-type": "application/x-ndjson" } });
}

async function bridged() {
  vi.stubEnv("VITE_ASTROMECH_BRIDGE", "/__astromech/call");
  vi.resetModules();
  return await import("./transport");
}

afterEach(() => {
  vi.unstubAllEnvs();
  vi.unstubAllGlobals();
  vi.resetModules();
});

it("posts the call to the bridge and publishes its progress on the command's name", async () => {
  const fetch = vi.fn(() => Promise.resolve(streamed('{"progress":{"done":1,"tot', 'al":4}}\n{"progress":{"done":4,"total":4}}\n{"ok":true,', '"value":{"file":null,"error":null}}\n')));
  vi.stubGlobal("fetch", fetch);
  const t = await bridged();
  expect(t.bridged()).toBe(true);
  const seen: unknown[] = [];
  const off = t.subscribe("neutron_download", (p) => seen.push(p));
  await expect(t.call("neutron_download")).resolves.toEqual({ file: null, error: null });
  off();
  expect(seen).toEqual([
    { done: 1, total: 4 },
    { done: 4, total: 4 },
  ]);
  expect(fetch).toHaveBeenCalledWith("/__astromech/call", expect.objectContaining({ method: "POST", body: JSON.stringify({ cmd: "neutron_download", args: null }) }));
});

it("rejects with the core's error, as the shell's proxy does", async () => {
  vi.stubGlobal("fetch", () => Promise.resolve(streamed('{"ok":false,"error":"no position in the journal yet"}\n')));
  const t = await bridged();
  await expect(t.call("galaxy_nearby", { query: "famine", origin: null, filters: null })).rejects.toBe("no position in the journal yet");
});

it("fails a call whose stream ends before the core answered", async () => {
  vi.stubGlobal("fetch", () => Promise.resolve(streamed('{"progress":1}\n')));
  const t = await bridged();
  await expect(t.call("status")).rejects.toMatch(/closed before the core answered status/);
});
