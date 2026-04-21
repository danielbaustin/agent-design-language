# Multi-Agent Repo Review Proof Demo

## Purpose

This v0.90.2 demo hardens the earlier multi-agent repo-review surface into a
stricter, findings-first proof packet.

The v0.89 baseline already showed that ADL can produce one bounded review
packet, four specialist reviews, and one synthesis artifact. This follow-on
raises the bar: the packet must preserve specialist lane boundaries, include
evidence and impact for findings, explicitly record clean/non-finding lanes,
surface disagreements and residual risk, and refuse publication or merge
approval claims.

## Command

```bash
bash adl/tools/demo_v0902_multi_agent_repo_review_proof.sh
```

Default artifact root:

```text
artifacts/v0902/multi_agent_repo_review_proof/
```

## Primary Proof Surfaces

- `run_manifest.json`
- `review_packet/repo_scope.md`
- `review_packet/evidence_index.json`
- `review_packet/specialist_assignments.json`
- `specialist_reviews/code.md`
- `specialist_reviews/security.md`
- `specialist_reviews/tests.md`
- `specialist_reviews/docs.md`
- `synthesis/final_findings_first_review.md`
- `synthesis/coverage_matrix.json`
- `quality_gate/review_quality_evaluation.md`
- `quality_gate/redaction_and_publication_gate.md`
- `README.md`

## Specialist Lanes

- `repo-packet-builder`: records bounded scope and evidence assignments.
- `repo-review-code`: reviews proof semantics and maintainability risk.
- `repo-review-security`: reviews publication, mutation, and leakage boundary.
- `repo-review-tests`: reviews fail-closed validation and overclaim guards.
- `repo-review-docs`: reviews reader navigation and packet clarity.
- `repo-review-synthesis`: preserves severity, non-findings, disagreement, and
  residual risk.
- `review-quality-evaluator`: represented as an internal quality gate.
- `redaction-and-evidence-auditor`: represented as a publication-blocking gate.

## What It Proves

- The demo emits a serious, inspectable review packet rather than a loose set
  of role notes.
- Findings include severity, evidence, impact, recommended action, validation
  gap, and residual risk.
- The security lane can truthfully emit no material findings without being
  erased by synthesis.
- Synthesis preserves overlapping findings, disagreement, coverage, caveats,
  and no-approval boundaries.
- Validation fails closed on private host paths and secret-like markers.

## What It Does Not Claim

- It does not run live model providers.
- It does not review a customer repository.
- It does not publish a customer-facing report.
- It does not create remediation PRs or tracker issues.
- It does not claim merge approval or remediation completion.

## Validation

Run:

```bash
bash adl/tools/test_demo_v0902_multi_agent_repo_review_proof.sh
```

The test generates the packet in a temporary artifact root, validates the
manifest, specialist assignments, non-finding coverage, findings-first
synthesis, quality gate, redaction gate, and negative leakage checks.
