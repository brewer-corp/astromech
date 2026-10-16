/**
 * The voice command tester's calls to the core (crates/astromech-core/src/tester.rs). A test is
 * a dry run: the core matches the phrase and plans the keys but presses nothing. Only
 * `voiceLive` presses, through the core's policy. Outside Tauri the sample results stand in.
 */
import { coreCall, inTauri } from "./core";
import sample from "./fixtures/voice.json";

export type Would =
  | "press"
  | "confirm"
  | "already"
  | "refused"
  | "speak"
  | "no_binding"
  | "unavailable"
  | "did_you_mean"
  | "not_understood"
  | "ignored";

export interface DryRun {
  source: "typed" | "voice";
  transcript: string;
  intent: { command: string; phrase: string; score: number; decision: "execute" | "ask" | "no_match" } | null;
  would: Would;
  /** What astromech would say. */
  message: string;
  action: string | null;
  keys: { key: string; modifiers: string[]; slot: string } | null;
  /** The command a live run would execute, or null. */
  live: string | null;
}

export interface Poll {
  /** True while Elite runs and push-to-talk is live (so it can be tested). */
  listening: boolean;
  last: { seq: number; result: DryRun } | null;
}

const samples = sample as unknown as Record<string, DryRun>;
const norm = (s: string) => s.toLowerCase().replace(/[^a-z ]/g, "").trim();

/** Browser preview: the sample phrases match, anything else is "didn't catch that". */
function sampleRun(text: string): DryRun {
  const hit = samples[norm(text)];
  if (hit) return { ...hit, transcript: text };
  return { ...samples["banana bread"]!, transcript: text };
}

export async function voiceTest(text: string): Promise<DryRun> {
  if (!inTauri) return sampleRun(text);
  const r = await coreCall("voice_test", text);
  if (!r.ok) throw new Error(String(r.error));
  return r.result as DryRun;
}

/** Keeps push-to-talk in test mode for a few seconds; call it every second while the page is open. */
export async function voicePoll(): Promise<Poll | null> {
  if (!inTauri) return { listening: false, last: null };
  const r = await coreCall("voice_test_poll");
  return r.ok ? (r as unknown as Poll) : null;
}

/** Hand push-to-talk back to the assistant now (the page closed). */
export async function voiceStop(): Promise<void> {
  if (inTauri) await coreCall("voice_test_stop");
}

/** Run a command for real. Returns what happened, in words. */
export async function voiceLive(command: string): Promise<string> {
  const r = await coreCall("voice_test_live", command);
  if (!r.ok) throw new Error(String(r.error));
  return String(r.message);
}
