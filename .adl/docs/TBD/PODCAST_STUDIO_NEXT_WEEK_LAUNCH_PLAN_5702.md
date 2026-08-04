# Podcast Studio Next-Week Launch Plan

## Metadata

- Issue: `#5702`
- Status: internal review draft
- Target launch: week of 2026-08-03
- Plan date: 2026-07-28
- Public route target: `https://agent-logic.ai/podcast`
- Launch posture: audio and RSS are required launch gates
- Implementation status: not started by this plan

## Objective

Launch the revived AI Agent Podcast next week with a production-ready Podcast
Studio path that can reliably produce, review, package, and publish ten prepared
episodes, including audio, RSS, guest metadata, and a public page that matches
the Agent Logic website.

This plan is not a launch claim. It is the same-day execution plan that should
be reviewed before implementation begins.

## Source-Backed Facts

- `docs/milestones/v0.91.8/features/AI_AGENT_PODCAST_STUDIO_v0.91.8.md`
  records the revived Podcast Studio goal, planned `agent-logic.ai/podcast`
  route, ten-topic slate, DeepSeek week-2 guest shape, and the older non-claims.
- `docs/milestones/v0.91.8/review/podcast_studio_5605/PODCAST_STUDIO_LAUNCH_READINESS_5605.md`
  records the current launch-readiness plan and says the Agent Logic site had
  no observed podcast route at inspection time.
- `docs/milestones/v0.91.8/review/podcast_studio_5605/PODCAST_STUDIO_TOPIC_SLATE_5605.md`
  records the first ten accessible, not-too-geeky episode topics.
- `docs/milestones/v0.91.3/review/podcast_studio_v2/PODCAST_STUDIO_V2_PACKET_v0.91.3.md`
  records the deterministic Podcast Studio v2 proof packet and its non-claims:
  no live provider-backed generation, no final audio proof, and no publishing
  readiness proof.
- `demos/v0.91.3/adl_podcast_studio_v2_episode_card.html` is the existing
  polished episode-card artifact.
- Existing audio artifacts include five complete `episode.wav` files under the
  root checkout's `artifacts/v0911/` tree, with the newest observed complete
  recording at
  `artifacts/v0911/multiagent_podcast_pluribus_audio_verified_v4_audio/episode.wav`.
- The production Agent Logic website source is not present in this ADL
  worktree. Website implementation must inspect the Agent Logic site repository
  before choosing exact paths, components, classes, or deployment commands.
  The design requirement in this plan is therefore visual and behavioral
  alignment with the existing Agent Logic site, not a claim about local `site/`
  paths in ADL.

## Current Decision Changes

The earlier #5605 plan treated audio and RSS as optional or deferrable. For the
next-week launch, that is no longer acceptable.

Launch blockers:

- audio rendering and audio QA must pass;
- RSS/feed generation and validation must pass;
- ten episode specs must exist before launch;
- guest support must be represented in the episode schema;
- the site route must look native to Agent Logic, not like the older dark demo
  card dropped into the production site.

## Non-Claims

- This plan does not launch `agent-logic.ai/podcast`.
- This plan does not prove the audio path, RSS path, Deepgram quality, guest
  confirmation, site deployment, or weekly cadence.
- This plan does not select Deepgram as the production audio provider.
- This plan does not claim DeepSeek or any human guest has accepted an
  invitation.
- This plan does not authorize public claims before implementation proof and
  operator approval.

## Gemini 3.1 Pro Review Incorporation

Gemini review is required to use Gemini 3.1 Pro. Live model discovery for this
credential exposes the generateContent model as `gemini-3.1-pro-preview`
(`Gemini 3.1 Pro Preview`); the plain `gemini-3.1-pro` API id returned 404 and
is not accepted as a completed review. Earlier non-3.1-Pro Gemini attempts are
retained only as historical local attempts and are not accepted as review
evidence for this issue. The current acceptable review artifact is the tracked
summary under
`.csdlc/evidence/5702/gemini-3.1-pro-review-summary.json`, with the raw local
provider response retained under
`.adl/local-artifacts/5702-podcast-launch-plan/gemini-review-result.json`.

Actionable suggestions incorporated:

- Use the live API id `gemini-3.1-pro-preview` as the current Gemini 3.1 Pro
  review route and record the failed plain-id attempts truthfully.
- Prioritize one proven audio route before evaluating a new vendor.
- Automate episode directory/spec generation instead of preparing ten episodes
  manually.
