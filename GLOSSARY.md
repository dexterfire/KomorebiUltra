# Komorebi

Desktop Live2D companion that talks with the user by voice and text and acts on their computer.

## Language

### Live conversation

**Turn**:
One unit of the user's input that the companion answers with one reply.
_Avoid_: request, message (when meaning voice input)

**Utterance**:
A single stretch of user speech bounded by silence; one or more utterances make a Turn.
_Avoid_: phrase, recording

**Barge-in**:
User speech during the companion's reply that means "stop and listen to me"; the companion finishes the current phrase (or word, if the phrase would run past ~1 s), stops, and answers the new Turn.
_Avoid_: interrupt (ambiguous with Aside), cancel

**Aside**:
User speech during the companion's reply that is not meant to stop it (backchannel, commentary, talking to someone else); the companion keeps speaking. Ambiguous speech is treated as an Aside.
_Avoid_: comment, noise

**Steer**:
User speech during the companion's reply that corrects its direction; the companion winds down the current phrase and continues with a changed answer instead of starting over.
_Avoid_: correction, redirect

**Self-echo**:
The companion's own voice picked up by the microphone and recognized as user speech; it must never become a Turn, Barge-in, or Aside.
_Avoid_: feedback, noise

**Pending utterances**:
Utterances the user produced while the companion was thinking or speaking and that have not been answered yet; they are merged into one Turn rather than answered one by one.
_Avoid_: queue of messages

**Spoken transcript**:
The part of a reply the user actually heard; only this part is remembered in the conversation history after a Barge-in or Steer.
_Avoid_: reply text, generated text

**Karaoke highlight**:
The reply text in the bubble revealed and highlighted in step with the voice, word by word.
_Avoid_: subtitles

**Interruption cue**:
A word or phrase the companion has learned from this user that signals Barge-in, Aside, or Steer; kept in the Phrasebook. She proposes a new cue and asks the user before adding it.
_Avoid_: stop word, hotword

**Thinking bubble**:
The speech bubble shown while the companion prepares a reply: one fixed emoji plus a rotating playful status phrase.

**Subtitle style**:
A user-selectable preset for how reply text appears in the bubble: reveal animation, per-word vs whole text, look, and how the spoken word is highlighted (glow, shadow, colour).
_Avoid_: theme, template (alone)

**Phrase boundary**:
The nearest pause point in the companion's speech (end of a clause or sentence) where stopping sounds natural.
_Avoid_: cut point

### Memory

**Companion**:
One named character the user can talk with, defined by a Soul and bound to an Avatar model and a voice; the user can have several and switch between them.
_Avoid_: assistant, agent, bot, waifu

**Soul**:
A Companion's own character: name, gender, personality, manner, how she or he speaks. Editable by the user; changing the Avatar model does not change the Soul.
_Avoid_: persona, system prompt

**Session tone**:
A short-lived shift in a Companion's manner (e.g. more playful) picked up from the user's reactions during one session; it fades by the next day. The Soul keeps a note that it happened (as a past, temporary shift), but its core character is unchanged.
_Avoid_: mood (reserved for the per-reply emotion shown by the avatar)

**Soul setup**:
The guided first-run wizard where the user shapes the companion's Soul by picking from ready-made options.
_Avoid_: onboarding (alone), character creator

**Memory**:
Long-term facts the companion keeps about the user and their shared history.
_Avoid_: RAG, notes (the user's indexed folders are separate)

**Phrasebook**:
The companion's list of learned phrases with what each means and when it applies (including Interruption cues); the user can add entries by telling her.
_Avoid_: dictionary, cue list

### Voice models

**Voice stack**:
The chosen combination of voice-activity detector, speech recognizer, and voice for speaking; any part can be switched.

**Model catalog**:
The in-app list of downloadable models (speech recognition, voices, local language models) with their download state.

**Game profile**:
The lighter Voice stack the companion switches to automatically while the user is playing a game, and back afterwards.
_Avoid_: low mode, performance mode

**Voice lab**:
The in-app bench where the user records their own sample phrases once and compares Voice stacks on speed and accuracy.
_Avoid_: benchmark, test page

**Reply latency**:
Time from the end of the user's speech to the first sound of the companion's reply.
_Avoid_: lag, ping

### Roadmap areas (one line each, specced separately)

**Skill**:
An action the companion can take on the user's computer or the world (weather, screenshot, open app, media, desktop automation).
_Avoid_: tool (when talking to the user), command

**Avatar model**:
A Live2D character the companion is rendered as; the user can choose among several.
_Avoid_: model (ambiguous with LLM)

**Music reaction**:
The avatar moving or emoting in response to music playing on the computer (e.g. in a game).

**Roaming**:
The companion moving its own window across the screen.

**Duo mode**:
Two Companions on screen at once, talking with the user and with each other. Later.

**Bubble mode**:
Optional minimal presentation: all UI hidden except a small speech-bubble chat window.
_Avoid_: mini mode, compact mode
