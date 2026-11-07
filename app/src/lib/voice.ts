/**
 * The voice command tester's calls to the core (crates/astromech-core/src/tester.rs). A test is
 * a dry run: the core matches the phrase and plans the keys but presses nothing. Only
 * `voiceLive` presses, through the core's policy. The browser preview answers from sample results.
 */
import type { Answer } from "./api/types/Answer";
import type { DryRun } from "./api/types/DryRun";
import type { Phrase } from "./api/types/Phrase";
import type { Phrases } from "./api/types/Phrases";
import type { Poll } from "./api/types/Poll";
import { coreCall } from "./core";

export type { DryRun } from "./api/types/DryRun";
export type { Poll } from "./api/types/Poll";
export type { Would } from "./api/types/Would";
/** A phrase astromech learned from a "Did you mean ...?" answered yes: the words as they were
 * heard, the command it now runs and that command's usual phrase. */
export type LearnedPhrase = Phrase;

export async function voiceTest(text: string): Promise<DryRun> {
  const r = await coreCall("voice_test", text);
  if (!r.ok) throw new Error(String(r.error));
  return r.result;
}

/** Keeps push-to-talk in test mode for a few seconds; call it every second while the page is open. */
export async function voicePoll(): Promise<Poll | null> {
  const r = await coreCall("voice_test_poll");
  return r.ok ? r : null;
}

/** Hand push-to-talk back to the assistant now (the page closed). */
export async function voiceStop(): Promise<void> {
  await coreCall("voice_test_stop");
}

async function learned(r: Promise<Answer<Phrases>>): Promise<LearnedPhrase[]> {
  const v = await r;
  if (!v.ok) throw new Error(v.error ?? "the core did not answer");
  if (!Array.isArray(v.phrases)) throw new Error("the core did not answer");
  return v.phrases;
}

/** The phrases learned so far, most used first. */
export async function learnedPhrases(): Promise<LearnedPhrase[]> {
  return learned(coreCall("learned_phrases"));
}

/** Forget one learned phrase; it asks again next time. Returns the phrases that are left. */
export async function forgetPhrase(heard: string): Promise<LearnedPhrase[]> {
  return learned(coreCall("learned_forget", heard));
}

/** How long a live run waits for Elite to be the active window (the core's `tester::SWITCH_WAIT`). */
export const SWITCH_SECONDS = 5;

/**
 * Run a command for real: the core presses once Elite is in front. A lookup searches from `from`
 * when the phrase named a system. Returns what happened, in words.
 */
export async function voiceLive(command: string, from?: string | null): Promise<string> {
  const r = await coreCall("voice_test_live", from ? { command, from } : command);
  if (!r.ok) throw new Error(String(r.error));
  return r.message;
}
