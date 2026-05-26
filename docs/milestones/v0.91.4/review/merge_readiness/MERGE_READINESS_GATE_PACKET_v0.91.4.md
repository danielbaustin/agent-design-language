# v0.91.4 Merge-Readiness Gate Packet

## Scope

Provide the first bounded `WP-07` proof surface showing that merge-readiness
and `pr finish` validation posture are transition-aware, fail closed on stale
lifecycle truth, and preserve the boundary between local validation and live
GitHub merge state.

## Tracked Proof Surfaces

- `docs/milestones/v0.91.4/review/merge_readiness/ct_demo_001_merge_gate_profile_report.md`
- `docs/milestones/v0.91.4/review/merge_readiness/ct_demo_001_merge_gate_snapshot.json`
- `docs/tooling/merge_readiness_gate_policy_v0.91.4.md`
- `adl/tools/validate_v0914_merge_readiness_gate.py`
- `adl/tools/test_v0914_merge_readiness_gate.sh`

## Validation

- `python3 adl/tools/validate_v0914_merge_readiness_gate.py docs/milestones/v0.91.4/review/merge_readiness`
- `bash adl/tools/test_v0914_merge_readiness_gate.sh`

## What This Proves

- focused merge-readiness validation now covers the real `adl/src/cli/pr_cmd/`
  subtree plus the bounded `WP-07` policy/proof docs instead of a thinner
  single-file slice
- docs-only policy remains narrow and truthful
- stale lifecycle truth still blocks finish-readiness
- explicit docs-only review exceptions still remain review-bound rather than
  silently implying merge truth
- local focused validation and live GitHub merge truth remain separate claims

## What This Does Not Prove

- automatic merge
- bypass of human review or branch protection
- repository-wide Rust health for arbitrary code changes
- live GitHub reconciliation by this packet alone
- release readiness by PR-gate success alone
