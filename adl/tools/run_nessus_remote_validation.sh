#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

usage() {
  cat <<'EOF'
Usage:
  adl/tools/run_nessus_remote_validation.sh --command <shell-command> [options]

Options:
  --command <shell-command>        Command to run inside the remote ADL checkout. Required.
  --executor <ssh|local>           Execution transport. Defaults to ssh. local is for bounded contract tests.
  --host <host>                    Remote host. Defaults to nessus.local.
  --ssh-user <user>                SSH login user. Defaults to danie.
  --wsl-user <user>                WSL Linux user. Defaults to root.
  --remote-root <path>             Remote workspace root. Defaults to /root/adl-remote-runner.
  --repo-url <url>                 Git remote used to materialize/refresh the checkout.
  --git-ref <ref>                  Git ref to fetch and check out. Defaults to origin/main.
  --run-id <id>                    Deterministic run id. Defaults to UTC timestamp.
  --local-artifact-dir <path>      Optional local artifact directory for fetched summary + log tarball.
  --summary-name <name>            Remote summary filename. Defaults to summary.json.
  --help                           Show this help.

Environment overrides:
  ADL_NESSUS_REMOTE_EXECUTOR
  ADL_NESSUS_REMOTE_HOST
  ADL_NESSUS_REMOTE_SSH_USER
  ADL_NESSUS_REMOTE_WSL_USER
  ADL_NESSUS_REMOTE_ROOT
  ADL_NESSUS_REMOTE_REPO_URL
  ADL_NESSUS_REMOTE_GIT_REF
  ADL_NESSUS_REMOTE_ARTIFACT_DIR
  ADL_NESSUS_APT_SOURCES_LIST
  ADL_NESSUS_APT_KUBERNETES_LIST
  SSH_BIN
EOF
}

timestamp_id() {
  date -u +"%Y%m%d-%H%M%S"
}

quote_remote_single() {
  printf "%s" "$1" | sed "s/'/'\"'\"'/g"
}

COMMAND_STRING=""
EXECUTOR="${ADL_NESSUS_REMOTE_EXECUTOR:-ssh}"
HOST="${ADL_NESSUS_REMOTE_HOST:-nessus.local}"
SSH_USER="${ADL_NESSUS_REMOTE_SSH_USER:-danie}"
WSL_USER="${ADL_NESSUS_REMOTE_WSL_USER:-root}"
REMOTE_ROOT="${ADL_NESSUS_REMOTE_ROOT:-/root/adl-remote-runner}"
REPO_URL="${ADL_NESSUS_REMOTE_REPO_URL:-https://github.com/danielbaustin/agent-design-language.git}"
GIT_REF="${ADL_NESSUS_REMOTE_GIT_REF:-origin/main}"
RUN_ID=""
LOCAL_ARTIFACT_DIR="${ADL_NESSUS_REMOTE_ARTIFACT_DIR:-}"
SUMMARY_NAME="summary.json"
SSH_BIN="${SSH_BIN:-ssh}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --command)
      COMMAND_STRING="${2:-}"
      shift 2
      ;;
    --executor)
      EXECUTOR="${2:-}"
      shift 2
      ;;
    --host)
      HOST="${2:-}"
      shift 2
      ;;
    --ssh-user)
      SSH_USER="${2:-}"
      shift 2
      ;;
    --wsl-user)
      WSL_USER="${2:-}"
      shift 2
      ;;
    --remote-root)
      REMOTE_ROOT="${2:-}"
      shift 2
      ;;
    --repo-url)
      REPO_URL="${2:-}"
      shift 2
      ;;
    --git-ref)
      GIT_REF="${2:-}"
      shift 2
      ;;
    --run-id)
      RUN_ID="${2:-}"
      shift 2
      ;;
    --local-artifact-dir)
      LOCAL_ARTIFACT_DIR="${2:-}"
      shift 2
      ;;
    --summary-name)
      SUMMARY_NAME="${2:-}"
      shift 2
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      echo "run_nessus_remote_validation: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$COMMAND_STRING" ]]; then
  echo "run_nessus_remote_validation: --command is required" >&2
  usage >&2
  exit 2
fi

if [[ "$EXECUTOR" != "ssh" && "$EXECUTOR" != "local" ]]; then
  echo "run_nessus_remote_validation: unsupported --executor '$EXECUTOR' (expected ssh or local)" >&2
  exit 2
fi

