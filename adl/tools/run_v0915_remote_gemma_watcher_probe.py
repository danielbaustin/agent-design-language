#!/usr/bin/env python3
"""Run the bounded v0.91.5 remote Gemma watcher usefulness probe."""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request
from dataclasses import dataclass
from datetime import UTC, datetime
from pathlib import Path, PurePosixPath

OLLAMA_BASE_URL = "http://192.168.68.70:11434"
ISSUE_NUMBER = 3724
RUN_DATE = "2026-06-15"
RUN_ID = "v0915-remote-gemma-watcher-20260615"
STATE_SCHEMA = "adl.remote_gemma_watcher_probe.v1"
PACKET_DIR = PurePosixPath("docs/milestones/v0.91.5/review/remote_gemma_watcher")
LANE_OUTPUT_DIR = PACKET_DIR / "lane_outputs"
LOCAL_ARTIFACT_DIR = PurePosixPath(".adl/reports/v0.91.5/remote_gemma_watcher/artifacts")
STATE_PATH = PACKET_DIR / "v0915_remote_gemma_watcher_state_2026-06-15.json"
PACKET_PATH = PACKET_DIR / "REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md"
HISTORICAL_EMPTY_OUTPUT = PurePosixPath(
    "docs/milestones/v0.91.5/review/multi_agent_workcell/lane_outputs/"
    "watcher_remote_gemma4_e2b.md"
)
HISTORICAL_WORKCELL_STATE = PurePosixPath(
    "docs/milestones/v0.91.5/review/multi_agent_workcell/"
    "v0915_parallel_csdlc_workcell_state_2026-06-14.json"
)
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
PROBE_REQUEST = """You are evaluating remote Gemma watcher usefulness for issue #3724.
Read this bounded context:
- repo: agent-design-language
- review surface: docs/milestones/v0.91.5/review/multi_agent_workcell
Return short markdown with exactly these headings:
# Status
# Signal
# Next-Step
Requirements:
- Mention issue #3724
- Mention remote Gemma watcher usefulness
- Cite docs/milestones/v0.91.5/review/multi_agent_workcell exactly
- Mention the historical empty output from the older watcher lane
- State whether this probe produced a non-empty watcher summary
- Include the exact phrase route probe completed
- Give one concrete next step tied to remote Gemma watcher usefulness
- Do not invent incidents like latency spikes, gateway failures, or infrastructure alerts
- Keep total length under 140 words.
"""


@dataclass
class LaneResult:
    lane_id: str
    execution_surface: str
    model: str
    status: str
    duration_seconds: float
    output_path: PurePosixPath
    output_text: str
    notes: list[str]


def fail(message: str) -> None:
    print(f"run_v0915_remote_gemma_watcher_probe: {message}", file=sys.stderr)
    raise SystemExit(1)


def repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def repo_path(path: PurePosixPath) -> Path:
    return repo_root() / Path(path.as_posix())


def write_text(path: PurePosixPath, text: str) -> None:
    abs_path = repo_path(path)
    abs_path.parent.mkdir(parents=True, exist_ok=True)
    abs_path.write_text(text, encoding="utf-8")


def write_json(path: PurePosixPath, data: object) -> None:
    write_text(path, json.dumps(data, indent=2) + "\n")


def http_get_json(url: str, timeout: int = 30) -> object:
    with urllib.request.urlopen(url, timeout=timeout) as response:
        return json.load(response)


