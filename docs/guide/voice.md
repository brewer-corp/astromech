# Voice

astromech listens while you hold one button, works out what you asked, and answers out loud.
It is built for hands on the stick: you never need the window while flying.

Speech recognition and the spoken replies both run on your PC. Nothing you say is sent
anywhere. The full list of things you can say is in [Voice commands](voice-commands.md).

## How to talk to it

1. **Hold** your push-to-talk button.
2. **Speak** one command, a short phrase.
3. **Release.** A moment later astromech does it, or answers.

Press-and-release without speaking does nothing. A hold shorter than 0.3 seconds is treated as a
tap and dropped, and so is a clip that is nearly silent or only noise ("Mm-hmm", "Yeah").
Those get no reply. The same goes for a button that bounces.

Pressing push-to-talk cuts off whatever astromech is saying, and callouts wait while you talk and
for the reply.

Speech is English only. Say the phrase the way the [command list](voice-commands.md) writes
it, or close to it: "gear down", "lights on", "deploy hardpoints". Filler words such as "please"
and "the" are ignored, and "hard points" is heard as "hardpoints".

## What it needs

- **Setup done:** a push-to-talk button, the speech models and a microphone (see
  [Getting started](getting-started.md)). **Settings** shows each one.
- **Elite running.** Voice loads when Elite or its launcher starts. The tray says "Waiting for
  Elite" until then.
- **For commands that press a key:** a keyboard key bound in Elite for that action. astromech
  presses keys. A stick button alone is not enough. If the key is missing it says which action and
  how to bind one.
- **Elite in front.** astromech presses keys only while Elite is the active window, and neither
  program may run as administrator. If Elite is not in front, you hear "Elite is not the active
  window. Nothing pressed."
- **For lookups:** a network connection, and the position of your ship (it comes from your
  next jump or login). See [Voice lookups](lookups.md).

## How it decides

astromech matches what it heard against a fixed list of commands. It does not guess at
anything outside the list. Each match gets a score:

- **Clear match:** it does it.
- **Close, but not sure:** it asks "Did you mean gear down?" Answer **yes** or **no** with
  your next push-to-talk, within five seconds. "Yes" runs it. "No" cancels ("Cancelled."). You
  can also say "confirm", "do it" or "affirmative" for yes, and "cancel", "never mind",
  "negative" or "abort" for no.
- **Not close:** "Say again."

When you answer **yes** to a "Did you mean", astromech remembers what you said as a personal
phrase for that command, so next time the same mishearing works at once. It is stored on your PC.
There is no page to review or clear these phrases yet. Phrases that carry a name (a
commodity, a station service, a module, a ship) are not remembered.

A command that is recognised but not built yet is answered: "Supercruise is not available
yet. Flight commands come later." (See the list.)

## What it checks before it acts

Every command that presses a key goes through the same checks:

- **Where you are.** Landing gear, hardpoints and cargo scoop need you in your ship, not
  docked and not landed. Gear and hardpoints are also refused in supercruise. Refusals are one
  clause: "Landing gear not available: supercruise." Lights, night vision and the HUD
  mode work in the ship or the SRV. Handbrake and drive assist work in the SRV only.
- **Already there.** "Landing gear is already deployed." and nothing is pressed.
- **Slow down.** A repeat of the same action within three seconds is "Already on it." More than
  eight actions in ten seconds is "Too many actions, slow down."
- **Silent running.** Turning it on asks first: "Silent running produces heat. Confirm?" Say yes
  within five seconds.
- **The game's answer.** astromech says its confirmation as soon as it has sent the key. It then
  watches the game's own state file for the change and writes the result to the log. If the game
  did not do it, nothing more is said, so look at the ship if it matters.

astromech never presses a key you did not ask for by voice, and never fires weapons, jumps
or docks for you.

## The kill switch

The kill switch is a second button you choose in setup. Press it and astromech stops all
actions at once: it cancels anything in flight, releases any key it holds, says "Kill switch on.
All actions stopped." and refuses every command that would press a key until you **re-arm**.

To re-arm, **hold** the kill switch button for about a second ("Re-armed."), or use **Re-arm**
in the tray menu (the same item turns the kill switch on). A short press while it is on says "Still killed. Hold the kill switch to
re-arm."

Read-only questions ("fuel", "where am I") still work while the kill switch is on.

## Spoken replies

Replies and callouts are spoken by a local voice, Kokoro, coloured to sound like a ship's
public address. Short lines, no chat. The copy is in the same register as Elite's own ship
computer: it reports the fact.

In [Settings](settings.md#voice-responses) you can change:

- **Speaker**: British or American voices. **Preview** speaks a sample line.
- **Colour**: **Cockpit** (the default; a ship's PA, close to Elite's own voice), **Natural**
  (the raw voice) or **Harsh** (a narrower band, more grit).
- **Speed**: 0.7 to 1.3, where Normal is the colour's own pace.

A change applies from the next reply.

If the speech models are missing, or speech is switched off in the config (`tts = false`),
replies are printed in the log instead of spoken.

**Pause voice** (tray or Settings) makes astromech ignore push-to-talk. It does not stop
[callouts](callouts.md).

## When nothing happens

Check these in order:

1. The tray line says "Elite running" and not "Waiting for Elite" or "voice failed". If voice
   failed, see the log in Diagnostics.
2. [Settings](settings.md) shows push-to-talk as set, the models as Installed, and the voice
   as "Listening while Elite runs", not Paused.
3. Open the [Voice tester](voice-tester.md) and press push-to-talk with the page open. If it
   shows what you said, voice works and the problem is the key press.
4. Run the Key tester (in [Controls](controls.md#key-tester)), press your push-to-talk button.
   If nothing shows, astromech cannot see the device.
5. Say the phrase with the Voice tester's **Would** line: it says if the action is refused,
   has no keyboard key, or needs Elite in front.
6. Calibrate again if quiet speech is dropped (Settings, Microphone).

## Limits

- English only; a fixed list of commands, not free conversation.
- Presses keyboard keys only.
- One command per push.
- Flight commands (supercruise, hyperspace jump, docking requests, route plotting) are
  recognised but do nothing yet.
- Speech recognition can mishear game words. Ask for "Did you mean" by saying it clearly, and
  use the Voice tester to see what it heard.
- Nothing is automated beyond the one key press you asked for. There are no macros, timers or
  combat help.
