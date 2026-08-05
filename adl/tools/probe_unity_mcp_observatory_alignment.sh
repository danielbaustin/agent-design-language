#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  probe_unity_mcp_observatory_alignment.sh --project <unity-project-path> [options]

Options:
  --url <mcp-url>       Use a discovered local endpoint explicitly.
  --editor-pid <pid>    Corroborate a known editor PID when Unity-MCP process
                        discovery reports a false negative.
  -h, --help            Show this help.

Environment:
  UNITY_MCP_CLI      Path to the repository Unity-MCP cli.js.
  ADL_PROCESS_BIN    Path to the repository adl binary.

The probe is read-only. It fails closed unless the canonical project, one local
endpoint, permission-safe liveness evidence, and a read-only scene call agree.
USAGE
}

project_path=""
mcp_url=""
editor_pid=""
tool_name="scene-list-opened"
identity_tool_name="reflection-method-call"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --project)
      project_path="${2:-}"
      shift 2
      ;;
    --url)
      mcp_url="${2:-}"
      shift 2
      ;;
    --editor-pid)
      editor_pid="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 64
      ;;
  esac
done

if [[ -z "$project_path" ]]; then
  echo "error: --project is required" >&2
  exit 64
fi
if [[ ! -d "$project_path" ]]; then
  echo "error: project path does not exist" >&2
  exit 66
fi
if [[ -n "$editor_pid" && ! "$editor_pid" =~ ^[0-9]+$ ]]; then
  echo "error: --editor-pid must be numeric" >&2
  exit 64
fi

project_abs="$(cd "$project_path" && pwd -P)"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
unity_mcp_cli="${UNITY_MCP_CLI:-$HOME/git/Unity-MCP/cli/dist/cli.js}"
adl_process_bin="${ADL_PROCESS_BIN:-$repo_root/.adl/bin/adl}"

sanitize() {
  sed -E \
    -e 's#(https?://)[^/@[:space:]]+@#\1<redacted-userinfo>@#g' \
    -e 's#([?&](token|authorization|api[_-]?key|credential|password)=)[^&[:space:]]+#\1<redacted>#Ig' \
    -e 's#(Authorization([[:space:]]+Token)?[[:space:]]*:[[:space:]]*).*$#\1<redacted>#Ig' \
    -e 's#Bearer[[:space:]]+[A-Za-z0-9._~+/=-]+#Bearer <redacted>#Ig' \
    -e 's#([A-Za-z0-9_]*(TOKEN|SECRET|PASSWORD|CREDENTIAL|API_KEY)[A-Za-z0-9_]*=)[^[:space:]]+#\1<redacted>#Ig' \
    -e 's#("(token|accessToken|refreshToken|cloudToken|authorization|authorizationToken|credential|credentials|password|apiKey|clientSecret)"[[:space:]]*:[[:space:]]*)"[^"]*"#\1"<redacted>"#Ig'
}

fail_closed() {
  echo
  echo "## Result"
  echo "FAIL_CLOSED: $1"
  exit 70
}

if [[ ! -f "$unity_mcp_cli" ]]; then
  fail_closed "the repository Unity-MCP CLI is unavailable."
fi
if [[ ! -x "$adl_process_bin" ]]; then
  fail_closed "the permission-safe adl process-status binary is unavailable."
fi

echo "# Unity-MCP Observatory Alignment Probe"
echo
echo "- Canonical project: \`$project_abs\`"
echo "- Endpoint input: \`$([[ -n "$mcp_url" ]] && echo explicit-discovered || echo project-config)\`"
echo "- Read-only tool: \`$tool_name\`"

status_args=(status "$project_abs" --timeout 10000 --verbose)
if [[ -n "$mcp_url" ]]; then
  status_args+=(--url "$mcp_url")
fi

set +e
status_output="$(node "$unity_mcp_cli" "${status_args[@]}" 2>&1)"
status_code=$?
set -e

echo
echo "## Sanitized CLI Status"
printf '%s\n' '```text'
printf '%s\n' "$status_output" | sanitize
printf '%s\n' '```'

if [[ $status_code -ne 0 ]]; then
  fail_closed "Unity-MCP status failed before alignment was established."
fi
status_project="$(printf '%s\n' "$status_output" | sed -nE 's/^[[:space:]]*Project:[[:space:]]+(.*)$/\1/p' | head -1)"
if [[ "$status_project" != "$project_abs" ]]; then
  fail_closed "status did not echo the canonical intended project."
fi
if printf '%s\n' "$status_output" | grep -Eiq 'Config loaded:[^[:cntrl:]]*connectionMode=(Cloud|1)(,|[[:space:]]|$)'; then
  fail_closed "the persisted Unity-MCP project configuration is Cloud, not Custom local mode."
fi
if printf '%s\n' "$status_output" | grep -Eiq 'External MCP|MCP server:[[:space:]]*External'; then
  fail_closed "status resolved to an external MCP server."
fi

resolved_url=""
endpoint_count=0
if [[ -n "$mcp_url" ]]; then
  resolved_url="$mcp_url"
  if ! printf '%s\n' "$status_output" \
    | sed -nE 's/^[[:space:]]*URL:[[:space:]]+(https?:\/\/[^[:space:]]+).*$/\1/p' \
    | grep -Fxq "$resolved_url"
  then
    fail_closed "the explicit discovered endpoint was not reported by project status."
  fi