def http_post_json(url: str, payload: object, timeout: int = 180) -> object:
    request = urllib.request.Request(
        url,
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(request, timeout=timeout) as response:
        return json.load(response)


def classify_output(text: str) -> tuple[str, list[str]]:
    notes: list[str] = []
    stripped = text.strip()
    if not stripped:
        return "empty_output", ["provider returned empty text"]
    missing = [heading for heading in REQUIRED_HEADINGS if heading not in stripped]
    if missing:
        notes.append(f"missing required headings: {', '.join(missing)}")
    lowered = stripped.lower()
    if REQUIRED_PHRASE not in lowered:
        notes.append(f"missing required phrase: {REQUIRED_PHRASE}")
    for snippet in REQUIRED_CONTEXT_SNIPPETS:
        if snippet.lower() not in lowered:
            notes.append(f"missing required context snippet: {snippet}")
    if len(stripped) > 800:
        notes.append("output is longer than expected for a bounded watcher summary")
    return ("useful_output" if not notes else "partially_useful_output", notes)


def fetch_tags_snapshot() -> dict[str, object]:
    tags = http_get_json(f"{OLLAMA_BASE_URL}/api/tags", timeout=30)
    if not isinstance(tags, dict):
        fail("unexpected /api/tags response shape")
    return tags


def ensure_model_present(tags: dict[str, object], model: str) -> None:
    names = {item.get("name") for item in tags.get("models", []) if isinstance(item, dict)}
    if model not in names:
        fail(f"required model missing from remote host: {model}")


def run_raw_lane(model: str, output_name: str, artifact_name: str) -> LaneResult:
    payload = {"model": model, "prompt": PROBE_REQUEST, "stream": False, "options": {"temperature": 0}}
    start = time.perf_counter()
    response = http_post_json(f"{OLLAMA_BASE_URL}/api/generate", payload, timeout=240)
    duration = time.perf_counter() - start
    if not isinstance(response, dict):
        fail(f"unexpected raw response shape for {model}")
    output_text = str(response.get("response", "") or "")
    status, notes = classify_output(output_text)
    output_path = LANE_OUTPUT_DIR / output_name
    artifact_path = LOCAL_ARTIFACT_DIR / artifact_name
    write_json(artifact_path, response)
    return LaneResult(
        lane_id=output_name.removesuffix(".md"),
        execution_surface="raw_ollama_http",
        model=model,
        status=status,
        duration_seconds=duration,
        output_path=output_path,
        output_text=output_text,
        notes=notes,
    )


def build_adapter() -> Path:
    root = repo_root()
    subprocess.run(
        ["cargo", "build", "--manifest-path", "adl/Cargo.toml", "--bin", "adl-provider-adapter"],
        cwd=root,
        check=True,
    )
    return root / "adl" / "target" / "debug" / "adl-provider-adapter"


def run_adapter_lane(adapter_bin: Path, model: str, output_name: str) -> LaneResult:
    with tempfile.TemporaryDirectory() as tmp_dir:
        temp = Path(tmp_dir)
        request_path = temp / "request.json"
        out_path = temp / "result.json"
        log_path = temp / "adapter.log"
        request = {
            "route": {
                "provider_kind": "local",
                "provider": "ollama",
                "runtime_surface": "ollama_http",
                "provider_model_id": model,
                "endpoint_ref": OLLAMA_BASE_URL,
            },
            "model_identity": {
                "provider_kind": "local",
                "provider": "ollama",
                "model_ref": model,
                "provider_model_id": model,
                "runtime_surface": "ollama_http",
                "identity_strength": "tag_only",
                "observed_at": f"unix:{int(time.time())}",
            },
            "prompt_contract_ref": "v0915.remote_gemma_watcher_probe.v1",
            "lane_ref": "remote_gemma_watcher_probe",
            "run_id": RUN_ID,
            "request_id": f"{RUN_ID}-{model.replace(':', '-')}",
            "attempt_policy": {
                "max_attempts": 1,
                "timeout_ms": 180000,
                "retry_backoff_ms": 1000,
            },
            "input_text": PROBE_REQUEST,
        }
        request_path.write_text(json.dumps(request, indent=2), encoding="utf-8")
        start = time.perf_counter()
        subprocess.run(
            [
                str(adapter_bin),
                "--request",
                str(request_path),
                "--out",
                str(out_path),
                "--log",
                str(log_path),
            ],
            cwd=repo_root(),
            check=True,
        )
        duration = time.perf_counter() - start
        result = json.loads(out_path.read_text(encoding="utf-8"))
        output_text = str(result.get("output_text", "") or "")
        status, notes = classify_output(output_text)
        if result.get("final_status") != "ok":
            status = "failed"
            notes.append(f"provider final_status={result.get('final_status')}")
        output_path = LANE_OUTPUT_DIR / output_name
        result_artifact = LOCAL_ARTIFACT_DIR / output_name.replace(".md", "_result.json")
        log_artifact = LOCAL_ARTIFACT_DIR / output_name.replace(".md", "_adapter.log")
        write_json(result_artifact, result)
        write_text(log_artifact, log_path.read_text(encoding="utf-8"))
        return LaneResult(
            lane_id=output_name.removesuffix(".md"),
            execution_surface="adl_provider_adapter",
            model=model,
            status=status,
            duration_seconds=duration,
            output_path=output_path,
            output_text=output_text,
            notes=notes,
        )


def build_state(tags: dict[str, object], lanes: list[LaneResult]) -> dict[str, object]:
    gemma_models = sorted(
        item["name"]
        for item in tags.get("models", [])
        if isinstance(item, dict)
        and isinstance(item.get("name"), str)
        and item["name"].startswith("gemma")
    )
    return {
        "schema_version": STATE_SCHEMA,
        "issue_number": ISSUE_NUMBER,
        "run_id": RUN_ID,
        "generated_at": datetime.now(UTC).isoformat().replace("+00:00", "Z"),
        "endpoint_ref": OLLAMA_BASE_URL,
        "historical_context": {
            "issue_number": 3415,
            "state_packet_path": HISTORICAL_WORKCELL_STATE.as_posix(),
            "empty_output_path": HISTORICAL_EMPTY_OUTPUT.as_posix(),
            "historical_status": "completed_unhelpful_output",
        },
        "inventory": {
            "model_count": len(tags.get("models", [])),
            "gemma_models": gemma_models,
            "tags_snapshot_policy": "local_only_not_tracked",
        },
        "lanes": [
            {
                "lane_id": lane.lane_id,
                "execution_surface": lane.execution_surface,
                "model": lane.model,
                "status": lane.status,
                "duration_seconds": round(lane.duration_seconds, 3),
                "output_path": lane.output_path.as_posix(),
                "notes": lane.notes,
            }
            for lane in lanes
        ],
        "summary": {
            "useful_models": [lane.model for lane in lanes if lane.status == "useful_output"],
            "primary_proving_lane": PRIMARY_PROVING_LANE,
            "primary_required_status": "useful_output",
            "minimum_useful_lanes": MIN_USEFUL_LANES,
            "reliability_gate": "passed",
            "disposition": "useful_with_limits",
        },
    }


def enforce_reliability_gate(lanes: list[LaneResult]) -> None:
    by_lane = {lane.lane_id: lane for lane in lanes}
    primary = by_lane.get(PRIMARY_PROVING_LANE)
    if primary is None:
        fail(f"missing primary Gemma proving lane: {PRIMARY_PROVING_LANE}")
    if primary.status != "useful_output":
        notes = "; ".join(primary.notes) if primary.notes else "no notes"
        fail(f"{PRIMARY_PROVING_LANE} did not produce useful output: {primary.status}; {notes}")
    useful = [lane for lane in lanes if lane.status == "useful_output"]
    if len(useful) < MIN_USEFUL_LANES:
        statuses = ", ".join(f"{lane.lane_id}={lane.status}" for lane in lanes)
        fail(f"Gemma reliability gate requires at least {MIN_USEFUL_LANES} useful lanes; observed {statuses}")


def packet_text(state: dict[str, object], lanes: list[LaneResult]) -> str:
    lane_rows = []
    for lane in lanes:
        lane_rows.append(
            f"| `{lane.lane_id}` | `{lane.execution_surface}` | `{lane.model}` | "
            f"`{lane.status}` | `{lane.output_path.as_posix()}` |"
        )
    useful = [lane for lane in lanes if lane.status == "useful_output"]
    useful_models = ", ".join(f"`{lane.model}`" for lane in useful) or "none"
    return f"""# Remote Gemma Watcher Proof 2026-06-15

Date: {RUN_DATE}

Issue: `#{ISSUE_NUMBER}`

Run ID: `{RUN_ID}`

Status: `useful_with_limits`

## Purpose

This packet records a bounded follow-on probe for the Sprint 2 remote Gemma
watcher non-claim. The historical `#3415` workcell packet kept the watcher lane
truthful as completed but empty. This issue tests whether a larger remote Gemma4
route can produce useful watcher output under a tighter bounded prompt.

## Historical Baseline

The prior tracked watcher output remains unchanged:

- historical empty output: `{HISTORICAL_EMPTY_OUTPUT.as_posix()}`
- historical state packet: `{HISTORICAL_WORKCELL_STATE.as_posix()}`

That older lane is still true as a historical fact. This packet does not rewrite
that run. It adds new bounded evidence from `#3724`.

## Probe Summary

| Lane | Surface | Model | Status | Output |
| --- | --- | --- | --- | --- |
{chr(10).join(lane_rows)}

## What Was Proven

- The remote Ollama host at `{OLLAMA_BASE_URL}` is reachable for bounded watcher probes.
- The larger Gemma4 routes can return non-empty structured watcher text.
- The ADL-native provider path is proven through `adl-provider-adapter` on
  `gemma4:31b`, not only through raw HTTP.
- The historical empty-output issue is no longer the only observed watcher outcome.

## Primary Result

The strongest proving lane is `adapter_gemma4_31b`. It returned reviewer-usable
markdown with the required watcher headings and the exact phrase
`{REQUIRED_PHRASE}` through the real ADL provider adapter surface.

Secondary useful routes also returned structured watcher text: {useful_models}.

## Reliability Gate

This runner fails closed unless `{PRIMARY_PROVING_LANE}` returns
`useful_output` through the ADL provider adapter and at least
`{MIN_USEFUL_LANES}` Gemma lanes return useful structured watcher text. Smaller
or historically weak routes such as `gemma4:e2b` are not promoted as reliable
watcher lanes by this proof.

## Disposition

Remote Gemma watcher usefulness is now **proven in a bounded way** for short,
structured watcher prompts on larger Gemma4 routes. The lane remains
`useful_with_limits` rather than broadly proven because:

- this packet only covers one bounded prompt shape
- it does not prove full multi-agent planning or janitor usefulness
- it does not prove `gemma4:e2b` is universally recovered for the original
  historical workcell prompt

## Validation

- `python3 adl/tools/run_v0915_remote_gemma_watcher_probe.py`
- `python3 adl/tools/validate_v0915_remote_gemma_watcher_probe.py docs/milestones/v0.91.5/review/remote_gemma_watcher`
- `bash adl/tools/test_v0915_remote_gemma_watcher_probe.sh`
- `git diff --check`

## Non-Claims

- This packet does not claim broad remote Gemma autonomy.
- This packet does not claim Sprint 2 multi-agent quality is fully proven.
- This packet does not replace the historical `#3415` workcell packet.
- This packet does not prove every Gemma4 size or prompt shape is equally useful.
"""


def main() -> None:
    tags = fetch_tags_snapshot()
    write_json(LOCAL_ARTIFACT_DIR / "ollama_tags_snapshot.json", tags)
    for model in ("gemma4:31b", "gemma4:26b", "gemma4:e4b"):
        ensure_model_present(tags, model)
    adapter_bin = build_adapter()
    lanes = [
        run_adapter_lane(adapter_bin, "gemma4:31b", "adapter_gemma4_31b.md"),
        run_raw_lane("gemma4:26b", "raw_gemma4_26b.md", "raw_gemma4_26b_response.json"),
        run_raw_lane("gemma4:e4b", "raw_gemma4_e4b.md", "raw_gemma4_e4b_response.json"),
    ]
    enforce_reliability_gate(lanes)
    for lane in lanes:
        write_text(lane.output_path, lane.output_text.rstrip() + "\n")
    state = build_state(tags, lanes)
    write_json(STATE_PATH, state)
    write_text(PACKET_PATH, packet_text(state, lanes))
    print(f"Wrote {PACKET_PATH.as_posix()}")
    print(f"Wrote {STATE_PATH.as_posix()}")


if __name__ == "__main__":
    try:
        main()
    except urllib.error.URLError as exc:
        fail(f"network probe failed: {exc}")
