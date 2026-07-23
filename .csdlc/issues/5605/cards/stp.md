# Structured Task Prompt

Template: 1.0.0

Issue: 5605

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Create and validate podcast revival planning artifacts for future launch execution.

## Deliverables

- Podcast studio feature plan
- Launch-readiness packet for `agent-logic.ai/podcast`
- First ten-topic slate
- v0.91.8 feature/review/demo/proof index links

## Acceptance

1. AC-1: old v0.91.1/v0.91.3 podcast artifacts are inventoried as historical evidence, not live proof
2. AC-2: `agent-logic.ai/podcast` route, page, archive, feed, studio, and launch gates are specified
3. AC-3: first ten weekly topics are documented with a non-geeky audience posture and week-two DeepSeek invitation option
4. AC-4: v0.91.8 feature, review, demo, and proof indexes link the new planning surfaces
5. AC-5: validation proves referenced paths exist, stale host paths/secrets are absent, old packet demo still passes, and bounded review is clean

## Dependencies

- Historical podcast demos and packets from v0.91.1 and v0.91.3
- Future agent-logic.ai website execution issue for live route deployment

## Inputs

- demos/v0.91.1/multiagent_podcast_pilot_demo.md
- demos/v0.91.1/multiagent_podcast_audio_demo.md
- adl/tools/demo_v0911_multiagent_podcast_pilot.sh
- adl/tools/demo_v0911_multiagent_podcast_audio.sh
- adl/tools/demo_v0913_podcast_studio_v2.sh
- adl/tools/generate_podcast_studio_v2_packet.py
- docs/milestones/v0.91.3/review/podcast_studio_v2/
- demos/v0.91.3/adl_podcast_studio_v2_episode_card.html

## Non Goals

- No live `agent-logic.ai/podcast` deployment
- No RSS feed publication
- No new audio production
- No guest outreach or representation that a guest has accepted
- No durable weekly cadence claim
