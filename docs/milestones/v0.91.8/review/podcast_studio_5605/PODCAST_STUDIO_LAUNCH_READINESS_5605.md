# Podcast Studio Launch Readiness Plan

## Metadata

- Issue: `#5605`
- Target launch window: week of 2026-07-27
- Planned public route: `agent-logic.ai/podcast`
- Status: ready-to-execute plan, not launched

## Objective

Update Podcast Studio so the revived AI agent podcast can launch next week with
a truthful first episode packet, a public website route plan, and a repeatable
weekly operating cadence.

## Source Evidence

| Evidence | Current status |
| --- | --- |
| `demos/v0.91.1/multiagent_podcast_pilot_demo.md` | transcript-first pilot and one-week cadence intent exist |
| `demos/v0.91.1/multiagent_podcast_audio_demo.md` | audio path exists as bounded follow-on, with author/renderer separation |
| `adl/tools/demo_v0911_multiagent_podcast_pilot.sh` | older pilot wrapper exists |
| `adl/tools/demo_v0911_multiagent_podcast_audio.sh` | older audio wrapper exists |
| `adl/tools/demo_v0913_podcast_studio_v2.sh` | fixed v0.91.3 Podcast Studio generator exists |
| `adl/tools/generate_podcast_studio_v2_packet.py` | hard-coded packet generator exists |
| `docs/milestones/v0.91.3/review/podcast_studio_v2/` | retained packet evidence exists |
| `demos/v0.91.3/adl_podcast_studio_v2_episode_card.html` | polished fixed episode card exists |
| Agent Logic website `site/` tree | static site tree exists; no podcast route observed in current inspection |

## Workstream Plan

### 1. Generator Upgrade

Make Podcast Studio consume an episode spec instead of only the hard-coded
v0.91.3 fixture.

Required output:

- episode metadata JSON;
- topic brief;
- host lineup;
- transcript;
- best-lines cut;
- show notes;
- audio manifest;
- reviewer proof note;
- public episode card/page data.

Negative cases:

- missing title fails;
- unsafe slug fails;
- `publish_ready=true` without review fails;
- final audio claim without audio manifest fails;
- public route claim without deployed proof fails.

### 2. Pilot Episode Packet

Prepare week 1 as a transcript-first launch unless audio proof clears before the
content freeze.

Required launch-week packet:

- episode spec;
- source/research notes;
- transcript;
- show notes;
- publication-safe summary;
- review note;
- hold-or-publish decision.

### 3. `agent-logic.ai/podcast`

Plan the public route in the Agent Logic static site.

Minimum launch route:

- `site/podcast/index.html`;
- `site/podcast/episodes/<episode-slug>/index.html`;
- reused existing CSS/brand assets;
- optional `site/podcast/feed.xml`, only if RSS is explicitly in launch scope.

### 4. Review Gate

Before publication, require:

- content review;
- evidence review;
- redaction review;
- site link/render review;
- operator approval.

### 5. Weekly Cadence

Default cadence:

- Monday: topic and source packet.
- Tuesday: transcript.
- Wednesday: review and revision.
- Thursday: show notes and optional audio.
- Friday: publish or hold.

## Launch-Week Checklist

- [ ] Confirm week 1 topic and title.
- [ ] Prepare episode spec.
- [ ] Produce source packet.
- [ ] Produce transcript.
- [ ] Produce show notes.
- [ ] Decide transcript-first versus audio+transcript.
- [ ] Generate public page content.
- [ ] Review claims/non-claims.
- [ ] Scan for secrets and host paths.
- [ ] Implement website route in Agent Logic repo.
- [ ] Verify local render.
- [ ] Deploy only with operator approval.
- [ ] Verify live `https://agent-logic.ai/podcast`.
- [ ] Record publication receipt or hold note.

## Recommended Execution Split

1. `#5605`: planning/readiness packet and tracker truth.
2. Follow-on: Podcast Studio generator upgrade.
3. Follow-on: Agent Logic static podcast route.
4. Follow-on: pilot episode packet and review.
5. Optional follow-on: audio-rendering hardening.

## Validation Performed For This Plan

- Current issue `#5605` was inspected through `adl-issue view`.
- Existing source paths listed above were inspected or verified by repository
  search.
- Agent Logic static site tree was inspected for route shape.

## Validation Not Run

- No Podcast Studio generator changes were made in this issue.
- No Agent Logic website files were changed in this issue.
- No episode was generated, rendered, deployed, or published.

## Non-Claims

- This plan does not claim `agent-logic.ai/podcast` is live.
- This plan does not claim RSS exists.
- This plan does not claim final audio is ready.
- This plan does not claim the weekly cadence has been proven.
