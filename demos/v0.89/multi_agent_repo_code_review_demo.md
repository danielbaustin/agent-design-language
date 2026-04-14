# Multi-Agent Repo Code Review Demo

This `v0.89` demo shows ADL hosting a calm, specialist full-repo review over
one bounded packet instead of pretending one reviewer can cover every concern
perfectly at once.

The review roles are:

- code reviewer
- security reviewer
- test reviewer
- docs reviewer
- synthesis reviewer

## Command

```bash
bash adl/tools/demo_v089_multi_agent_repo_code_review.sh
```

## What It Builds

The demo creates:

- one bounded review packet
- four specialist reviewer artifacts
- one cross-review note surface
- one final synthesized review
- one demo manifest and reviewer README

## Artifact Root

Default artifact root:

```text
artifacts/v089/multi_agent_repo_code_review/
```

Primary proof surfaces:

- `review_packet/review_packet_manifest.json`
- `reviewers/code_review.md`
- `reviewers/security_review.md`
- `reviewers/test_review.md`
- `reviewers/docs_review.md`
- `reviewers/cross_review_notes.md`
- `synthesis/final_synthesis_review.md`
- `demo_manifest.json`

## Important Boundary

This is a bounded review demo.

It does **not** claim:

- autonomous merge approval
- repository-wide autofix
- unbounded browsing outside the selected packet

The point is to show specialization, explicit review roles, and a final
findings-first synthesis artifact that a human reviewer can read quickly.
