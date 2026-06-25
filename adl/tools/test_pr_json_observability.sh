#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
PR_DELEGATE_SRC="$ROOT_DIR/adl/tools/pr_delegate.sh"
PR_USAGE_SRC="$ROOT_DIR/adl/tools/pr_usage.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
OBS_SRC="$ROOT_DIR/adl/tools/observability.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mockbin="$tmpdir/mockbin"
mkdir -p "$repo/adl/tools" "$repo/adl" "$mockbin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$OBS_SRC" "$repo/adl/tools/observability.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"

cat >"$mockbin/adl-delegate" <<'EOF_DELEGATE'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$TMP_DELEGATE_ARGS"
if [[ "${1:-}" != "pr" ]]; then
  echo "unexpected delegate invocation: $*" >&2
  exit 1
fi
mode="${2:-}"
case "$mode" in
  ready|preflight|doctor)
    printf 'adl_event schema=adl.observability.event.v1 command=adl stage=%s result=started\n' "$mode" >&2
    printf '{\n  "schema": "adl.pr.doctor.v1",\n  "mode": "%s"\n}\n' "$mode"
    ;;
  validation)
    printf 'adl_event schema=adl.observability.event.v1 command=adl stage=validation result=started\n' >&2
    printf '{\n  "pr_number": 1152,\n  "disposition": "success",\n  "projection_status": "ready_to_merge_or_review",\n  "pending_checks": []\n}\n'
    ;;
  issue)
    printf 'adl_event schema=adl.observability.event.v1 command=adl stage=issue result=started\n' >&2
    submode="${3:-}"
    case "$submode" in
      search)
        printf '[\n  {"number": 1152, "title": "validation manager", "state": "OPEN"}\n]\n'
        ;;
      view)
        printf '{\n  "number": 1152,\n  "title": "validation manager",\n  "state": "OPEN"\n}\n'
        ;;
      *)
        echo "unexpected issue mode: $*" >&2
        exit 1
        ;;
    esac
    ;;
  projection-map)
    printf 'adl_event schema=adl.observability.event.v1 command=adl stage=projection-map result=started\n' >&2
    printf '{\n  "schema": "adl.pr.projection_map.v1",\n  "issue_projection_owner": "github",\n  "pr_projection_owner": "github"\n}\n'
    ;;
  *)
    echo "unexpected mode: $mode" >&2
    exit 1
    ;;
esac
EOF_DELEGATE
chmod +x "$mockbin/adl-delegate"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "seed" > README.md
  git add README.md
  git commit -q -m "init"
)

export TMP_DELEGATE_ARGS="$tmpdir/delegate_args.txt"
export ADL_PR_RUST_BIN="$mockbin/adl-delegate"
export PATH="$mockbin:$PATH"

run_json_command() {
  local name="$1"
  shift
  local stdout_file="$tmpdir/${name}.stdout"
  local stderr_file="$tmpdir/${name}.stderr"
  (
    cd "$repo"
    "$BASH_BIN" adl/tools/pr.sh "$@" >"$stdout_file" 2>"$stderr_file"
  )
  python3 - <<'PY' "$stdout_file" "$name"
import json
import pathlib
import sys
payload = pathlib.Path(sys.argv[1]).read_text()
json.loads(payload)
print(f"validated {sys.argv[2]}")
PY
  if grep -Fq 'adl_event schema=adl.observability.event.v1' "$stdout_file"; then
    echo "$name stdout was polluted by observability events" >&2
    cat "$stdout_file" >&2
    exit 1
  fi
  grep -Fq 'adl_event schema=adl.observability.event.v1' "$stderr_file" || {
    echo "$name stderr did not preserve observability events" >&2
    cat "$stderr_file" >&2
    exit 1
  }
}

run_json_command ready ready 1152 --slug rust-start --no-fetch-issue --version v0.86 --json
run_json_command preflight preflight 1152 --slug rust-start --no-fetch-issue --version v0.86 --json
run_json_command doctor doctor 1152 --slug rust-start --no-fetch-issue --version v0.86 --mode full --json
run_json_command validation validation 1152 --json
run_json_command issue-search issue search --query "validation manager" --state open --json
run_json_command issue-view issue view 1152 --json
run_json_command projection-map projection-map --json

args="$(cat "$TMP_DELEGATE_ARGS")"
[[ "$args" == *"pr ready 1152 --slug rust-start --no-fetch-issue --version v0.86 --json"* ]] || {
  echo "expected ready delegation args" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"pr preflight 1152 --slug rust-start --no-fetch-issue --version v0.86 --json"* ]] || {
  echo "expected preflight delegation args" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"pr doctor 1152 --slug rust-start --no-fetch-issue --version v0.86 --mode full --json"* ]] || {
  echo "expected doctor delegation args" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"pr validation 1152 --json"* ]] || {
  echo "expected validation delegation args" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"pr issue search --query validation manager --state open --json"* ]] || {
  echo "expected issue search delegation args" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"pr issue view 1152 --json"* ]] || {
  echo "expected issue view delegation args" >&2
  echo "$args" >&2
  exit 1
}
[[ "$args" == *"pr projection-map --json"* ]] || {
  echo "expected projection-map delegation args" >&2
  echo "$args" >&2
  exit 1
}

echo "PASS test_pr_json_observability"
