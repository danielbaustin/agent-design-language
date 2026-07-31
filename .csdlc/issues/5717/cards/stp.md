# Structured Task Prompt

Template: 1.0.0

Issue: 5717

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Apply the operator-requested studio copy/logo/layout fixes without redesigning the page or modifying podcast generator behavior.

## Deliverables

- updated studio reference HTML
- updated generated served studio HTML
- updated studio reference/generated digest files
- copied Agent Logic logo asset in the studio bundle
- focused validation evidence for studio route, digest, audio, and RSS
- truthful issue-local lifecycle record

## Acceptance

1. AC-1: The studio page uses the Agent Logic logo asset rather than the placeholder/export logo.
2. AC-2: The visible show name is Synthetic Minds Podcast.
3. AC-3: The hero copy says 'Special guests join us occasionally.'
4. AC-4: Episodes are proposed launch topics numbered from 1, with fake historical numbers removed.
5. AC-5: The gap below the Listen now button is reduced without disrupting the design.
6. AC-6: The contact link uses podcast@agent-logic.ai.
7. AC-7: The FAQ answer says there is no video.
8. AC-8: FAQ remains capitalized.
9. AC-9: The footer puts the copyright line and disclaimer on separate lines.
10. AC-10: Existing studio assets load, and audio/RSS validation still passes.

## Dependencies

- #5715 integrated studio route foundation
- #5711 audio/RSS launch foundation

## Inputs

- demos/podcast/studio-reference/podcast-studio.html
- demos/podcast/studio-reference/REFERENCE_DIGESTS.txt
- demos/podcast/studio/podcast-studio.html
- demos/podcast/studio/REFERENCE_DIGESTS.txt
- demos/podcast/studio/reference.sha256
- docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json

## Non Goals

- production deployment
- redesigning the studio page beyond the requested copy/logo/spacing fixes
- changing podcast generator code
- changing audio or RSS implementation
- claiming that proposed episode topics are recorded episodes
