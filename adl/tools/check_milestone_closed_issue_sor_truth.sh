#!/usr/bin/env bash
set -euo pipefail
cat >&2 <<'MSG'
check_milestone_closed_issue_sor_truth.sh is retired.
Milestone closed-issue truth must move to a Rust/PVF lane before it can be used
again. Do not run GitHub CLI based milestone issue scans from shell helpers.
MSG
exit 2
