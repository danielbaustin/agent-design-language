#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
planner="$repo_root/adl/tools/plan_multi_agent_workcell.py"
allowed_fixture="$repo_root/docs/milestones/v0.91.4/review/multi_agent_workcell/fixtures/workcell_assignment_allowed.json"
blocked_fixture="$repo_root/docs/milestones/v0.91.4/review/multi_agent_workcell/fixtures/workcell_assignment_blocked.json"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

allowed_json="$tmpdir/allowed.json"
blocked_json="$tmpdir/blocked.json"

python3 "$planner" "$allowed_fixture" --json-out "$allowed_json" > "$tmpdir/allowed.txt"
python3 "$planner" "$blocked_fixture" --json-out "$blocked_json" > "$tmpdir/blocked.txt"

grep -Fq 'Safe parallel worker set:' "$tmpdir/allowed.txt"
grep -Fq -- '- ollama-worker-a' "$tmpdir/allowed.txt"
grep -Fq -- '- codex-worker-b' "$tmpdir/allowed.txt"
grep -Fq 'serialized-worker-c (issue #4003, role=worker, backend=local_ollama, model=mistral-small3.2:24b): serial_only' "$tmpdir/allowed.txt"
grep -Fq 'independent-reviewer (issue #4004, role=reviewer): review_ready' "$tmpdir/allowed.txt"
grep -Fq 'pr-janitor (issue #4005, role=janitor): janitor_ready' "$tmpdir/allowed.txt"
grep -Fq 'closeout-lane (issue #4006, role=closeout): closeout_ready' "$tmpdir/allowed.txt"
grep -Fq 'codex-worker-b (issue #4002, role=worker, backend=hosted_codex, model=gpt-5.5-codex): ready' "$tmpdir/allowed.txt"
grep -Fq 'ollama-worker-a (issue #4001, role=worker, backend=local_ollama, model=qwen2.5-coder:32b): ready' "$tmpdir/allowed.txt"

grep -Fq 'conflict-worker-a (issue #4011, role=worker, backend=local_ollama, model=qwen2.5-coder:32b): blocked' "$tmpdir/blocked.txt"
grep -Fq 'conflict-worker-b (issue #4012, role=worker, backend=hosted_codex, model=gpt-5.5-codex): blocked' "$tmpdir/blocked.txt"
grep -Fq 'not-ready-worker (issue #4013, role=worker, backend=local_ollama, model=mistral-small3.2:24b): blocked' "$tmpdir/blocked.txt"
grep -Fq 'blocked-dependency-worker (issue #4014, role=worker, backend=hosted_codex, model=gpt-5.5-codex): blocked' "$tmpdir/blocked.txt"
grep -Fq 'transitive-dependent-worker (issue #4015, role=worker, backend=local_ollama, model=qwen2.5-coder:32b): blocked' "$tmpdir/blocked.txt"

python3 - "$allowed_json" "$blocked_json" <<'PY'
import json
import sys
from pathlib import Path

allowed = json.loads(Path(sys.argv[1]).read_text())
blocked = json.loads(Path(sys.argv[2]).read_text())
assert allowed["safe_parallel_workers"] == ["codex-worker-b", "ollama-worker-a"]
assert allowed["serial_only_workers"] == ["serialized-worker-c"]
status_map = {item["shard_id"]: item["classification"] for item in allowed["results"]}
backend_map = {item["shard_id"]: item["execution_backend"] for item in allowed["results"]}
model_map = {item["shard_id"]: item["model_hint"] for item in allowed["results"]}
assert status_map["independent-reviewer"] == "review_ready"
assert status_map["pr-janitor"] == "janitor_ready"
assert status_map["closeout-lane"] == "closeout_ready"
assert backend_map["ollama-worker-a"] == "local_ollama"
assert backend_map["codex-worker-b"] == "hosted_codex"
assert model_map["codex-worker-b"] == "gpt-5.5-codex"
blocked_map = {item["shard_id"]: item["classification"] for item in blocked["results"]}
assert set(blocked_map.values()) == {"blocked"}
assert blocked_map["transitive-dependent-worker"] == "blocked"
PY

echo "PASS test_plan_multi_agent_workcell"
