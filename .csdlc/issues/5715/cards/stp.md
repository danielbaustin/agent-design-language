# Structured Task Prompt

Template: 1.0.0

Issue: 5715

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Integrate the attached studio page export into the generated podcast demo route while preserving the export bytes and #5711 audio/RSS behavior.

## Deliverables

- updated podcast generator studio-copy path
- generated demos/podcast/studio route
- committed studio reference bundle with clean podcast-studio.html filename
- validated RSS/feed output
- focused validation updates for source/copy digest, route wiring, audio, and RSS
- truthful issue-local lifecycle record

## Acceptance

1. AC-1: The exported studio HTML content is preserved byte-for-byte from the operator zip in both the reference copy and served studio copy.
2. AC-2: The integrated studio HTML uses clean podcast-studio.html filenames only, with no .dc suffix and no filenames with spaces.
3. AC-3: demos/podcast/index.html links to the studio route.
4. AC-4: The studio route opens the exported HTML and preserves its referenced image/script assets.
5. AC-5: The audio player and generated/local audio artifact remain valid.
6. AC-6: demos/podcast/feed.xml remains well-formed RSS with a valid audio enclosure.
7. AC-7: Validation covers generation, route wiring, source/copy digest identity, RSS, and audio artifact existence.

## Dependencies

- #5711 merged audio/RSS launch foundation
- operator-provided Podcast studio page design zip

## Inputs

- demos/podcast/index.html
- demos/podcast/feed.xml
- demos/podcast/episodes/meet-the-ai-coworkers/index.html
- demos/podcast/studio-reference/podcast-studio.html
- demos/podcast/studio/podcast-studio.html
- adl/tools/generate_podcast_launch_packet.py
- adl/tools/validate_podcast_launch_packet.py
- adl/tools/test_podcast_launch_packet.sh
- docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json

## Non Goals

- production deployment
- editing the exported reference HTML content
- rewriting the exported text or images
- committing the operator zip without explicit approval
- replacing working audio/RSS with a visual-only mock
