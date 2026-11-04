---
created_at: "2026-11-04T21:25:48.498950217+00:00"
id: "astromech-1n9b"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-1n9b
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "review"
    id: "astromech-c4nj"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T22:53:09.641817495+00:00"
status: "done"
title: "Voice lookups from a named system: more phrasings, colonisation, copy checks"
updated_at: "2026-11-04T22:53:09.641817495+00:00"
---

## Goal

A spoken origin system is understood in more phrasings and checked against the copy rules, and colonisation buy questions can take one too.

## Done when

- 'in <system>', 'around <system>' and 'near <system> system' search from that system (or say plainly what was heard), and non-system words after 'from' or 'near' ('from a station') are not read as a system name; with tests (R4, astromech-c4nj).
- Colonisation 'where can I buy <commodity>' accepts a named system, or the strategy's 'every lookup' is narrowed with Sumeet (R5, astromech-1sl7).
- crates/astromech-core/tests/copy.rs runs the named-system wording ('From X.', the unknown-system line) through the §6.5 checks (R6, astromech-53q1).
- A non-lookup sentence with ' near <system>' or ' from <system>' added ('paste from Sol', 'cancel near Sol') gets 'Say again', not a lookup did-you-mean; the worst cases are tests (R7, astromech-temh).
- Ways of saying here that aren't in HERE_WORDS ('near mi', 'near you', 'near my current location') search from the current system again instead of 'Spansh does not know a system called mi' (R8, astromech-5w3d).

## Context

Deferred review findings R4–R6 on task/astromech-j75v: crates/astromech-intent/src/lib.rs:472 and :462, crates/astromech-core/tests/copy.rs:696.

## Notes

- 2026-11-04 [decision] Colonisation 'where can I buy <commodity> for the depot' takes a named system like the other lookups (takes_origin includes colonisation.buy; Galaxy::locate, 'From <system>.' lead). The strategy's 'every lookup' is not narrowed. Progress and what-is-left take none (they are about the project, not a place).
  Reason: plan_search takes any Here, and named_origin/search_here/from_named already give one from a name, so accepting it was a few lines. 'for the depot near Sol' is read as where to search from, not which project; the project is still the one docked at or last delivered to.
  Rejected: Narrow the strategy with Sumeet: not needed, accepting a system was practical.
- 2026-11-04 [decision] Origin words are near, from, in and around. The place after them drops a leading 'the' and a trailing 'system'. A place that is HERE_WORDS (now with you, by, my ship, my current location/position/system), a word of up to 3 letters with phonetic(me), or no system's name (first word a/an/any/some/another/my/your/our/their/that/those/these, or 'the' without 'system') means the current system: split with no origin, so 'from a station' searches from here.
  Reason: Reviewer findings c4nj and 5w3d. Treating a non-name as here rather than not splitting: unsplit, 'where can I buy gold from a station' matched colonisation.buy 'for the construction' at 0.86 Execute. The 3-letter limit on the phonetic rule keeps names like Maia (also phonetic n) as systems.
  Rejected: Leave non-names unsplit (whole transcript): wrong colonisation Execute above.
  Rejected: Phonetic 'me' at any length: would make Maia, Nemi etc. mean here.
- 2026-11-04 [decision] The head (words before the origin word) is scored against the phrases with a trailing 'near me' left off, replacing the second match with 'near me' appended. It wins when it is a lookup taking an origin and scores at least the whole transcript or the whole transcript's best is a lookup. When the head is no such lookup and a system was named, a lookup matched by the whole transcript becomes NoMatch.
  Reason: Reviewer finding temh: appending 'near me' let short non-lookups borrow the shared words of 'famine near me' (paste from Sol 0.78). The reviewer's sweep (every non-lookup MVP phrase with ' near Sol'/' from Sol' against assistant::grammar(), zz_probe3.rs) re-run on this change: phrases answered with an ask or execute on a lookup other than their own command, 3 before j75v, 58 at j75v, 0 now. 'where am i near Sol' was a trade.sell ask before j75v too; it is NoMatch now.
- 2026-11-04 [decision] The unknown-system voice line is now '<name>. Spansh does not know that system.' (astromech_tools::lookup::unknown_system), chosen in named_origin by downcasting a new typed error astromech_galaxy::UnknownSystem; other locate errors say 'Could not look that up. <reason>'. The galaxy error text the app shows ('Spansh does not know a system called <name>.') is unchanged.
  Reason: R6's copy check found the j75v line breaks the 16-word answer budget on a long procedural name (19 words for NGC 7822 Sector AB-C d14-123). §6.5: a line that may carry a long name says it as its own sentence. A typed error keeps the app's text and lets voice word its own line without matching strings.
  Rejected: Keep the line and test only short names: the check would not exercise the case it exists for.

## Close Reason

Implementer report. Changed (2882f5ff): astromech-intent split_origin takes near/from/in/around, drops a leading 'the' and trailing 'system'; HERE_WORDS gained you, by, my ship, my current location/position/system; a <=3-letter word with phonetic(me) and a place that names no system (a/my/your... or 'the' without 'system') mean here. Grammar::best scores the head against lookup phrases with their trailing 'near me' left off (replacing the appended-'near me' match) and turns a whole-transcript lookup after a named system into NoMatch when the head is no lookup. takes_origin includes colonisation.buy; colonisation::answer takes from (named_origin + search_here + from_named). New astromech_galaxy::UnknownSystem error; voice says '<name>. Spansh does not know that system.' (lookup::unknown_system), other locate errors 'Could not look that up. <reason>'; app text unchanged. lookups::from_named is pub for copy.rs. copy.rs: named variants of target/faction answers, the 'From X.' lead and the unknown line on NGC 7822 Sector AB-C d14-123. Colonisation tester text drops 'near you'. Docs: ARCHITECTURE 5.12, guide lookups/voice-commands/colonisation. All five done-when bullets met; R5 accepted a named system, strategy not narrowed. Deviations: the j75v unknown-system wording changed, because the new copy check showed it at 19 words on a long name (budget 16). Surprises: unsplit 'where can I buy gold from a station' matched colonisation.buy at 0.86 Execute, so non-name places are treated as here, not left unsplit. Proof: evidence astromech-sy10; new intent tests seen failing on the pre-change grammar; reviewer sweep re-run: non-lookup phrases + near/from Sol answered with another command's lookup: 3 before j75v, 58 at j75v, 0 now. Open items: independent review outstanding (no subagents in this assignment); 'in' widens the split, so 'where can I sell gold in bulk' answers 'Bulk. Spansh does not know that system.'; a 1-3 letter m/n-and-vowel system name would be heard as here; voice recognition of these phrasings not measured (no voice eval run); app not touched so pnpm checks not run.
