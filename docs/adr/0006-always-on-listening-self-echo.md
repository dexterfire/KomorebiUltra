# ADR 0006 — Always-on listening without Self-echo

* **Status:** Accepted
* **Date:** 2026-09-25

## Context

With `auto_listen` on and no wake word the microphone stays open while
the companion speaks. The VAD stream (`src/vad.ts`) has browser echo
cancellation, but the recorder that feeds speech recognition (cpal,
`startRecording` / `stopRecording` with a pre-roll buffer) has none, and
any speech-start stops TTS. So the companion can hear her own voice
(**Self-echo**) and act on it: issue #1 showed bursts of screenshots
9–28 s apart after skill replies like "Saved screenshot to …" were read
aloud. Self-echo is not yet proven by logs.

## Decision

* **The microphone stays open while she speaks.** Voice Barge-in is the
  target behaviour (see `GLOSSARY.md`); muting the mic during replies is
  rejected.
* **Echo cancellation on the recognition path.** Candidates: record from
  the WebView stream that already has echo cancellation, or enable
  Windows system echo cancellation on the native recorder. Pick by
  measurement. First step: log recognized text next to TTS playback
  times to prove or rule out Self-echo.
* **Interim mode until echo cancellation passes the check.** While she
  speaks, only a built-in list of stop cues acts ("стоп", "подожди",
  "хватит", "замолчи", "тихо" / "stop", "wait" / "зачекай"); all other
  speech during her reply is dropped. During her reply "тихо" means stop,
  not volume.
* **After the check passes:** speech during her reply becomes Pending
  utterances and is answered as one Turn when she finishes; stop cues
  still trigger Barge-in.
* **Check:** 20 ordinary replies spoken through speakers at normal
  volume, and zero recognized texts in the log matching her speech.
  Headphones don't count.
* **Skill replies are spoken as a short fixed phrase** in the interface
  language ("Готово, скриншот сохранён"); file paths and technical detail
  stay in the chat text only.

## Consequences

* Interim mode loses Asides and non-stop speech said over her reply;
  the user repeats it after she finishes.
* Voice commands that happen to be stop cues ("тихо") change meaning
  while she speaks.
* No default wake word and no matching of transcripts against her last
  reply; both are unnecessary once Self-echo is cancelled.

## Alternatives considered

* **Mute the mic while she speaks (+ ~0.5 s tail).** Simplest and
  removes Self-echo entirely, but loses voice Barge-in; rejected.
* **Drop transcripts similar to the last reply.** Fragile with
  recognition errors and partial matches; rejected.
* **Default wake word with always-on listening.** Handles people talking
  nearby, but that belongs to Aside handling, not Self-echo; rejected.