if [[ -z "$RUN_ID" ]]; then
  RUN_ID="$(timestamp_id)"
fi

COMMAND_B64="$(printf "%s" "$COMMAND_STRING" | base64 | tr -d '\n')"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-nessus-remote-validation.XXXXXX")"
REMOTE_SCRIPT="$TMP_DIR/remote_runner.sh"
trap 'rm -rf "$TMP_DIR"' EXIT

cat >"$REMOTE_SCRIPT" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

REMOTE_ROOT="$1"
REPO_URL="$2"
GIT_REF="$3"
RUN_ID="$4"
COMMAND_B64="$5"
SUMMARY_NAME="$6"
APT_SOURCES_LIST="${ADL_NESSUS_APT_SOURCES_LIST:-/etc/apt/sources.list}"
APT_KUBERNETES_LIST="${ADL_NESSUS_APT_KUBERNETES_LIST:-/etc/apt/sources.list.d/kubernetes.list}"
COMMAND_STRING="$(printf '%s' "$COMMAND_B64" | base64 -d)"
RUN_ROOT="$REMOTE_ROOT/logs/$RUN_ID"
REPO_DIR="$REMOTE_ROOT/agent-design-language"
CACHE_ROOT="$REMOTE_ROOT/cache"
TARGET_DIR="$CACHE_ROOT/target"
SCCACHE_DIR="$CACHE_ROOT/sccache"
SUMMARY_PATH="$RUN_ROOT/$SUMMARY_NAME"
WINDOWS_IDENTITY_FILE="$RUN_ROOT/windows-identity.txt"
WSL_IDENTITY_FILE="$RUN_ROOT/wsl-identity.txt"
STATUS="failed"
RESOLVED_COMMIT="unknown"
COMMAND_EXIT=1
APT_MASKED=false
HASHICORP_MASKED=false
KUBERNETES_BACKUP=""
SOURCES_BACKUP=""
START_EPOCH="$(date +%s)"
START_ISO="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

mkdir -p "$RUN_ROOT" "$CACHE_ROOT" "$TARGET_DIR" "$SCCACHE_DIR"

restore_apt_sources() {
  if [[ -n "$KUBERNETES_BACKUP" && -f "$KUBERNETES_BACKUP" ]]; then
    mv "$KUBERNETES_BACKUP" "$APT_KUBERNETES_LIST"
  fi
  if [[ -n "$SOURCES_BACKUP" && -f "$SOURCES_BACKUP" ]]; then
    mv "$SOURCES_BACKUP" "$APT_SOURCES_LIST"
  fi
}

