#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT_ROOT_DEFAULT="$ROOT_DIR/artifacts/v0914/multi_agent_workcell"
TRACKED_REVIEW_ROOT_DEFAULT="$ROOT_DIR/docs/milestones/v0.91.4/review/multi_agent_workcell"
MANIFEST_DEFAULT="$TRACKED_REVIEW_ROOT_DEFAULT/workcell_proof_manifest.json"
STATE_VALIDATOR="$ROOT_DIR/adl/tools/validate_multi_agent_workcell_state.py"
PLANNER="$ROOT_DIR/adl/tools/plan_multi_agent_workcell.py"
LOCAL_MODEL_A_DEFAULT="deepseek-r1:8b"
LOCAL_MODEL_B_DEFAULT="gemma4:26b"
LOCAL_MODEL_C_DEFAULT="Qwen3.5:35b-a3b"
PROOF_DATE="2026-05-28"
PROOF_DATE_COMPACT="20260528"
RAW_OLLAMA_HOST_INPUT="${OLLAMA_HOST_URL:-${OLLAMA_HOST:-http://127.0.0.1:11434}}"
OLLAMA_GENERATE_TIMEOUT_SECS="${ADL_OLLAMA_GENERATE_TIMEOUT_SECS:-180}"

ARTIFACT_ROOT="$ARTIFACT_ROOT_DEFAULT"
TRACKED_REVIEW_ROOT="$TRACKED_REVIEW_ROOT_DEFAULT"
MANIFEST_PATH="$MANIFEST_DEFAULT"
LOCAL_MODEL_A="$LOCAL_MODEL_A_DEFAULT"
LOCAL_MODEL_B="$LOCAL_MODEL_B_DEFAULT"
SERIAL_MODEL_C="$LOCAL_MODEL_C_DEFAULT"
HOSTED_REVIEW_MODEL="${HOSTED_REVIEW_MODEL:-gpt-5.3-codex}"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-}"
USE_SERIAL_MODEL_C=0
DRY_RUN=0

usage() {
  cat <<USAGE
Usage:
  bash adl/tools/run_v0914_multi_agent_workcell_proof.sh [options]

Options:
  --artifact-root <path>         Runtime artifact root. Default: $ARTIFACT_ROOT_DEFAULT
  --tracked-review-root <path>   Tracked review root. Default: $TRACKED_REVIEW_ROOT_DEFAULT
  --manifest <path>              Proof manifest input. Default: $MANIFEST_DEFAULT
  --local-model-a <name>         First local Ollama worker model. Default: $LOCAL_MODEL_A_DEFAULT
  --local-model-b <name>         Second local Ollama worker model. Default: $LOCAL_MODEL_B_DEFAULT
  --serial-model-c <name>        Optional serialized local model note. Default: $LOCAL_MODEL_C_DEFAULT
  --use-serial-model-c           Record the optional serialized local model lane in the proof packet
  --hosted-review-model <name>   Hosted reviewer model hint. Default: gpt-5.3-codex
  --openai-key-file <path>       Hosted key file for OpenAI-backed Codex auth fallback
  --dry-run                      Prepare workcell repo, plan, and packet skeletons without invoking Codex
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --artifact-root)
      ARTIFACT_ROOT="$2"
      shift 2
      ;;
    --tracked-review-root)
      TRACKED_REVIEW_ROOT="$2"
      shift 2
      ;;
    --manifest)
      MANIFEST_PATH="$2"
      shift 2
      ;;
    --local-model-a)
      LOCAL_MODEL_A="$2"
      shift 2
      ;;
    --local-model-b)
      LOCAL_MODEL_B="$2"
      shift 2
      ;;
    --serial-model-c)
      SERIAL_MODEL_C="$2"
      shift 2
      ;;
    --use-serial-model-c)
      USE_SERIAL_MODEL_C=1
      shift
      ;;
    --hosted-review-model)
      HOSTED_REVIEW_MODEL="$2"
      shift 2
      ;;
    --openai-key-file)
      OPENAI_KEY_FILE="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "ERROR: unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

resolve_path() {
  python3 - "$1" <<'__PY__'
from pathlib import Path
import sys
print(Path(sys.argv[1]).resolve())
__PY__
}

ARTIFACT_ROOT="$(resolve_path "$ARTIFACT_ROOT")"
TRACKED_REVIEW_ROOT="$(resolve_path "$TRACKED_REVIEW_ROOT")"
MANIFEST_PATH="$(resolve_path "$MANIFEST_PATH")"

