# AI Agent Podcast Studio

## Metadata

- Issue: `#5605`
- Version: `v0.91.8`
- Status: launch-readiness planning
- Planned public home: `agent-logic.ai/podcast`
- Launch target: week of 2026-07-27
- Doc role: feature/readiness plan

## Summary

The AI Agent Podcast Studio revives the earlier multi-agent podcast pilot and
Podcast Studio v2 demo as a weekly, human-readable production lane.

The near-term goal is not to prove a broad media platform. The goal is to make
one weekly episode repeatable: select a topic, produce a source packet, generate
or author an inspectable transcript, review claims, optionally render audio,
publish a simple page at `agent-logic.ai/podcast`, and retain enough evidence
that the next weekly episode can reuse the same path.

## Historical Baseline

Source-backed facts:

- `demos/v0.91.1/multiagent_podcast_pilot_demo.md` defines the transcript-first
  pilot and names an intended `1 episode / week` cadence.
- `demos/v0.91.1/multiagent_podcast_audio_demo.md` defines the audio follow-on
  and separates transcript authorship identity from audio-renderer identity.
- `docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md` upgrades the
  older pilot into a deterministic production-system demo.
- `docs/milestones/v0.91.3/review/podcast_studio_v2/` retains the topic brief,
  host lineup, transcript, best-lines, audio manifest, episode packet, reviewer
  note, and generated card for `#3223`.
- `docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md` records `#3223` as passed,
  with explicit non-claims for live provider-backed generation, final audio,
  five-minute end-to-end production, and publishing readiness.
- `.adl/reviews/sprint-3219-closeout.md` records the broader demo mini-sprint
  `#3219` through `#3224` as closed out.

Current implication: the old work is strong enough to revive, but it is not yet
current proof of a weekly show.

## Required Upgrade

Podcast Studio must become configurable and launch-oriented:

- episode spec input for title, slug, topic, publish date, hosts, transcript
  source, audio route, proof status, and publication status;
- generated weekly packet with topic brief, host lineup, transcript, best
  lines, show notes, audio manifest, episode metadata, reviewer proof note, and
  static episode page/card;
- old v0.91.3 demo retained as a regression fixture rather than the only
  episode;
- fail-closed checks for missing topic, unsafe slug, unpublished review status,
  audio claim without manifest, or public page claims that exceed proof.

## Planned Public Route

The planned public home is:

- `https://agent-logic.ai/podcast`

Expected static-site surfaces in the Agent Logic website repository:

- `site/podcast/index.html` for the show landing/archive page;
- `site/podcast/episodes/<episode-slug>/index.html` for each episode;
- optional `site/podcast/feed.xml` if RSS is part of the launch decision;
- static audio assets only after audio rendering and review pass.

Route non-claim:

- `agent-logic.ai/podcast` is planned, not live-proven by this ADL issue.
- A later website issue or launch issue must implement, deploy, and verify the
  route before any public claim.

## Weekly Production Model

| Day | Activity | Required evidence |
| --- | --- | --- |
| Monday | Choose topic and source packet | episode spec and source notes |
| Tuesday | Draft transcript | transcript and role attribution |
| Wednesday | Review and revise | review note and claim/non-claim updates |
| Thursday | Render audio if included; write show notes | audio manifest, show notes, redaction pass |
| Friday | Publish or hold | publication receipt or hold note |

Fail closed if credentials, source packet, review, audio manifest, or public
route proof is missing for the claim being made.

## First Ten Episode Slate

| Week | Target week | Working title | Guest / cast note | Listener question |
| --- | --- | --- | --- | --- |
| 1 | 2026-07-27 | Meet the AI Coworkers | Core hosts | What is an AI agent, and why does it feel different from a chatbot? |
| 2 | 2026-08-03 | Can an AI Be a Good Teammate? | DeepSeek special guest | What makes an AI helpful in a real team instead of just impressive in a demo? |
| 3 | 2026-08-10 | The Promise and Weirdness of Talking to Machines | Core hosts | Why do conversations with AI feel personal, useful, awkward, and powerful all at once? |
| 4 | 2026-08-17 | What Should We Let AI Do for Us? | Core hosts | Where is the line between useful delegation and giving away too much control? |
| 5 | 2026-08-24 | Can AI Help Us Think Better? | Core hosts | Do agents just answer questions, or can they improve how we reason? |
| 6 | 2026-08-31 | The New Creative Room | Core hosts, optional operator clip | What happens when people and AI make things together in real time? |
| 7 | 2026-09-07 | Trust, Receipts, and Proof | Core hosts | How do we know what an AI actually did? |
| 8 | 2026-09-14 | Local AI vs Cloud AI | Core hosts | Should your AI run on your laptop, in the cloud, or both? |
| 9 | 2026-09-21 | When AI Gets Stuck | Core hosts | What should an AI system do when it is confused, blocked, or wrong? |
| 10 | 2026-09-28 | What Does a Weekly AI Studio Look Like? | Core hosts | What did we learn from ten weeks of trying to make a real AI-assisted show? |

## Week 2 DeepSeek Guest Shape

DeepSeek should be invited as a special AI guest for week 2, not framed as a
benchmark target.

Suggested roles:

- ChatGPT: host and audience translator.
- Gemini: systems/product questioner.
- Claude: human and ethical stakes.
- DeepSeek: guest teammate focused on reasoning, tradeoffs, and useful
  collaboration.

Avoid:

- model leaderboard framing;
- "which model is best" discourse;
- dense provider setup details;
- claims of stable long-term guest identity unless separately proven.

## Human Luminary Guest Ladder

The show can try for a human AI luminary after it has a credible public page and
several good episodes.

Recommended ladder:

- Weeks 1-2: launch the format and DeepSeek guest episode.
- Weeks 3-5: build a credible archive and clean show notes.
- Weeks 6-10: invite one human guest for a short, low-friction segment.

Best initial human guest targets:

- Ethan Mollick, for an accessible AI-at-work conversation.
- Simon Willison, for practical AI tooling and local/cloud model culture.
- Swyx, for the AI engineering and agent-builder audience.
- Jeremy Howard, for practical AI education and open models.
- Sara Hooker, for open models and responsible AI research depth.

High-friction aspirational guests such as Andrej Karpathy, Noam Brown, Percy
Liang, Yoshua Bengio, or Clement Delangue should be treated as later outreach,
not a launch dependency.

## Launch-Week Decision

Recommended launch posture:

- Launch transcript-first if final audio proof is not green by the content
  freeze.
- Publish audio only if the manifest records renderer identity, loudness/basic
  QA, and reviewer approval.
- Keep RSS optional for week 1; prioritize a good landing page and one clean
  episode page.

## Validation Expectations

ADL-side validation:

- old v0.91.3 packet regression still passes;
- new episode spec validates required fields and negative cases;
- public-page metadata is generated from the episode packet rather than
  separately improvised.

Website-side validation:

- local static render for `site/podcast/index.html`;
- first episode page links and assets resolve;
- no credentials, private host paths, or unpublished account details are
  present;
- deployed `https://agent-logic.ai/podcast` is verified before public claims.

## Non-Claims

- This issue does not launch the podcast.
- This issue does not prove final audio rendering.
- This issue does not prove RSS or external distribution.
- This issue does not prove a durable weekly cadence until the first launched
  episode and at least one follow-up weekly cycle complete.
