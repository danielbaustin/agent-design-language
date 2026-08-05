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

## Owned Paths

- `demos/podcast/episodes/001-meet-the-ai-coworkers`
- `demos/podcast/episodes/002-can-an-ai-be-a-good-teammate`
- `demos/podcast/episodes/003-the-promise-and-weirdness-of-talking-to-machines`
- `demos/podcast/episodes/004-what-should-we-let-ai-do-for-us`
- `demos/podcast/episodes/005-can-ai-help-us-think-better`
- `demos/podcast/episodes/006-the-new-creative-room`
- `demos/podcast/episodes/007-trust-receipts-and-proof`
- `demos/podcast/episodes/008-local-ai-vs-cloud-ai`
- `demos/podcast/episodes/009-when-ai-gets-stuck`
- `demos/podcast/episodes/010-what-does-a-weekly-ai-studio-look-like`
- `demos/podcast/feed.xml`
- `demos/podcast/LAUNCH_READINESS.md`
- `adl/tools/generate_podcast_launch_packet.py`
- `adl/tools/validate_podcast_launch_packet.py`
- `adl/tools/test_podcast_launch_packet.sh`
- `adl/tools/record_podcast_native_playback.sh`
- `adl/tools/record_podcast_browser_playback.mjs`
- `adl/tools/record_podcast_ios_safari_playback.sh`
- `.csdlc/prepared/issues/5845/validate-platform-playback-receipts.rb`
- `.csdlc/prepared/issues/5845/validate-second-pass-readiness.rb`
- `.csdlc/evidence/5845`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

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

## Native Playback Evidence Contract

Implementation must add three bounded evidence producers under the owned
paths. They are product-validation deliverables, not publication tools:

- macOS and Linux native playback:
  `bash adl/tools/record_podcast_native_playback.sh --platform <macos|linux> --source-sha <sha> --episode demos/podcast/episodes/001-meet-the-ai-coworkers/episode.mp3 --evidence-dir .csdlc/evidence/5845/platform/<platform>-native`
- desktop Chromium playback:
  `node adl/tools/record_podcast_browser_playback.mjs --browser chromium --source-sha <sha> --episode-url <loopback-url> --evidence-dir .csdlc/evidence/5845/platform/desktop-chromium`
- physical-device iOS Safari playback:
  `bash adl/tools/record_podcast_ios_safari_playback.sh --source-sha <sha> --device-id-hash <sha256> --episode-url <device-reachable-url> --evidence-dir .csdlc/evidence/5845/platform/ios-safari-device`

Each producer must capture one complete playback of the canonical episode,
write its capture artifact below the named evidence directory, and emit a
`receipt.json`. Credentials, raw device identifiers, and externally reachable
tokens are forbidden. Loopback or device-reachable URLs are evidence inputs,
not publication claims.

The receipt is an object with `schema`, `payload`, and `payload_sha256`.
`schema` is `adl.podcast_playback_receipt.v1`. The payload contains:

- `platform_id`: exactly `macos-native`, `linux-native`,
  `desktop-chromium`, or `ios-safari-device`
- `source_sha`: the exact 40-hex candidate commit
- `argv`: the complete producer command, including the expected owned script
- `runner`: nonempty `kind`, `os`, `os_version`, `architecture`, and
  privacy-safe `identity`
- `device`: required nonempty browser/version for desktop Chromium and
  hashed device/model/OS/Safari identity for iOS; native lanes use `null`
- `media_path` and `media_sha256`: repo-relative final episode input and its
  recomputed SHA-256
- `capture_path` and `capture_sha256`: repo-relative playback capture and its
  recomputed SHA-256
- `started_at`, `ended_at`, and `duration_seconds`
- `result`: `passed`, `playback_started`, `playback_completed`, `audible`, and
  `controls_operable`, all true

`.csdlc/prepared/issues/5845/validate-platform-playback-receipts.rb` is the
fail-closed validator. It recomputes the canonical payload digest and both file
digests, binds every receipt to one requested source SHA, verifies the expected
producer script and platform flag, requires all four distinct platform IDs,
rejects paths outside the repository or `.csdlc/evidence/5845/platform/`, and
rejects missing native runner or browser/device identity. Hand-authored status
fields without the bound files, command, and digests are not proof.

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
