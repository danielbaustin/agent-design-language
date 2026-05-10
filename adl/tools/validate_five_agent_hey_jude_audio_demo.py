#!/usr/bin/env python3
import json
import sys
import wave
from pathlib import Path


REQUIRED_FILES = [
    "episode.wav",
    "audio_manifest.json",
    "audio_packet.md",
    "proof_note.md",
    "playback_transcript.md",
    "cue_timing_register.json",
    "source_episode/performance_manifest.json",
    "source_episode/transcript.md",
]

REQUIRED_SPEAKERS = {"Layer 8", "ChatGPT", "Claude", "Gemini", "DeepSeek"}
BANNED_STRINGS = [
    "Take a sad song and make it better",
    "Remember to let her into your heart",
]


def fail(message: str) -> int:
    print(message, file=sys.stderr)
    return 1


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: validate_five_agent_hey_jude_audio_demo.py <artifact_root>")
    root = Path(sys.argv[1])
    for rel in REQUIRED_FILES:
        if not (root / rel).is_file():
            return fail(f"missing required artifact: {rel}")

    manifest = json.loads((root / "audio_manifest.json").read_text(encoding="utf-8"))
    register = json.loads((root / "cue_timing_register.json").read_text(encoding="utf-8"))
    packet = (root / "audio_packet.md").read_text(encoding="utf-8")
    transcript = (root / "playback_transcript.md").read_text(encoding="utf-8")
    source_transcript = (root / "source_episode/transcript.md").read_text(encoding="utf-8")

    if manifest["schema_version"] != "adl.hey_jude_audio_manifest.v1":
        return fail("unexpected audio manifest schema version")
    if manifest["source_demo_id"] != "v0.89.1.five_agent_hey_jude_midi_demo":
        return fail("unexpected source demo id")

    insert_speakers = {entry["speaker"] for entry in register["voice_inserts"]}
    if insert_speakers != REQUIRED_SPEAKERS:
        return fail(f"unexpected insert speakers: {sorted(insert_speakers)}")

    if len(register["voice_inserts"]) != 5:
        return fail("expected exactly five voice inserts")

    for entry in register["voice_inserts"]:
        if entry["end_ms"] <= entry["start_ms"]:
            return fail(f"invalid timing window for {entry['speaker']}")

    if "operator-supplied local media" not in packet:
        return fail("audio packet missing local-media copyright note")

    for banned in BANNED_STRINGS:
        if banned in transcript or banned in source_transcript:
            return fail("copyright-sensitive lyric text leaked into transcript surfaces")

    with wave.open(str(root / "episode.wav"), "rb") as wf:
        if wf.getnframes() <= 0:
            return fail("mixed audio file is empty")

    print("validate_five_agent_hey_jude_audio_demo: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
