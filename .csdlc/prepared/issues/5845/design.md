# Issue 5845 Design: Podcast Studio First Ten Episodes

## Decision

WP-24A produces ten complete review-ready production packages, not topic cards
or smoke artifacts. Every episode includes final script, transcript, show
notes, final MP3 plus archive audio, QA and listen-check evidence, guest truth,
artwork/ID3 metadata, RSS-ready enclosure data, redaction results, and
editorial/audio review. Publication and deployment remain separately gated.

## Source Baseline

- `.adl/docs/TBD/PODCAST_STUDIO_NEXT_WEEK_LAUNCH_PLAN_5702.md`
- `docs/milestones/v0.91.3/review/podcast_studio_v2/`
- `docs/milestones/v0.91.8/review/podcast_launch_5711/`
- `demos/podcast/LAUNCH_READINESS.md`
- `adl/tools/generate_podcast_launch_packet.py`
- `adl/tools/validate_podcast_launch_packet.py`
- existing WAV/feed routes are smoke proof only, not ten final episodes.

## Proposed Artifacts And Protected-Path Candidates

- `demos/podcast/episodes/001-meet-the-ai-coworkers/` through `010-what-does-a-weekly-ai-studio-look-like/`
- `demos/podcast/audio/`
- `demos/podcast/feed.xml`
- `demos/podcast/LAUNCH_READINESS.md`
- `adl/tools/generate_podcast_launch_packet.py`
- `adl/tools/validate_podcast_launch_packet.py`
- `adl/tools/test_podcast_launch_packet.sh`
- `.csdlc/evidence/5845/`

The Agent Logic public route or storage repository is not claimed until #5819
and the route/storage decision identify the canonical destination. Cross-repo
publication must use its own authorized lifecycle.

## Episode Package Contract

Each numbered directory contains `episode.yaml`, `source-packet.md`,
`script.md`, `transcript.md`, `show-notes.md`, `episode.mp3`, archive audio,
`audio-manifest.json`, `qa-report.md`, `guest-metadata.json`, artwork,
`rss-enclosure.json`, `redaction-report.md`, and `review.md`. Provider/model,
source digest, output digest, duration, sample rate, channels, loudness, peak,
ID3 version/tags, artwork digest, and listen-check result are recorded without
credentials.

## Execution Plan

1. Verify #5819, #3223/#3256 retained Podcast Studio v2 proof, and the current Agent Logic route/storage decision.
2. Lock the ten episode briefs and guest states from the #5702 plan.
3. Produce full scripts, audio segments, final audio, transcripts, notes, metadata, and artwork for all ten.
4. Generate RSS-ready enclosure records and validate episode-to-feed parity without deploying.
5. Run audio QA, redaction, missing-asset, guest-consent, metadata, and platform playback checks.
6. Complete editorial/audio review and exact-head issue review.

## Production Wave Budget

The ten episodes are ten full production waves. Each episode budgets 8
agent-hours and 70,000 model tokens: 2 hours and 24,000 tokens for source work
and the final script; 2.5 hours and 18,000 tokens for generation, editing, and
audio mastering; 1.5 hours and 14,000 tokens for transcript, show notes,
artwork, metadata, and enclosure data; 1.5 hours and 10,000 tokens for listen
review and revisions; and 30 minutes and 4,000 tokens for machine validation
and packaging. The aggregate is 80 agent-hours and 700,000 tokens. With five
independent episode owners, allow 16-24 hours wall-clock plus 4-6 hours for
feed-wide consistency, platform playback, exact-head review, and final
revisions. Budget exhaustion blocks the episode instead of permitting smoke
audio, draft metadata, or skipped listen review.

## Negative And Platform Lanes

- Missing/silent/clipped audio, digest mismatch, invalid ID3/artwork, or failed listen check blocks the episode.
- Draft or unapproved human guest metadata cannot appear as confirmed.
- Episode 002 may invite DeepSeek but cannot imply acceptance or persistent identity.
- Feed data must reject local paths, draft episodes, unstable GUIDs, missing enclosure bytes, and MIME/duration mismatch.
- macOS and Linux validators must agree; browser playback includes desktop and iOS Safari evidence where required.
- Tests set `TMPDIR` inside `.csdlc/evidence/5845/`; no system temporary directory is used.

## Non-Goals

- Automatic deployment, podcast-directory submission, mailbox verification, or publication approval.
- Treating the historical smoke WAV/feed as final audio.
- Claiming a weekly cadence, guest acceptance, or public route before live proof.
- Absorbing article production or Agent Logic infrastructure ownership.

## Exit Evidence

All ten packages are complete and independently reviewable, final audio and
metadata pass machine and human QA, RSS records match episode specs, redaction
passes, and exact-head review has no unresolved actionable finding.