write_summary() {
  local exit_code="$1"
  local end_epoch elapsed end_iso
  end_epoch="$(date +%s)"
  elapsed="$((end_epoch - START_EPOCH))"
  end_iso="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  python3 - <<'PY' \
    "$SUMMARY_PATH" "$RUN_ID" "$STATUS" "$exit_code" "$START_ISO" "$end_iso" "$elapsed" \
    "$REPO_URL" "$GIT_REF" "$RESOLVED_COMMIT" "$COMMAND_STRING" "$REMOTE_ROOT" "$RUN_ROOT" \
    "$REPO_DIR" "$TARGET_DIR" "$SCCACHE_DIR" "$APT_SOURCES_LIST" "$APT_KUBERNETES_LIST"
import json
import os
import sys
from pathlib import Path

(
    summary_path,
    run_id,
    status,
    exit_code,
    started_at,
    finished_at,
    elapsed_seconds,
    repo_url,
    git_ref,
    resolved_commit,
    command,
    remote_root,
    run_root,
    repo_dir,
    target_dir,
    sccache_dir,
    apt_sources_list,
    apt_kubernetes_list,
) = sys.argv[1:]

run_root_path = Path(run_root)
sccache_stats_path = run_root_path / "sccache-stats.log"
cache_status = {
    "target_dir": target_dir,
    "sccache_dir": sccache_dir,
    "sccache_stats_log": str(sccache_stats_path),
}
if sccache_stats_path.exists():
    for raw_line in sccache_stats_path.read_text(encoding="utf-8", errors="replace").splitlines():
        normalized = " ".join(raw_line.split())
        if normalized.startswith("Compile requests "):
            cache_status["compile_requests"] = normalized.split()[-1]
        elif normalized.startswith("Compile requests executed "):
            cache_status["compile_requests_executed"] = normalized.split()[-1]
        elif normalized.startswith("Cache hits ") and not normalized.startswith("Cache hits rate "):
            cache_status["cache_hits"] = normalized.split()[-1]
        elif normalized.startswith("Cache misses ") and not normalized.startswith("Cache misses ("):
            cache_status["cache_misses"] = normalized.split()[-1]

payload = {
    "schema_version": "adl.remote_validation_run.v1",
    "runner": "nessus",
    "run_id": run_id,
    "status": status,
    "exit_code": int(exit_code),
    "started_at": started_at,
    "finished_at": finished_at,
    "elapsed_seconds": int(elapsed_seconds),
    "repo_url": repo_url,
    "git_ref": git_ref,
    "resolved_commit": resolved_commit,
    "command": command,
    "remote_root": remote_root,
    "run_root": run_root,
    "repo_dir": repo_dir,
    "target_dir": target_dir,
    "sccache_dir": sccache_dir,
    "cache_status": cache_status,
    "apt_sources_list": apt_sources_list,
    "apt_kubernetes_list": apt_kubernetes_list,
    "logs": {
        "host_facts": os.path.join(run_root, "host-facts.log"),
        "rustc_version": os.path.join(run_root, "rustc-version.log"),
        "cargo_version": os.path.join(run_root, "cargo-version.log"),
        "sccache_version": os.path.join(run_root, "sccache-version.log"),
        "apt_update": os.path.join(run_root, "apt-update.log"),
        "git_fetch": os.path.join(run_root, "git-fetch.log"),
        "git_checkout": os.path.join(run_root, "git-checkout.log"),
        "command": os.path.join(run_root, "command.log"),
        "sccache_stats": os.path.join(run_root, "sccache-stats.log"),
        "windows_identity": os.path.join(run_root, "windows-identity.txt"),
        "wsl_identity": os.path.join(run_root, "wsl-identity.txt"),
    },
}

with open(summary_path, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, indent=2, sort_keys=True)
    handle.write("\n")
PY
}

finish() {
  local exit_code="$1"
  set +e
  if command -v sccache >/dev/null 2>&1; then
    sccache --show-stats >"$RUN_ROOT/sccache-stats.log" 2>&1
  fi
  restore_apt_sources
  write_summary "$exit_code"
  exit "$exit_code"
}

trap 'finish $?' EXIT

printf '%s\n' "${ADL_NESSUS_WINDOWS_IDENTITY:-unknown}" >"$WINDOWS_IDENTITY_FILE"
whoami >"$WSL_IDENTITY_FILE"
{
  echo "whoami=$(whoami)"
  echo "uname=$(uname -s)"
  echo "kernel=$(uname -r)"
  if command -v nproc >/dev/null 2>&1; then
    echo "cpus=$(nproc)"
  fi
  if command -v free >/dev/null 2>&1; then
    free -h
  fi
} >"$RUN_ROOT/host-facts.log" 2>&1

if [[ -f "$HOME/.cargo/env" ]]; then
  # Non-login WSL shells need the Cargo toolchain path restored explicitly.
  # The proven interactive path already had rust/cargo/sccache available.
  # The wrapper preserves that truth for automation without assuming systemwide
  # installation.
  # shellcheck disable=SC1090
  . "$HOME/.cargo/env"
fi

command -v git >/dev/null 2>&1
command -v rustc >/dev/null 2>&1
command -v cargo >/dev/null 2>&1
command -v sccache >/dev/null 2>&1

rustc --version >"$RUN_ROOT/rustc-version.log" 2>&1
cargo --version >"$RUN_ROOT/cargo-version.log" 2>&1
sccache --version >"$RUN_ROOT/sccache-version.log" 2>&1
sccache --zero-stats >/dev/null 2>&1 || true

if [[ -f "$APT_KUBERNETES_LIST" ]]; then
  KUBERNETES_BACKUP="$RUN_ROOT/kubernetes.list.backup"
  mv "$APT_KUBERNETES_LIST" "$KUBERNETES_BACKUP"
  APT_MASKED=true
fi
if [[ -f "$APT_SOURCES_LIST" ]] && grep -q 'apt.releases.hashicorp.com' "$APT_SOURCES_LIST"; then
  SOURCES_BACKUP="$RUN_ROOT/sources.list.backup"
  cp "$APT_SOURCES_LIST" "$SOURCES_BACKUP"
  python3 - <<'PY' "$APT_SOURCES_LIST"
from pathlib import Path
import sys

