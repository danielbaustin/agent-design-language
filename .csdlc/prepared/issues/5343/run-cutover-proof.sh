#!/usr/bin/env bash
set -euo pipefail

usage() {
  printf 'usage: %s --v1-binary ABSOLUTE_PATH [--output REPO_RELATIVE_PATH]\n' "$0" >&2
  exit 64
}

v1_source=
output=docs/milestones/v0.91.8/evidence/wp12/cutover-5343/report.json
while (($#)); do
  case "$1" in
    --v1-binary)
      (($# >= 2)) || usage
      v1_source=$2
      shift 2
      ;;
    --output)
      (($# >= 2)) || usage
      output=$2
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done

[[ "$v1_source" == /* && -x "$v1_source" ]] ||
  { printf 'v1 binary must be an executable absolute path\n' >&2; exit 65; }
[[ "$output" != /* && "$output" != *".."* ]] ||
  { printf 'output must be a repo-relative path without parent traversal\n' >&2; exit 65; }

root=$(git rev-parse --show-toplevel)
cd "$root"
ruby .csdlc/prepared/issues/5343/check-dependencies.rb >/dev/null

sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

work_parent=.csdlc/evidence/5343/work
mkdir -p "$work_parent"
run_root=$(mktemp -d "$work_parent/cutover.XXXXXX")
cleanup() {
  if [[ "${ADL_WP12_KEEP_WORK:-0}" != 1 ]]; then
    rm -rf "$run_root"
  fi
}
trap cleanup EXIT

selector_root="$run_root/selector"
target_dir=${ADL_WP12_TARGET_DIR:-${CARGO_TARGET_DIR:-$root/target}}
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$target_dir" \
  bash adl-v2/tools/install-adl-v2.sh --test-root "$selector_root" \
  >"$run_root/install.stdout"

adl_v2="$selector_root/bin/adl-v2"
v2_receipt="$selector_root/receipts/adl-v2.json"
[[ -x "$adl_v2" && -f "$v2_receipt" ]] ||
  { printf 'fresh ADL v2 installation is incomplete\n' >&2; exit 66; }

v1_source_digest=$(sha256 "$v1_source")
v1_source_version=$("$v1_source" --version)
cp "$v1_source" "$selector_root/bin/adl-v1"
chmod 755 "$selector_root/bin/adl-v1"
v1_copy_digest=$(sha256 "$selector_root/bin/adl-v1")
[[ "$v1_copy_digest" == "$v1_source_digest" ]] ||
  { printf 'isolated v1 copy is not byte-identical\n' >&2; exit 66; }
jq -n -c --arg sha256 "$v1_copy_digest" \
  '{schema:"adl.install.receipt.v1",binary:"adl-v1",sha256:$sha256}' \
  >"$selector_root/receipts/adl-v1.json"

v2_digest=$(sha256 "$adl_v2")
jq -e --arg sha256 "$v2_digest" '
  .schema == "adl.install.receipt.v1" and
  .binary == "adl-v2" and
  .sha256 == $sha256
' "$v2_receipt" >/dev/null

run_cli() {
  "$adl_v2" "$@" --root "$selector_root"
}

selector_digest() {
  sha256 "$selector_root/selector.json"
}

assert_selector_unchanged() {
  local expected=$1 label=$2 observed
  observed=$(selector_digest)
  [[ "$observed" == "$expected" ]] ||
    { printf '%s changed selector bytes\n' "$label" >&2; exit 67; }
}

expect_failure_unchanged() {
  local label=$1 expected=$2
  shift 2
  if "$@" >"$run_root/$label.stdout" 2>"$run_root/$label.stderr"; then
    printf '%s unexpectedly succeeded\n' "$label" >&2
    exit 68
  fi
  assert_selector_unchanged "$expected" "$label"
}

# Establish the exact prior v1 default with v2 retained as the verified
# previous generation. This makes an actual rollback byte-for-byte reversible.
run_cli select adl-v2 >"$run_root/seed-v2.json"
run_cli select adl-v1 --expected-current-digest "$v2_digest" \
  >"$run_root/seed-v1.json"
prior_selector_sha256=$(selector_digest)
cp "$selector_root/selector.json" "$run_root/prior-selector.json"
isolated_v1_version=$("$selector_root/bin/adl-v1" --version)
[[ "$isolated_v1_version" == "$v1_source_version" ]] ||
  { printf 'isolated v1 executable identity mismatch\n' >&2; exit 69; }

expect_failure_unchanged invalid-generation "$prior_selector_sha256" \
  run_cli select ../escape --expected-current-digest "$v1_copy_digest"

cp "$adl_v2" "$selector_root/bin/missing-receipt"
chmod 755 "$selector_root/bin/missing-receipt"
expect_failure_unchanged missing-receipt "$prior_selector_sha256" \
  run_cli select missing-receipt --expected-current-digest "$v1_copy_digest"

cp "$adl_v2" "$selector_root/bin/wrong-digest"
chmod 755 "$selector_root/bin/wrong-digest"
jq -n -c \
  '{schema:"adl.install.receipt.v1",binary:"wrong-digest",sha256:"0000000000000000000000000000000000000000000000000000000000000000"}' \
  >"$selector_root/receipts/wrong-digest.json"
expect_failure_unchanged wrong-digest "$prior_selector_sha256" \
  run_cli select wrong-digest --expected-current-digest "$v1_copy_digest"

expect_failure_unchanged stale-cas "$prior_selector_sha256" \
  run_cli select adl-v2 \
  --expected-current-digest 0000000000000000000000000000000000000000000000000000000000000000

malformed_root="$run_root/malformed-selector"
cp -R "$selector_root" "$malformed_root"
printf '{"schema":"adl.selector.v0","current":null,"previous":null}\n' \
  >"$malformed_root/selector.json"
malformed_selector_sha256=$(sha256 "$malformed_root/selector.json")
if "$adl_v2" select adl-v2 --root "$malformed_root" \
  >"$run_root/malformed.stdout" 2>"$run_root/malformed.stderr"; then
  printf 'malformed selector unexpectedly succeeded\n' >&2
  exit 68
fi
[[ "$(sha256 "$malformed_root/selector.json")" == "$malformed_selector_sha256" ]] ||
  { printf 'malformed selector transaction changed selector bytes\n' >&2; exit 67; }
assert_selector_unchanged "$prior_selector_sha256" malformed-selector-isolation

# A contender interrupted while blocked on the authoritative lock cannot
# commit. The active selector must remain byte-identical.
ruby -e '
  path, ready = ARGV
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
[[ -f "$run_root/lock-ready" ]] ||
  { kill "$lock_holder" 2>/dev/null || true; exit 69; }
run_cli select adl-v2 --expected-current-digest "$v1_copy_digest" \
  >"$run_root/contended.stdout" 2>"$run_root/contended.stderr" &
contender=$!
sleep 0.25
kill -TERM "$contender" 2>/dev/null || true
wait "$contender" 2>/dev/null || true
kill -TERM "$lock_holder" 2>/dev/null || true
wait "$lock_holder" 2>/dev/null || true
assert_selector_unchanged "$prior_selector_sha256" interrupted-contender

# A persistence precondition failure also leaves the prior selector untouched.
chmod 500 "$selector_root"
set +e
run_cli select adl-v2 --expected-current-digest "$v1_copy_digest" \
  >"$run_root/unwritable.stdout" 2>"$run_root/unwritable.stderr"
unwritable_status=$?
set -e
chmod 700 "$selector_root"
[[ "$unwritable_status" -ne 0 ]] ||
  { printf 'unwritable selector root unexpectedly succeeded\n' >&2; exit 68; }
assert_selector_unchanged "$prior_selector_sha256" persistence-failure

# Perform the real cutover, execute v2, roll back and execute v1, then select
# and execute v2 as the final default while retaining v1.
run_cli select adl-v2 --expected-current-digest "$v1_copy_digest" \
  >"$run_root/select-v2.json"
selected_selector_sha256=$(selector_digest)
v2_version=$("$adl_v2" --version)

run_cli rollback >"$run_root/rollback-v1.json"
restored_selector_sha256=$(selector_digest)
cmp -s "$run_root/prior-selector.json" "$selector_root/selector.json" ||
  { printf 'rollback did not restore exact prior selector bytes\n' >&2; exit 70; }
[[ "$restored_selector_sha256" == "$prior_selector_sha256" ]] ||
  { printf 'rollback selector digest mismatch\n' >&2; exit 70; }
restored_v1_version=$("$selector_root/bin/adl-v1" --version)
[[ "$restored_v1_version" == "$v1_source_version" ]] ||
  { printf 'restored v1 execution mismatch\n' >&2; exit 70; }

cutover_started_at=$(ruby -rtime -e 'puts Time.now.utc.iso8601')
cutover_start_epoch=$(date +%s)
run_cli select adl-v2 --expected-current-digest "$v1_copy_digest" \
  >"$run_root/final-select-v2.json"
final_selector_sha256=$(selector_digest)
final_v2_version=$("$adl_v2" --version)
final_state=$(run_cli inspect)
jq -e '
  .ok == true and
  .result.current.generation == "adl-v2" and
  .result.previous.generation == "adl-v1"
' <<<"$final_state" >/dev/null

[[ "$(sha256 "$v1_source")" == "$v1_source_digest" ]] ||
  { printf 'source v1 installation changed during proof\n' >&2; exit 71; }
[[ -x "$v1_source" && -x "$selector_root/bin/adl-v1" ]] ||
  { printf 'v1 executable was not retained\n' >&2; exit 71; }

rollback_end_epoch=$((cutover_start_epoch + 14 * 24 * 60 * 60))
rollback_ends_at=$(ruby -rtime -e 'puts Time.at(ARGV.fetch(0).to_i).utc.iso8601' "$rollback_end_epoch")
revision=$(git rev-parse HEAD)
mkdir -p "$(dirname "$output")"
report_tmp=$(mktemp "$(dirname "$output")/.cutover-report.XXXXXX")
jq -n \
  --arg revision "$revision" \
  --arg prior "$prior_selector_sha256" \
  --arg selected "$selected_selector_sha256" \
  --arg restored "$restored_selector_sha256" \
  --arg final "$final_selector_sha256" \
  --arg v1_sha256 "$v1_source_digest" \
  --arg v2_sha256 "$v2_digest" \
  --arg v1_version "$v1_source_version" \
  --arg v2_version "$v2_version" \
  --arg final_v2_version "$final_v2_version" \
  --arg started_at "$cutover_started_at" \
  --arg ends_at "$rollback_ends_at" \
  '{
    schema:"adl.wp12.cutover_report.v1",
    issue:5343,
    status:"pass",
    revision:$revision,
    selector_root:"isolated_issue_evidence",
    prior_selector_sha256:$prior,
    selected_selector_sha256:$selected,
    restored_selector_sha256:$restored,
    final_selector_sha256:$final,
    exact_prior_bytes_restored:($prior == $restored),
    final_default:"adl-v2",
    retained_previous:"adl-v1",
    v1:{sha256:$v1_sha256,version:$v1_version,source_unchanged:true,isolated_copy_byte_identical:true,executed_before_cutover:true,executed_after_rollback:true},
    v2:{sha256:$v2_sha256,version:$v2_version,final_version:$final_v2_version,fresh_install:true,executed:true},
    rollback_window:{duration_days:14,starts_at:$started_at,ends_at:$ends_at,deletion_authorized:false},
    failure_preservation:[
      "invalid_generation",
      "missing_receipt",
      "wrong_digest",
      "stale_compare_and_swap",
      "malformed_selector",
      "lock_contention_interruption",
      "persistence_precondition_failure"
    ],
    runtime_v2_edited:false,
    legacy_deleted:false,
    selector_implementation_reused:true
  }' >"$report_tmp"
chmod 644 "$report_tmp"
mv -f "$report_tmp" "$output"
jq -e '
  .status == "pass" and
  .exact_prior_bytes_restored == true and
  .final_default == "adl-v2" and
  .retained_previous == "adl-v1" and
  .v1.source_unchanged == true and
  .v1.executed_after_rollback == true and
  .v2.fresh_install == true and
  .runtime_v2_edited == false and
  .legacy_deleted == false
' "$output" >/dev/null
cat "$output"
