#!/usr/bin/env bash
if [ -z "${BASH_VERSION:-}" ]; then
  exec bash "$0" "$@"
fi
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TEMP_DIR"' EXIT

BIN_DIR="$TEMP_DIR/bin"
mkdir -p "$BIN_DIR"
mkdir -p "$TEMP_DIR/repo/adl/tools"

cat >"$BIN_DIR/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == "pr view" ]]; then
  printf '{"state":"MERGED","mergedAt":"2026-04-11T12:00:00Z","url":"https://example.test/pr/1"}\n'
  exit 0
fi
if [[ "$1 $2" == "issue view" ]]; then
  printf '{"state":"CLOSED","stateReason":"COMPLETED","url":"https://example.test/issues/1"}\n'
  exit 0
fi
exit 1
EOF
chmod +x "$BIN_DIR/gh"

CLOSEOUT_LOG="$TEMP_DIR/closeout.log"
cat >"$TEMP_DIR/repo/adl/tools/pr.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "\$*" >>"$CLOSEOUT_LOG"
exit 0
EOF
chmod +x "$TEMP_DIR/repo/adl/tools/pr.sh"

SUMMARY_FILE="$TEMP_DIR/summary.md"
RUN_LOG="$TEMP_DIR/run.log"
PATH="$BIN_DIR:$PATH" \
  bash "$ROOT_DIR/attach_post_merge_closeout.sh" \
  --watch \
  --repo-root "$TEMP_DIR/repo" \
  --repo "owner/repo" \
  --issue "1" \
  --branch "codex/1-test" \
  --pr-url "https://example.test/pr/1" \
  --summary-file "$SUMMARY_FILE" \
  --run-log "$RUN_LOG"

grep -Fq 'closeout 1' "$CLOSEOUT_LOG"
grep -Fq 'status: normalized' "$SUMMARY_FILE"
grep -Fq 'canonical sor.md reconciled to closed merged truth' "$SUMMARY_FILE"

cat >"$BIN_DIR/gh-open" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == "pr view" ]]; then
  printf '{"state":"OPEN","mergedAt":"","url":"https://example.test/pr/2"}\n'
  exit 0
fi
if [[ "$1 $2" == "issue view" ]]; then
  printf '{"state":"OPEN","stateReason":"","url":"https://example.test/issues/2"}\n'
  exit 0
fi
exit 1
EOF
chmod +x "$BIN_DIR/gh-open"

if PATH="$BIN_DIR:$PATH" GH="$BIN_DIR/gh-open" ADL_POST_MERGE_CLOSEOUT_MAX_ATTEMPTS=1 ADL_POST_MERGE_CLOSEOUT_SLEEP_SECS=0 bash -c '
  function gh(){ "$GH" "$@"; }
  export -f gh
  bash "'"$ROOT_DIR"'/attach_post_merge_closeout.sh" \
    --watch \
    --repo-root "'"$TEMP_DIR"'/repo" \
    --repo "owner/repo" \
    --issue "2" \
    --branch "codex/2-test" \
    --pr-url "https://example.test/pr/2" \
    --summary-file "'"$TEMP_DIR"'/timeout-summary.md" \
    --run-log "'"$TEMP_DIR"'/timeout.log"
'; then
  echo "expected timeout watcher to fail non-zero" >&2
  exit 1
fi

grep -Fq 'status: timeout' "$TEMP_DIR/timeout-summary.md"

echo "PASS test_attach_post_merge_closeout"
