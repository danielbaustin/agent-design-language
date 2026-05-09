# Multi-Agent Podcast Pilot Demo

## Summary

This bounded `v0.91.1` demo turns the earlier multi-agent conversation wave
into one recurring transcript-first episode format.

Unlike the earlier panel that discussed whether a podcast should exist, this
pilot is an actual episode run with:

- one bounded topic
- one stable three-participant role split
- one explicit episode contract
- one reusable packet shape for future episodes
- one intended recurring weekly cadence

## Scope Boundary

This pilot proves:

- ADL can run one reusable transcript-first podcast episode through the
  existing three-provider multi-agent runtime path
- participant roles can stay explicit and attributable across the episode
- the episode packet can preserve transcript, proof note, role story, and
  reviewability without widening into a media platform

It does **not** prove:

- stable long-term identity continuity across episodes
- native audio production for all providers
- an always-on show infrastructure
- autonomous multi-agent federation or social cognition

## Canonical Command

From repository root:

```bash
bash adl/tools/demo_v0911_multiagent_podcast_pilot.sh
```

Useful bounded overrides:

- `ADL_PODCAST_TOPIC`
- `ADL_PODCAST_EPISODE_TITLE`
- `ADL_PODCAST_SERIES_NAME`
- `ADL_LIVE_OPENAI_MODEL`
- `ADL_LIVE_GEMINI_MODEL`
- `ADL_LIVE_ANTHROPIC_MODEL`

## What Runs

- dedicated podcast pilot wrapper:
  - `adl/tools/demo_v0911_multiagent_podcast_pilot.sh`
- dedicated runtime workflow:
  - `adl/examples/v0-91-1-multi-agent-podcast-pilot.adl.yaml`
- provider bridge:
  - `adl/tools/real_chatgpt_gemini_claude_provider_adapter.py`

## Stable Pilot Roles

- `ChatGPT`: host / synthesizer
- `Gemini`: challenger / systems analyst
- `Claude`: refiner / moral stylist

## Recurring Series Intention

This pilot is designed as the first bounded episode in a repeatable series:

- target cadence: `1 episode / week`
- stable infrastructure: same canonical wrapper and packet shape
- bounded per-episode variation: topic, title, model overrides, and later audio
  rendering choices

The point is to make the format regularly repeatable without turning the pilot
into a broad media-platform commitment.

## Tracked Episode Contract

- `demos/v0.91.1/multiagent_podcast_episode_contract.md`

## Primary Proof Surfaces

The paths below are operator-generated runtime outputs. They are written when
the canonical command is executed and are not tracked artifacts in the primary
checkout.

- `artifacts/v0911/multiagent_podcast_pilot/transcript.md`
- `artifacts/v0911/multiagent_podcast_pilot/proof_note.md`
- `artifacts/v0911/multiagent_podcast_pilot/episode_contract.json`
- `artifacts/v0911/multiagent_podcast_pilot/best_lines.md`

## Secondary Proof Surfaces

- `artifacts/v0911/multiagent_podcast_pilot/provider_invocations.json`
- `artifacts/v0911/multiagent_podcast_pilot/observatory_projection.json`
- `artifacts/v0911/multiagent_podcast_pilot/series_manifest.json`
- `artifacts/v0911/multiagent_podcast_pilot/episode_packet.md`
- `artifacts/v0911/multiagent_podcast_pilot/runtime/runs/v0-91-1-multi-agent-podcast-pilot/run_summary.json`
- `artifacts/v0911/multiagent_podcast_pilot/runtime/runs/v0-91-1-multi-agent-podcast-pilot/logs/trace_v1.json`

## Default Episode Topic

The default pilot question is:

- `Should AI systems have consistent personalities across conversations?`

That topic was chosen because it is:

- directly relevant to the recurring-series idea
- broad enough to invite disagreement
- concrete enough to avoid abstract drift

Later weekly episodes can rotate topics while preserving the same packet and
proof structure.

## Success Signal

The pilot is successful when:

- one saved six-turn episode packet exists under a stable output root
- all three participants remain explicit and easy to distinguish
- the episode reads like one coherent show segment rather than three detached monologues
- the proof note remains honest about what the pilot does not prove
- the packet shape is stable enough to be run again next week without bespoke
  redesign
