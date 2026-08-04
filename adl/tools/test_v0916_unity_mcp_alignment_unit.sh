#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROBE="$ROOT/adl/tools/probe_unity_mcp_observatory_alignment.sh"
TMP_ROOT="$ROOT/.adl/tmp"
mkdir -p "$TMP_ROOT"
TMP="$(mktemp -d "$TMP_ROOT/unity-mcp-alignment.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

PROJECT="$TMP/unity-observatory"
FAKE_CLI="$TMP/fake-cli.js"
FAKE_ADL="$TMP/adl"
mkdir -p "$PROJECT/Logs"

cat >"$FAKE_CLI" <<'JS'
const command = process.argv[2];
const project = process.argv[3];
const mode = process.env.FIXTURE_MODE || "matching";
const callLog = process.env.CALL_LOG;
if (callLog) require("fs").appendFileSync(callLog, `${command} ${process.argv.slice(3).join(" ")}\n`);

if (command === "status") {
  const shownProject = mode === "project-mismatch" ? `${project}-other` : project;
  const configMode = mode === "cloud" || mode === "cloud-explicit"
    ? "Cloud"
    : mode === "legacy-cloud-explicit" ? "1" : "Custom";
  console.log(`[verbose] Config loaded: connectionMode=${configMode}, configUrl=http://localhost:23011, hasToken=false`);
  console.log("Unity-MCP Status");
  console.log(`  Project: ${shownProject}`);
  console.log("Unity Editor Process");
  if (mode === "missing-editor" || mode === "missing-editor-recovered") {
    console.log("WARN: Unity is not running with this project");
  } else {
    console.log("SUCCESS: Unity is running (PID: 4242)");
  }
  console.log("Local MCP Server");
  if (mode === "malformed") {
    console.log("URL unavailable");
  } else if (mode === "cloud") {
    console.log("  URL: https://ai-game.dev/mcp");
  } else if (mode === "random-port" || mode === "ambiguous") {
    console.log("  URL: http://localhost:24645");
    console.log("Config Server");
    console.log("  URL: http://localhost:23011");
  } else {
    console.log("  URL: http://localhost:23011");
  }
  if (mode === "external") console.log("MCP server: External");
  if (mode === "redaction") {
    console.log("diagnostic=https://operator:password@localhost:23011/?token=top-secret");
    console.log("Authorization: Bearer bearer-secret");
    console.log("Authorization Token: auth-token-secret");
    console.log("UNITY_MCP_AUTH_TOKEN=env-token-secret");
    console.log("CLIENT_SECRET=env-client-secret");
    console.log('{"credential":"credential-secret","apiKey":"api-secret","accessToken":"access-secret","cloudToken":"cloud-secret","clientSecret":"client-secret"}');
  }
  console.log("SUCCESS: Connected");
  console.log("SUCCESS: MCP server is reachable - ready for tool calls");
  process.exit(0);
}

if (command === "run-tool") {
  const tool = process.argv[3];
  const pathIndex = process.argv.indexOf("--path");
  const requestedProject = pathIndex >= 0 ? process.argv[pathIndex + 1] : "";
  if (tool === "reflection-method-call") {
    const identityProject = mode === "identity-mismatch" ? `${requestedProject}-other` : requestedProject;
    console.log(JSON.stringify({
      status: "success",
      structured: { result: { typeName: "System.String", value: `${identityProject}/Assets` } }
    }));
    process.exit(0);
  }
  if (mode === "tool-failure") {
    console.error("read-only failure");
    process.exit(1);
  }
  if (mode === "redaction") {
    console.log('{"status":"success","token":"tool-secret","structured":{"result":[]}}');
  } else {
    console.log('{"status":"success","structured":{"result":[{"Name":"FlagshipObservatoryStage","IsLoaded":true}]}}');
  }
  process.exit(0);
}

process.exit(64);
JS

