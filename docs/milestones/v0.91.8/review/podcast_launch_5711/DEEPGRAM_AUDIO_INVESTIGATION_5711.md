# Deepgram Audio Investigation For Podcast Launch

## Metadata

- Issue: `#5711`
- Scope: launch-readiness investigation only
- Status: candidate provider, not required for the first local launch proof
- Current sources checked: Deepgram official docs, 2026-07-29

## Finding

Deepgram is a plausible follow-on voice/audio provider for the Agent Logic
podcast, especially if we want lower-latency streamed speech, a broad voice
catalog, and a simple REST path.

Current official docs show:

- Text-to-speech REST endpoint: `POST https://api.deepgram.com/v1/speak`
- Authentication: `Authorization: Token <API_KEY>` or bearer JWT
- Example voice model style: `aura-2-thalia-en`
- Default TTS model if omitted: `aura-asteria-en`
- Output controls include encoding, container, bit rate, sample rate, and speed
- Aura input text limit: `2000` characters per request
- The API streams the audio response back, so playback can begin before the
  whole response is downloaded

Sources:

- https://developers.deepgram.com/docs/text-to-speech
- https://developers.deepgram.com/reference/text-to-speech/speak-request
- https://developers.deepgram.com/docs/tts-models

## Recommendation

Do not block week-one launch on Deepgram. The current #5711 path should first
ship a provider-neutral production shape:

- episode spec;
- transcript/source turns;
- local playable audio output;
- RSS enclosure;
- landing page and episode page;
- validation that proves local audio and feed coherence.

Then add Deepgram as a selectable renderer after the first launch path is green.

## Candidate Integration Shape

Add a later provider mode such as:

```bash
ADL_PODCAST_AUDIO_RENDERER=deepgram
DEEPGRAM_API_KEY=...
bash adl/tools/demo_v0918_podcast_launch.sh
```

The renderer should:

- split long turns before the `2000` character Aura limit;
- record model, voice, response content type, byte count, and request id when
  available;
- fail closed if an error body is written where audio was expected;
- avoid committing raw provider responses or credentials;
- keep renderer identity separate from transcript authorship identity.

## Non-Claims

- This issue does not prove Deepgram quality is better than the existing TTS
  route.
- This issue does not commit a Deepgram API key or require a Deepgram account.
- This issue does not claim deployed public audio until the site route is
  deployed and verified.
