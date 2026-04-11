# Internal Review - v0.87.1

## Metadata
- Issue: `#1494`
- Scope: bounded internal review of the `v0.87.1` reviewer-facing milestone package
- Decision: review findings resolved in the same issue before external handoff

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

## Review Corrections Applied In This Issue

- Updated `docs/milestones/v0.87.1/README.md` so the status block reflects the current closed/open boundary for `#1463` through `#1498`.
- Removed the `v0.88` native-provider note from `docs/tooling/PROVIDER_SETUP.md` so the bounded `v0.87.1` reviewer package stays milestone-scoped.
- Clarified historical `docs/records/` editor/task-bundle surfaces so they are not presented as the current canonical workflow record model.
- Corrected the Rust watch-list table heading typo from `1289 disposition` to `Current disposition`.

## Scope Limits
- This issue records review evidence and the bounded reviewer-surface truth corrections needed before external handoff.
- It does not perform the external / 3rd-party review.
- It does not widen into unrelated runtime implementation work.

## Outcome
- The bounded review-surface walkthrough passed.
- The runtime review-surface validator passed.
- No new runtime correctness failure was found in the reviewed proof package.
- The reviewer-facing truth drift identified during the review was corrected in the same issue, so no separate remediation issue is required for these specific findings.