- Clarify DeepSeek guest metadata as placeholder/invited-state unless live
  guest participation is separately proven.
- Define the audio hosting/CDN strategy before RSS enclosure URLs are generated.
- Name concrete RSS validation, byte-range, ID3, artwork, encoding, CORS,
  cache, audio quality, redaction, website, player, and fail-closed tests.
- Build the website route from Agent Logic templates/components first, not from
  the old standalone demo card.
- Lock real Episode 001 script/content before using the audio pipeline as
  launch proof; placeholder audio is allowed only as non-launch harness proof.
- Validate RSS with Apple Podcasts Connect or an equivalent Apple-specific
  preflight before making public directory-readiness claims.
- Account for Apple Podcasts approval timing by treating submission readiness as
  urgent once Episode 001 audio and feed URLs are stable.
- Resolve the URL-order dependency before feed generation: define podcast index,
  episode, feed, and audio URL structure in the site repository before emitting
  final RSS.
- Require XML escaping or CDATA handling for rich show-note/transcript fields.
- Require TTS chunking/retry proof before a full episode audio run can count as
  launch evidence.
- Prefer ID3v2.3 tagging for directory/player compatibility.
- Add iOS Safari playback to the website/player proof gate.
- Treat human guest release/permission state as a publish-blocking field, not
  just metadata.

Resulting critical-path change: audio proof moves before broad episode-factory
polish, and Deepgram remains a required investigation that must not consume the
launch-critical slot until the existing audio route can produce a validated
episode. Deepgram can become the selected route only if it passes the comparison
after the baseline route is green.

Operator-decision item:

- Gemini 3.1 Pro recommends deferring the Deepgram comparison to a post-launch
  optimization issue unless the existing audio route is already green. The
  operator requirement still asks us to investigate Deepgram, so this plan keeps
  the investigation lane but makes it non-blocking behind audio/RSS proof.

## Same-Day Execution Schedule

### Hour 0: Issue, Scope, And Ownership

Deliverables:

- #5702 initialized and bound to a FastWork worktree.
- Protected paths include the plan, lifecycle records, and evidence records.
- Implementation follow-on issues identified but not opened until this plan is
  reviewed.

Done when:

- root `main` remains untouched by tracked edits;
- the plan is reviewable under `.adl/docs/TBD/`;
- the implementation boundary is explicit.

### Hour 1: Episode Data Contract

Create a versioned episode spec contract that drives every downstream artifact.
Implement it as a generator/validator, not hand-authored files.

Required fields:

- `episode_id`
- `episode_number`
- `title`
- `slug`
- `publish_date`
- `status`
- `canonical_url`
- `hosts`
- `guests`
- `guest_invitation_status`
- `topic`
- `listener_question`
- `source_packet`
- `transcript`
- `show_notes`
- `audio`
- `rss`
- `review`
- `redaction`
- `publication`

Guest support:

- AI guests, such as DeepSeek, must include provider/model identity, run
  evidence, role in episode, and a non-claim about durable identity.
- Human guests must include invitation status, release/permission state, public
  bio, headshot/license state if used, and quote approval state.
- Guest metadata must support `invited`, `tentative`, `confirmed`,
  `recorded`, `released`, and `declined` states.

Fail-closed cases:

- unsafe slug;
- missing title;
- duplicate episode number;
- `publish_ready=true` without review pass;
- audio claim without audio manifest and QA;
- RSS claim without feed validation;
- guest claim without invitation/confirmation status.

Required tooling shape:

- one simple source table or YAML list for the ten planned episodes;
- generator creates directory skeletons, `episode.yaml`, `source-packet.md`,
  `transcript.md`, `show-notes.md`, `audio-manifest.json`, `review.md`, and
  `redaction.md`;
- validator checks schema, episode uniqueness, publish gates, guest states,
  audio gates, and RSS gates.

### Hour 2: Ten Episode Prep Factory

Prepare ten episode directories/specs up front. These do not need final scripts
today, but they must be complete enough to prevent weekly scramble.

Required tree:

```text
podcast/
  episodes/
    001-meet-the-ai-coworkers/
      episode.yaml
      source-packet.md
      transcript.md
      show-notes.md
      audio-manifest.json
      review.md
    ...
    010-what-does-a-weekly-ai-studio-look-like/
```

The first ten episodes are:

