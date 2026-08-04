# Structured Task Prompt

Template: 1.0.0

Issue: 5708

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Recover lifecycle truth for the existing bounded Podcast Studio redesign merge only.

## Deliverables

- Exact committed-patch validation evidence
- Bounded exact-head review evidence
- Merged PR reconciliation
- Retained terminal receipt

## Acceptance

1. AC-1: The exact implementation head and merged PR are recorded.
2. AC-2: The focused deterministic Podcast Studio packet test passes and the committed patch has no whitespace errors.
3. AC-3: A bounded review confirms generator/rendered agreement and public-launch non-claims.
4. AC-4: Terminal evidence is retained without changing product content.

## Dependencies

- Closed issue #5708
- Merged PR #5709

## Inputs

- adl/tools/generate_podcast_studio_v2_packet.py
- adl/tools/validate_podcast_studio_v2_packet.py
- demos/v0.91.3/adl_podcast_studio_v2_episode_card.html
- docs/milestones/v0.91.3/review/podcast_studio_v2

## Non Goals

- Changing documentation content
- Launching a public podcast route, RSS feed, or final audio
- Publishing another PR
