#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path


REQUIRED_FILES = [
    "README.md",
    "PODCAST_STUDIO_V2_PACKET_v0.91.3.md",
    "ct_demo_004_topic_brief.md",
    "ct_demo_004_host_lineup.md",
    "ct_demo_004_transcript.md",
    "ct_demo_004_best_lines.md",
    "ct_demo_004_audio_render_manifest.json",
    "ct_demo_004_episode_packet.md",
    "ct_demo_004_reviewer_proof_note.md",
]

REQUIRED_PACKET_SNIPPETS = [
    "## Demo Identity",
    "## Bounded Purpose",
    "## Claims",
    "## Non-Claims",
    "## Run Path",
    "## Timebox Truth",
    "## Validation Evidence",
    "## Review Evidence",
    "## Result Classification",
]

REQUIRED_CARD_SNIPPETS = [
    "ADL Podcast Studio v2",
    "Packet status: local pass",
    "Audio status: manifest only",
    "Stable Host Lineup",
    "Transcript",
    "Best Lines",
]


def fail(message: str) -> int:
    print(f"FAIL: {message}", file=sys.stderr)
    return 1


def require_snippets(path: Path, snippets: list[str], label: str) -> int:
    text = path.read_text(encoding="utf-8")
    for snippet in snippets:
        if snippet not in text:
            return fail(f"{label} missing snippet: {snippet}")
    return 0


def main() -> int:
    if len(sys.argv) not in (3, 4):
        return fail("usage: validate_podcast_studio_v2_packet.py <review-dir> <episode-card-path> [feature-path]")

    review_dir = Path(sys.argv[1])
    card_path = Path(sys.argv[2])
    feature_path = (
        Path(sys.argv[3])
        if len(sys.argv) == 4
        else Path(__file__).resolve().parents[2] / "docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md"
    )

    if not review_dir.is_dir():
        return fail(f"review dir missing: {review_dir}")
    if not card_path.is_file():
        return fail(f"episode card missing: {card_path}")
    if not feature_path.is_file():
        return fail(f"feature doc missing: {feature_path}")

    for rel_path in REQUIRED_FILES:
        path = review_dir / rel_path
        if not path.is_file():
            return fail(f"required file missing: {path}")

    if require_snippets(review_dir / "PODCAST_STUDIO_V2_PACKET_v0.91.3.md", REQUIRED_PACKET_SNIPPETS, "packet"):
        return 1

    if require_snippets(
        review_dir / "ct_demo_004_topic_brief.md",
        ["# Topic Brief", "## Episode Question", "## Production boundary", "## Desired listener outcome"],
        "topic brief",
    ):
        return 1

    if require_snippets(
        review_dir / "ct_demo_004_host_lineup.md",
        [
            "# Host Lineup",
            "### ChatGPT",
            "### Gemini",
            "### Claude",
            "show role:",
            "studio job:",
            "voice / style target:",
        ],
        "host lineup",
    ):
        return 1

    if require_snippets(
        review_dir / "ct_demo_004_episode_packet.md",
        ["# Episode Packet", "## Packet Checklist", "## Canonical Command", "## Reviewer Path"],
        "episode packet",
    ):
        return 1

    if require_snippets(
        review_dir / "ct_demo_004_reviewer_proof_note.md",
        [
            "# Reviewer Proof Note",
            "## Bounded claim",
            "## What this packet proves",
            "## What this packet suggests",
            "## What this packet does not prove",
            "## Review boundary",
        ],
        "reviewer proof note",
    ):
        return 1

    card_text = card_path.read_text(encoding="utf-8")
    for snippet in REQUIRED_CARD_SNIPPETS:
        if snippet not in card_text:
            return fail(f"episode card missing snippet: {snippet}")

    manifest = json.loads((review_dir / "ct_demo_004_audio_render_manifest.json").read_text(encoding="utf-8"))
    if manifest.get("render_status") != "manifest_only":
        return fail("audio render manifest must report render_status=manifest_only")
    if manifest.get("rendered_audio_present") is not False:
        return fail("audio render manifest must report rendered_audio_present=false")
    if manifest.get("render_policy", {}).get("hidden_credentials_required") is not False:
        return fail("audio render manifest must report hidden_credentials_required=false")

    transcript_text = (review_dir / "ct_demo_004_transcript.md").read_text(encoding="utf-8")
    for snippet in ("# ADL Podcast Studio", "## Episode 04: Can governed creative production feel alive?", "## Transcript"):
        if snippet not in transcript_text:
            return fail(f"transcript missing snippet: {snippet}")
    for speaker in ("ChatGPT", "Gemini", "Claude"):
        if speaker not in transcript_text:
            return fail(f"transcript missing speaker: {speaker}")
    if transcript_text.count("### Turn ") != 6:
        return fail("transcript must contain exactly 6 turns")

    best_lines_text = (review_dir / "ct_demo_004_best_lines.md").read_text(encoding="utf-8")
    if best_lines_text.count("- ") < 5:
        return fail("best lines must contain at least five bullet quotes")

    feature_text = feature_path.read_text(encoding="utf-8")
    for snippet in (
        "# Podcast Studio v2 Demo",
        "## Canonical Command",
        "## What It Proves",
        "## What It Does Not Prove",
    ):
        if snippet not in feature_text:
            return fail(f"feature doc missing snippet: {snippet}")

    print("PASS: podcast studio v2 packet and episode card satisfy the bounded contract")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
