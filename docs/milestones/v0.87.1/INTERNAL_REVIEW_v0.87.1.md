# Internal Review - v0.87.1

## Metadata
- Issue: `#1494`
- Scope: bounded internal review of the `v0.87.1` reviewer-facing milestone package
- Decision: findings recorded

## Review Surface
- `docs/milestones/v0.87.1/README.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- `docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md`
- `docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md`
- `docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md`
- `docs/tooling/PROVIDER_SETUP.md`
- `artifacts/v0871/review_surface/demo_manifest.json`
- `artifacts/v0871/review_surface/README.md`

## Validation
- `python3 adl/tools/skills/repo-code-review/scripts/repo_inventory.py .`
- `bash adl/tools/check_release_notes_commands.sh`
- `bash adl/tools/demo_v0871_review_surface.sh`
- `cargo run --manifest-path adl/Cargo.toml -- tooling review-runtime-surface --review-root artifacts/v0871/review_surface`
- `gh issue view 1463 --json number,state`
- `gh issue view 1464 --json number,state`
- `gh issue view 1495 --json number,state`
- `gh issue view 1496 --json number,state`
- `gh issue view 1497 --json number,state`
- `gh issue view 1498 --json number,state`

## Findings

### P2: milestone README status block is stale

- File: `docs/milestones/v0.87.1/README.md:160`
- Summary: the status section still says Sprint 2 tail issues `#1463` and `#1464` are open, but both issues are closed.
- Why it matters: the README is a primary reviewer entry surface, so stale issue-state claims weaken trust in the release-tail package.
- Proposed follow-up: update the status lines to reflect the actual current boundary between closed implementation work and the still-open release-tail issues.
- Proposed owner: `WP-18 / #1496`

### P3: provider setup guide includes next-milestone demo guidance

- File: `docs/tooling/PROVIDER_SETUP.md:70`
- Summary: the guide includes a `v0.88 native provider demo note` even though this internal review is scoped to the `v0.87.1` milestone package.
- Why it matters: next-milestone guidance inside the current reviewer-facing package blurs which proof surfaces belong to `v0.87.1`.
- Proposed follow-up: move or clearly isolate the `v0.88` note so the `v0.87.1` provider guidance remains milestone-scoped.
- Proposed owner: `WP-18 / #1496`

## Scope Limits
- This issue records findings and review evidence only.
- It does not remediate findings.
- It does not perform the external / 3rd-party review.

## Outcome
- The bounded review-surface walkthrough passed.
- The runtime review-surface validator passed.
- No new runtime correctness failure was found in the reviewed proof package.
- The accepted finding set is reviewer-truth drift that should be handled in `WP-18`.
