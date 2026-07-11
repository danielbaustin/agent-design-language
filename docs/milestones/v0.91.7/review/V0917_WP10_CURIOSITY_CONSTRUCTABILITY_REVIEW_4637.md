# WP-10 Curiosity And Constructability Review

Status: pre_pr_review_complete_closeout_active
Issue: #4637
WP: WP-10
Date: 2026-07-11

## Findings

No blocking findings remain for WP-10 closeout.

Pre-PR review findings were repaired before publication:

- P1: the packet initially claimed `closeout_ready` before the umbrella SRP/SOR
  had been normalized. Fixed by recording the current pre-PR review-complete,
  closeout-active lifecycle state.
- P2: the sprint register header still named the prior WP-11 update. Fixed by
  advancing its date and current-update reference to #4637.

The canonical implementation children are merged and closed:

- #4692 / PR #5161 implements the bounded Runtime v2 Curiosity Engine core,
  deterministic packet generation, governed discovery budgets and gates, CLI
  exposure, negative cases, and retained feature documentation.
- #4693 / PR #5163 implements the Runtime v2 Constructability Anchor Validator,
  fail-closed packet validation, canonical output, CLI exposure, negative cases,
  compiled CLI smoke coverage, and runtime owner-lane proof.

The remaining boundary is explicit rather than deferred WP-10 work: these are
host-agnostic Runtime v2 cores. WP-07A owns CSM supervisor hosting, typed-channel
integration, lifecycle supervision, and component placement. WP-10 must not
claim those integration properties from #4692 or #4693 alone.

## Scope Summary

- Reviewed scope type: sprint.
- Umbrella issue: #4637 `[v0.91.7][WP-10] Implement Curiosity and
  Constructability in full`.
- Canonical child issues: #4692 and #4693.
- Reviewed PRs:
  - #4692 / PR #5161: merged; issue closed.
  - #4693 / PR #5163: merged; issue closed.
- Primary implementation surfaces:
  - `adl/src/runtime_v2/curiosity_engine.rs`
  - `adl/src/runtime_v2/constructability_anchor_validator.rs`
  - `adl/src/cli/runtime_v2_cmd/`
  - `adl/src/runtime_v2/tests/curiosity_engine.rs`
  - `adl/src/runtime_v2/tests/constructability_anchor_validator.rs`
  - `adl/tests/cli_smoke.rs`
- Retained reviewer-facing surfaces:
  - `docs/milestones/v0.91.7/features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md`
  - `docs/milestones/v0.91.7/features/CONSTRUCTABILITY_GATE_v0.91.7.md`
  - `docs/milestones/v0.91.7/review/pr_finish_release_gate_disposition/ISSUE_4693_CONSTRUCTABILITY_ANCHOR_VALIDATOR_RUNTIME_OWNER_LANE_DISPOSITION.yaml`

## Lane Coverage

| Lane | Status | Evidence / reason |
| --- | --- | --- |
| gap_analysis | run | Compared the WP-10 WBS scope, child issue state, merged changes, feature docs, and release-gate evidence. |
| code | evidence_reviewed | Both code-bearing child PRs merged after focused review and required checks; this umbrella introduces no runtime code. |
| docs | run | Reviewed feature docs and repaired the sprint register and retained umbrella review truth. |
| tests | run | Reviewed focused child tests, CLI smoke coverage, negative cases, and merged CI state; umbrella validation is docs/card focused. |
| evidence_and_closeout | run | Verified both canonical children are closed with merged PRs and identified #4637 as the only remaining WP-10 lifecycle item. |
| synthesis | run | This packet synthesizes implementation, validation, lifecycle, and architectural-boundary truth. |
| review_quality | run | Independent pre-PR review found one P1 lifecycle-status issue and one P2 register-metadata issue; both were repaired before publication. |
| security | evidence_reviewed | Curiosity preserves explicit Freedom Gate, CAV, operator-review, and Constructability references; no new security code is added here. |
| architecture | run | Confirmed host-agnostic Runtime v2 core ownership and preserved the WP-07A CSM-hosting boundary. |
| dependency | skipped | No dependency manifests are changed by umbrella closeout. |
| release_evidence | partial | This is retained milestone evidence, not v0.91.7 release approval. |

## Lifecycle And Closeout Truth

- #4692 and #4693 are closed and their PRs are merged into `main`.
- #4637 remains open while this packet is authored and should close only through
  the repo-native #4637 PR and closeout lifecycle.
- No additional WP-10 implementation child is required by the canonical WBS.
- The pre-execution #4637 SRP/SOR must be normalized with the actual independent
  review, validation, PR, merge, and closeout results as those stages occur.

## Validation Summary

Reviewed child proof includes:

- #4692 focused Curiosity Engine unit, trace, CLI, determinism, budget, gate,
  and negative-case coverage, followed by green required PR checks.
- #4693 focused Constructability validator unit, trace, CLI, fail-closed and
  compiled smoke coverage; the exact runtime owner lane also passed on the
  retained CodeBuild run in 297.3 seconds, followed by green required PR checks.
- The failed Nessus no-space run and stopped non-hermetic full-nextest diagnostic
  recorded by #4693 are retained as negative platform evidence and are not
  counted as proof.

Umbrella validation before publication is intentionally Wuji-local and focused:

```bash
git diff --check
bash adl/tools/validation_manager.sh --changed-files <changed-files> --json --run
bash adl/tools/validate_structured_prompt.sh --type srp --phase pre_pr --input .adl/v0.91.7/tasks/issue-4637__v0-91-7-wp-10-curiosity-and-constructability/srp.md
bash adl/tools/validate_structured_prompt.sh --type sor --phase pre_run --input .adl/v0.91.7/tasks/issue-4637__v0-91-7-wp-10-curiosity-and-constructability/sor.md
```

## Residual Risk

- The umbrella consumes merged child review and CI evidence instead of rerunning
  every child test on Wuji.
- Runtime v2 core availability does not prove CSM supervisor hosting, typed
  channels, lifecycle supervision, or production activation.
- The #4693 retained release-gate disposition was authored before merge; this
  packet records the later merged/closed truth without rewriting that historical
  pre-merge artifact.

## Follow-up Routing

- No new WP-10 follow-up issue is required.
- Keep CSM component hosting and supervision in WP-07A; do not reopen WP-10 to
  duplicate that architecture work.
- v0.92 activation and public claims remain subject to their milestone security,
  runtime, review, and release gates.

## Non-Claims

- This packet does not approve v0.91.7 or v0.92 release readiness.
- This packet does not claim CSM supervisor hosting or typed-channel integration.
- This packet does not claim unbounded autonomous curiosity.
- This packet does not treat provisional cognition as authoritative shared reality.
- This packet does not convert failed remote-platform runs into validation proof.
