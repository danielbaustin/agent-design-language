#!/usr/bin/env python3
"""Validate the tracked v0.91.5 multi-agent quality comparison bundle."""

from __future__ import annotations

import json
import sys
from pathlib import Path, PurePosixPath

STATE_NAME = "v0915_multi_agent_quality_comparison_state_2026-06-15.json"
PACKET_NAME = "MULTI_AGENT_QUALITY_COMPARISON_2026-06-15.md"
EXPECTED_SCHEMA = "adl.multi_agent_quality_comparison.v1"
EXPECTED_SOURCE_PACKETS = {
    "openrouter": "docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md",
    "remote_gemma": "docs/milestones/v0.91.5/review/remote_gemma_watcher/REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md",
    "overhead": "docs/milestones/v0.91.5/review/multi_agent_overhead/MULTI_AGENT_OVERHEAD_COMPARISON_2026-06-14.md",
}
SINGLE_REQUIRED_HEADINGS = ("# OpenRouter", "# Remote-Gemma", "# Overhead", "# Verdict")
SINGLE_REQUIRED_SNIPPETS = (
    "supported_with_limits",
    "provider_auth_missing",
    "watcher_remote_gemma4_e2b",
    "adapter_gemma4_31b",
    "single_agent_preferred_for_tiny_docs_audit",
    "faster only for raw lane execution",
)
MULTI_EXPECTED = {
    "multi_agent_overhead_claude_3_5_haiku": {
        "model": "anthropic/claude-3.5-haiku",
        "execution_surface": "openrouter_chat_completions",
        "snippets": (
            "single_agent_preferred_for_tiny_docs_audit",
            "faster only for raw lane execution",
            "disjoint surfaces",
        ),
    },
    "multi_agent_provider_openai_gpt4o_mini": {
        "model": "openai/gpt-4o-mini",
        "execution_surface": "openrouter_chat_completions",
        "snippets": (
            "supported_with_limits",
            "provider_auth_missing",
            "structured route execution",
        ),
    },
    "multi_agent_watcher_gemini_2_5_flash_lite": {
        "model": "google/gemini-2.5-flash-lite",
        "execution_surface": "openrouter_chat_completions",
        "snippets": (
            "#3724",
            "adapter_gemma4_31b",
            "route probe completed",
        ),
    },
}


def fail(message: str) -> None:
    print(f"validate_v0915_multi_agent_quality_comparison: {message}", file=sys.stderr)
    raise SystemExit(1)


def find_repo_root(start: Path) -> Path:
    for candidate in (start, *start.parents):
        if (candidate / ".git").exists() and (candidate / "adl").is_dir():
            return candidate
    fail(f"could not locate repo root from {start}")


def expect_relative_path(path: str, label: str) -> PurePosixPath:
    pure = PurePosixPath(path)
    if pure.is_absolute():
        fail(f"{label} must be repo-relative")
    if any(part in {"", ".."} for part in pure.parts):
        fail(f"{label} must stay within the repo tree")
    return pure


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON at {path}: {exc}")


