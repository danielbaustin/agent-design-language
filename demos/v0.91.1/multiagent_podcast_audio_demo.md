# Multi-Agent Podcast Audio Demo

## Summary

This bounded `v0.91.1` follow-on adds an audio-rendering path to the
transcript-first multi-agent podcast pilot.

The audio path preserves a key truth boundary:

- transcript authorship identity
- audio renderer identity

remain explicitly separable.

## Scope Boundary

This audio follow-on proves:

- one podcast episode can be rendered into a listenable audio artifact
- `ChatGPT` and `Gemini` can use native/provider-specific TTS-capable paths
- `Claude` can be rendered truthfully through a surrogate TTS lane while
  preserving Claude transcript authorship identity
- the audio can be presented in a listener-friendly way with spoken speaker
  intros and simple loudness normalization

It does **not** prove:

- native audio support from every provider
- long-term identity continuity through audio
- a broad publishing, hosting, or distribution platform

## Canonical Command

From repository root:

```bash
bash adl/tools/demo_v0911_multiagent_podcast_audio.sh
```

## What Runs

- source transcript pilot wrapper:
  - `adl/tools/demo_v0911_multiagent_podcast_pilot.sh`
- audio wrapper:
  - `adl/tools/demo_v0911_multiagent_podcast_audio.sh`

## Voice Routing

Default routing:

- `ChatGPT`: native OpenAI TTS
- `Gemini`: native Gemini TTS
- `Claude`: surrogate OpenAI TTS

Default native Gemini voice:

- `Kore`

This follows the official Gemini TTS examples more closely than the earlier
experimental voice choice.

Operational override:

- if Gemini native TTS preview is unavailable or too unstable for a scheduled
  weekly run, the wrapper supports an explicit truthful override:
  - `ADL_PODCAST_GEMINI_AUDIO_PROVIDER=openai`

That override is meant as a bounded operational escape hatch, not a silent
default.

Current practical truth:

- the preferred default is still native Gemini TTS for `Gemini`
- the verified fallback path may render `Gemini` through the explicit OpenAI
  override when the native Gemini preview lane is unavailable or unstable
- a minimal official-shape Gemini TTS sample succeeded locally during
  development, so the remaining risk appears to be full-episode preview-lane
  reliability rather than total incompatibility

This keeps the authorship claim honest:

- Claude writes Claude's lines
- another renderer voices them when native public API audio is unavailable

## Primary Proof Surfaces

The paths below are operator-generated runtime outputs. They are written when
 the canonical command is executed and are not tracked artifacts in the primary
 checkout.

- `artifacts/v0911/multiagent_podcast_pilot_audio/episode.wav`
- `artifacts/v0911/multiagent_podcast_pilot_audio/audio_manifest.json`
- `artifacts/v0911/multiagent_podcast_pilot_audio/audio_packet.md`

## Secondary Proof Surfaces

- `artifacts/v0911/multiagent_podcast_pilot_audio/source_episode/transcript.md`
- `artifacts/v0911/multiagent_podcast_pilot_audio/source_episode/proof_note.md`
- `artifacts/v0911/multiagent_podcast_pilot_audio/audio/segments/*.wav`

## Success Signal

The audio follow-on is successful when:

- one listenable combined episode audio file exists
- per-turn audio segments exist with explicit speaker routing
- the manifest shows which provider/voice rendered each speaker
- Claude's surrogate rendering is disclosed instead of implied away
- each segment begins by naming the speaker
- overall segment volume is normalized enough that the episode does not feel
  erratic or lopsided
