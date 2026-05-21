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

test_speculative_showcase() {
  local report="$TMP/speculative-demo-report.json"
  local out
  out="$(bash "$ROOT/adl/tools/demo_v0912_speculative_decoding_showcase.sh" --out "$report")"
  assert_contains "WP-11 speculative decoding showcase" "$out" "speculative showcase header"
  assert_contains "worthwhile_for_adl: true" "$out" "speculative worthiness"
  assert_contains "same_family_code_generation" "$out" "speculative scenario listing"
  assert_contains "cross_family_tokenizer_mismatch" "$out" "speculative mismatch listing"
  [[ -f "$report" ]] || { echo "missing speculative demo report" >&2; exit 1; }
}

test_workflow_guardrails_showcase() {
  local out
  out="$(bash "$ROOT/adl/tools/demo_v0912_workflow_guardrails_showcase.sh")"
  assert_contains "WP-16 workflow guardrails showcase" "$out" "workflow showcase header"
  assert_contains "main_write_clean: PASS main-write branch=main clean=true" "$out" "workflow clean main"
  assert_contains "main_write_dirty: BLOCKED main-write branch=main clean=false" "$out" "workflow dirty main"
  assert_contains "safe_report_command_blocked: BLOCKED safe-report-command" "$out" "workflow unsafe report"
  assert_contains "safe_report_command_pass: PASS safe-report-command" "$out" "workflow safe report"
  assert_contains "card_drift_invocation: doctor 3015 --version v0.91.2 --mode full --slug v0-91-2-wp-16-workflow-guardrails-hardening" "$out" "workflow card drift"
}

test_speculative_showcase
test_workflow_guardrails_showcase

echo "PASS test_wp17a_demo_follow_ons"
