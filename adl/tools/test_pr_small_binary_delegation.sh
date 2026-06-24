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
mkdir -p "$repo/adl/tools" "$repo/adl" "$repo/bin"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$OBS_SRC" "$repo/adl/tools/observability.sh"
chmod +x "$repo/adl/tools/pr.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git config commit.gpgsign false
  echo "seed" > README.md
  git add README.md
  git commit -q -m "init"
)

doctor_bin="$repo/bin/adl-pr-doctor"
cat >"$doctor_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'doctor:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$doctor_bin"

run_bin="$repo/bin/adl-pr-run"
cat >"$run_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'run:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$run_bin"

finish_bin="$repo/bin/adl-pr-finish"
cat >"$finish_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'finish:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$finish_bin"

validation_bin="$repo/bin/adl-pr-validation"
cat >"$validation_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'validation:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$validation_bin"

closing_linkage_bin="$repo/bin/adl-pr-closing-linkage"
cat >"$closing_linkage_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'closing-linkage:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$closing_linkage_bin"

issue_bin="$repo/bin/adl-issue"
cat >"$issue_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'issue:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$issue_bin"

closeout_bin="$repo/bin/adl-pr-closeout"
cat >"$closeout_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'closeout:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$closeout_bin"

broad_bin="$repo/bin/adl-broad"
cat >"$broad_bin" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'broad:%s\n' "$*" >"${ADL_TEST_LOG}"
EOF
chmod +x "$broad_bin"

doctor_log="$tmpdir/doctor.log"
(
  cd "$repo"
  ADL_TEST_LOG="$doctor_log" \
    ADL_PR_DOCTOR_BIN="$doctor_bin" \
    "$BASH_BIN" adl/tools/pr.sh doctor 3838 --slug demo --no-fetch-issue --version v0.91.5 --mode full >/dev/null
)
grep -Fqx 'doctor:3838 --slug demo --no-fetch-issue --version v0.91.5 --mode full' "$doctor_log" || {
  echo "assertion failed: doctor should delegate directly to adl-pr-doctor without broad 'pr doctor' argv" >&2
  exit 1
}

run_log="$tmpdir/run.log"
(
  cd "$repo"
  ADL_TEST_LOG="$run_log" \
    ADL_PR_RUN_BIN="$run_bin" \
    "$BASH_BIN" adl/tools/pr.sh run 3838 --slug demo --no-fetch-issue --version v0.91.5 >/dev/null
)
grep -Fqx 'run:3838 --slug demo --no-fetch-issue --version v0.91.5' "$run_log" || {
  echo "assertion failed: issue-mode run should delegate directly to adl-pr-run without broad 'pr start' argv" >&2
  exit 1
}

finish_log="$tmpdir/finish.log"
(
  cd "$repo"
  ADL_TEST_LOG="$finish_log" \
    ADL_PR_FINISH_BIN="$finish_bin" \
    "$BASH_BIN" adl/tools/pr.sh finish 3838 --title "demo finish" --output-card demo-output.md >/dev/null
)
grep -Fqx 'finish:3838 --title demo finish --output-card demo-output.md' "$finish_log" || {
  echo "assertion failed: finish should delegate directly to adl-pr-finish without broad 'pr finish' argv" >&2
  exit 1
}

validation_log="$tmpdir/validation.log"
(
  cd "$repo"
  ADL_TEST_LOG="$validation_log" \
    ADL_PR_VALIDATION_BIN="$validation_bin" \
    "$BASH_BIN" adl/tools/pr.sh validation 3888 --json >/dev/null
)
grep -Fqx 'validation:3888 --json' "$validation_log" || {
  echo "assertion failed: validation should delegate directly to adl-pr-validation without broad 'pr validation' argv" >&2
  exit 1
}

closing_linkage_log="$tmpdir/closing-linkage.log"
(
  cd "$repo"
  ADL_TEST_LOG="$closing_linkage_log" \
    ADL_PR_CLOSING_LINKAGE_BIN="$closing_linkage_bin" \
    "$BASH_BIN" adl/tools/pr.sh closing-linkage --event-name pull_request --head-ref codex/4286-demo >/dev/null
)
grep -Fqx 'closing-linkage:--event-name pull_request --head-ref codex/4286-demo' "$closing_linkage_log" || {
  echo "assertion failed: closing-linkage should delegate directly to adl-pr-closing-linkage without broad wrapper argv" >&2
  exit 1
}

issue_log="$tmpdir/issue.log"
(
  cd "$repo"
  ADL_TEST_LOG="$issue_log" \
    ADL_ISSUE_BIN="$issue_bin" \
    "$BASH_BIN" adl/tools/pr.sh issue search --query "validation manager" --state open --json >/dev/null
)
grep -Fqx 'issue:search --query validation manager --state open --json' "$issue_log" || {
  echo "assertion failed: issue operations should delegate directly to adl-issue without broad 'pr issue' argv" >&2
  exit 1
}

closeout_log="$tmpdir/closeout.log"
(
  cd "$repo"
  ADL_TEST_LOG="$closeout_log" \
    ADL_PR_CLOSEOUT_BIN="$closeout_bin" \
    "$BASH_BIN" adl/tools/pr.sh closeout 3838 --slug demo --no-fetch-issue --version v0.91.5 >/dev/null
)
grep -Fqx 'closeout:3838 --slug demo --no-fetch-issue --version v0.91.5' "$closeout_log" || {
  echo "assertion failed: closeout should delegate directly to adl-pr-closeout without broad 'pr closeout' argv" >&2
  exit 1
}

broad_log="$tmpdir/broad.log"
(
  cd "$repo"
  ADL_TEST_LOG="$broad_log" \
    ADL_PR_RUST_BIN="$broad_bin" \
    ADL_PR_DOCTOR_BIN="$doctor_bin" \
    "$BASH_BIN" adl/tools/pr.sh doctor 3838 --slug demo --no-fetch-issue --version v0.91.5 --mode full >/dev/null
)
grep -Fqx 'broad:pr doctor 3838 --slug demo --no-fetch-issue --version v0.91.5 --mode full' "$broad_log" || {
  echo "assertion failed: ADL_PR_RUST_BIN must retain precedence over direct small-binary overrides" >&2
  exit 1
}

missing_helper_log="$tmpdir/missing-helper.stderr"
rm "$repo/adl/tools/pr_delegate.sh"
set +e
(
  cd "$repo"
  "$BASH_BIN" adl/tools/pr.sh help >/dev/null 2>"$missing_helper_log"
)
status=$?
set -e
[[ "$status" -ne 0 ]] || {
  echo "assertion failed: missing pr.sh helper should fail closed" >&2
  exit 1
}
grep -Fq 'stage=source-helper result=missing helper=pr_delegate.sh' "$missing_helper_log" || {
  echo "assertion failed: missing helper failure should emit observable helper name" >&2
  cat "$missing_helper_log" >&2
  exit 1
}
grep -Fq "missing pr.sh helper: pr_delegate.sh" "$missing_helper_log" || {
  echo "assertion failed: missing helper failure should include actionable helper name" >&2
  cat "$missing_helper_log" >&2
  exit 1
}
if grep -Fq "$repo" "$missing_helper_log"; then
  echo "assertion failed: missing helper failure should not leak host fixture path" >&2
  cat "$missing_helper_log" >&2
  exit 1
fi

echo "pr.sh small-binary delegation: ok"
