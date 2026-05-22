# Podcast Studio v2 Demo Proof Packet v0.91.3

## Demo Identity

- demo name: ADL Podcast Studio v2
- issue / WP: demo WP-04 / #3223
- milestone version: `v0.91.3`
- primary artifact: `demos/v0.91.3/adl_podcast_studio_v2_episode_card.html`

## Bounded Purpose

Show a repeatable, inspectable media-production system that turns a bounded topic into a full episode packet without hidden credentials or fake audio claims.

## Claims

- ADL can package one deterministic recurring episode packet with visible role boundaries.
- The demo can stay truthful about audio render status without requiring hidden credentials.

## Non-Claims

- This packet does not claim live provider-backed conversation generation.
- This packet does not claim real final-audio rendering or publication readiness.

## Run Path

- primary command: `bash adl/tools/demo_v0913_podcast_studio_v2.sh`
- operator prerequisites: repository checkout only; no secrets or external services required
- run status: `passed`

## Timebox Truth

- timebox claim: packet generation is fast and deterministic, but literal five-minute end-to-end show production is not claimed here
- evidence type: `estimated`
- start evidence: local bounded generator invocation
- end evidence: tracked packet regeneration plus validator/test completion
- elapsed result: bounded local packet regeneration only; no five-minute proof claim

## Validation Evidence

```bash
bash adl/tools/demo_v0913_podcast_studio_v2.sh
python3 adl/tools/validate_podcast_studio_v2_packet.py docs/milestones/v0.91.3/review/podcast_studio_v2 demos/v0.91.3/adl_podcast_studio_v2_episode_card.html docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md
bash adl/tools/test_podcast_studio_v2_packet.sh
```

Validation not run:

- real provider-backed audio generation, because the bounded demo intentionally avoids hidden credentials and fake live-audio claims

## Review Evidence

- review surface: bounded local review over the generated packet, helper, validator, and episode card
- findings fixed before publication: any packet-shape, role-visibility, or audio-status truth drift found during bounded review
- residual risks: the packet is a deterministic production-system demo, not a proof of real publishing or live-render reliability

## Result Classification

| Claim | Classification | Reason |
| --- | --- | --- |
| deterministic recurring episode packet exists | `passed` | one-command packet generation writes all required review surfaces without hidden credentials |
| audio render status stays truthful | `passed` | manifest records `manifest_only` instead of implying a real render |
| literal five-minute creative production is proven | `partial` | the artifact is strong, but this packet does not measure or prove the full timebox target |

## Skipped Work

- skipped scope: live provider-backed generation and final audio synthesis
- why it was skipped: this bounded issue requires a no-secrets-needed proof path and exact render claims

## Repo-Relative Artifacts

- `docs/milestones/v0.91.3/review/podcast_studio_v2/ct_demo_004_topic_brief.md`
- `docs/milestones/v0.91.3/review/podcast_studio_v2/ct_demo_004_host_lineup.md`
- `docs/milestones/v0.91.3/review/podcast_studio_v2/ct_demo_004_transcript.md`
- `docs/milestones/v0.91.3/review/podcast_studio_v2/ct_demo_004_best_lines.md`
- `docs/milestones/v0.91.3/review/podcast_studio_v2/ct_demo_004_audio_render_manifest.json`
- `docs/milestones/v0.91.3/review/podcast_studio_v2/ct_demo_004_episode_packet.md`
- `docs/milestones/v0.91.3/review/podcast_studio_v2/ct_demo_004_reviewer_proof_note.md`
- `demos/v0.91.3/adl_podcast_studio_v2_episode_card.html`
- `docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md`
