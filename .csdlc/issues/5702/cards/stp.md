# Structured Task Prompt

Template: 1.0.0

Issue: 5702

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare the reviewable launch plan only; do not implement website, generator, audio, RSS, or deployment changes in this issue.

## Deliverables

- `.adl/docs/TBD/PODCAST_STUDIO_NEXT_WEEK_LAUNCH_PLAN_5702.md`
- Captured Gemini review request/result or truthful unavailable note
- Validation that referenced source files exist and Markdown is readable

## Acceptance

1. AC-1 The plan defines all same-day workstreams needed for next-week launch, including audio, RSS, ten episode specs, guest support, Deepgram investigation, and site design alignment
2. AC-2 Audio and RSS are launch blockers with explicit tests and fail-closed criteria
3. AC-3 The plan separates source-backed facts, launch decisions, assumptions, open questions, and non-claims
4. AC-4 The plan includes a practical today schedule and multiple validation passes
5. AC-5 Gemini is asked to review the plan and suggestions are recorded or truthfully marked pending/unavailable

## Dependencies

- #5605 podcast revival/readiness planning
- Existing v0.91.1 podcast audio artifacts
- Existing v0.91.3 Podcast Studio v2 packet
- Agent Logic website styling/site structure
- Gemini provider availability

## Inputs

- docs/milestones/v0.91.8/features/AI_AGENT_PODCAST_STUDIO_v0.91.8.md
- docs/milestones/v0.91.8/review/podcast_studio_5605/PODCAST_STUDIO_LAUNCH_READINESS_5605.md
- docs/milestones/v0.91.8/review/podcast_studio_5605/PODCAST_STUDIO_TOPIC_SLATE_5605.md
- docs/milestones/v0.91.3/review/podcast_studio_v2/PODCAST_STUDIO_V2_PACKET_v0.91.3.md
- demos/v0.91.3/adl_podcast_studio_v2_episode_card.html
- artifacts/v0911

## Non Goals

- Implementing the podcast generator
- Deploying `agent-logic.ai/podcast`
- Generating all ten final episodes
- Selecting Deepgram before investigation
- Creating public guest claims before confirmation