else
  while IFS= read -r candidate; do
    [[ -n "$candidate" ]] || continue
    if [[ -z "$resolved_url" ]]; then
      resolved_url="$candidate"
      endpoint_count=1
    elif [[ "$candidate" != "$resolved_url" ]]; then
      endpoint_count=$((endpoint_count + 1))
    fi
  done < <(printf '%s\n' "$status_output" | sed -nE 's/^[[:space:]]*URL:[[:space:]]+(https?:\/\/[^[:space:]]+).*$/\1/p')

  if [[ $endpoint_count -ne 1 || -z "$resolved_url" ]]; then
    fail_closed "status did not resolve exactly one MCP endpoint without an explicit discovery result."
  fi
fi
if ! [[ "$resolved_url" =~ ^https?://(localhost|127\.0\.0\.1|\[::1\]):[0-9]+(/[^[:space:]]*)?$ ]]; then
  fail_closed "the resolved endpoint is not a loopback-local MCP endpoint."
fi
if ! printf '%s\n' "$status_output" | grep -Eq 'SUCCESS: (Connected|MCP server is reachable)'; then
  fail_closed "the local MCP endpoint did not pass the CLI reachability probe."
fi

editor_aligned=false
if printf '%s\n' "$status_output" | grep -Eiq 'SUCCESS:.*Unity.*running|Unity is running'; then
  editor_aligned=true
elif [[ -n "$editor_pid" ]]; then
  set +e
  pid_status="$("$adl_process_bin" process status --pid "$editor_pid" --json 2>/dev/null)"
  pid_status_code=$?
  set -e
  editor_log="$project_abs/Logs/Editor.log"
  if [[ $pid_status_code -eq 0 ]] \
    && printf '%s\n' "$pid_status" | grep -Fq '"status": "live_pid"' \
    && [[ -f "$editor_log" ]] \
    && grep -Fq "\"processId\":$editor_pid" "$editor_log" \
    && { grep -Fq "Content root path: $project_abs" "$editor_log" \
      || grep -Fq "WorkingDir: $project_abs" "$editor_log"; }
  then
    editor_aligned=true
    echo
    echo "## Editor Corroboration"
    echo "- Unity-MCP process discovery reported a false negative."
    echo "- Known PID \`$editor_pid\` is live and the project-local editor log binds it to the canonical project."
  fi
fi
if [[ "$editor_aligned" != true ]]; then
  fail_closed "the intended Unity editor was not proven live for the canonical project."
fi

identity_input='{"filter":{"Namespace":"UnityEngine","TypeName":"Application","MethodName":"get_dataPath","InputParameters":[]},"knownNamespace":true,"typeNameMatchLevel":6,"methodNameMatchLevel":6,"parametersMatchLevel":2,"executeInMainThread":true}'
set +e
identity_output="$(node "$unity_mcp_cli" run-tool "$identity_tool_name" \
  --path "$project_abs" --url "$resolved_url" --input "$identity_input" --raw 2>&1)"
identity_code=$?
set -e

echo
echo "## MCP Project Identity"
printf '%s\n' '```json'
printf '%s\n' "$identity_output" | sanitize
printf '%s\n' '```'
if [[ $identity_code -ne 0 ]]; then
  fail_closed "the MCP endpoint could not report Unity Application.dataPath."
fi
expected_data_path="$project_abs/Assets"
if ! printf '%s\n' "$identity_output" | grep -Fq "\"value\":\"$expected_data_path\""; then
  fail_closed "the MCP endpoint is not connected to the canonical intended project."
fi

endpoint_authority="${resolved_url#*://}"
endpoint_authority="${endpoint_authority%%/*}"
resolved_port="${endpoint_authority##*:}"
set +e
port_status="$("$adl_process_bin" process status --port "$resolved_port" --json 2>/dev/null)"
port_status_code=$?
set -e

echo
echo "## Permission-Safe Endpoint Liveness"
printf '%s\n' '```json'
printf '%s\n' "$port_status" | sanitize
printf '%s\n' '```'
if [[ $port_status_code -ne 0 ]] || ! printf '%s\n' "$port_status" | grep -Fq '"status": "bound_port"'; then
  fail_closed "permission-safe process status did not prove the loopback port is bound."
fi

set +e
tool_output="$(node "$unity_mcp_cli" run-tool "$tool_name" \
  --path "$project_abs" --url "$resolved_url" --raw 2>&1)"
tool_code=$?
set -e

echo
echo "## Read-Only Scene Proof"
printf '%s\n' '```json'
printf '%s\n' "$tool_output" | sanitize
printf '%s\n' '```'
if [[ $tool_code -ne 0 ]]; then
  fail_closed "the read-only MCP scene call failed."
fi
if ! printf '%s\n' "$tool_output" | grep -Eq '"status"[[:space:]]*:[[:space:]]*"success"'; then
  fail_closed "the read-only MCP response was malformed or unsuccessful."
fi

echo
echo "## Result"
echo "PASS: canonical project, persisted local mode, endpoint, editor liveness, MCP project identity, and read-only scene proof agree."
