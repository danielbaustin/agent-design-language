# Five-Agent Hey Jude MIDI Flagship Demo

Canonical command:

```bash
bash adl/tools/demo_v0891_five_agent_hey_jude.sh
```

## What It Does

This `v0.89.1` flagship demo turns the earlier multi-provider groundwork into a
five-participant performance packet:

- Layer 8 as the human bandleader
- ChatGPT
- Claude
- Gemini
- DeepSeek

The package stays transcript-first and bounded. It uses section cues instead of
a tracked lyric sheet, and it records one real MIDI cue layer through a small
profile-driven bridge surface.

## Why It Matters

This demo proves something more memorable than another review packet:

- one human and four provider voices can coordinate on one ADL runtime
- the cue layer is visible rather than hand-waved
- the performance can be warm and strange without becoming structurally vague

## Primary Proof Surfaces

- `cast.json`
- `section_plan.json`
- `cue_timeline.json`
- `transcript.md`
- `midi_event_log.json`
- `performance_summary.md`
- `runtime/runs/v0-89-1-five-agent-hey-jude-midi-demo/run_summary.json`

## Notes

- This demo explicitly builds on the earlier multi-agent discussion and
  provider-harmony groundwork.
- It does not store a full lyric sheet in tracked repo artifacts.
