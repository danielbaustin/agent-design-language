#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/check_pr_closing_linkage.sh"
TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

run_event_payload_only() {
  env -u GH_TOKEN -u GITHUB_TOKEN -u GITHUB_REPOSITORY ADL_PR_CLOSING_LINKAGE_DISABLE_RUST=1 "$@"
}

run_with_fake_live_metadata() {
  env -u GITHUB_REPOSITORY ADL_PR_CLOSING_LINKAGE_DISABLE_RUST=1 "$@"
}

make_event() {
  local path="$1"
  local body="$2"
  local repo="${3:-example/repo}"
  local pr_number="${4:-77}"
  python3 - "$path" "$body" "$repo" "$pr_number" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
body = sys.argv[2]
repo = sys.argv[3]
pr_number = int(sys.argv[4])
path.write_text(json.dumps({
    "repository": {"full_name": repo},
    "pull_request": {"body": body, "number": pr_number}
}))
PY
}

event_ok="$TMPDIR/ok.json"
make_event "$event_ok" "Closes #1414"
run_event_payload_only bash "$SCRIPT" --event-name pull_request --event-path "$event_ok" --head-ref "codex/1414-remediation"

event_fix="$TMPDIR/fix.json"
make_event "$event_fix" $'Some notes\n\nFixes #1414'
run_event_payload_only bash "$SCRIPT" --event-name pull_request --event-path "$event_fix" --head-ref "codex/1414-remediation"

event_bad="$TMPDIR/bad.json"
make_event "$event_bad" "Refs #1414"
if run_event_payload_only bash "$SCRIPT" --event-name pull_request --event-path "$event_bad" --head-ref "codex/1414-remediation"; then
  echo "expected failure for missing closing linkage" >&2
  exit 1
fi
if run_event_payload_only bash "$SCRIPT" --event-name pull_request --event-path "$event_bad" --head-ref "codex/1414-remediation" 2>"$TMPDIR/bad.err"; then
  echo "expected failure for stale-event fallback diagnostic" >&2
  exit 1
fi
grep -F "stale pull_request event payload" "$TMPDIR/bad.err" >/dev/null
grep -F "push a fresh commit to refresh the event payload" "$TMPDIR/bad.err" >/dev/null

event_non_closing="$TMPDIR/non-closing.json"
make_event "$event_non_closing" "Non-closing lifecycle PR: issue 1414 remains open"
run_event_payload_only bash "$SCRIPT" --event-name pull_request --event-path "$event_non_closing" --head-ref "codex/1414-remediation"

fake_bin="$TMPDIR/bin"
mkdir -p "$fake_bin"
cat >"$fake_bin/gh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "pr" && "${2:-}" == "view" && "${3:-}" == "77" ]]; then
  repo=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --repo)
        repo="${2:-}"
        shift 2
        ;;
      *)
        shift
        ;;
    esac
  done
  if [[ "$repo" != "example/repo" ]]; then
    echo "unexpected fake gh repo: $repo" >&2
    exit 2
  fi
  printf '%s\n' "${FAKE_GH_PR_BODY:-}"
  exit 0
fi

echo "unexpected gh invocation: $*" >&2
exit 2
SH
chmod +x "$fake_bin/gh"

event_stale="$TMPDIR/stale.json"
make_event "$event_stale" "Refs #1414"
GH_TOKEN="test-token" FAKE_GH_PR_BODY="Closes #1414" PATH="$fake_bin:$PATH" \
  run_with_fake_live_metadata \
  bash "$SCRIPT" --event-name pull_request --event-path "$event_stale" --head-ref "codex/1414-remediation"

GH_TOKEN="test-token" FAKE_GH_PR_BODY="Non-closing lifecycle PR: issue 1414 remains open" PATH="$fake_bin:$PATH" \
  run_with_fake_live_metadata \
  bash "$SCRIPT" --event-name pull_request --event-path "$event_stale" --head-ref "codex/1414-remediation"

if GH_TOKEN="test-token" FAKE_GH_PR_BODY="Refs #1414" PATH="$fake_bin:$PATH" \
  run_with_fake_live_metadata \
  bash "$SCRIPT" --event-name pull_request --event-path "$event_stale" --head-ref "codex/1414-remediation" 2>"$TMPDIR/live-missing.err"; then
  echo "expected failure for live PR body missing closing linkage" >&2
  exit 1
fi
grep -F "live PR body for PR #77 is missing closing linkage" "$TMPDIR/live-missing.err" >/dev/null

cat >"$fake_bin/gh-fail" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
echo "simulated gh failure with sensitive-looking text" >&2
exit 1
SH
chmod +x "$fake_bin/gh-fail"
if GH_TOKEN="test-token" CHECK_PR_CLOSING_LINKAGE_GH_BIN="$fake_bin/gh-fail" \
  run_with_fake_live_metadata \
  bash "$SCRIPT" --event-name pull_request --event-path "$event_stale" --head-ref "codex/1414-remediation" 2>"$TMPDIR/live-failed.err"; then
  echo "expected failure for failed live PR body fetch" >&2
  exit 1
fi
grep -F "live PR metadata fetch failed" "$TMPDIR/live-failed.err" >/dev/null
if grep -F "sensitive-looking text" "$TMPDIR/live-failed.err" >/dev/null; then
  echo "raw gh stderr leaked into fallback diagnostic" >&2
  exit 1
fi

delegated_log="$TMPDIR/delegated.log"
delegate_bin="$TMPDIR/delegate.sh"
cat >"$delegate_bin" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >"${ADL_TEST_LOG}"
SH
chmod +x "$delegate_bin"
ADL_TEST_LOG="$delegated_log" ADL_PR_CLOSING_LINKAGE_BIN="$delegate_bin" \
  bash "$SCRIPT" --event-name pull_request --head-ref codex/1414-remediation
grep -Fqx -- '--event-name pull_request --head-ref codex/1414-remediation' "$delegated_log" || {
  echo "expected compatibility helper to delegate directly to configured Rust binary" >&2
  exit 1
}

event_other="$TMPDIR/other.json"
make_event "$event_other" "Refs #1414"
run_event_payload_only bash "$SCRIPT" --event-name push --event-path "$event_other" --head-ref "codex/1414-remediation"
run_event_payload_only bash "$SCRIPT" --event-name pull_request --event-path "$event_other" --head-ref "feature/no-issue-branch"

echo "test_pr_closing_linkage.sh: PASS"