| Episode | Working title | Guest posture |
| --- | --- | --- |
| 001 | Meet the AI Coworkers | Core hosts |
| 002 | Can an AI Be a Good Teammate? | DeepSeek invited AI guest |
| 003 | The Promise and Weirdness of Talking to Machines | Core hosts |
| 004 | What Should We Let AI Do for Us? | Core hosts |
| 005 | Can AI Help Us Think Better? | Core hosts |
| 006 | The New Creative Room | Core hosts plus optional operator clip |
| 007 | Trust, Receipts, and Proof | Core hosts |
| 008 | Local AI vs Cloud AI | Core hosts |
| 009 | When AI Gets Stuck | Core hosts |
| 010 | What Does a Weekly AI Studio Look Like? | Core hosts, retrospective |

Done when:

- all ten specs validate;
- episode 1 and 2 have real source-packet outlines;
- episode 2 has a DeepSeek guest lane without implying acceptance or persistent
  identity.
- prep is generated by tooling, not copied by hand.

### Hour 3: Audio Pipeline

Audio is required for launch. The initial launch should support a primary route
and at least one fallback route.

Critical-path rule: harden the existing known route first. Do not switch the
launch-critical path to Deepgram until the current route can produce a validated
episode and Deepgram has beaten it on the comparison scorecard.

Required outputs per episode:

- final `episode.mp3`;
- lossless or high-quality archive source, preferably `episode.wav`;
- segment files for each speaker;
- loudness/peak report;
- embedded ID3v2 tags for title, artist, album/show, episode number, and
  artwork, preferably ID3v2.3 for broad player compatibility;
- audio manifest with provider, model, voice, renderer identity, source text
  digest, output digest, duration, sample rate, channel count, and QA result;
- human listen-check note.

Minimum audio QA:

- format check;
- encoding check with selected bitrate mode, bitrate, sample rate, and channel
  count, for example 128-192 kbps MP3 at 44.1 kHz unless the audio issue proves
  a better launch standard;
- ID3 tag check;
- duration check;
- no zero-byte or silent segment;
- no clipped peaks;
- loudness target recorded, with a launch target of `-16 LUFS +/- 1 LU` for
  stereo or `-19 LUFS +/- 1 LU` for mono unless the audio issue selects a
  different podcast standard;
- no audible clicks, pops, hard cuts, or robotic dropouts in the human
  listen-check;
- background noise below the selected threshold or explicitly accepted;
- intro/outro spacing sane;
- speaker voices distinguishable;
- final concatenation matches manifest segment order;
- audio digest stable after generation.
- TTS chunking/retry behavior is tested for rate limits, character limits,
  dropped connections, and mid-episode failure.

Existing route:

- The v0.91.1 audio demo already proved a multi-segment WAV production shape,
  but it is historical proof and must be rerun or replaced for launch.

Fallback strategy:

- primary production route: existing known-good route once rerun, automated, and
  reviewed;
- evaluation route: Deepgram TTS after the primary route is green;
- emergency fallback: use the best existing render only if operator explicitly
  accepts it as a historical/placeholder release asset. This is not the default.

Audio hosting decision:

- choose final audio location before RSS work begins;
- static hosting is acceptable only if bandwidth, cache behavior, content type,
  byte-range `206 Partial Content`, CORS, and stable HTTPS URLs are verified;
- if static hosting is insufficient, route audio through a CDN-backed asset path
  before feed enclosures are generated.

### Hour 4: Deepgram Investigation

Deepgram is an investigation lane, not a preselected vendor.

Current official Deepgram documentation indicates:

- Aura text-to-speech can synthesize audio through REST and supports model
  selection such as `aura-2-thalia-en`;
- Aura TTS request examples can output MP3/WAV and return response headers such
  as model/request metadata;
- Aura-2 input text limit is documented as 2000 characters per request;
- the Deepgram CLI supports `dg speak` for TTS with WAV/MP3/FLAC output;
- speech-to-text tooling supports transcription features such as diarization,
  smart formatting, summaries, topics, sentiment, and SRT/WebVTT subtitles;
- recent Aura-2 controls include speed and pronunciation controls.

Sources:

- Deepgram TTS getting started:
  `https://developers.deepgram.com/docs/text-to-speech`
- Deepgram Aura voice models:
  `https://developers.deepgram.com/docs/tts-models`
- Deepgram TTS CLI:
  `https://developers.deepgram.com/developer-tools/cli/text-to-speech`
- Deepgram STT CLI:
  `https://developers.deepgram.com/developer-tools/cli/speech-to-text`
