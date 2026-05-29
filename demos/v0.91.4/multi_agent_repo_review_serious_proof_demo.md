# Multi-Agent Repo Review Serious Proof Demo

This `v0.91.4` demo is the reviewer-first companion to the older `v0.89`
multi-agent repo-review sample.

It is meant to feel like a serious bounded review packet rather than just a
coordination sketch.

## Command

```bash
bash adl/tools/demo_v0914_multi_agent_repo_review_serious_proof.sh
```

## What It Builds

The demo creates:

- one bounded review packet scope
- one visible heuristic contract mapped to specialist roles
- four specialist review artifacts
- one findings-first synthesis review
- one quality-gate pair for review quality and publication blocking
- one artifact README that gives the reviewer reading order

## Artifact Root

Default artifact root:

```text
artifacts/v0914/multi_agent_repo_review_serious_proof/
```

Primary proof surfaces:

- `run_manifest.json`
- `review_packet/repo_scope.md`
- `review_packet/heuristic_contract.json`
- `review_packet/evidence_index.json`
- `review_packet/specialist_assignments.json`
- `specialist_reviews/code.md`
- `specialist_reviews/security.md`
- `specialist_reviews/tests.md`
- `specialist_reviews/docs.md`
- `synthesis/final_findings_first_review.md`
- `quality_gate/review_quality_evaluation.md`
- `quality_gate/redaction_and_publication_gate.md`

## Important Boundary

This is a bounded proving fixture.

It does **not** claim:

- live provider execution quality
- merge approval authority
- customer-ready publication
- autonomous remediation

The point is to show a serious review packet shape with visible heuristics,
explicit non-findings, role-specific caveats, and findings-first synthesis.