cat >"$FAKE_ADL" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == *"--pid"* ]]; then
  printf '%s\n' '{"status": "live_pid", "uses_ps": false, "broad_process_scan": false}'
else
  printf '%s\n' '{"status": "bound_port", "uses_ps": false, "broad_process_scan": false}'
fi
SH
chmod +x "$FAKE_ADL"

cat >"$PROJECT/Logs/Editor.log" <<EOF
Content root path: $PROJECT
##utp:{"type":"ProjectInfo","processId":4242}
EOF

run_case() {
  local name="$1"
  local expected="$2"
  local mode="$3"
  shift 3
  local output="$TMP/$name.out"
  local calls="$TMP/$name.calls"
  local code=0
  FIXTURE_MODE="$mode" CALL_LOG="$calls" UNITY_MCP_CLI="$FAKE_CLI" ADL_PROCESS_BIN="$FAKE_ADL" \
    bash "$PROBE" --project "$PROJECT" "$@" >"$output" 2>&1 || code=$?
  if [[ "$expected" == pass ]]; then
    [[ $code -eq 0 ]] || {
      cat "$output" >&2
      echo "expected $name to pass" >&2
      exit 1
    }
    grep -Fq "PASS: canonical project" "$output"
  else
    [[ $code -ne 0 ]] || {
      cat "$output" >&2
      echo "expected $name to fail closed" >&2
      exit 1
    }
    grep -Fq "FAIL_CLOSED:" "$output"
  fi
}

assert_no_tool_call() {
  local name="$1"
  if grep -q '^run-tool ' "$TMP/$name.calls"; then
    echo "$name invoked an MCP tool after a failed precondition" >&2
    exit 1
  fi
}

assert_no_scene_call() {
  local name="$1"
  if grep -q '^run-tool scene-list-opened ' "$TMP/$name.calls"; then
    echo "$name invoked scene proof after project identity failed" >&2
    exit 1
  fi
  grep -q '^run-tool reflection-method-call ' "$TMP/$name.calls"
}

run_case matching pass matching --url http://localhost:23011
run_case project-mismatch fail project-mismatch --url http://localhost:23011
assert_no_tool_call project-mismatch
run_case missing-editor fail missing-editor --url http://localhost:23011
assert_no_tool_call missing-editor
run_case missing-editor-recovered pass missing-editor-recovered \
  --url http://localhost:23011 --editor-pid 4242
run_case cloud-fallback fail cloud
assert_no_tool_call cloud-fallback
run_case cloud-explicit fail cloud-explicit --url http://localhost:23011
assert_no_tool_call cloud-explicit
run_case legacy-cloud-explicit fail legacy-cloud-explicit --url http://localhost:23011
assert_no_tool_call legacy-cloud-explicit
run_case external-fallback fail external --url http://localhost:23011
assert_no_tool_call external-fallback
run_case malformed-status fail malformed
assert_no_tool_call malformed-status
run_case random-port pass random-port --url http://localhost:23011
run_case endpoint-ambiguity fail ambiguous
assert_no_tool_call endpoint-ambiguity
run_case identity-mismatch fail identity-mismatch --url http://localhost:23011
assert_no_scene_call identity-mismatch
run_case tool-failure fail tool-failure --url http://localhost:23011
run_case redaction pass redaction --url http://localhost:23011

for secret in password top-secret bearer-secret auth-token-secret env-token-secret \
  env-client-secret credential-secret api-secret access-secret cloud-secret client-secret tool-secret
do
  if grep -Fq "$secret" "$TMP/redaction.out"; then
    echo "redaction fixture leaked: $secret" >&2
    exit 1
  fi
done
grep -Fq "<redacted" "$TMP/redaction.out"
grep -Fq '"uses_ps": false' "$TMP/matching.out"
grep -Fq '"broad_process_scan": false' "$TMP/matching.out"

echo "Unity-MCP alignment classifier fixtures passed"