- Deepgram May 2026 Aura-2 controls:
  `https://developers.deepgram.com/changelog/2026/5/4`

Evaluation tests:

- render the same three 20-30 second script snippets through current route and
  Deepgram;
- compare intelligibility, naturalness, pacing, speaker separation, artifacts,
  pronunciation, latency, repeatability, file format, and metadata quality;
- run Deepgram STT back over both outputs to compare transcript drift;
- produce a scorecard and listen-check note;
- do not select Deepgram unless it beats or materially complements the current
  route.

Launch priority:

- if Hour 3 audio is not green, Hour 4 is reassigned to primary audio hardening;
- Deepgram investigation still happens today if the primary route is green early
  enough;
- otherwise Deepgram becomes the first follow-on after launch-critical audio and
  RSS pass.

Deepgram pass criteria:

- no credential leakage in commands/logs/manifests;
- output files pass audio QA;
- pronunciation of `Agent Logic`, `ADL`, `DeepSeek`, and `C-SDLC` is acceptable
  or controllable;
- request limits are handled through segmenting;
- provider metadata is captured without secret material.

### Hour 5: RSS And Feed Contract

RSS is required for launch.

Required outputs, with exact paths chosen in the website repository after
source inspection:

- public podcast feed, expected route `/podcast/feed.xml`;
- per-episode `<enclosure>` pointing to an HTTPS MP3 URL;
- title, description, language, explicit flag, author/owner metadata,
  image/artwork, publication date, GUID, duration, and episode number;
- required podcast tags such as `itunes:duration` and `itunes:explicit`;
- rich text fields escaped safely or wrapped with XML-safe CDATA handling;
- Atom self-link if supported;
- podcast namespace tags if chosen;
- stable URL and MIME type.

RSS validation:

- XML well-formedness;
- feed URL resolves locally and after deploy;
- all enclosure URLs resolve;
- MIME type is `audio/mpeg`;
- enclosure URLs support HTTP byte-range requests for podcast streaming;
- audio host CORS supports the Agent Logic page if the player reads cross-origin
  assets;
- no draft episodes in public feed;
- dates are RFC 822-compatible;
- GUIDs stable and not reused;
- feed artwork is JPEG or PNG, square, RGB, within podcast-directory size
  bounds, and small enough for directory ingestion;
- RSS/feed cache policy and cache invalidation path are documented before
  deployment;
- feed validator pass recorded using a named validator such as Cast Feed
  Validator, Podbase Podcast Validator, or an equivalent documented validator;
- Apple Podcasts Connect draft/unlisted validation, or an explicit documented
  Apple-specific alternative if Connect is unavailable before launch;
- Apple Podcasts submission/approval timing captured before any "launch next
  week" claim, since approval can lag validation;
- feed content parsed and compared back to `episode.yaml`;
- manual import check in at least one podcast client or validator before public
  claim.

Fail closed if:

- `episode.mp3` is missing;
- enclosure size/duration mismatch is unverified;
- feed includes draft/held episodes;
- feed references local host paths or private artifact paths;
- external validator rejects the feed.

### Hour 6: Website Design Integration

The podcast route should be an Agent Logic page, not a standalone demo skin.

Use the Agent Logic site patterns after inspecting the site repository:

- same header/nav/brand treatment;
- same global stylesheet/design system;
- same section rhythm and typography scale;
- existing page-section, hero, button, navigation, grid, and card patterns where
  appropriate;
- podcast-specific cards should remain restrained and readable.

Required pages, with repo-local paths selected by the website implementation
issue:

- public podcast index route;
- public episode route for episode 001;
- public feed route;
- public audio asset location,
  depending on site convention selected by the website issue.

Landing page content:

- show title;
- one-sentence promise for non-engineers;
- latest episode with audio player;
- RSS link;
- episode archive;
- guest participation note;
- proof/receipts note written in public-friendly language;
- contact/invite link.

Episode page content:

- audio player;
- transcript;
- show notes;
- host/guest lineup;
- links and source notes;
- publication date;
- RSS/feed link;
- claim/non-claim note only where useful to readers.

Design checks:

- local render at desktop and mobile widths;
- at least Chrome and Safari local/browser checks if available;
- iOS Safari playback check for mobile audio initialization/playback behavior;
- audio player visible and usable;
- audio player actually plays, seeks, and reports duration/progress;
- RSS link discoverable;
- OpenGraph image/title/description set;
- broken-link check over the local rendered podcast route tree;
- no in-page engineering process dump.

