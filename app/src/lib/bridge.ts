/**
 * The dev bridge's page side: a call posted to the dev server's `/__astromech/call`, which the
 * Vite plugin (app/dev/bridge.ts) sends to the headless core of `ASTROMECH_WORLD=<dir> pnpm dev`
 * and answers with the core's reply lines as they come (NDJSON): progress, then the envelope.
 * lib/transport.ts loads this only in the dev server with the plugin on, so `pnpm build` drops it
 * (scripts/check-dist.py).
 */

type Line = { progress: unknown } | { ok: true; value: unknown } | { ok: false; error: string };

/** Post `cmd` with `args` to `path`; resolves the core's value, rejects with its error (a string,
 * as Tauri's invoke rejects with the shell's), and gives each progress value to `progress`. */
export async function bridgeCall(path: string, cmd: string, args: unknown, progress: (p: unknown) => void): Promise<unknown> {
  const res = await fetch(path, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ cmd, args: args ?? null }) });
  if (!res.body) return Promise.reject(`the dev bridge answered ${cmd} with ${res.status} and no body`);
  const reader = res.body.pipeThrough(new TextDecoderStream()).getReader();
  let buffered = "";
  for (;;) {
    const { done, value } = await reader.read();
    buffered += value ?? "";
    let end: number;
    while ((end = buffered.indexOf("\n")) >= 0) {
      const line = JSON.parse(buffered.slice(0, end)) as Line;
      buffered = buffered.slice(end + 1);
      if ("progress" in line) progress(line.progress);
      else {
        void reader.cancel();
        return line.ok ? line.value : Promise.reject(line.error);
      }
    }
    if (done) return Promise.reject(`the dev bridge closed before the core answered ${cmd} (${res.status})`);
  }
}