path = Path(sys.argv[1])
lines = path.read_text(encoding="utf-8").splitlines()
rewritten = []
for line in lines:
    if "apt.releases.hashicorp.com" in line and not line.lstrip().startswith("#"):
        rewritten.append(f"# ADL-temporarily-masked {line}")
    else:
        rewritten.append(line)
path.write_text("\n".join(rewritten) + "\n", encoding="utf-8")
PY
  HASHICORP_MASKED=true
fi
apt-get update >"$RUN_ROOT/apt-update.log" 2>&1

mkdir -p "$REMOTE_ROOT"
if [[ ! -d "$REPO_DIR/.git" ]]; then
  git clone "$REPO_URL" "$REPO_DIR" >"$RUN_ROOT/git-clone.log" 2>&1
fi

git -C "$REPO_DIR" reset --hard HEAD >"$RUN_ROOT/git-reset.log" 2>&1
git -C "$REPO_DIR" clean -fd >"$RUN_ROOT/git-clean.log" 2>&1
git -C "$REPO_DIR" fetch origin --prune >"$RUN_ROOT/git-fetch.log" 2>&1
git -C "$REPO_DIR" checkout --detach "$GIT_REF" >"$RUN_ROOT/git-checkout.log" 2>&1
RESOLVED_COMMIT="$(git -C "$REPO_DIR" rev-parse HEAD)"

export CARGO_TARGET_DIR="$TARGET_DIR"
export SCCACHE_DIR
export RUSTC_WRAPPER="sccache"

if bash -lc "cd '$REPO_DIR' && $COMMAND_STRING" >"$RUN_ROOT/command.log" 2>&1; then
  COMMAND_EXIT=0
  STATUS="passed"
else
  COMMAND_EXIT=$?
  STATUS="failed"
fi

exit "$COMMAND_EXIT"
EOF
chmod +x "$REMOTE_SCRIPT"

REMOTE_SUMMARY_PATH="$REMOTE_ROOT/logs/$RUN_ID/$SUMMARY_NAME"
REMOTE_RUN_ROOT="$REMOTE_ROOT/logs/$RUN_ID"
LOCAL_SUMMARY_PATH=""

run_remote() {
  if [[ "$EXECUTOR" == "ssh" ]]; then
    local remote_cmd
    remote_cmd="wsl.exe -u $WSL_USER -- bash -s -- '$(quote_remote_single "$REMOTE_ROOT")' '$(quote_remote_single "$REPO_URL")' '$(quote_remote_single "$GIT_REF")' '$(quote_remote_single "$RUN_ID")' '$(quote_remote_single "$COMMAND_B64")' '$(quote_remote_single "$SUMMARY_NAME")'"
    "$SSH_BIN" -o BatchMode=yes -o ConnectTimeout=15 "${SSH_USER}@${HOST}" "$remote_cmd" <"$REMOTE_SCRIPT"
  else
    ADL_NESSUS_WINDOWS_IDENTITY="local-executor" bash "$REMOTE_SCRIPT" \
      "$REMOTE_ROOT" "$REPO_URL" "$GIT_REF" "$RUN_ID" "$COMMAND_B64" "$SUMMARY_NAME"
  fi
}

fetch_summary() {
  local destination="$1"
  if [[ "$EXECUTOR" == "ssh" ]]; then
    local remote_cmd
    remote_cmd="wsl.exe -u $WSL_USER -- bash -lc 'cat '\''$(quote_remote_single "$REMOTE_SUMMARY_PATH")'\'''"
    "$SSH_BIN" -o BatchMode=yes -o ConnectTimeout=15 "${SSH_USER}@${HOST}" "$remote_cmd" >"$destination"
  else
    cp "$REMOTE_SUMMARY_PATH" "$destination"
  fi
}

fetch_logs_tarball() {
  local destination="$1"
  if [[ "$EXECUTOR" == "ssh" ]]; then
    local remote_cmd
    remote_cmd="wsl.exe -u $WSL_USER -- bash -lc 'tar -C '\''$(quote_remote_single "$REMOTE_RUN_ROOT")'\'' -czf - .'"
    "$SSH_BIN" -o BatchMode=yes -o ConnectTimeout=15 "${SSH_USER}@${HOST}" "$remote_cmd" >"$destination"
  else
    tar -C "$REMOTE_RUN_ROOT" -czf "$destination" .
  fi
}

