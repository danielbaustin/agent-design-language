#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VALIDATOR="$ROOT_DIR/adl/tools/validate_structured_prompt.sh"
EXAMPLES_DIR="$ROOT_DIR/docs/tooling/csdlc-prompt-editor/repair_examples"

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/adl-card-editor-repair-examples.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

"$VALIDATOR" --type sip --phase bootstrap --input "$EXAMPLES_DIR/sip_repaired_pre_run.md"
"$VALIDATOR" --type stp --input "$EXAMPLES_DIR/stp_repaired_issue_ready.md"
"$VALIDATOR" --type spp --input "$EXAMPLES_DIR/spp_repaired_issue_plan.md"
"$VALIDATOR" --type srp --input "$EXAMPLES_DIR/srp_repaired_review_truth.md"
"$VALIDATOR" --type sor --phase final --input "$EXAMPLES_DIR/sor_repaired_pr_open.md"

cp "$EXAMPLES_DIR/stp_repaired_issue_ready.md" "$tmpdir/stp_invalid_card_status.md"
perl -0pi -e 's/card_status: "ready"/card_status: "nearly-ready"/' "$tmpdir/stp_invalid_card_status.md"
if "$VALIDATOR" --type stp --input "$tmpdir/stp_invalid_card_status.md" >"$tmpdir/stp_invalid_card_status.out" 2>&1; then
  echo "expected repaired STP with invalid card_status mutation to fail validation" >&2
  exit 1
fi
grep -Fq "card_status must be one of: draft, ready, reviewed, approved, completed, blocked, superseded" "$tmpdir/stp_invalid_card_status.out"

cp "$EXAMPLES_DIR/srp_repaired_review_truth.md" "$tmpdir/srp_legacy_policy.md"
perl -0pi -e 's/artifact_type: "structured_review_prompt"/artifact_type: "structured_review_policy"/; s/# Structured Review Prompt/# Structured Review Policy/' "$tmpdir/srp_legacy_policy.md"
if "$VALIDATOR" --type srp --input "$tmpdir/srp_legacy_policy.md" >"$tmpdir/srp_legacy_policy.out" 2>&1; then
  echo "expected legacy SRP policy mutation to fail validation" >&2
  exit 1
fi
grep -Fq "artifact_type must be structured_review_prompt; legacy structured_review_policy scaffolds must be routed through srp-editor before validation passes" "$tmpdir/srp_legacy_policy.out"

cp "$EXAMPLES_DIR/sor_repaired_pr_open.md" "$tmpdir/sor_completed_without_closeout.md"
perl -0pi -e 's/Card Status: ready/Card Status: completed/' "$tmpdir/sor_completed_without_closeout.md"
if "$VALIDATOR" --type sor --phase final --input "$tmpdir/sor_completed_without_closeout.md" >"$tmpdir/sor_completed_without_closeout.out" 2>&1; then
  echo "expected completed SOR without terminal closeout to fail validation" >&2
  exit 1
fi
grep -Fq "Card Status completed requires terminal Integration state: merged or closed_no_pr" "$tmpdir/sor_completed_without_closeout.out"

echo "PASS test_card_editor_repair_examples"
