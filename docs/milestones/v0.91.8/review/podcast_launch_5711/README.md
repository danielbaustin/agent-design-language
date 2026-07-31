# Podcast Launch Packet For #5711

This packet turns the revived podcast plan into a launchable local path.

## Surfaces

- `episodes.json`: first ten reusable episode records.
- `DEEPGRAM_AUDIO_INVESTIGATION_5711.md`: bounded Deepgram follow-on
  investigation.
- `demos/podcast/index.html`: generated show landing page.
- `demos/podcast/episodes/meet-the-ai-coworkers/index.html`: first episode
  page.
- `demos/podcast/feed.xml`: RSS feed with audio enclosure.
- `demos/podcast/audio/meet-the-ai-coworkers.wav`: local playable audio proof.

## Commands

```bash
ADL_PODCAST_AUDIO_TEST_TONES=1 \
ADL_PODCAST_LAUNCH_WORK_DIR=/Volumes/FastWork/adl-podcast-launch-5711-work \
bash adl/tools/demo_v0918_podcast_launch.sh demos/podcast
```

```bash
bash adl/tools/test_podcast_launch_packet.sh
```

## Truth Boundary

The checked-in audio is a deterministic local proof asset. It proves that the
page, RSS enclosure, audio path, and validation can work end to end without
credentials. Provider-backed final voice rendering remains a selectable launch
mode and must record renderer identity before public release.

This packet does not claim `https://agent-logic.ai/podcast` is deployed.
