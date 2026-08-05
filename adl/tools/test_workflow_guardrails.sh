#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_contains() {
  local needle="$1"
  local haystack="$2"
  local label="$3"
  if ! grep -Fq -- "$needle" <<<"$haystack"; then
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

  local unsafe_backtick="$TMP/unsafe-report-backtick.txt"
  cat >"$unsafe_backtick" <<'UNSAFE_BACKTICK'
echo `pwd` > report.md
UNSAFE_BACKTICK
  if out="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" safe-report-command --file "$unsafe_backtick" 2>&1)"; then
    echo "expected unsafe backtick report command to fail" >&2
    exit 1
  fi
  assert_contains "Unsafe command substitution detected" "$out" "unsafe backtick command"

  local safe_markdown="$TMP/safe-markdown-report-command.txt"
  cat >"$safe_markdown" <<'SAFE_MARKDOWN'
cat > report.md <<'MD'
# Report

```bash
echo safe
```

Literal text may mention $(pwd) without executing because this heredoc is quoted.
MD
SAFE_MARKDOWN
  out="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" safe-report-command --file "$safe_markdown")"
  assert_contains "PASS safe-report-command" "$out" "safe markdown report command"

  out="$(bash "$ROOT/adl/tools/workflow_guardrails.sh" safe-report-command --command "python3 - <<'PY'\nprint('# Report')\nPY")"
  assert_contains "PASS safe-report-command" "$out" "safe report command"
}

test_card_drift_wrapper() {
  local repo="$TMP/card-drift-repo"
  mkdir -p "$repo/adl/tools" "$repo/.adl/bin/csdlc-v2"
  cp "$ROOT/adl/tools/workflow_guardrails.sh" "$repo/adl/tools/workflow_guardrails.sh"
  chmod +x "$repo/adl/tools/workflow_guardrails.sh"
  local log="$TMP/card-drift.log"
  cat >"$repo/.adl/bin/csdlc-v2/csdlc-install" <<EOF
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "\$*" >>"$log"
exit 0
EOF
  cat >"$repo/.adl/bin/csdlc-v2/csdlc-doctor" <<EOF
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "\$*" >>"$log"
exit 0
EOF
  chmod +x "$repo/.adl/bin/csdlc-v2/csdlc-install" "$repo/.adl/bin/csdlc-v2/csdlc-doctor"
  (cd "$repo" && bash ./adl/tools/workflow_guardrails.sh card-drift --issue 100 --root "$repo")
  assert_contains "resolve --repo $repo --issue 100" "$(cat "$log")" "card drift selector"
  assert_contains "--repo $repo --issue 100" "$(cat "$log")" "card drift doctor"
}

test_main_write_guardrail
test_safe_report_command_guardrail
test_card_drift_wrapper

echo "PASS test_workflow_guardrails"
