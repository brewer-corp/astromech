/// <reference types="node" />
import { createServer as createHttp, request, type Server } from "node:http";
import { createServer as createTcp, type AddressInfo } from "node:net";
import { afterEach, expect, it } from "vitest";
import { answer } from "./bridge.ts";
import type { WorldCore } from "./core.ts";

// The bridge passes a call to the core with the core's token, as the window, so only the page
// the dev server serves may make one (finding astromech-igbw). The core here is a TCP server
// speaking the control wire that records each request it gets; the bridge's middleware runs in a
// real HTTP server on a loopback port.
const closers: (() => void)[] = [];
afterEach(() => closers.splice(0).forEach((c) => c()));

async function bridge(): Promise<{ port: number; reached: string[] }> {
  const reached: string[] = [];
  const core = createTcp((s) =>
    s.once("data", (d) => {
      reached.push(String(d));
      s.end(`${JSON.stringify({ ok: true, value: { commander: "Fixture" } })}\n`);
    }),
  );
  await new Promise<void>((r) => core.listen(0, "127.0.0.1", r));
  const contact = { port: (core.address() as AddressInfo).port, token: "t0ken" };
  const http: Server = createHttp((req, res) => void answer(Promise.resolve({ contact } as WorldCore), req, res, (http.address() as AddressInfo).port));
  await new Promise<void>((r) => http.listen(0, "127.0.0.1", r));
  closers.push(() => core.close(), () => http.close());
  return { port: (http.address() as AddressInfo).port, reached };
}

/** POST `body` to the bridge with exactly `headers` (fetch won't send a Host of our choosing). */
function post(port: number, headers: Record<string, string>): Promise<{ status: number; text: string }> {
  return new Promise((done, fail) => {
    const req = request({ host: "127.0.0.1", port, method: "POST", path: "/", headers }, (res) => {
      let text = "";
      res.on("data", (d) => (text += String(d)));
      res.on("end", () => done({ status: res.statusCode ?? 0, text }));
    });
    req.on("error", fail);
    req.end(body);
  });
}

const body = JSON.stringify({ cmd: "logbook", args: { query: "", cmdr: null } });

it("answers the page it serves", async () => {
  const { port, reached } = await bridge();
  // A same-origin fetch from the page: the browser sends Sec-Fetch-Site and Origin.
  const res = await post(port, { "content-type": "application/json", "sec-fetch-site": "same-origin", origin: `http://127.0.0.1:${port}` });
  expect(res.status).toBe(200);
  expect(res.text).toBe('{"ok":true,"value":{"commander":"Fixture"}}\n');
  expect(reached).toHaveLength(1);
  expect(JSON.parse(reached[0]!)).toMatchObject({ cmd: "logbook", token: "t0ken", origin: "app_window" });
});

it.each<[string, Record<string, string>]>([
  ["a no-cors text/plain POST from another site", { "content-type": "text/plain", "sec-fetch-site": "cross-site", origin: "https://evil.example" }],
  ["a JSON POST from another origin", { "content-type": "application/json", origin: "https://evil.example" }],
  ["a JSON POST the browser marks cross-site", { "content-type": "application/json", "sec-fetch-site": "cross-site" }],
  ["a JSON POST with no origin at all", { "content-type": "application/json" }],
])("refuses %s, before it reaches the core", async (_name, headers) => {
  const { port, reached } = await bridge();
  const res = await post(port, headers);
  expect(res.status).toBe(403);
  expect(reached).toEqual([]);
});

it("refuses a name that isn't the dev server's (DNS rebinding)", async () => {
  const { port, reached } = await bridge();
  // A rebound name resolves to 127.0.0.1, so the page is "same-origin" with the bridge, but the
  // Host is the attacker's name.
  const host = `rebind.evil.example:${port}`;
  const res = await post(port, { host, "content-type": "application/json", "sec-fetch-site": "same-origin", origin: `http://${host}` });
  expect(res.status).toBe(403);
  expect(res.text).toMatch(/not this dev server's host/);
  expect(reached).toEqual([]);
});
