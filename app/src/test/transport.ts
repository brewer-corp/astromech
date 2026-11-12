/**
 * The one seam UI behaviour tests fake: the window's transport (CONTRIBUTING.md, Tests). Importing
 * this installs a transport that the browser preview's fake answers, as in `pnpm dev`, except for
 * the commands a test answers itself with `answer`; `watch` and `sent` record the calls, and
 * `idle` waits until every call has answered and the page has drawn the answers.
 *
 * `answer` called at a test file's top level holds for every test in the file; called in a test or
 * a `beforeEach`, it holds until the next test. Before each test the watches and `sent` are
 * cleared, real timers are back, the page query is emptied and the fake core, the neutron file
 * and the shared plot setup start over; `startPreview` sets a query.
 */
import { tick } from "svelte";
import { beforeEach, vi, type Mock } from "vitest";
import { setup as plotSetup } from "../lib/plot.svelte";
import { setTransport } from "../lib/transport";
import { resetFakeCore } from "../preview/fakecore";
import { resetPreviewNeutron } from "../preview/shell";
import { previewTransport } from "../preview/transport";

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- a command's args are its own shape
type Answer = (args: any) => unknown;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Watch = Mock<(args: any) => void>;

const fileAnswers = new Map<string, Answer>();
let answers = fileAnswers;
const watches = new Map<string, Watch>();
const inflight = new Set<Promise<unknown>>();

/** A mock called with every command the window sends and its args. */
export const sent = vi.fn<(cmd: string, args: unknown) => void>();

/** Answer `cmd` with `f`, given the call's args (`{}` for none), instead of the preview's fake. */
export function answer(cmd: string, f: Answer): void {
  answers.set(cmd, f);
}

/** A mock called with the args of each `cmd` the window sends; the answer is unchanged. */
export function watch(cmd: string): Watch {
  const w = watches.get(cmd) ?? vi.fn();
  watches.set(cmd, w);
  return w;
}

/** The browser preview's own answer: what the fake core holds now, or a reply to adjust. */
// eslint-disable-next-line @typescript-eslint/no-explicit-any -- a test reads the reply it expects
export function previewAnswer<T = any>(cmd: string, args: unknown = null): Promise<T> {
  return previewTransport.call(cmd, args) as Promise<T>;
}

/** Once every call the window has sent has answered and the page has drawn the answers. A call
 * that never answers (`?slow=never`) never lets this resolve: wait on `sent` for those. */
export async function idle(): Promise<void> {
  while (inflight.size) await Promise.allSettled([...inflight]);
  await tick();
}

/** Run the preview at `url` (`/?core=ready&fail=route`), from a fresh fake core. */
export function startPreview(url: string): void {
  history.replaceState(null, "", url);
  resetFakeCore();
}

setTransport({
  ...previewTransport,
  call(cmd, args) {
    sent(cmd, args);
    watches.get(cmd)?.(args ?? {});
    const f = answers.get(cmd);
    const p = f ? (async () => f(args ?? {}))() : previewTransport.call(cmd, args);
    inflight.add(p);
    const done = () => void inflight.delete(p);
    p.then(done, done);
    return p;
  },
});

beforeEach(() => {
  answers = new Map(fileAnswers);
  for (const w of watches.values()) w.mockClear();
  sent.mockClear();
  inflight.clear();
  vi.useRealTimers();
  startPreview("/");
  resetPreviewNeutron();
  plotSetup.reset();
});
