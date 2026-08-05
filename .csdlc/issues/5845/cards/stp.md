# Structured Task Prompt

Template: 1.0.0

Issue: 5845

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Create and validate the ten review-ready production packages using retained Podcast Studio v2 proof and the approved route/storage decision; do not deploy, submit to directories, verify mailboxes, or claim publication.

## Deliverables

- Ten complete episode directories
- Ten final MP3 and archive-audio artifacts with manifests
- Validated feed and enclosure records
- Audio, redaction, guest, metadata, playback, and listen-check QA
- Exact-head editorial/audio review

## Acceptance

1. AC-1: All ten episodes contain every required script, audio, transcript, note, metadata, artwork, enclosure, redaction, QA, and review artifact.
2. AC-2: Audio digests, duration, sample rate, channels, loudness, peak, ID3/artwork, listen check, and archive records are internally consistent.
3. AC-3: Feed/enclosure records match episode specs and reject local paths, drafts, unstable GUIDs, missing bytes, and MIME/duration mismatches.
4. AC-4: Guest consent, DeepSeek invitation language, source rights, credentials, and redaction remain truthful and privacy-safe.
5. AC-5: macOS, Linux, desktop-browser, and iOS Safari evidence is recorded where required, with no deployment or publication claim.

## Dependencies

- #5819 canonical naming/link truth
- #3223 and #3256 retained Podcast Studio v2 proof
- Agent Logic route/storage decision

## Inputs

- .adl/docs/TBD/PODCAST_STUDIO_NEXT_WEEK_LAUNCH_PLAN_5702.md
- docs/milestones/v0.91.3/review/podcast_studio_v2/
- docs/milestones/v0.91.8/review/podcast_launch_5711/
- demos/podcast/LAUNCH_READINESS.md
- adl/tools/generate_podcast_launch_packet.py
- adl/tools/validate_podcast_launch_packet.py

## Non Goals

- Deployment or podcast-directory submission
- Mailbox verification
- Treating historical smoke artifacts as final
- Claiming guest acceptance, weekly cadence, or a public route without proof
- Article or infrastructure ownership
