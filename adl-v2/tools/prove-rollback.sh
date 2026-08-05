#!/usr/bin/env bash
set -euo pipefail

usage() {
  printf 'usage: %s --manifest PATH\n' "$0" >&2
  exit 64
}

manifest=
while (($#)); do
  case "$1" in
    --manifest)
      (($# >= 2)) || usage
      manifest=$2
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done
[[ -n "$manifest" ]] || usage

root=$(git rev-parse --show-toplevel)
cd "$root"
[[ "$manifest" != /* && "$manifest" != *".."* && -f "$manifest" ]] ||
  { printf 'rollback manifest must be a regular repo-relative path\n' >&2; exit 65; }

jq -e '
  .schema == "adl.wp12.soak_manifest.v1" and
  .issue == 5344 and
  .selector.generations == ["adl-v1-fixture", "adl-v2"] and
  (.selector.timeout_seconds | type == "number" and . >= 1 and . <= 120)
' "$manifest" >/dev/null

target_dir=${ADL_WP12_TARGET_DIR:-${CARGO_TARGET_DIR:-"$root/.adl/target/wp12"}}
revision=$(git rev-parse HEAD)
work_parent=".csdlc/evidence/5344/work"
mkdir -p "$work_parent"
run_root="$work_parent/rollback-$revision"
mkdir "$run_root" || {
  printf 'deterministic rollback scratch root already exists: %s\n' "$run_root" >&2
  exit 66
}
cleanup() {
  if [[ "${ADL_WP12_KEEP_WORK:-0}" != 1 ]]; then
    rm -rf "$run_root"
  fi
}
trap cleanup EXIT
selector_root="$run_root/selector"
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$target_dir" \
  bash adl-v2/tools/install-adl-v2.sh \
  --test-root "$selector_root" >"$run_root/fresh-install.stdout"
adl_v2_bin="$selector_root/bin/adl-v2"
[[ -x "$adl_v2_bin" ]] || { printf 'fresh ADL v2 install unavailable\n' >&2; exit 66; }

sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

install_generation() {
  local generation=$1 source=$2 digest receipt
  cp "$source" "$selector_root/bin/$generation"
  chmod 755 "$selector_root/bin/$generation"
  digest=$(sha256 "$selector_root/bin/$generation")
  receipt="$selector_root/receipts/$generation.json"
  jq -n -c \
    --arg binary "$generation" \
    --arg sha256 "$digest" \
    '{schema:"adl.install.receipt.v1",binary:$binary,sha256:$sha256}' >"$receipt"
  printf '%s' "$digest"
}

# The installer owns the v2 binary and receipt. The v1 fixture is transaction
# evidence only; #5343 owns real v1 restoration.
v2_receipt="$selector_root/receipts/adl-v2.json"
v2_digest=$(sha256 "$adl_v2_bin")
jq -e --arg sha256 "$v2_digest" '
  .schema == "adl.install.receipt.v1" and
  .binary == "adl-v2" and
  .sha256 == $sha256
' "$v2_receipt" >/dev/null
v1_digest=$(install_generation adl-v1-fixture "$adl_v2_bin")

run_cli() {
  "$adl_v2_bin" "$@" --root "$selector_root"
}

selector_digest() {
  sha256 "$selector_root/selector.json"
}

assert_unchanged() {
  local expected=$1 label=$2 observed
  observed=$(selector_digest)
  [[ "$observed" == "$expected" ]] || {
    printf '%s changed selector bytes: expected %s observed %s\n' \
      "$label" "$expected" "$observed" >&2
    exit 67
  }
}

expect_failure_unchanged() {
  local label=$1 expected=$2
  shift 2
  if "$@" >"$run_root/$label.stdout" 2>"$run_root/$label.stderr"; then
    printf '%s unexpectedly succeeded\n' "$label" >&2
    exit 68
  fi
  assert_unchanged "$expected" "$label"
}

# Seed the prior selector only through the authoritative CLI. Selecting v2 and
# then the v1 fixture makes v2 the verified previous generation, so rollback
# returns the selector to byte-identical prior state.
run_cli select adl-v2 >"$run_root/seed-v2.json"
run_cli select adl-v1-fixture \
  --expected-current-digest "$v2_digest" >"$run_root/seed-v1.json"
prior_digest=$(selector_digest)
cp "$selector_root/selector.json" "$run_root/prior-selector.json"

expect_failure_unchanged invalid-generation "$prior_digest" \
  run_cli select ../escape --expected-current-digest "$v1_digest"

cp "$adl_v2_bin" "$selector_root/bin/missing-receipt"
chmod 755 "$selector_root/bin/missing-receipt"
expect_failure_unchanged missing-receipt "$prior_digest" \
  run_cli select missing-receipt --expected-current-digest "$v1_digest"

cp "$adl_v2_bin" "$selector_root/bin/mismatched-receipt"
chmod 755 "$selector_root/bin/mismatched-receipt"
jq -n -c \
  --arg binary mismatched-receipt \
  '{schema:"adl.install.receipt.v1",binary:$binary,sha256:"0000000000000000000000000000000000000000000000000000000000000000"}' \
  >"$selector_root/receipts/mismatched-receipt.json"
expect_failure_unchanged mismatched-receipt "$prior_digest" \
  run_cli select mismatched-receipt --expected-current-digest "$v1_digest"

expect_failure_unchanged stale-cas "$prior_digest" \
  run_cli select adl-v2 \
    --expected-current-digest 0000000000000000000000000000000000000000000000000000000000000000

# A process interrupted while blocked on the selector lock cannot commit.
ruby -e '
  path = ARGV.fetch(0)
  ready = ARGV.fetch(1)
  file = File.open(path, File::RDWR | File::CREAT, 0o600)
  file.flock(File::LOCK_EX)
  File.write(ready, "ready")
  sleep 30
' "$selector_root/selector.lock" "$run_root/lock-ready" &
lock_holder=$!
for _ in {1..100}; do
  [[ -f "$run_root/lock-ready" ]] && break
  sleep 0.05
done
[[ -f "$run_root/lock-ready" ]] || { kill "$lock_holder" 2>/dev/null || true; exit 69; }
"$adl_v2_bin" select adl-v2 --root "$selector_root" \
  --expected-current-digest "$v1_digest" \
  >"$run_root/contended.stdout" 2>"$run_root/contended.stderr" &
blocked_selector=$!
sleep 0.25
kill -TERM "$blocked_selector" 2>/dev/null || true
wait "$blocked_selector" 2>/dev/null || true
kill -TERM "$lock_holder" 2>/dev/null || true
wait "$lock_holder" 2>/dev/null || true
assert_unchanged "$prior_digest" interrupted-before-commit

# Successful opt-in followed by a simulated failed soak must take the explicit
# authoritative rollback path and restore the exact prior bytes.
run_cli select adl-v2 \
  --expected-current-digest "$v1_digest" >"$run_root/opt-in.json"
selected_digest=$(selector_digest)
[[ "$selected_digest" != "$prior_digest" ]] ||
  { printf 'opt-in did not change isolated selector state\n' >&2; exit 70; }

failed_soak_status=42
if [[ "$failed_soak_status" -eq 0 ]]; then
  exit 71
fi
run_cli rollback >"$run_root/rollback.json"
restored_digest=$(selector_digest)
cmp -s "$run_root/prior-selector.json" "$selector_root/selector.json" ||
  {
    printf 'rollback did not restore exact prior selector bytes (work=%s)\n' \
      "$run_root" >&2
    exit 72
  }
[[ "$restored_digest" == "$prior_digest" ]] ||
  { printf 'rollback digest mismatch\n' >&2; exit 72; }

selector=$(run_cli inspect)
jq -e '
  .ok == true and
  .result.current.generation == "adl-v1-fixture" and
  .result.previous.generation == "adl-v2"
' <<<"$selector" >/dev/null

receipt_path="$root/.csdlc/evidence/5344/rollback/fresh-install-receipt.json"
mkdir -p "$(dirname "$receipt_path")"
receipt_tmp=$(mktemp "$(dirname "$receipt_path")/.fresh-install-receipt.XXXXXX")
cp "$v2_receipt" "$receipt_tmp"
chmod 644 "$receipt_tmp"
mv -f "$receipt_tmp" "$receipt_path"
fresh_install_receipt_sha256=$(sha256 "$receipt_path")

jq -n -c \
  --arg revision "$revision" \
  --arg prior_sha256 "$prior_digest" \
  --arg selected_sha256 "$selected_digest" \
  --arg restored_sha256 "$restored_digest" \
  --arg v1_fixture_sha256 "$v1_digest" \
  --arg v2_sha256 "$v2_digest" \
  --arg fresh_install_receipt_ref ".csdlc/evidence/5344/rollback/fresh-install-receipt.json" \
  --arg fresh_install_receipt_sha256 "$fresh_install_receipt_sha256" \
  '{
    schema:"adl.wp12.rollback_report.v1",
    issue:5344,
    status:"pass",
    revision:$revision,
    selector_root:"isolated_ephemeral",
    prior_sha256:$prior_sha256,
    selected_sha256:$selected_sha256,
    restored_sha256:$restored_sha256,
    v1_fixture_sha256:$v1_fixture_sha256,
    v2_sha256:$v2_sha256,
    fresh_install_receipt_ref:$fresh_install_receipt_ref,
    fresh_install_receipt_sha256:$fresh_install_receipt_sha256,
    exact_prior_bytes_restored:true,
    authoritative_cli_only:true,
    cases:[
      "invalid_generation",
      "missing_receipt",
      "mismatched_receipt",
      "stale_compare_and_swap",
      "interrupted_before_commit",
      "successful_opt_in",
      "failed_soak_explicit_rollback"
    ],
    boundary:"The v1 fixture proves selector transaction restoration only; #5343 owns real legacy executable restoration."
  }'