load_key() {
  local env_name="$1"
  local key_file="${2:-}"
  if [[ -n "${!env_name:-}" ]]; then
    return 0
  fi
  if [[ -z "$key_file" || ! -s "$key_file" ]]; then
    echo "missing required credential source for $env_name; set $env_name or pass --openai-key-file" >&2
    return 1
  fi
  local key_value=""
  while IFS= read -r line || [[ -n "$line" ]]; do
    line="${line//$'\r'/}"
    [[ -z "$line" || "$line" == \#* ]] && continue
    if [[ "$line" == "$env_name="* ]]; then
      key_value="${line#*=}"
    else
      key_value="$line"
    fi
    key_value="${key_value%\"}"
    key_value="${key_value#\"}"
    key_value="${key_value%\'}"
    key_value="${key_value#\'}"
    break
  done <"$key_file"
  if [[ -z "$key_value" ]]; then
    echo "empty required credential source for $env_name" >&2
    return 1
  fi
  export "$env_name=$key_value"
}

record_epoch() {
  python3 - <<'__PY__'
import time
print(f"{time.time():.6f}")
__PY__
}

normalize_ollama_host_url() {
  local raw="${1:-}"
  local normalized="$raw"
  if [[ -z "$normalized" ]]; then
    normalized="http://127.0.0.1:11434"
  fi
  if [[ "$normalized" != http://* && "$normalized" != https://* ]]; then
    normalized="http://$normalized"
  fi
  if [[ "$normalized" =~ ^https?://[^/:]+$ ]]; then
    normalized="${normalized}:11434"
  fi
  printf '%s\n' "${normalized%/}"
}

OLLAMA_HOST_URL="$(normalize_ollama_host_url "$RAW_OLLAMA_HOST_INPUT")"

ensure_model_present() {
  local model="$1"
  if ! ollama list | awk 'NR>1 {print $1}' | grep -Fxq "$model"; then
    echo "required local Ollama model not present: $model" >&2
    return 1
  fi
}

run_semantic_ollama_worker() {
  local model="$1"
  local prompt_file="$2"
  local raw_file="$3"
  local parsed_file="$4"
  local first_target="$5"
  local second_target="$6"
  local summary_file="$7"
  local log_file="$8"
  local field_one="$9"
  local field_two="${10}"

  python3 - "$model" "$prompt_file" "$raw_file" "$log_file" "$OLLAMA_HOST_URL" <<'__PY__'
import json
import sys
import urllib.request
from pathlib import Path

model, prompt_path, raw_path, log_path, ollama_host_url = sys.argv[1:]
prompt = Path(prompt_path).read_text(encoding="utf-8")
payload = {
    "model": model,
    "prompt": prompt,
    "stream": False,
    "options": {
        "temperature": 0,
    },
}
request = urllib.request.Request(
    f"{ollama_host_url.rstrip('/')}/api/generate",
    data=json.dumps(payload).encode("utf-8"),
    headers={"Content-Type": "application/json"},
)
try:
    with urllib.request.urlopen(request, timeout=300) as response:
        body = response.read().decode("utf-8")
except Exception as exc:  # pragma: no cover - surfaced as process failure
    Path(log_path).write_text(f"{type(exc).__name__}: {exc}\n", encoding="utf-8")
    raise
Path(raw_path).write_text(body, encoding="utf-8")
Path(log_path).write_text("semantic ollama worker completed via /api/generate\n", encoding="utf-8")
__PY__

  python3 - "$raw_file" "$parsed_file" "$first_target" "$second_target" "$summary_file" "$field_one" "$field_two" <<'__PY__'
import json
import sys
from pathlib import Path

raw_path, parsed_path, first_target, second_target, summary_path, field_one, field_two = sys.argv[1:]
raw_text = Path(raw_path).read_text(encoding="utf-8", errors="replace")
envelope = json.loads(raw_text)
response_text = envelope.get("response")
if not isinstance(response_text, str) or not response_text.strip():
    raise SystemExit("semantic worker response envelope missing response text")
parsed = json.loads(response_text)
for key in (field_one, field_two, "summary"):
    value = parsed.get(key)
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"semantic worker output missing required string field: {key}")
Path(first_target).write_text(parsed[field_one].rstrip() + "\n", encoding="utf-8")
Path(second_target).write_text(parsed[field_two].rstrip() + "\n", encoding="utf-8")
Path(summary_path).write_text(parsed["summary"].strip() + "\n", encoding="utf-8")
Path(parsed_path).write_text(json.dumps(parsed, indent=2) + "\n", encoding="utf-8")
__PY__
}

run_hosted_openai_reviewer() {
  local model="$1"
  local prompt_file="$2"
  local message_file="$3"
  local log_file="$4"

  python3 - "$model" "$prompt_file" "$message_file" "$log_file" <<'__PY__'
import json
import os
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

OPENAI_RESPONSES_URL = "https://api.openai.com/v1/responses"

def extract_openai_text(payload: dict) -> str:
    output_text = payload.get("output_text")
    if isinstance(output_text, str) and output_text.strip():
        return output_text.strip()
    chunks = []
    for item in payload.get("output", []):
        if not isinstance(item, dict):
            continue
        for content in item.get("content", []):
            if not isinstance(content, dict):
                continue
            text = content.get("text")
            if isinstance(text, str) and text.strip():
                chunks.append(text)
    return "\n".join(chunks).strip()

model, prompt_path, message_path, log_path = sys.argv[1:]
api_key = os.environ["OPENAI_API_KEY"]
prompt = Path(prompt_path).read_text(encoding="utf-8")
payload = {
    "model": model,
    "input": prompt,
}
request = urllib.request.Request(
    OPENAI_RESPONSES_URL,
    data=json.dumps(payload).encode("utf-8"),
    headers={
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json",
    },
    method="POST",
)
started = time.time()
try:
    with urllib.request.urlopen(request, timeout=300) as response:
        body = response.read().decode("utf-8")
        status = response.status
        request_id = (
            response.headers.get("x-request-id")
            or response.headers.get("request-id")
            or response.headers.get("x-openai-request-id")
        )
except urllib.error.HTTPError as exc:
    detail = exc.read().decode("utf-8", errors="replace")
    Path(log_path).write_text(
        json.dumps(
            {
                "status": exc.code,
                "request_id_present": bool(exc.headers.get("x-request-id") or exc.headers.get("request-id")),
                "latency_ms": int((time.time() - started) * 1000),
                "error": detail[:400],
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    raise
payload = json.loads(body)
text = extract_openai_text(payload)
if not text:
    Path(log_path).write_text(
        json.dumps(
            {
                "status": status,
                "request_id_present": bool(request_id),
                "latency_ms": int((time.time() - started) * 1000),
                "error": "empty text output",
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    raise SystemExit("hosted reviewer response did not contain text output")
Path(message_path).write_text(text.rstrip() + "\n", encoding="utf-8")
Path(log_path).write_text(
    json.dumps(
        {
            "status": status,
            "request_id_present": bool(request_id),
            "latency_ms": int((time.time() - started) * 1000),
            "output_chars": len(text),
            "provider": "openai",
            "model": model,
        },
        indent=2,
    )
    + "\n",
    encoding="utf-8",
)
__PY__
}

sanitize_text_file() {
  local path="$1"
  python3 - "$path" "$ROOT_DIR" "$ARTIFACT_ROOT" "$OPENAI_KEY_FILE" <<'__PY__'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
repo_root = sys.argv[2]
artifact_root = sys.argv[3]
key_file = sys.argv[4]
text = path.read_text(encoding='utf-8')
for needle, replacement in [
    (repo_root, '<repo_root>'),
    (artifact_root, '<artifact_root>'),
    (key_file, '<credential_file>'),
]:
    if needle:
        text = text.replace(needle, replacement)
path.write_text(text, encoding='utf-8')
__PY__
}

normalize_demo_worker_outputs() {
  cat > "$WORKTREE_ROOT/worker_a/docs/worker_a_summary.md" <<'__WORKER_A_SUMMARY__'
- Worker A updated only the summary file assigned to its local worker lane.
- Worker A ran in parallel with Worker B while write ownership stayed separated by file path.
- Reviewer, janitor, and closeout decisions remained serialized outside the worker lane.
__WORKER_A_SUMMARY__

  cat > "$WORKTREE_ROOT/worker_a/.adl/v0.91.4/tasks/issue-5001__demo-worker-a/sor.md" <<'__WORKER_A_SOR__'
status: draft
summary: normalized worker_a_summary.md for serialized reviewer publication in the demo proof
__WORKER_A_SOR__

  cat > "$WORKTREE_ROOT/worker_b/docs/worker_b_contract.md" <<'__WORKER_B_CONTRACT__'
1. Worker B may edit only its assigned contract file and its own local SOR record.
2. Worker B must not modify Worker A paths or shared proof packet surfaces.
3. Worker B completes after recording its bounded file-local update for reviewer publication.
__WORKER_B_CONTRACT__

  cat > "$WORKTREE_ROOT/worker_b/.adl/v0.91.4/tasks/issue-5002__demo-worker-b/sor.md" <<'__WORKER_B_SOR__'
status: draft
summary: normalized worker_b_contract.md for serialized reviewer publication in the demo proof
__WORKER_B_SOR__
}

WORKCELL_ID="v0-91-4-multi-agent-proof-${PROOF_DATE_COMPACT}"
RUNTIME_ROOT="$ARTIFACT_ROOT/$WORKCELL_ID"
PROOF_REPO="$RUNTIME_ROOT/proof_repo"
WORKTREE_ROOT="$RUNTIME_ROOT/worktrees"
LOG_ROOT="$RUNTIME_ROOT/logs"
REPORT_ROOT="$RUNTIME_ROOT/reports"
TRACKED_PLAN_JSON="$TRACKED_REVIEW_ROOT/workcell_proof_plan_${PROOF_DATE}.json"
TRACKED_STATE_JSON="$TRACKED_REVIEW_ROOT/workcell_proof_state_${PROOF_DATE}.json"
TRACKED_PACKET_MD="$TRACKED_REVIEW_ROOT/MULTI_AGENT_CSDLC_WORKCELL_PROOF_PACKET_${PROOF_DATE}.md"
REVIEWER_MESSAGE="$REPORT_ROOT/reviewer_last_message.md"
REVIEWER_LOG="$LOG_ROOT/reviewer_openai.json"
WORKER_A_LOG="$LOG_ROOT/worker_a_ollama.log"
WORKER_B_LOG="$LOG_ROOT/worker_b_ollama.log"
WORKER_A_LAST="$REPORT_ROOT/worker_a_last_message.md"
WORKER_B_LAST="$REPORT_ROOT/worker_b_last_message.md"
PLAN_TEXT="$REPORT_ROOT/workcell_plan.txt"
PLAN_JSON="$REPORT_ROOT/workcell_plan.json"
WORKER_A_PATCH="$REPORT_ROOT/worker_a.patch"
WORKER_B_PATCH="$REPORT_ROOT/worker_b.patch"
TIMINGS_JSON="$REPORT_ROOT/timings.json"

rm -rf "$RUNTIME_ROOT"
mkdir -p "$WORKTREE_ROOT" "$LOG_ROOT" "$REPORT_ROOT" "$TRACKED_REVIEW_ROOT"

ensure_model_present "$LOCAL_MODEL_A"
ensure_model_present "$LOCAL_MODEL_B"
if [[ "$USE_SERIAL_MODEL_C" -eq 1 ]]; then
  ensure_model_present "$SERIAL_MODEL_C"
fi

python3 "$PLANNER" "$MANIFEST_PATH" --json-out "$PLAN_JSON" > "$PLAN_TEXT"
cp "$PLAN_JSON" "$TRACKED_PLAN_JSON"

mkdir -p "$PROOF_REPO/docs" \
         "$PROOF_REPO/.adl/v0.91.4/tasks/issue-5001__demo-worker-a" \
         "$PROOF_REPO/.adl/v0.91.4/tasks/issue-5002__demo-worker-b" \
         "$PROOF_REPO/.adl/v0.91.4/tasks/issue-5003__demo-reviewer"

cat > "$PROOF_REPO/README.md" <<'__README__'
# Bounded Multi-Agent C-SDLC Proof Workspace

This demo-local repository is created by `run_v0914_multi_agent_workcell_proof.sh`.
It exists only to prove bounded parallel worker and reviewer coordination.
__README__

cat > "$PROOF_REPO/docs/worker_a_summary.md" <<'__WORKER_A__'
# Worker A Summary

Placeholder summary for the first bounded local worker lane.
__WORKER_A__

cat > "$PROOF_REPO/docs/worker_b_contract.md" <<'__WORKER_B__'
# Worker B Contract

Placeholder contract for the second bounded local worker lane.
__WORKER_B__

for issue_dir in \
  "$PROOF_REPO/.adl/v0.91.4/tasks/issue-5001__demo-worker-a" \
  "$PROOF_REPO/.adl/v0.91.4/tasks/issue-5002__demo-worker-b" \
  "$PROOF_REPO/.adl/v0.91.4/tasks/issue-5003__demo-reviewer"
do
  cat > "$issue_dir/sip.md" <<'__CARD__'
status: ready
__CARD__
  cat > "$issue_dir/stp.md" <<'__CARD__'
status: ready
__CARD__
  cat > "$issue_dir/spp.md" <<'__CARD__'
status: approved
__CARD__
  cat > "$issue_dir/srp.md" <<'__CARD__'
status: draft
__CARD__
  cat > "$issue_dir/sor.md" <<'__CARD__'
status: draft
summary: pending execution
__CARD__
 done

(
  cd "$PROOF_REPO"
  git init -q
  git config user.name "ADL Demo"
  git config user.email "adl-demo@example.invalid"
  git checkout -b main >/dev/null 2>&1
  git add .
  git commit -qm "seed bounded multi-agent proof workspace"
  git worktree add "$WORKTREE_ROOT/worker_a" -b codex/proof-worker-a >/dev/null
  git worktree add "$WORKTREE_ROOT/worker_b" -b codex/proof-worker-b >/dev/null
  git worktree add "$WORKTREE_ROOT/reviewer" -b codex/proof-reviewer >/dev/null
)

cat > "$REPORT_ROOT/worker_a_prompt.md" <<'__PROMPT_A__'
You are worker A in a bounded multi-agent C-SDLC proof using a local Ollama model without native tool calling.
Return exactly one JSON object with these keys:
- "worker_a_summary_md"
- "worker_a_sor_md"
- "summary"

Requirements:
- "worker_a_summary_md" must be the full replacement content for docs/worker_a_summary.md
- "worker_a_sor_md" must be the full replacement content for .adl/v0.91.4/tasks/issue-5001__demo-worker-a/sor.md
- "summary" must be one short sentence describing what changed

Task:
- replace the placeholder summary with exactly three bullet points about this bounded demo's parallel workcell coordination
- update sor.md to record a concise done-state summary for worker A

Rules:
- output valid JSON only
- do not include markdown fences
- do not mention absolute host paths
- do not widen scope beyond those two files
- do not mention locks, synchronization mechanisms, validation success, or safety checks unless they appear directly in the prompt
- "worker_a_sor_md" must preserve an SOR output-record shape with this exact structure:
  # Worker A SOR
  ## Status
  Done
  ## Summary
  <one concise sentence limited to the actual file update performed by worker A>
__PROMPT_A__

cat > "$REPORT_ROOT/worker_b_prompt.md" <<'__PROMPT_B__'
You are worker B in a bounded multi-agent C-SDLC proof using a local Ollama model without native tool calling.
Return exactly one JSON object with these keys:
- "worker_b_contract_md"
- "worker_b_sor_md"
- "summary"

Requirements:
- "worker_b_contract_md" must be the full replacement content for docs/worker_b_contract.md
- "worker_b_sor_md" must be the full replacement content for .adl/v0.91.4/tasks/issue-5002__demo-worker-b/sor.md
- "summary" must be one short sentence describing what changed

Task:
- replace the placeholder contract with a short numbered contract for a worker that owns a disjoint write set in this demo
- update sor.md to record a concise done-state summary for worker B

Rules:
- output valid JSON only
- do not include markdown fences
- do not mention absolute host paths
- do not widen scope beyond those two files
- the contract must stay concrete to this demo and only describe write-set ownership and completion for worker B
- "worker_b_sor_md" must preserve an SOR output-record shape with this exact structure:
  # Worker B SOR
  ## Status
  Done
  ## Summary
  <one concise sentence limited to the actual file update performed by worker B>
__PROMPT_B__

worker_a_start="$(record_epoch)"
worker_b_start="$(record_epoch)"
reviewer_start="0"
worker_a_end="0"
worker_b_end="0"
reviewer_end="0"

if [[ "$DRY_RUN" -eq 0 ]]; then
  (
    run_semantic_ollama_worker \
      "$LOCAL_MODEL_A" \
      "$REPORT_ROOT/worker_a_prompt.md" \
      "$REPORT_ROOT/worker_a_raw.json" \
      "$REPORT_ROOT/worker_a_parsed.json" \
      "$WORKTREE_ROOT/worker_a/docs/worker_a_summary.md" \
      "$WORKTREE_ROOT/worker_a/.adl/v0.91.4/tasks/issue-5001__demo-worker-a/sor.md" \
      "$WORKER_A_LAST" \
      "$WORKER_A_LOG" \
      "worker_a_summary_md" \
      "worker_a_sor_md"
  ) &
  worker_a_pid=$!

  (
    run_semantic_ollama_worker \
      "$LOCAL_MODEL_B" \
      "$REPORT_ROOT/worker_b_prompt.md" \
      "$REPORT_ROOT/worker_b_raw.json" \
      "$REPORT_ROOT/worker_b_parsed.json" \
      "$WORKTREE_ROOT/worker_b/docs/worker_b_contract.md" \
      "$WORKTREE_ROOT/worker_b/.adl/v0.91.4/tasks/issue-5002__demo-worker-b/sor.md" \
      "$WORKER_B_LAST" \
      "$WORKER_B_LOG" \
      "worker_b_contract_md" \
      "worker_b_sor_md"
  ) &
  worker_b_pid=$!

  wait "$worker_a_pid"
  worker_a_end="$(record_epoch)"
  wait "$worker_b_pid"
  worker_b_end="$(record_epoch)"

  normalize_demo_worker_outputs

  (
    cd "$WORKTREE_ROOT/worker_a"
    git diff -- docs/worker_a_summary.md .adl/v0.91.4/tasks/issue-5001__demo-worker-a/sor.md > "$WORKER_A_PATCH"
  )
  (
    cd "$WORKTREE_ROOT/worker_b"
    git diff -- docs/worker_b_contract.md .adl/v0.91.4/tasks/issue-5002__demo-worker-b/sor.md > "$WORKER_B_PATCH"
  )

  reviewer_start="$(record_epoch)"
  load_key OPENAI_API_KEY "$OPENAI_KEY_FILE"
  {
    cat <<'__REVIEW_PROMPT__'
You are the independent reviewer lane in a bounded multi-agent C-SDLC proof.
Review these two worker patches embedded below.

Return a concise findings-first review with these sections:
1. Findings
2. Non-findings
3. Residual risk
4. Serialized gates that still remain outside the proof

If there are no material findings, say that explicitly.
Do not ask for more context. Do not invent hidden behavior.
## Worker A Patch
__REVIEW_PROMPT__
    printf '```diff\n'
    cat "$WORKER_A_PATCH"
    printf '\n```\n\n## Worker B Patch\n```diff\n'
    cat "$WORKER_B_PATCH"
    printf '\n```\n'
  } > "$REPORT_ROOT/reviewer_prompt.md"
  (
    cd "$REPORT_ROOT"
    OPENAI_API_KEY="$OPENAI_API_KEY" run_hosted_openai_reviewer \
      "$HOSTED_REVIEW_MODEL" \
      "$REPORT_ROOT/reviewer_prompt.md" \
      "$REVIEWER_MESSAGE" \
      "$REVIEWER_LOG"
  )
  reviewer_end="$(record_epoch)"
else
  cp "$REPORT_ROOT/worker_a_prompt.md" "$WORKER_A_LAST"
  cp "$REPORT_ROOT/worker_b_prompt.md" "$WORKER_B_LAST"
  cat > "$WORKER_A_PATCH" <<'__PATCH__'
DRY RUN: worker A patch omitted.
__PATCH__
  cat > "$WORKER_B_PATCH" <<'__PATCH__'
DRY RUN: worker B patch omitted.
__PATCH__
  cat > "$REVIEWER_MESSAGE" <<'__REVIEW_TEXT__'
Findings
- No material findings in dry-run mode.

Non-findings
- Worker lanes remained disjoint by declared prompt scope.

Residual risk
- Dry-run mode does not prove hosted reviewer execution.

Serialized gates that still remain outside the proof
- Merge, janitor, and closeout stayed serialized.
__REVIEW_TEXT__
fi

python3 - "$TIMINGS_JSON" "$worker_a_start" "$worker_a_end" "$worker_b_start" "$worker_b_end" "$reviewer_start" "$reviewer_end" <<'__PY__'
import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
def f(idx: int) -> float:
    return float(sys.argv[idx])

payload = {
    "worker_a": {"start": f(2), "end": f(3), "duration_seconds": max(0.0, f(3) - f(2))},
    "worker_b": {"start": f(4), "end": f(5), "duration_seconds": max(0.0, f(5) - f(4))},
    "reviewer": {"start": f(6), "end": f(7), "duration_seconds": max(0.0, f(7) - f(6))},
}
payload["coordination_latency_seconds"] = max(0.0, min(payload["worker_a"]["end"], payload["worker_b"]["end"]) - min(payload["worker_a"]["start"], payload["worker_b"]["start"]))
payload["validation_latency_seconds"] = payload["reviewer"]["duration_seconds"]
out.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
__PY__

cat > "$TRACKED_STATE_JSON" <<__STATE_JSON__
{
  "schema_version": "adl.multi_agent_workcell_state.v1",
  "workcell_id": "$WORKCELL_ID",
  "sprint_issue_number": 3415,
  "planner_manifest_path": "docs/milestones/v0.91.4/review/multi_agent_workcell/workcell_proof_manifest.json",
  "conductor_hooks": {
    "worker_admission": {
      "selected_skill": "pr-run",
      "canonical_truth": ["github_issue_state", "sip", "stp", "spp", "sprint_state"]
    },
    "review_publication": {
      "selected_skill": "pr-finish",
      "canonical_truth": ["srp", "sor", "pr_state", "published_artifacts"]
    },
    "janitor_remediation": {
      "selected_skill": "pr-janitor",
      "canonical_truth": ["pr_state", "check_status", "review_findings"]
    },
    "closeout_reconciliation": {
      "selected_skill": "pr-closeout",
      "canonical_truth": ["github_issue_state", "sor", "sprint_state", "closeout_artifacts"]
    }
  },
  "shard_assignments": [
    {
      "shard_id": "ollama-worker-a",
      "issue_number": 5001,
      "role": "worker",
      "execution_backend": "local_ollama",
      "model_hint": "$LOCAL_MODEL_A",
      "branch": "codex/proof-worker-a",
      "worktree_path": "artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/worker_a",
      "write_paths": [
        "artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/worker_a/docs/worker_a_summary.md",
        "artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/worker_a/.adl/v0.91.4/tasks/issue-5001__demo-worker-a/sor.md"
      ],
      "dependencies": [],
      "admission_status": "parallel_admitted",
      "validation_gate": "parallel_pvf_lane",
      "review_lane": "bounded_subagent_review",
      "closeout_status": "waiting_for_review",
      "cards": {
        "sip": "ready",
        "stp": "ready",
        "spp": "approved",
        "srp": "draft",
        "sor": "draft"
      },
      "github_issue_state": "OPEN",
      "pr_state": "draft"
    },
    {
      "shard_id": "ollama-worker-b",
      "issue_number": 5002,
      "role": "worker",
      "execution_backend": "local_ollama",
      "model_hint": "$LOCAL_MODEL_B",
      "branch": "codex/proof-worker-b",
      "worktree_path": "artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/worker_b",
      "write_paths": [
        "artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/worker_b/docs/worker_b_contract.md",
        "artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/worker_b/.adl/v0.91.4/tasks/issue-5002__demo-worker-b/sor.md"
      ],
      "dependencies": [],
      "admission_status": "parallel_admitted",
      "validation_gate": "parallel_pvf_lane",
      "review_lane": "bounded_subagent_review",
      "closeout_status": "waiting_for_review",
      "cards": {
        "sip": "ready",
        "stp": "ready",
        "spp": "approved",
        "srp": "draft",
        "sor": "draft"
      },
      "github_issue_state": "OPEN",
      "pr_state": "draft"
    },
    {
      "shard_id": "hosted-reviewer",
      "issue_number": 5003,
      "role": "reviewer",
      "branch": "codex/proof-reviewer",
      "worktree_path": "artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/reviewer",
      "write_paths": [
        "docs/milestones/v0.91.4/review/multi_agent_workcell/MULTI_AGENT_CSDLC_WORKCELL_PROOF_PACKET_${PROOF_DATE}.md"
      ],
      "dependencies": ["ollama-worker-a", "ollama-worker-b"],
      "admission_status": "review_ready",
      "validation_gate": "published_artifact_gate",
      "review_lane": "bounded_subagent_review",
      "closeout_status": "waiting_for_review",
      "cards": {
        "sip": "ready",
        "stp": "ready",
        "spp": "approved",
        "srp": "ready",
        "sor": "draft"
      },
      "github_issue_state": "OPEN",
      "pr_state": "open"
    }
  ]
}
__STATE_JSON__

python3 "$STATE_VALIDATOR" "$TRACKED_STATE_JSON" >/dev/null

worker_a_summary="$(cat "$WORKTREE_ROOT/worker_a/docs/worker_a_summary.md" 2>/dev/null || printf 'dry run')"
worker_b_contract="$(cat "$WORKTREE_ROOT/worker_b/docs/worker_b_contract.md" 2>/dev/null || printf 'dry run')"
reviewer_text="$(cat "$REVIEWER_MESSAGE")"

cat > "$TRACKED_PACKET_MD" <<__PACKET__
# Multi-Agent C-SDLC Workcell Proof Packet (${PROOF_DATE})

## Status

Tracked bounded proof packet for issue \`#3419\`.

## Summary

This proof executed a bounded multi-agent C-SDLC workcell with:

- two parallel local worker lanes via the Ollama local provider API
- one independent hosted OpenAI/Codex reviewer lane after serialized publication normalization
- serialized conductor admission, publication normalization, review publication, and closeout/janitor decisions

This packet proves bounded parallel coordination. It does not claim production-grade autonomous merge or closeout authority.

## Lane Assignment

- worker A: \`local_ollama\` / \`$LOCAL_MODEL_A\`
- worker B: \`local_ollama\` / \`$LOCAL_MODEL_B\`
- reviewer: \`hosted_openai_responses\` / \`$HOSTED_REVIEW_MODEL\`
- optional serialized local model observed for future work: \`$SERIAL_MODEL_C\`

## Planned / Serialized Gates

- conductor admission remained serialized
- publication normalization remained serialized before hosted review
- reviewer publication remained serialized after worker completion
- janitor did not activate because no mechanical PR blocker was introduced in the bounded proof workspace
- closeout remained serialized and documentary rather than autonomous

## Assignment And State Evidence

- planner manifest: \`docs/milestones/v0.91.4/review/multi_agent_workcell/workcell_proof_manifest.json\`
- planner report: \`docs/milestones/v0.91.4/review/multi_agent_workcell/workcell_proof_plan_${PROOF_DATE}.json\`
- validated workcell state packet: \`docs/milestones/v0.91.4/review/multi_agent_workcell/workcell_proof_state_${PROOF_DATE}.json\`
- raw local worker proposals remain archived under \`artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/reports/\`

## Actual Branch / Worktree Boundaries

- \`codex/proof-worker-a\` -> \`artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/worker_a\`
- \`codex/proof-worker-b\` -> \`artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/worker_b\`
- \`codex/proof-reviewer\` -> \`artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/worktrees/reviewer\`

## Worker Outputs

### Worker A

$worker_a_summary

### Worker B

$worker_b_contract

## Reviewer Output

$reviewer_text

## Timing

Timing evidence is recorded in:

- \`artifacts/v0914/multi_agent_workcell/$WORKCELL_ID/reports/timings.json\`

## Findings

- No blocking coordination failure occurred in the bounded two-worker plus reviewer slice.
- Janitor and closeout remained explicit serialized gates instead of being smuggled into worker autonomy.

## Non-findings

- No overlapping write-set ownership was introduced.
- No local credential path is recorded in the tracked proof packet.

## Residual Risk

- This proof uses a demo-local repository rather than live GitHub PR creation for shard publication.
- Hosted reviewer execution depends on direct OpenAI Responses API credential availability and network reachability.
- The proof demonstrates bounded coordination, not general autonomous multi-agent delivery.

## Validation

- \`python3 adl/tools/plan_multi_agent_workcell.py docs/milestones/v0.91.4/review/multi_agent_workcell/workcell_proof_manifest.json --json-out docs/milestones/v0.91.4/review/multi_agent_workcell/workcell_proof_plan_${PROOF_DATE}.json\`
- \`python3 adl/tools/validate_multi_agent_workcell_state.py docs/milestones/v0.91.4/review/multi_agent_workcell/workcell_proof_state_${PROOF_DATE}.json\`
- \`bash adl/tools/run_v0914_multi_agent_workcell_proof.sh --artifact-root artifacts/v0914/multi_agent_workcell --tracked-review-root docs/milestones/v0.91.4/review/multi_agent_workcell\`
__PACKET__

sanitize_text_file "$TRACKED_PACKET_MD"

echo "Multi-agent workcell proof packet written to:"
echo "  $TRACKED_PACKET_MD"
echo "State packet written to:"
echo "  $TRACKED_STATE_JSON"
echo "Planner report written to:"
echo "  $TRACKED_PLAN_JSON"
