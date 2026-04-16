#!/usr/bin/env python3
import json
import sys
from pathlib import Path


REQUIRED_FILES = [
    "performance_manifest.json",
    "cast.json",
    "section_plan.json",
    "cue_timeline.json",
    "transcript.md",
    "performance_summary.md",
    "midi_event_log.json",
    "midi_binding.json",
    "midi_event_summary.json",
]

REQUIRED_PARTICIPANTS = {"Layer 8", "ChatGPT", "Claude", "Gemini", "DeepSeek"}
REQUIRED_SECTIONS = ["Opening", "Verse Rotation", "Chorus Build", "Long Fade", "Curtain Call"]
BANNED_STRINGS = [
    "Take a sad song and make it better",
    "Remember to let her into your heart",
]


def fail(message: str) -> int:
    print(message, file=sys.stderr)
    return 1


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: validate_five_agent_music_demo.py <artifact_root>")

    root = Path(sys.argv[1])
    for rel in REQUIRED_FILES:
        if not (root / rel).is_file():
            return fail(f"missing required artifact: {rel}")

    cast = json.loads((root / "cast.json").read_text(encoding="utf-8"))
    section_plan = json.loads((root / "section_plan.json").read_text(encoding="utf-8"))
    cue_timeline = json.loads((root / "cue_timeline.json").read_text(encoding="utf-8"))
    midi_log = json.loads((root / "midi_event_log.json").read_text(encoding="utf-8"))
    transcript = (root / "transcript.md").read_text(encoding="utf-8")
    manifest = json.loads((root / "performance_manifest.json").read_text(encoding="utf-8"))

    cast_names = {entry["participant"] for entry in cast["cast"]}
    if cast_names != REQUIRED_PARTICIPANTS:
        return fail(f"unexpected cast participants: {sorted(cast_names)}")

    sections = [entry["section"] for entry in section_plan["sections"]]
    if sections != REQUIRED_SECTIONS:
        return fail(f"unexpected section order: {sections}")

    for name in REQUIRED_PARTICIPANTS:
        if name not in transcript:
            return fail(f"missing participant in transcript: {name}")
    for section in REQUIRED_SECTIONS:
        if f"## {section}" not in transcript:
            return fail(f"missing transcript section: {section}")

    if len(midi_log["events"]) < 4:
        return fail("expected at least four MIDI events")
    cue_sections = [entry["section"] for entry in cue_timeline["cues"]]
    if cue_sections[0] != "Opening" or cue_sections[-1] != "Curtain Call":
        return fail("cue timeline does not span opening to curtain call")

    if manifest["schema_version"] != "adl.five_agent_music_demo.v1":
        return fail("unexpected performance manifest schema version")

    for banned in BANNED_STRINGS:
        if banned in transcript:
            return fail("copyright-sensitive lyric text leaked into transcript")

    if "/Users/" in transcript or "/tmp/" in transcript or "/private/tmp" in transcript:
        return fail("absolute path leaked into transcript")

    print("validate_five_agent_music_demo: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
