# Podcast Studio v2 Demo

## Summary

`WP-04` upgrades the older podcast pilot into a deterministic production-system demo.

The result is not a live provider-backed episode factory. It is a repeatable, inspectable one-command packet generator that emits a topic brief, host lineup, transcript, best-lines extract, truthful audio render manifest, reviewer proof note, and polished episode card.

## Canonical Command

```bash
bash adl/tools/demo_v0913_podcast_studio_v2.sh
```

## What It Proves

- one recurring episode packet can be regenerated deterministically
- role boundaries are visible across the packet
- audio render status can stay exact without hidden credentials
- the production artifact can feel like a show package rather than a bare validation log

## What It Does Not Prove

- live provider-backed episode generation
- final rendered audio output
- literal five-minute end-to-end creative production
- publishing or distribution readiness

## Proof Surfaces

- `docs/milestones/v0.91.3/review/podcast_studio_v2/`
- `demos/v0.91.3/adl_podcast_studio_v2_episode_card.html`
- `adl/tools/demo_v0913_podcast_studio_v2.sh`
- `adl/tools/validate_podcast_studio_v2_packet.py`
