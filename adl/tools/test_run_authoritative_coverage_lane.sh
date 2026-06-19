#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEFAULT_BUILD_ROOT="$(bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" --print-plan | awk -F= '/^build_root=/{print $2}')"

plan="$(bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" --print-plan)"

for required in \
  "authority=push_main" \
  "event_name=push" \
  "mode=full_authoritative_default_features" \
  "build_root=$DEFAULT_BUILD_ROOT" \
  "features=default" \
  "workspace=full"
do
  if ! grep -F "$required" <<<"$plan" >/dev/null 2>&1; then
    echo "missing authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

policy_plan="$(bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" --authority pr_policy_surface_tooling_only --event-name pull_request --print-plan)"

for required in \
  "authority=pr_policy_surface_tooling_only" \
  "event_name=pull_request" \
  "mode=bounded_policy_surface_pr" \
  "build_root=$DEFAULT_BUILD_ROOT" \
  "features=default" \
  "workspace=bounded_policy_surface"
do
  if ! grep -F "$required" <<<"$policy_plan" >/dev/null 2>&1; then
    echo "missing policy-surface authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

runtime_policy_plan="$(bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" --authority pr_policy_surface_runtime_mixed --event-name pull_request --print-plan)"

for required in \
  "authority=pr_policy_surface_runtime_mixed" \
  "event_name=pull_request" \
  "mode=full_authoritative_default_features" \
  "build_root=$DEFAULT_BUILD_ROOT" \
  "features=default" \
  "workspace=full"
do
  if ! grep -F "$required" <<<"$runtime_policy_plan" >/dev/null 2>&1; then
    echo "missing mixed policy-surface authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

temp_root="$(mktemp -d)"
trap 'rm -rf "$temp_root"' EXIT
bin_dir="$temp_root/bin"
mkdir -p "$bin_dir"
scratch_root="$temp_root/scratch"
cargo_log="$temp_root/cargo.log"
cat >"$bin_dir/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'cmd=%s\n' "$*" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'target=%s\n' "${CARGO_TARGET_DIR:-}" >> "$AUTHORITATIVE_CARGO_LOG"
printf 'llvm_cov_target=%s\n' "${CARGO_LLVM_COV_TARGET_DIR:-}" >> "$AUTHORITATIVE_CARGO_LOG"
exit 0
EOF
chmod +x "$bin_dir/cargo"

PATH="$bin_dir:$PATH" \
AUTHORITATIVE_CARGO_LOG="$cargo_log" \
ADL_COVERAGE_BUILD_ROOT="$scratch_root" \
  bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" \
    --authority pr_policy_surface_tooling_only \
    --event-name pull_request

for required_dir in "$scratch_root/target" "$scratch_root/llvm-cov-target"; do
  if [ ! -d "$required_dir" ]; then
    echo "expected authoritative coverage scratch dir: $required_dir" >&2
    exit 1
  fi
done

for required in \
  "cmd=llvm-cov nextest --workspace --status-level all --final-status-level slow --no-report" \
  "cmd=llvm-cov report --json --summary-only --output-path coverage-summary.json" \
  "target=$scratch_root/target" \
  "llvm_cov_target=$scratch_root/llvm-cov-target"
do
  if ! grep -F "$required" "$cargo_log" >/dev/null 2>&1; then
    echo "missing authoritative coverage execution token: $required" >&2
    cat "$cargo_log" >&2
    exit 1
  fi
done

echo "PASS test_run_authoritative_coverage_lane"
