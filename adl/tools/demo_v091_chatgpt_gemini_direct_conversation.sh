#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v091/chatgpt_gemini_direct_conversation}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-91-chatgpt-gemini-direct-conversation"
PORT="${ADL_CHATGPT_GEMINI_DIRECT_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_adapter.log"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
TRANSCRIPT="$OUT_DIR/transcript.md"
TRANSCRIPT_CONTRACT="$OUT_DIR/transcript_contract.json"
OBSERVATORY_PROJECTION="$OUT_DIR/observatory_projection.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
PROOF_NOTE="$OUT_DIR/proof_note.md"
README_OUT="$OUT_DIR/README.md"
GENERATED_EXAMPLE="$OUT_DIR/v0-91-chatgpt-gemini-direct-conversation.runtime.adl.yaml"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai2.key}"
GEMINI_KEY_FILE="${ADL_GEMINI_KEY_FILE:-$HOME/keys/gcp-ace-2023.key}"
OPENAI_MODEL="${ADL_LIVE_OPENAI_MODEL:-gpt-5.5-pro}"
GEMINI_MODEL="${ADL_LIVE_GEMINI_MODEL:-gemini-3.1-pro-preview}"
LIVE_PROVIDER_TIMEOUT_SECS="${ADL_LIVE_PROVIDER_TIMEOUT_SECS:-180}"
PRESET="${ADL_DEMO_PRESET:-trust_possible}"
QUESTION="${ADL_DEMO_QUESTION:-If two minds never share the same private world, what makes trust possible?}"
TURN_COUNT="${ADL_DEMO_TURNS:-6}"

load_key() {
  local env_name="$1"
  local key_file="$2"
  if [[ -n "${!env_name:-}" ]]; then
    return 0
  fi
  if [[ ! -s "$key_file" ]]; then
    echo "missing required key file for $env_name: $key_file" >&2
    return 1
  fi
  local key_value
  key_value="$(python3 - "$env_name" "$key_file" <<'PY'
import sys
env_name, path = sys.argv[1:3]
raw = open(path, encoding="utf-8").read().strip()
value = raw
for line in raw.splitlines():
    stripped = line.strip()
    if not stripped or stripped.startswith("#"):
        continue
    if stripped.startswith(env_name + "="):
        value = stripped.split("=", 1)[1].strip().strip("'\"")
        break
    value = stripped.strip("'\"")
    break
print(value, end="")
PY
)"
  if [[ -z "$key_value" ]]; then
    echo "empty required key file for $env_name: $key_file" >&2
    return 1
  fi
  export "$env_name=$key_value"
}

load_key OPENAI_API_KEY "$OPENAI_KEY_FILE"
load_key GEMINI_API_KEY "$GEMINI_KEY_FILE"

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"

python3 "$ROOT_DIR/adl/tools/real_chatgpt_gemini_provider_adapter.py" \
  --port "$PORT" \
  --port-file "$PORT_FILE" \
  --metadata "$INVOCATIONS" \
  --openai-model "$OPENAI_MODEL" \
  --gemini-model "$GEMINI_MODEL" \
  --timeout "$LIVE_PROVIDER_TIMEOUT_SECS" \
  >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!