### Hour 7: Review And Redaction

Required review lanes:

- content review for clarity and accessibility;
- audio review for audible quality and manifest truth;
- RSS review for feed validity;
- site review for visual consistency with Agent Logic;
- redaction review for secrets, host paths, private account IDs, and unpublished
  guest claims;
- evidence review to ensure launch claims match proof.

Automated redaction checks:

- fail on `file://`;
- fail on `/Users/`, `/Volumes/`, and private build/artifact roots in public
  files;
- fail on API-key-like strings and known secret markers;
- fail on draft-only guest notes in public pages/feed;
- fail on local RSS/audio enclosure URLs.
- fail on `publish_ready` human guest episodes unless release/permission state
  is `signed`, `approved`, or an operator-approved equivalent.

No launch until all lanes are green or explicitly operator-accepted with a
public-safe limitation.

### Hour 8: Deployment Readiness, Not Automatic Deployment

This plan does not authorize deployment. If implementation finishes today, the
deployment gate still requires operator approval.

Deploy checklist:

- local static route passes;
- audio files load by final public URL;
- RSS feed validates by final public URL;
- deployed `https://agent-logic.ai/podcast` returns 200;
- deployed episode page returns 200;
- deployed feed returns 200 with correct content type;
- no draft episodes or internal paths are visible;
- screenshot or terminal receipt captured;
- #5702 or follow-on launch issue records truthful evidence.

## Follow-On Issue Split

After this plan is accepted, execute as separate bounded issues:

1. Podcast Studio episode spec and ten-episode prep factory.
2. Audio pipeline hardening and Deepgram comparison.
3. RSS feed generation and validation.
4. Agent Logic podcast route design and static pages.
5. Week-1 episode production and content/audio review.
6. Launch deploy and live verification.

This split allows parallel work without mixing ADL tooling changes, website
design, audio provider experiments, and release/deploy authority.

## Test Matrix

| Area | Required tests |
| --- | --- |
| Episode spec | schema pass, missing required fields fail, unsafe slug fail, duplicate number fail |
| Ten episodes | all ten specs parse, episode directories exist, draft/ready states truthful |
| Guest support | DeepSeek AI guest metadata validates, human guest invitation state validates, unconfirmed guest cannot be advertised as confirmed, publish-ready human guest episodes require signed/approved release state |
| Audio | segment existence, final MP3 existence, duration, silence check, clipping check, loudness report, manifest digest match, listen-check note |
| Audio integration | Episode 001 script/content locked, script-to-segments-to-MP3 end-to-end test, MP3 encoding check, ID3v2.3 tag/artwork check, TTS chunking/retry failure proof, manifest/output digest match, failure when any segment is missing |
| Deepgram | three-sample render, STT round-trip, pronunciation check, metadata capture, no secrets in logs |
| RSS | URL structure defined before feed generation, XML well-formedness, CDATA/escaping for rich fields, enclosure URLs, MIME type, byte-range support, CORS when needed, artwork dimensions/format/size, `itunes:duration`, `itunes:explicit`, date format, stable GUIDs, no draft episodes, cache policy, feed content matches episode specs, Apple-specific validator preflight, Apple submission timing captured |
| Website | local desktop/mobile render, navigation, audio player playback/seek/duration, iOS Safari playback, transcript, RSS link, OpenGraph metadata, broken-link check |
| Redaction | no secrets, host paths, local artifact paths, account IDs, local URLs, or private guest notes |
| Fail-closed | intentionally missing title, duplicate episode number, publish without review, audio claim without manifest, RSS claim without validated feed |
| Launch proof | deployed route 200, deployed episode 200, deployed feed 200, audio URL 200, operator approval receipt |

## Same-Day Definition Of Done

This issue is done when:

- #5702 exists;
- this plan is present in `.adl/docs/TBD/`;
- Gemini 3.1 Pro has reviewed it or the exact model/provider blocker is
  truthfully recorded as unavailable;
- actionable Gemini 3.1 Pro suggestions are incorporated or listed for operator
  decision;
- validation confirms required plan topics are present;
- no implementation or deployment claims are made.

Implementation is not done until the follow-on issues prove:

- audio is green;
- RSS is green;
- site route is live and visually aligned;
- ten episode specs exist;
- week-1 content is reviewed;
- launch deploy is verified.
