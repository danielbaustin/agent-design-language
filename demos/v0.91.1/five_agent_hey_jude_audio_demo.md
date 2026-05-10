# Five-Agent Hey Jude Audio Follow-On

Canonical command:

```bash
bash adl/tools/demo_v0911_five_agent_hey_jude_audio.sh
```

## What It Does

This `v0.91.1` follow-on keeps the original `v0.89.1` five-agent `Hey Jude`
proof packet intact, then adds one bounded spoken-performance audio layer on
top.

The follow-on:

- reuses the source transcript/MIDI/cue packet from the original demo
- accepts an operator-supplied local backing WAV through
  `ADL_HEY_JUDE_BACKING_WAV`
- renders five timed spoken interjections for the existing cast
- exports one mixed replayable audio artifact plus aligned timing/transcript
  proof surfaces

## Inputs

- optional local backing track:
  - `ADL_HEY_JUDE_BACKING_WAV=/path/to/local_backing.wav`
- optional output directory:
  - `bash adl/tools/demo_v0911_five_agent_hey_jude_audio.sh /custom/out`

If no backing WAV is supplied, the follow-on still renders a voice-only mix so
the proof route remains runnable.

## Primary Proof Surfaces

- `episode.wav`
- `audio_manifest.json`
- `audio_packet.md`
- `proof_note.md`
- `playback_transcript.md`
- `cue_timing_register.json`
- `source_episode/performance_manifest.json`
- `source_episode/transcript.md`

## Why It Matters

This turns the original flagship from:

- a transcript-and-MIDI proof packet

into:

- a more listenable replay artifact that still keeps the coordination evidence
  visible and reviewable

## Bounded Claim

This follow-on proves:

- the existing ensemble timing plan can drive a replayable spoken-performance
  mix
- the source proof packet remains intact and machine-reviewable
- a local operator-supplied backing track can be layered in without shipping
  copyrighted audio in the repo

## Non-Claims

- no full synthetic singing of the song
- no repo-distributed copyrighted backing track
- no generalized music engine
- no claim of studio mastering or live-performance reliability