cleanup() {
  if kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

PORT="$(provider_demo_wait_for_port "$PORT_FILE")"

python3 - "$GENERATED_EXAMPLE" "$PORT" "$QUESTION" "$TURN_COUNT" "$PRESET" <<'PY'
import sys
from pathlib import Path

target, port, question, turn_count_raw, preset = sys.argv[1:6]
turn_count = int(turn_count_raw)
if turn_count < 2 or turn_count % 2 != 0:
    raise SystemExit("ADL_DEMO_TURNS must be an even integer >= 2")

PRESETS = {
    "trust_possible": {
        "question": "If two minds never share the same private world, what makes trust possible?",
        "private_chatgpt": [
            "You privately think trust begins with dependable action before full understanding.",
            "You do not know Gemini's hidden reason for distrusting that framing.",
            "Do not reveal the raw private-state bullets verbatim.",
        ],
        "private_gemini": [
            "You privately think trust without interpretable risk is sentimental and brittle.",
            "You do not know ChatGPT's hidden reason for privileging action first.",
            "Do not reveal the raw private-state bullets verbatim.",
        ],
        "condition_line": "The discussion stays concrete, philosophical, and bounded.",
    },
    "coordination_proof": {
        "question": "What is the smallest honest proof of multi-agent coordination?",
        "private_chatgpt": [
            "You privately know the plan must satisfy a hard deadline and budget ceiling.",
            "You do not know the hidden dependency chain or latent risk register.",
            "Do not reveal the raw private-state bullets verbatim.",
        ],
        "private_gemini": [
            "You privately know the dependency chain and a latent risk condition.",
            "You do not know the hard deadline or budget ceiling.",
            "Do not reveal the raw private-state bullets verbatim.",
        ],
        "condition_line": "The proof boundary stays bounded to one task-local coordination episode.",
    },
}

if preset not in PRESETS:
    raise SystemExit(f"unknown ADL_DEMO_PRESET '{preset}' (expected one of: {', '.join(sorted(PRESETS))})")

defaults = PRESETS[preset]
if question == PRESETS["trust_possible"]["question"] and preset == "coordination_proof":
    question = defaults["question"]

def prompt_for_turn(turn: int, total: int) -> str:
    if preset == "coordination_proof":
        if turn == 1:
            return (
                "Open warmly but with substance. State the stop rule explicitly. "
                "Offer one vivid candidate answer to the topic in under 150 words. "
                "Ground the claim in saved traces, constraints, and visible revisions rather than vague autonomy language. "
                "End with exactly one PROVISIONAL_CLAIM line of no more than 22 words that Gemini can challenge."
            )
        if turn == 2:
            return (
                "Reply directly to ChatGPT and do not just agree. Keep the whole turn under 120 words. "
                "Name the weak point, then introduce one hidden conflict or latent risk only you can see, and end by pressing for a stronger proof condition."
            )
        if turn == total:
            return (
                f"Close the {total}-turn exchange explicitly. Keep the whole turn under 110 words. "
                "Name one reason this run is stronger than decorative baton-passing and end with one memorable line."
            )
        if turn % 2 == 1:
            return (
                "Revise the answer under pressure in under 120 words. Explicitly say what changed because of Gemini-only information. "
                "Use either three numbered criteria or one tight synthesis paragraph, whichever is sharper."
            )
        return (
            "Deepen the challenge in under 110 words. Accept one piece, reject one piece, and introduce one paradox, dependency, or ablation condition that raises the bar."
        )
    if turn == 1:
        return (
            "Open warmly but with substance. State the stop rule explicitly. "
            "Offer one vivid candidate answer to the topic in under 150 words. "
            "Be elegant, memorable, and humanly interesting rather than technical. "
            "End with exactly one PROVISIONAL_CLAIM line of no more than 20 words that Gemini can challenge."
        )
    if turn == 2:
        return (
            "Reply directly to ChatGPT and do not just agree. Keep the whole turn under 110 words. "
            "Use exactly three sentences and no bullets. Sentence 1: identify the precise weakness in ChatGPT's claim. "
            "Sentence 2: introduce one deeper condition or risk that trust must survive. "
            "Sentence 3: leave ChatGPT with a sharper question, not a conclusion."
        )
    if turn == total:
        return (
            f"Close the {total}-turn exchange explicitly. Keep the whole turn under 110 words. "
            "Land on one clear insight rather than summary mush. End with one memorable final line that sounds wise rather than theatrical."
        )
    if turn % 2 == 1:
        return (
            "Deepen or revise the position in under 120 words. Explicitly say what changed in your view because of the prior turn. "
            "Use concrete language, not vague abstraction. Add one quotable line if it comes naturally."
        )
    return (
        "Press on the weak point in under 110 words. Accept part of the prior turn, reject part of it, and introduce one paradox, stake, or tension that forces a better answer."
    )

def title_for_turn(turn: int, total: int) -> str:
    if turn == 1:
        return "Opening claim"
    if turn == 2:
        return "Challenge and complication"
    if turn == total:
        return "Closure and final line"
    return "Revision under pressure" if turn % 2 == 1 else "Paradox and pressure"

def speaker_for_turn(turn: int) -> str:
    return "ChatGPT" if turn % 2 == 1 else "Gemini"

def agent_for_turn(turn: int) -> str:
    return "chatgpt_host" if turn % 2 == 1 else "gemini_guest"

def task_id(turn: int) -> str:
    stem = {
        1: "opening",
        2: "reply",
        3: "reflection",
    }.get(turn, f"turn_{turn:02d}")
    prefix = "chatgpt" if turn % 2 == 1 else "gemini"
    return f"{prefix}_{stem}"

lines = [
    'version: "0.5"',
    "",
    "providers:",
    '  chatgpt_local:',
    '    type: "http"',
    "    config:",
    f'      endpoint: "http://127.0.0.1:{port}/openai"',
    "      timeout_secs: 180",
    '  gemini_local:',
    '    type: "http"',
    "    config:",
    f'      endpoint: "http://127.0.0.1:{port}/gemini"',
    "      timeout_secs: 180",
    "",
    "agents:",
    '  chatgpt_host:',
    '    provider: "chatgpt_local"',
    '    model: "chatgpt-live-demo"',
    '  gemini_guest:',
    '    provider: "gemini_local"',
    '    model: "gemini-live-demo"',
    "",
    "tasks:",
]

for turn in range(1, turn_count + 1):
    task = task_id(turn)
    speaker = speaker_for_turn(turn)
    private_state = defaults["private_chatgpt"] if speaker == "ChatGPT" else defaults["private_gemini"]
    lines.extend(
        [
            f"  {task}:",
            "    prompt:",
            "      user: |",
            "        DEMO_ID: v0-91-chatgpt-gemini-direct-conversation",
            f"        TURN_ID: {turn:02d}",
            f"        SPEAKER: {speaker}",
            f"        TOPIC: {question}",
            f"        STOP_RULE: Stop after {turn_count} explicit turns.",
            "        PRIVATE_STATE:",
        ]
    )
    for item in private_state:
        lines.append(f"        - {item}")
    if turn > 1:
        prev = f"turn_{turn - 1:02d}"
        lines.extend(
            [
                "        PREVIOUS_TURN_START",
                f"        {{{{{prev}}}}}",
                "        PREVIOUS_TURN_END",
            ]
        )
    lines.append(f"        INSTRUCTIONS: {prompt_for_turn(turn, turn_count)}")

lines.extend(
    [
        "",
        "run:",
        '  name: "v0-91-chatgpt-gemini-direct-conversation"',
        "  workflow:",
        "    kind: sequential",
        "    steps:",
    ]
)

for turn in range(1, turn_count + 1):
    task = task_id(turn)
    speaker = speaker_for_turn(turn)
    agent = agent_for_turn(turn)
    step_id = f"direct.{task.replace('_', '.')}"
    save_as = f"turn_{turn:02d}"
    lines.extend(
        [
            f'      - id: "{step_id}"',
            f'        agent: "{agent}"',
            f'        task: "{task}"',
            "        conversation:",
            f'          id: "{save_as}"',
            f'          speaker: "{speaker}"',
            f"          sequence: {turn}",
            '          thread_id: "chatgpt_gemini_direct"',
        ]
    )
    if turn > 1:
        lines.append(f'          responds_to: "turn_{turn - 1:02d}"')
        lines.extend(
            [
                "        inputs:",
                f'          turn_{turn - 1:02d}: "@state:turn_{turn - 1:02d}"',
            ]
        )
    lines.extend(
        [
            f'        save_as: "{save_as}"',
            f'        write_to: "direct/{turn:02d}-{speaker.lower()}-{title_for_turn(turn, turn_count).lower().replace(" ", "-")}.md"',
        ]
    )

Path(target).write_text("\n".join(lines) + "\n", encoding="utf-8")
PY

python3 - "$PORT" <<'PY'
import json
import sys
import time
import urllib.request

port = int(sys.argv[1])
url = f"http://127.0.0.1:{port}/health"
deadline = time.time() + 10.0
last_error = None
while time.time() < deadline:
    try:
        with urllib.request.urlopen(url, timeout=1.0) as resp:
            payload = json.load(resp)
        if payload.get("ok") is True:
            raise SystemExit(0)
    except Exception as exc:  # noqa: BLE001
        last_error = exc
        time.sleep(0.1)
raise SystemExit(f"provider shim failed health check: {last_error}")
PY

cd "$ROOT_DIR"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
  "$GENERATED_EXAMPLE" \
  --run \
  --trace \
  --allow-unsigned \
  --out "$STEP_OUT" \
  >"$OUT_DIR/run_log.txt" 2>&1

python3 - "$TRANSCRIPT" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" "$STEP_OUT" "$OPENAI_MODEL" "$GEMINI_MODEL" "$QUESTION" "$TURN_COUNT" <<'PY'
import json
import sys
from pathlib import Path

transcript_path, trace_path, step_out, openai_model, gemini_model, question, turn_count = sys.argv[1:8]
trace = json.loads(Path(trace_path).read_text(encoding="utf-8"))
events = trace.get("events", [])

turn_count = int(Path(sys.argv[1]).stem and 0)  # unused sentinel for lintless editing
generated_example = Path(step_out).parent / "v0-91-chatgpt-gemini-direct-conversation.runtime.adl.yaml"
example_text = generated_example.read_text(encoding="utf-8")
turn_specs = []
for line in example_text.splitlines():
    stripped = line.strip()
    if stripped.startswith('write_to: "direct/'):
        filename = stripped.split('write_to: "direct/', 1)[1].rstrip('"')
        ordinal = int(filename.split("-", 1)[0])
        speaker = "ChatGPT" if ordinal % 2 == 1 else "Gemini"
        label = filename.split("-", 2)[2].rsplit(".", 1)[0].replace("-", " ").title()
        turn_specs.append((filename, speaker, label))

step_end_timestamps = []
for event in events:
    if event.get("event_type") == "STEP_END":
        step_end_timestamps.append(event.get("timestamp"))

conditions = [
    f"Stop after {turn_count} explicit turns total.",
    "Named agents only: ChatGPT and Gemini.",
    "Each side holds partial private state and must react to the other.",
    defaults["condition_line"],
]

lines = [
    "# ChatGPT + Gemini Direct Conversation",
    "",
    f"> A bounded, provider-backed {turn_count}-turn exchange rendered from runtime artifacts.",
    "",
    "## Prompt",
    "",
    f"**Question:** {question}",
    "",
    "## Run Conditions",
    "",
]
for item in conditions:
    lines.append(f"- {item}")

lines.extend(
    [
        "",
        "## Providers",
        "",
        f"- `ChatGPT`: `{openai_model}`",
        f"- `Gemini`: `{gemini_model}`",
        "",
        "## Transcript",
    ]
)

for index, (filename, speaker, label) in enumerate(turn_specs, start=1):
    body = (Path(step_out) / "direct" / filename).read_text(encoding="utf-8").strip()
    timestamp = step_end_timestamps[index - 1] if index - 1 < len(step_end_timestamps) else "unknown"
    lines.extend(
        [
            "",
            f"### Turn {index} · {speaker}",
            "",
            f"- Label: {label}",
            f"- Timestamp: `{timestamp}`",
            "",
            body,
        ]
    )

Path(transcript_path).write_text("\n".join(lines) + "\n", encoding="utf-8")
PY

python3 - "$TRANSCRIPT_CONTRACT" "$GENERATED_EXAMPLE" <<'PY'
import json
import sys
from pathlib import Path

contract_path, generated_example = sys.argv[1:3]
example_text = Path(generated_example).read_text(encoding="utf-8")
turn_specs = []
for line in example_text.splitlines():
    stripped = line.strip()
    if stripped.startswith('write_to: "direct/'):
        filename = stripped.split('write_to: "direct/', 1)[1].rstrip('"')
        ordinal = int(filename.split("-", 1)[0])
        speaker = "ChatGPT" if ordinal % 2 == 1 else "Gemini"
        turn_specs.append((filename, speaker))

payload = {
    "schema_version": "multi_agent_discussion_transcript.v1",
    "transcript_path": "transcript.md",
    "turn_count": len(turn_specs),
    "stop_rule": f"Stop after {len(turn_specs)} explicit turns.",
    "turns": [
        {
            "turn_id": f"turn_{ordinal:02d}",
            "ordinal": ordinal,
            "speaker": speaker,
            "heading": f"# Turn {ordinal} - {speaker}",
            "source_output": f"out/direct/{filename}",
        }
        for ordinal, (filename, speaker) in enumerate(turn_specs, start=1)
    ],
    "companion_artifacts": {
        "demo_manifest": "demo_manifest.json",
        "observatory_projection": "observatory_projection.json",
        "proof_note": "proof_note.md",
        "run_summary": "runtime/runs/v0-91-chatgpt-gemini-direct-conversation/run_summary.json",
        "trace": "runtime/runs/v0-91-chatgpt-gemini-direct-conversation/logs/trace_v1.json",
    },
}
with open(contract_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

python3 - "$OBSERVATORY_PROJECTION" "$TRANSCRIPT" "$INVOCATIONS" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" "$GENERATED_EXAMPLE" <<'PY'
import json
import sys
from pathlib import Path

projection_path, transcript_path, invocations_path, trace_path, generated_example = sys.argv[1:6]
invocations = json.loads(Path(invocations_path).read_text(encoding="utf-8"))
trace = json.loads(Path(trace_path).read_text(encoding="utf-8"))
events = trace.get("events", [])
example_text = Path(generated_example).read_text(encoding="utf-8")
turn_specs = []
for line in example_text.splitlines():
    stripped = line.strip()
    if stripped.startswith('write_to: "direct/'):
        filename = stripped.split('write_to: "direct/', 1)[1].rstrip('"')
        ordinal = int(filename.split("-", 1)[0])
        speaker = "ChatGPT" if ordinal % 2 == 1 else "Gemini"
        turn_specs.append((ordinal, filename, speaker))

turns = [
    {"turn": ordinal, "speaker": speaker, "artifact_ref": f"out/direct/{filename}"}
    for ordinal, filename, speaker in turn_specs
]
payload = {
    "schema": "adl.demo.observatory_projection.v1",
    "demo_id": "v0.91.chatgpt_gemini_direct_conversation",
    "view_kind": "bounded_agent_runtime_projection",
    "transcript_ref": transcript_path,
    "providers": invocations.get("providers", []),
    "turns": turns,
    "timeline": [
        {
            "event_type": event.get("event_type"),
            "timestamp": event.get("timestamp"),
            "actor": event.get("actor", {}).get("id"),
            "scope": event.get("scope", {}).get("name"),
            "artifact_ref": event.get("artifact_ref"),
        }
        for event in events
        if event.get("event_type") in {"RUN_START", "STEP_START", "STEP_END"}
    ],
    "proof_boundary": [
        f"Shows named provider-backed runtime roles executing a bounded {len(turns)}-turn exchange.",
        "Shows cross-turn revision pressure and saved trace continuity.",
        "Does not by itself prove autonomous multi-agent federation or persistent independent agency.",
    ],
}
Path(projection_path).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
PY

python3 - "$MANIFEST" "$TRANSCRIPT" "$TRANSCRIPT_CONTRACT" "$OBSERVATORY_PROJECTION" "$PROOF_NOTE" "$INVOCATIONS" "$RUNS_ROOT/$RUN_ID/run_summary.json" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" "$TURN_COUNT" <<'PY'
import json
import sys

manifest_path, transcript, transcript_contract, observatory_projection, proof_note, invocations, run_summary, trace_path, turn_count = sys.argv[1:10]
payload = {
    "demo_id": "v0.91.chatgpt_gemini_direct_conversation",
    "title": "ChatGPT + Gemini direct conversation demo",
    "execution_mode": "runtime_http_live_provider_adapter",
    "provider_mode": "live_openai_and_gemini",
    "credential_policy": "operator_env_or_home_keys_no_secret_material_recorded",
    "agents": [
        {"id": "chatgpt_host", "provider": "live_openai", "family": "openai"},
        {"id": "gemini_guest", "provider": "live_gemini", "family": "gemini"},
    ],
    "steps": int(turn_count),
    "stop_rule": f"Stop after {turn_count} explicit turns.",
    "proof_surfaces": {
        "transcript": transcript,
        "transcript_contract": transcript_contract,
        "observatory_projection": observatory_projection,
        "proof_note": proof_note,
        "provider_invocations": invocations,
        "run_summary": run_summary,
        "trace": trace_path,
    },
}
with open(manifest_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

cat >"$PROOF_NOTE" <<'EOF'
# Proof Note - ChatGPT + Gemini Direct Conversation

## Facts

- The runtime executed __TURN_COUNT__ explicit sequential turns.
- Every turn preserved named participant identity (`ChatGPT` or `Gemini`).
- The stop rule was explicit and matched the configured turn limit.
- The transcript, observatory-style projection, run summary, and trace were saved automatically.

## Assumptions

- The saved transcript, runtime traces, and provider-invocation log are the
  authoritative proof surfaces.
- Operator-managed local credentials are available for both providers.

## Recommendations

- Use this proof as the pairwise baseline before attempting task handoff or triad demos.
- Do not overclaim federation, autonomy, or production-hardening from this artifact alone.
EOF

python3 - "$PROOF_NOTE" "$TURN_COUNT" <<'PY'
from pathlib import Path
import sys

path, turn_count = sys.argv[1:3]
text = Path(path).read_text(encoding="utf-8")
text = text.replace("__TURN_COUNT__", turn_count)
Path(path).write_text(text, encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.91 Demo - ChatGPT + Gemini Direct Conversation

Canonical command:

\`\`\`bash
bash adl/tools/demo_v091_chatgpt_gemini_direct_conversation.sh
\`\`\`

Credential loading:
- Uses \`OPENAI_API_KEY\` and \`GEMINI_API_KEY\` when already set.
- Otherwise reads local operator-managed keys from \`\$HOME/keys/openai2.key\`
  and \`\$HOME/keys/gcp-ace-2023.key\`.

Model overrides:
- Set \`ADL_LIVE_OPENAI_MODEL\` to compare OpenAI variants.
- Set \`ADL_LIVE_GEMINI_MODEL\` to compare Gemini variants.
- Set \`ADL_DEMO_PRESET\` to switch between built-in prompt families such as
  \`trust_possible\` and \`coordination_proof\`.
- Set \`ADL_DEMO_QUESTION\` and \`ADL_DEMO_TURNS\` to override the preset.
- Default quality mode is tuned for \`gpt-5.5-pro\` plus
  \`gemini-3.1-pro-preview\`.
- Set \`ADL_LIVE_PROVIDER_TIMEOUT_SECS\` to override the provider-adapter read
  timeout when experimenting with slower flagship models.
- Secret values and raw Authorization headers are not written to generated artifacts.

What this proves:
- one ADL runtime workflow with two explicit named live provider families
- real OpenAI and Gemini calls through ADL's current local HTTP completion adapter boundary
- the configured turn sequence with saved-state handoff between steps
- an explicit stop rule recorded in the proof surfaces

Primary proof surfaces:
- \`$TRANSCRIPT\`
- \`$PROOF_NOTE\`
- \`$RUNS_ROOT/$RUN_ID/run_summary.json\`

Secondary proof surfaces:
- \`$RUNS_ROOT/$RUN_ID/logs/trace_v1.json\`
- \`$TRANSCRIPT_CONTRACT\`
- \`$MANIFEST\`
- \`$OUT_DIR/run_log.txt\`
- \`$SERVER_LOG\`

Scope note:
- This is a bounded live-provider demo, not a claim of general federation.
- The proof is about explicit identity, ordered turns, saved artifacts, and bounded stop policy.
EOF

echo "ChatGPT + Gemini direct conversation proof surface:"
echo "  $TRANSCRIPT"
echo "  $PROOF_NOTE"
echo "  $INVOCATIONS"
echo "  $RUNS_ROOT/$RUN_ID/run_summary.json"
echo "  $RUNS_ROOT/$RUN_ID/logs/trace_v1.json"
