#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_contains() {
  local needle="$1"
  local haystack="$2"
  local label="$3"
  if ! grep -Fq "$needle" <<<"$haystack"; then
    echo "assertion failed: expected '$needle' in $label" >&2
    echo "$haystack" >&2
    exit 1
  fi
}

init_repo() {
  local repo="$1"
  mkdir -p "$repo/.git"
  git -C "$repo" init -q
  git -C "$repo" config user.name "Test User"
  git -C "$repo" config user.email "test@example.com"
  printf '.adl/\n' >"$repo/.gitignore"
  printf 'seed\n' >"$repo/README.md"
  git -C "$repo" add .gitignore README.md
  git -C "$repo" commit -q -m "init"
  git -C "$repo" branch -M main
}

test_main_write_guardrail() {
  local repo="$TMP/main-write-repo"
  mkdir -p "$repo"
  init_repo "$repo"
  local out
  out="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" main-write --repo "$repo")"
  assert_contains "PASS main-write branch=main clean=true" "$out" "clean main pass"

  printf 'tracked drift\n' >"$repo/README.md"
  if out="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" main-write --repo "$repo" 2>&1)"; then
    echo "expected dirty main guardrail to fail" >&2
    exit 1
  fi
  assert_contains "BLOCKED main-write branch=main clean=false" "$out" "dirty main fail"
  assert_contains "README.md" "$out" "dirty main status"
}

test_safe_report_command_guardrail() {
  local unsafe="$TMP/unsafe-report-command.txt"
  cat >"$unsafe" <<'UNSAFE'
cat <<EOF > report.md
$(pwd)
EOF
UNSAFE
  if out="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" safe-report-command --file "$unsafe" 2>&1)"; then
    echo "expected unsafe report command to fail" >&2
    exit 1
  fi
  assert_contains "Unsafe command substitution detected" "$out" "unsafe report command"

  out="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" safe-report-command --command "python3 - <<'PY'\nprint('# Report')\nPY")"
  assert_contains "PASS safe-report-command" "$out" "safe report command"
}

test_closeout_watch_guardrail() {
  local repo="$TMP/closeout-watch-repo"
  mkdir -p "$repo/adl/tools" "$repo/.adl/v0.88/tasks/issue-100__demo" "$repo/.git"
  printf 'Status: DONE\n' >"$repo/.adl/v0.88/tasks/issue-100__demo/sor.md"
  cp "$ROOT/adl/tools/closeout_completed_issue_wave.sh" "$repo/adl/tools/closeout_completed_issue_wave.sh"
  cp "$ROOT/adl/tools/workflow_guardrails.sh" "$repo/adl/tools/workflow_guardrails.sh"
  chmod +x "$repo/adl/tools/closeout_completed_issue_wave.sh" "$repo/adl/tools/workflow_guardrails.sh"
  cat >"$repo/.git/config" <<'EOF'
[remote "origin"]
	url = git@github.com:danielbaustin/agent-design-language.git
EOF
  cat >"$repo/adl/tools/pr.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
exit 0
EOF
  chmod +x "$repo/adl/tools/pr.sh"
  local bin="$TMP/bin-closeout"
  mkdir -p "$bin"
  cat >"$bin/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '[{"number":100,"stateReason":"COMPLETED"}]\n'
EOF
  chmod +x "$bin/gh"

  local report="$TMP/closeout-watch-report.md"
  if out="$(cd "$repo" && PATH="$bin:$PATH" bash ./adl/tools/workflow_guardrails.sh closeout-watch --version v0.88 --repo danielbaustin/agent-design-language --root "$repo" --report "$report" 2>&1)"; then
    echo "expected closeout-watch to block on pending candidate" >&2
    exit 1
  fi
  assert_contains "BLOCKED closeout-watch version=v0.88 candidates=1" "$out" "closeout watch fail"
  assert_contains "candidate_issues: 1" "$(cat "$report")" "closeout report"

  cat >"$bin/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '[]\n'
EOF
  chmod +x "$bin/gh"
  out="$(cd "$repo" && PATH="$bin:$PATH" bash ./adl/tools/workflow_guardrails.sh closeout-watch --version v0.88 --repo danielbaustin/agent-design-language --root "$repo" --report "$report")"
  assert_contains "PASS closeout-watch version=v0.88 candidates=0" "$out" "closeout watch pass"
}

test_card_drift_wrapper() {
  local repo="$TMP/card-drift-repo"
  mkdir -p "$repo/adl/tools"
  cp "$ROOT/adl/tools/workflow_guardrails.sh" "$repo/adl/tools/workflow_guardrails.sh"
  chmod +x "$repo/adl/tools/workflow_guardrails.sh"
  local log="$TMP/card-drift.log"
  cat >"$repo/adl/tools/pr.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "\$*" >>"$log"
exit 0
EOF
  chmod +x "$repo/adl/tools/pr.sh"
  (cd "$repo" && bash ./adl/tools/workflow_guardrails.sh card-drift --issue 100 --version v0.88 --slug demo --root "$repo" --mode full)
  assert_contains "doctor 100 --version v0.88 --mode full --slug demo" "$(cat "$log")" "card drift wrapper"
}

test_main_write_guardrail
test_safe_report_command_guardrail
test_closeout_watch_guardrail
test_card_drift_wrapper

echo "PASS test_workflow_guardrails"
