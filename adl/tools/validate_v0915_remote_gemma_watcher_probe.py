#!/usr/bin/env python3
"""Validate the tracked v0.91.5 remote Gemma watcher proof bundle."""

from __future__ import annotations

import json
import sys
from pathlib import Path, PurePosixPath

STATE_NAME = "v0915_remote_gemma_watcher_state_2026-06-15.json"
PACKET_NAME = "REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md"
EXPECTED_SCHEMA = "adl.remote_gemma_watcher_probe.v1"
EXPECTED_ENDPOINT = "http://192.168.68.70:11434"
REQUIRED_LANES = {
    "adapter_gemma4_31b": ("adl_provider_adapter", "gemma4:31b"),
    "raw_gemma4_26b": ("raw_ollama_http", "gemma4:26b"),
    "raw_gemma4_e4b": ("raw_ollama_http", "gemma4:e4b"),
}
REQUIRED_HEADINGS = ("# Status", "# Signal", "# Next-Step")
REQUIRED_PHRASE = "route probe completed"
REQUIRED_CONTEXT_SNIPPETS = (
    "#3724",
    "watcher usefulness",
    "docs/milestones/v0.91.5/review/multi_agent_workcell",
    "historical empty output",
    "non-empty",
)
PRIMARY_PROVING_LANE = "adapter_gemma4_31b"
MIN_USEFUL_LANES = 2


def fail(message: str) -> None:
    print(f"validate_v0915_remote_gemma_watcher_probe: {message}", file=sys.stderr)
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


def validate_lane(repo_root: Path, lane: dict[str, object]) -> None:
    lane_id = lane.get("lane_id")
    if lane_id not in REQUIRED_LANES:
        fail(f"unexpected lane_id: {lane_id}")
    expected_surface, expected_model = REQUIRED_LANES[lane_id]
    if lane.get("execution_surface") != expected_surface:
        fail(f"{lane_id} execution_surface must be {expected_surface}")
    if lane.get("model") != expected_model:
        fail(f"{lane_id} model must be {expected_model}")
    if lane.get("status") != "useful_output":
        fail(f"{lane_id} status must be useful_output")
    output_path = expect_relative_path(str(lane.get("output_path", "")), f"{lane_id}.output_path")
    output_abs = repo_root / output_path
    if not output_abs.exists():
        fail(f"{lane_id} output missing: {output_path.as_posix()}")
    output_text = output_abs.read_text(encoding="utf-8")
    lowered_output = output_text.lower()
    for heading in REQUIRED_HEADINGS:
        if heading not in output_text:
            fail(f"{lane_id} output missing heading: {heading}")
    if REQUIRED_PHRASE not in lowered_output:
        fail(f"{lane_id} output missing phrase: {REQUIRED_PHRASE}")
    for snippet in REQUIRED_CONTEXT_SNIPPETS:
        if snippet.lower() not in lowered_output:
            fail(f"{lane_id} output missing context snippet: {snippet}")
    if "artifact_paths" in lane:
        fail(f"{lane_id} must not require local-only artifact_paths in tracked state")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: validate_v0915_remote_gemma_watcher_probe.py <packet-dir>")
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
        "Issue: `#3724`",
        "historical empty output",
        "`gemma4:31b`",
        "useful_with_limits",
        "This packet does not replace the historical `#3415` workcell packet.",
        "`raw_gemma4_26b`",
        "`raw_gemma4_e4b`",
        "`adapter_gemma4_31b`",
        "## Reliability Gate",
        "fails closed",
        "`gemma4:e2b`",
    ):
        if snippet not in packet_text:
            fail(f"packet missing required text: {snippet}")

    state = load_json(state_path)
    if not isinstance(state, dict):
        fail("state packet must be a JSON object")
    if state.get("schema_version") != EXPECTED_SCHEMA:
        fail(f"state schema_version must be {EXPECTED_SCHEMA}")
    if state.get("issue_number") != 3724:
        fail("state issue_number must be 3724")
    if state.get("endpoint_ref") != EXPECTED_ENDPOINT:
        fail(f"state endpoint_ref must be {EXPECTED_ENDPOINT}")
    inventory = state.get("inventory")
    if not isinstance(inventory, dict):
        fail("inventory must be an object")
    gemma_models = inventory.get("gemma_models")
    if not isinstance(gemma_models, list):
        fail("inventory.gemma_models must be a list")
    for model in ("gemma4:31b", "gemma4:26b", "gemma4:e4b"):
        if model not in gemma_models:
            fail(f"inventory missing required model: {model}")
    if "tags_snapshot_path" in inventory:
        fail("inventory must not require local-only tags_snapshot_path in tracked state")

    historical = state.get("historical_context")
    if not isinstance(historical, dict):
        fail("historical_context must be an object")
    if historical.get("historical_status") != "completed_unhelpful_output":
        fail("historical_context.historical_status must preserve the old weak-output truth")

    lanes = state.get("lanes")
    if not isinstance(lanes, list):
        fail("lanes must be a list")
    if len(lanes) != len(REQUIRED_LANES):
        fail(f"lanes must contain exactly {len(REQUIRED_LANES)} entries")
    seen = set()
    for lane in lanes:
        if not isinstance(lane, dict):
            fail("each lane must be an object")
        lane_id = lane.get("lane_id")
        if lane_id in seen:
            fail(f"duplicate lane_id: {lane_id}")
        seen.add(lane_id)
        validate_lane(repo_root, lane)
    if seen != set(REQUIRED_LANES):
        fail("state packet is missing one or more required lanes")

    summary = state.get("summary")
    if not isinstance(summary, dict):
        fail("summary must be an object")
    if summary.get("primary_proving_lane") != PRIMARY_PROVING_LANE:
        fail(f"summary.primary_proving_lane must be {PRIMARY_PROVING_LANE}")
    if summary.get("primary_required_status") != "useful_output":
        fail("summary.primary_required_status must be useful_output")
    if summary.get("minimum_useful_lanes") != MIN_USEFUL_LANES:
        fail(f"summary.minimum_useful_lanes must be {MIN_USEFUL_LANES}")
    if summary.get("reliability_gate") != "passed":
        fail("summary.reliability_gate must be passed")
    if summary.get("disposition") != "useful_with_limits":
        fail("summary.disposition must be useful_with_limits")
    useful_models = summary.get("useful_models")
    if not isinstance(useful_models, list) or "gemma4:31b" not in useful_models:
        fail("summary.useful_models must include gemma4:31b")
    if len(useful_models) < MIN_USEFUL_LANES:
        fail(f"summary.useful_models must include at least {MIN_USEFUL_LANES} useful Gemma lanes")

    print("PASS: remote gemma watcher proof bundle valid")


if __name__ == "__main__":
    main()