def validate_markdown_output(path: Path, headings: tuple[str, ...], snippets: tuple[str, ...], label: str) -> None:
    text = path.read_text(encoding="utf-8")
    lowered = text.lower()
    for heading in headings:
        if heading not in text:
            fail(f"{label} missing heading: {heading}")
    for snippet in snippets:
        if snippet.lower() not in lowered:
            fail(f"{label} missing snippet: {snippet}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_v0915_multi_agent_quality_comparison.py <packet-dir>")
    packet_dir = Path(sys.argv[1]).resolve()
    repo_root = find_repo_root(packet_dir)
    state_path = packet_dir / STATE_NAME
    packet_path = packet_dir / PACKET_NAME
    if not state_path.exists():
        fail(f"missing state packet: {state_path}")
    if not packet_path.exists():
        fail(f"missing packet markdown: {packet_path}")

    packet_text = packet_path.read_text(encoding="utf-8")
    for snippet in (
        "Issue: `#3725`",
        "Status: `better`",
        "disjoint evidence briefs",
        "single-agent baseline",
        "multi-agent OpenAI provider lane",
        "multi-agent Claude overhead lane",
        "multi-agent Gemini watcher lane",
        "does not claim multi-agent is superior",
    ):
        if snippet not in packet_text:
            fail(f"packet missing required text: {snippet}")

    state = load_json(state_path)
    if not isinstance(state, dict):
        fail("state packet must be a JSON object")
    if state.get("schema_version") != EXPECTED_SCHEMA:
        fail(f"state schema_version must be {EXPECTED_SCHEMA}")
    if state.get("issue_number") != 3725:
        fail("state issue_number must be 3725")

    source_packets = state.get("source_packets")
    if not isinstance(source_packets, dict):
        fail("source_packets must be an object")
    for key, expected in EXPECTED_SOURCE_PACKETS.items():
        if source_packets.get(key) != expected:
            fail(f"source_packets.{key} must be {expected}")
        if not (repo_root / expected).exists():
            fail(f"source packet missing from repo: {expected}")

    single = state.get("single_agent")
    if not isinstance(single, dict):
        fail("single_agent must be an object")
    if single.get("status") != "useful_output":
        fail("single_agent.status must be useful_output")
    single_output = expect_relative_path(str(single.get("output_path", "")), "single_agent.output_path")
    validate_markdown_output(repo_root / single_output, SINGLE_REQUIRED_HEADINGS, SINGLE_REQUIRED_SNIPPETS, "single_agent output")
    single_duration = single.get("duration_seconds")
    if not isinstance(single_duration, (int, float)) or single_duration <= 0:
        fail("single_agent.duration_seconds must be positive")

    multi_agent = state.get("multi_agent")
    if not isinstance(multi_agent, dict):
        fail("multi_agent must be an object")
    multi_parallel = multi_agent.get("parallel_duration_seconds")
    if not isinstance(multi_parallel, (int, float)) or multi_parallel <= 0:
        fail("multi_agent.parallel_duration_seconds must be positive")
    lanes = multi_agent.get("lanes")
    if not isinstance(lanes, list) or len(lanes) != len(MULTI_EXPECTED):
        fail(f"multi_agent.lanes must contain exactly {len(MULTI_EXPECTED)} lanes")
    seen = set()
    for lane in lanes:
        if not isinstance(lane, dict):
            fail("each multi-agent lane must be an object")
        lane_id = lane.get("lane_id")
        if lane_id in seen:
            fail(f"duplicate multi-agent lane: {lane_id}")
        seen.add(lane_id)
        if lane_id not in MULTI_EXPECTED:
            fail(f"unexpected multi-agent lane: {lane_id}")
        expected = MULTI_EXPECTED[lane_id]
        if lane.get("model") != expected["model"]:
            fail(f"{lane_id}.model must be {expected['model']}")
        if lane.get("execution_surface") != expected["execution_surface"]:
            fail(f"{lane_id}.execution_surface must be {expected['execution_surface']}")
        if lane.get("status") != "useful_output":
            fail(f"{lane_id}.status must be useful_output")
        output_path = expect_relative_path(str(lane.get("output_path", "")), f"{lane_id}.output_path")
        validate_markdown_output(repo_root / output_path, ("# Status", "# Signal", "# Next-Step"), expected["snippets"], lane_id)
    if seen != set(MULTI_EXPECTED):
        fail("missing one or more expected multi-agent lanes")

    summary = state.get("summary")
    if not isinstance(summary, dict):
        fail("summary must be an object")
    if summary.get("comparison_result") != "better":
        fail("summary.comparison_result must be better")
    if summary.get("task_shape") != "disjoint_multi_surface_evidence_review":
        fail("summary.task_shape must preserve the disjoint-surface comparison truth")
    multi_summary = summary.get("multi_agent_parallel_duration_seconds")
    single_summary = summary.get("single_agent_duration_seconds")
    if not isinstance(multi_summary, (int, float)) or not isinstance(single_summary, (int, float)):
        fail("summary durations must be numeric")
    if multi_summary >= single_summary:
        fail("better comparison requires multi-agent parallel duration to be lower than single-agent duration")
    ratio = summary.get("duration_ratio")
    if not isinstance(ratio, (int, float)) or ratio >= 1:
        fail("summary.duration_ratio must prove a faster multi-agent path")

    print("PASS: multi-agent quality comparison bundle valid")


if __name__ == "__main__":
    main()
