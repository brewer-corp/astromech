/** Onboarding's steps, in order (docs/V1-DESIGN.md §1.5 #1). The id is also `#/setup/<id>`. */
export const STEPS = [
  { id: "elite", title: "Find Elite", lead: "astromech reads Elite's keybindings and journals. It never changes game files without asking." },
  { id: "devices", title: "Controllers", lead: "The sticks, throttles and pads Elite can see. Press a button to check it arrives." },
  { id: "buttons", title: "Push-to-talk", lead: "Hold push-to-talk while you speak. The kill switch stops everything astromech is doing." },
  { id: "mic", title: "Microphone", lead: "Pick the mic you speak into while flying (in VR, often the headset's), then check its level." },
  { id: "calibrate", title: "Calibration", lead: "A moment of silence and three phrases, so quiet speech is heard and room noise isn't." },
  { id: "health", title: "Health check", lead: "What still stands between you and “gear down” by voice, with a fix for each." },
] as const;

export type StepId = (typeof STEPS)[number]["id"];
