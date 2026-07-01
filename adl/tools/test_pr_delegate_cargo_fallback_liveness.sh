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
mkdir -p "$repo/adl/tools" "$mockbin" "$repo/adl/target"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$PR_DELEGATE_SRC" "$repo/adl/tools/pr_delegate.sh"
cp "$PR_USAGE_SRC" "$repo/adl/tools/pr_usage.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$OBS_SRC" "$repo/adl/tools/observability.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "seed" > README.md
  git add README.md
  git commit -q -m "init"
)

cat >"$mockbin/cargo" <<'EOF_CARGO'
#!/usr/bin/env bash
set -euo pipefail
mode="${ADL_TEST_CARGO_MODE:-heartbeat}"
printf '%s\n' "$*" >"${ADL_TEST_CARGO_ARGS}"
case "$mode" in
  heartbeat)
    sleep 0.15
    ;;
  timeout)
    trap 'exit 88' TERM INT
    sleep 30
    ;;
  *)
    echo "unknown ADL_TEST_CARGO_MODE=$mode" >&2
    exit 97
    ;;
esac
EOF_CARGO
chmod +x "$mockbin/cargo"

run_from_repo() {
  (
    cd "$repo"
    PATH="$mockbin:/usr/bin:/bin:/usr/sbin:/sbin" "$@"
  )
}

default_fail_log="$tmpdir/default-fail.log"
default_fail_args="$tmpdir/default-fail.args"
: >"$default_fail_args"
set +e
run_from_repo \
  env \
    ADL_OBSERVABILITY_LOG="$default_fail_log" \
    ADL_OBSERVABILITY_STDERR=0 \
    ADL_TEST_CARGO_MODE=heartbeat \
    ADL_TEST_CARGO_ARGS="$default_fail_args" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug demo --no-fetch-issue --version v0.91.6 --mode full >/dev/null 2>"$tmpdir/default-fail.stderr"
default_fail_status="$?"
set -e
[[ "$default_fail_status" == "75" ]] || {
  echo "assertion failed: missing owner binary should fail closed with exit 75, got $default_fail_status" >&2
  cat "$tmpdir/default-fail.stderr" >&2
  exit 1
}
[[ ! -s "$default_fail_args" ]] || {
  echo "assertion failed: cargo must not run when fallback is not explicitly enabled" >&2
  cat "$default_fail_args" >&2
  exit 1
}
grep -F "reason_code=missing_owner_binary_cargo_fallback_disabled" "$default_fail_log" >/dev/null || {
  echo "assertion failed: missing owner binary should emit a classified observability event" >&2
  cat "$default_fail_log" >&2
  exit 1
}
grep -F "ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1" "$tmpdir/default-fail.stderr" >/dev/null || {
  echo "assertion failed: missing owner binary diagnostic should explain the explicit fallback opt-in" >&2
  cat "$tmpdir/default-fail.stderr" >&2
  exit 1
}

heartbeat_log="$tmpdir/heartbeat.log"
heartbeat_args="$tmpdir/heartbeat.args"
run_from_repo \
  env \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    ADL_OBSERVABILITY_LOG="$heartbeat_log" \
    ADL_OBSERVABILITY_STDERR=0 \
    ADL_OBSERVABILITY_HEARTBEAT_MS=25 \
    ADL_TEST_CARGO_MODE=heartbeat \
    ADL_TEST_CARGO_ARGS="$heartbeat_args" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug demo --no-fetch-issue --version v0.91.6 --mode full >/dev/null
grep -F "stage=rust_delegate result=heartbeat" "$heartbeat_log" >/dev/null || {
  echo "assertion failed: cargo delegate fallback should emit heartbeat events" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug demo --no-fetch-issue --version v0.91.6 --mode full" "$heartbeat_args" >/dev/null || {
  echo "assertion failed: doctor fallback should execute the direct small binary through cargo" >&2
  exit 1
}

issue_log="$tmpdir/issue.log"
issue_args="$tmpdir/issue.args"
run_from_repo \
  env \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    ADL_OBSERVABILITY_LOG="$issue_log" \
    ADL_OBSERVABILITY_STDERR=0 \
    ADL_OBSERVABILITY_HEARTBEAT_MS=25 \
    ADL_TEST_CARGO_MODE=heartbeat \
    ADL_TEST_CARGO_ARGS="$issue_args" \
    "$BASH_BIN" adl/tools/pr.sh issue search --query "validation manager" --state open --json >/dev/null
grep -F -- "--bin adl-issue -- search --query validation manager --state open --json" "$issue_args" >/dev/null || {
  echo "assertion failed: issue fallback should execute the adl-issue binary through cargo" >&2
  exit 1
}

timeout_log="$tmpdir/timeout.log"
timeout_args="$tmpdir/timeout.args"
set +e
run_from_repo \
  env \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    ADL_OBSERVABILITY_LOG="$timeout_log" \
    ADL_OBSERVABILITY_STDERR=0 \
    ADL_OBSERVABILITY_HEARTBEAT_MS=25 \
    ADL_PR_CARGO_DELEGATE_TIMEOUT_SECS=1 \
    ADL_TEST_CARGO_MODE=timeout \
    ADL_TEST_CARGO_ARGS="$timeout_args" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug demo --no-fetch-issue --version v0.91.6 --mode full >/dev/null
