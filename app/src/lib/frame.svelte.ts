// The core's status and the ship's, as the frame reads them: one 2 s poll that the status bar,
// the toasts and the Now page share (the status bar's fuel and cargo follow the game's
// Status.json writes on the same tick). The frame (App.svelte) makes one and hands it down by
// context; a page drawn on its own (a test) makes its own and starts it.
import { getContext, setContext } from "svelte";
import { coreStatus, shipStatus, type CoreStatus, type ShipStatus } from "./core";

export class FramePolls {
  /** Null before the first read, and when the core didn't answer. */
  status = $state<CoreStatus | null>(null);
  /** Null before the first read, and when the call failed. */
  ship = $state<ShipStatus | null>(null);
  /** Both have been read at least once (answered or not), so a null above means what it says. */
  read = $state(false);

  tick = async (): Promise<void> => {
    [this.status, this.ship] = await Promise.all([coreStatus(), shipStatus()]);
    this.read = true;
  };

  /** Poll every 2 s until the returned stop is called. */
  start(): () => void {
    void this.tick();
    const t = setInterval(() => void this.tick(), 2000);
    return () => clearInterval(t);
  }
}

const KEY = Symbol("frame-polls");

/** The frame's side: one poll for the whole window. */
export function provideFrame(): FramePolls {
  const f = new FramePolls();
  setContext(KEY, f);
  return f;
}

/** A page's side: the frame's, or with no frame, one of its own (started by the caller). */
export function useFrame(): { frame: FramePolls; own: boolean } {
  const f = getContext<FramePolls | undefined>(KEY);
  return f ? { frame: f, own: false } : { frame: new FramePolls(), own: true };
}