write_transport_failure_summary() {
  local destination="$1"
  python3 - <<'PY' \
    "$destination" "$RUN_ID" "$RUN_EXIT" "$COMMAND_STRING" "$EXECUTOR" "$HOST" "$SSH_USER" \
    "$REMOTE_ROOT" "$REMOTE_SUMMARY_PATH" "$REMOTE_RUN_ROOT" "$REPO_URL" "$GIT_REF"
import json
import sys

(
    destination,
    run_id,
    exit_code,
    command,
    executor,
    host,
    ssh_user,
    remote_root,
    remote_summary_path,
    remote_run_root,
    repo_url,
    git_ref,
) = sys.argv[1:]

payload = {
    "schema_version": "adl.remote_validation_run.v1",
    "runner": "nessus",
    "run_id": run_id,
    "status": "failed",
    "exit_code": int(exit_code),
    "command": command,
    "repo_url": repo_url,
    "git_ref": git_ref,
    "resolved_commit": "unknown",
    "remote_root": remote_root,
    "run_root": remote_run_root,
    "repo_dir": f"{remote_root}/agent-design-language",
    "target_dir": f"{remote_root}/cache/target",
    "sccache_dir": f"{remote_root}/cache/sccache",
    "started_at": "unknown",
    "finished_at": "unknown",
    "elapsed_seconds": 0,
    "transport_failure": {
        "executor": executor,
        "host": host,
        "ssh_user": ssh_user,
        "summary_fetch_failed": True,
        "expected_remote_summary_path": remote_summary_path,
    },
    "logs": {
        "host_facts": "not_available",
        "rustc_version": "not_available",
        "cargo_version": "not_available",
        "sccache_version": "not_available",
        "apt_update": "not_available",
        "git_fetch": "not_available",
        "git_checkout": "not_available",
        "command": "not_available",
        "sccache_stats": "not_available",
        "windows_identity": "not_available",
        "wsl_identity": "not_available",
    },
    "cache_status": {
        "target_dir": f"{remote_root}/cache/target",
        "sccache_dir": f"{remote_root}/cache/sccache",
        "sccache_stats_log": "not_available",
    },
    "apt_sources_list": "unknown",
    "apt_kubernetes_list": "unknown",
}

with open(destination, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, indent=2, sort_keys=True)
    handle.write("\n")
PY
}

set +e
run_remote
RUN_EXIT=$?
set -e

SUMMARY_FETCH_OK=true
if [[ -n "$LOCAL_ARTIFACT_DIR" ]]; then
  mkdir -p "$LOCAL_ARTIFACT_DIR"
  LOCAL_SUMMARY_PATH="$LOCAL_ARTIFACT_DIR/$SUMMARY_NAME"
else
  LOCAL_SUMMARY_PATH="$TMP_DIR/$SUMMARY_NAME"
fi

set +e
fetch_summary "$LOCAL_SUMMARY_PATH"
SUMMARY_FETCH_RC=$?
set -e
if [[ "$SUMMARY_FETCH_RC" -ne 0 ]]; then
  SUMMARY_FETCH_OK=false
  if [[ "$RUN_EXIT" -ne 0 ]]; then
    write_transport_failure_summary "$LOCAL_SUMMARY_PATH"
  else
    echo "run_nessus_remote_validation: failed to fetch remote summary from $REMOTE_SUMMARY_PATH" >&2
    exit "$SUMMARY_FETCH_RC"
  fi
fi

if [[ -n "$LOCAL_ARTIFACT_DIR" ]]; then
  set +e
  fetch_logs_tarball "$LOCAL_ARTIFACT_DIR/run-logs.tar.gz"
  LOG_FETCH_RC=$?
  set -e
  if [[ "$LOG_FETCH_RC" -ne 0 && "$RUN_EXIT" -eq 0 ]]; then
    echo "run_nessus_remote_validation: failed to fetch remote log tarball from $REMOTE_RUN_ROOT" >&2
    exit "$LOG_FETCH_RC"
  fi
fi

python3 - <<'PY' "$LOCAL_SUMMARY_PATH"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
print(json.dumps(summary, indent=2, sort_keys=True))
PY

if [[ "$RUN_EXIT" -ne 0 ]]; then
  if [[ "$SUMMARY_FETCH_OK" == true ]]; then
    echo "run_nessus_remote_validation: remote command failed (see summary above)" >&2
  else
    echo "run_nessus_remote_validation: transport failed before remote summary was available; fallback summary written locally" >&2
  fi
fi

exit "$RUN_EXIT"
