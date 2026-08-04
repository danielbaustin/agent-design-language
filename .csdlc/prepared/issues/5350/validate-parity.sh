#!/usr/bin/env bash
set -euo pipefail

mode="${1:-}"
case "$mode" in
  subjects|compare|overlays|complete) ;;
  *) printf 'usage: %s subjects|compare|overlays|complete\n' "$0" >&2; exit 64 ;;
esac

root="$(git rev-parse --show-toplevel)"
characterize="${ADL_CHARACTERIZE_BINARY:?set ADL_CHARACTERIZE_BINARY to the reviewed adl-characterize executable}"
candidate="${ADL_V2_BINARY:?set ADL_V2_BINARY to the reviewed adl-v2 executable}"
receipt_root="${ADL_CSDLC_RECEIPT_ROOT:-$(git rev-parse --git-common-dir)/csdlc-v2/closeout}"
report="${ADL_SHADOW_REPORT:-$root/.csdlc/evidence/5350/shadow-report.json}"
corpus="$root/adl-characterization/corpus/v1/corpus.yaml"
observations="$root/adl-characterization/observations/v1"
manifest="$root/adl-characterization/corpus/v2/shadow.yaml"
work_root="${ADL_SHADOW_WORK_ROOT:-/Volumes/FastWork/adl-5350-shadow-work}"
runtime_plan="$root/docs/milestones/v0.91.8/runtime_v3_functional_parity_plan_v0.91.8.json"
lockfile="$root/adl-v2/Cargo.lock"
install_receipt="$root/.csdlc/evidence/5350/adl-v2-install/receipts/adl-v2.json"
selector="$root/.csdlc/evidence/5350/adl-v2-install/selector.json"

test -x "$characterize"
test -x "$candidate"

verify_subjects() {
  "$characterize" verify \
    --corpus "$corpus" \
    --observations "$observations" >/dev/null
  local expected_binary expected_lock
  expected_binary="$(ruby -ryaml -e 'puts YAML.safe_load(File.read(ARGV.fetch(0))).fetch("candidate_binary_sha256")' "$manifest")"
  expected_lock="$(ruby -ryaml -e 'puts YAML.safe_load(File.read(ARGV.fetch(0))).fetch("candidate_lock_sha256")' "$manifest")"
  test "$(shasum -a 256 "$candidate" | awk '{print $1}')" = "$expected_binary"
  test "$(shasum -a 256 "$lockfile" | awk '{print $1}')" = "$expected_lock"
}

run_shadow() {
  local output="$1"
  "$characterize" shadow \
    --binary "$candidate" \
    --lockfile "$lockfile" \
    --install-receipt "$install_receipt" \
    --selector "$selector" \
    --repo-root "$root" \
    --receipt-root "$receipt_root" \
    --runtime-plan "$runtime_plan" \
    --corpus "$corpus" \
    --observations "$observations" \
    --work-root "$work_root" \
    --manifest "$manifest" \
    --report "$output" >/dev/null
}

verify_report() {
  ruby -rjson -e '
    report = JSON.parse(File.read(ARGV.fetch(0)))
    abort "shadow status is not pass" unless report["status"] == "pass"
    abort "case count drift" unless report["case_count"] == 25
    abort "behavior count drift" unless report["behavior_count"] == 23
    abort "candidate observation count drift" unless report["rows"].sum { |row| row["candidate_observation_count"] } == 75
    abort "incumbent observation count drift" unless report["rows"].sum { |row| row["incumbent_observation_count"] } == 75
    abort "unclassified or blocking case" unless report["rows"].all? { |row| %w[exact_match normalized_match approved_intentional_difference].include?(row["disposition"]) }
    abort "runtime group drift" unless report["runtime_overlay"].flat_map { |row| row["groups"] }.sort == (1..10).to_a
    abort "runtime overlay blocker" unless report["runtime_overlay"].all? { |row| row["status"] == "pass" }
    abort "adapter overlay blocker" unless report["adapter_overlay"].all? { |row| row["status"] == "pass" }
    abort "WP-10A overlay blocker" unless report["wp10a_overlay"].all? { |row| row["status"] == "pass" }
    abort "live evidence missing" unless report["wp10a_overlay"].find { |row| row["issue"] == 5501 }["evidence_sha256"]
  ' "$1"
}

case "$mode" in
  subjects)
    verify_subjects
    ;;
  compare)
    verify_subjects
    run_shadow "$report"
    verify_report "$report"
    ;;
  overlays)
    test -f "$report"
    verify_report "$report"
    ;;
  complete)
    verify_subjects
    mkdir -p "$(dirname "$report")"
    first="$(mktemp /Volumes/FastWork/adl-5350-shadow-first.XXXXXX)"
    second="$(mktemp /Volumes/FastWork/adl-5350-shadow-second.XXXXXX)"
    trap 'rm -f "$first" "$second"' EXIT
    run_shadow "$first"
    run_shadow "$second"
    cmp "$first" "$second"
    cp "$first" "$report"
    verify_report "$report"
    ;;
esac

printf 'issue=5350 mode=%s status=pass cases=25 observations=75+75 behaviors=23 runtime_groups=10\n' "$mode"
