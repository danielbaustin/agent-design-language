#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

[[ ! -e "$ROOT/adl/tools/check_milestone_closed_issue_sor_truth.sh" ]] || {
  echo "retired milestone SOR-truth helper unexpectedly exists" >&2
  exit 1
}
[[ ! -e "$ROOT/.github/workflows/v0871_milestone_closeout_gate.yaml" ]] || {
  echo "obsolete v0.87.1 closeout workflow unexpectedly exists" >&2
  exit 1
}

grep -Fq 'csdlc-doctor' "$ROOT/adl/tools/release_ceremony.sh"
grep -Fq 'closed_out' "$ROOT/adl/tools/release_ceremony.sh"
if grep -Fq 'check_milestone_closed_issue_sor_truth.sh' "$ROOT/adl/tools/release_ceremony.sh"; then
  echo "release ceremony still delegates to the retired milestone helper" >&2
  exit 1
fi

echo "PASS test_check_milestone_closed_issue_sor_truth"