timeout_status="$?"
set -e
[[ "$timeout_status" == "124" ]] || {
  echo "assertion failed: cargo delegate timeout should exit 124, got $timeout_status" >&2
  exit 1
}
grep -F "stage=rust_delegate result=timeout" "$timeout_log" >/dev/null || {
  echo "assertion failed: cargo delegate timeout should emit timeout event" >&2
  exit 1
}
grep -F "reason_code=cargo_delegate_timeout" "$timeout_log" >/dev/null || {
  echo "assertion failed: cargo delegate timeout should classify timeout reason" >&2
  exit 1
}

lock_log="$tmpdir/lock.log"
lock_dir="$repo/adl/target/.adl-pr-rust-delegate-build.lock"
mkdir -p "$lock_dir"
printf '%s\n' "$$" >"$lock_dir/owner_pid"
date +%s >"$lock_dir/created_at_epoch"
printf '%s\n' "doctor" >"$lock_dir/subcommand"
printf '%s\n' "adl-pr-doctor" >"$lock_dir/delegate_bin"
set +e
run_from_repo \
  env \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    ADL_OBSERVABILITY_LOG="$lock_log" \
    ADL_OBSERVABILITY_STDERR=0 \
    ADL_PR_CARGO_DELEGATE_BUILD_LOCK_TIMEOUT_SECS=0 \
    ADL_TEST_CARGO_MODE=heartbeat \
    ADL_TEST_CARGO_ARGS="$tmpdir/lock.args" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug demo --no-fetch-issue --version v0.91.6 --mode full >/dev/null
lock_status="$?"
set -e
rm -f "$lock_dir/owner_pid" "$lock_dir/created_at_epoch" "$lock_dir/subcommand" "$lock_dir/delegate_bin"
rmdir "$lock_dir"
[[ "$lock_status" == "75" ]] || {
  echo "assertion failed: build-lock timeout should exit 75, got $lock_status" >&2
  exit 1
}
grep -F "stage=rust_delegate_wait result=timeout" "$lock_log" >/dev/null || {
  echo "assertion failed: build-lock timeout should emit wait-timeout event" >&2
  exit 1
}
grep -F "reason_code=build_lock_timeout" "$lock_log" >/dev/null || {
  echo "assertion failed: build-lock timeout should classify lock timeout reason" >&2
  exit 1
}
grep -F "recovery_hint=run_adl/tools/run_owner_validation_lane.sh_csdlc_--build" "$lock_log" >/dev/null || {
  echo "assertion failed: build-lock timeout should point to the delegate binary build helper" >&2
  exit 1
}
grep -F "lock_owner_pid=$$" "$lock_log" >/dev/null || {
  echo "assertion failed: build-lock timeout should report active owner pid" >&2
  exit 1
}

stale_log="$tmpdir/stale.log"
stale_args="$tmpdir/stale.args"
mkdir -p "$lock_dir"
printf '%s\n' "999999" >"$lock_dir/owner_pid"
date +%s >"$lock_dir/created_at_epoch"
printf '%s\n' "finish" >"$lock_dir/subcommand"
printf '%s\n' "adl-pr-finish" >"$lock_dir/delegate_bin"
run_from_repo \
  env \
    ADL_PR_RUST_ALLOW_CARGO_FALLBACK=1 \
    ADL_OBSERVABILITY_LOG="$stale_log" \
    ADL_OBSERVABILITY_STDERR=0 \
    ADL_OBSERVABILITY_HEARTBEAT_MS=25 \
    ADL_PR_CARGO_DELEGATE_BUILD_LOCK_TIMEOUT_SECS=0 \
    ADL_TEST_CARGO_MODE=heartbeat \
    ADL_TEST_CARGO_ARGS="$stale_args" \
    "$BASH_BIN" adl/tools/pr.sh doctor 4413 --slug demo --no-fetch-issue --version v0.91.6 --mode full >/dev/null
grep -F "reason_code=stale_build_lock_recovered" "$stale_log" >/dev/null || {
  echo "assertion failed: stale build lock should be recovered before cargo fallback" >&2
  exit 1
}
grep -F "lock_owner_pid=999999" "$stale_log" >/dev/null || {
  echo "assertion failed: stale build-lock recovery should report dead owner pid" >&2
  exit 1
}
grep -F -- "--bin adl-pr-doctor -- 4413 --slug demo --no-fetch-issue --version v0.91.6 --mode full" "$stale_args" >/dev/null || {
  echo "assertion failed: stale build-lock recovery should continue into cargo fallback" >&2
  exit 1
}
[[ ! -d "$lock_dir" ]] || {
  echo "assertion failed: recovered cargo fallback should remove the build lock" >&2
  exit 1
}

echo "pr.sh cargo fallback liveness: ok"
