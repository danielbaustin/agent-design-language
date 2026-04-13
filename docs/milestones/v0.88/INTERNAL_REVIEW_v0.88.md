# Internal Review - v0.88

## Metadata
- Issue: `#1659`
- Scope: full internal review of the `v0.88` release package, including root release surfaces, manifest/CI config, and implementation risk review
- Decision: release-surface truth findings corrected in this issue; implementation/tooling findings carried forward explicitly to remediation

## Review Surface
- `README.md`
- `CHANGELOG.md`
- `adl/Cargo.toml`
- `.github/workflows/ci.yaml`
- `adl/src/`
- `docs/milestones/v0.88/README.md`
- `docs/milestones/v0.88/WBS_v0.88.md`
- `docs/milestones/v0.88/SPRINT_v0.88.md`
- `docs/milestones/v0.88/FEATURE_DOCS_v0.88.md`
- `docs/milestones/v0.88/DEMO_MATRIX_v0.88.md`
- `docs/milestones/v0.88/QUALITY_GATE_v0.88.md`
- `docs/milestones/v0.88/MILESTONE_CHECKLIST_v0.88.md`
- `docs/milestones/v0.88/RELEASE_PLAN_v0.88.md`
- `docs/milestones/v0.88/RELEASE_NOTES_v0.88.md`
- `docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml`

## Validation
- `python3 adl/tools/skills/repo-code-review/scripts/repo_inventory.py .`
- `gh issue view 1652 --json number,state`
- `gh issue view 1658 --json number,state`
- `gh issue view 1659 --json number,state`
- `gh issue view 1660 --json number,state`
- `gh issue view 1661 --json number,state`
- `gh issue view 1662 --json number,state`
- `gh issue view 1663 --json number,state`
- `bash adl/tools/check_release_notes_commands.sh`
- `git diff --check -- docs/milestones/v0.88`

## Findings

### F1. Root release surfaces still present `v0.87.1` as the active milestone
The repo-root release package still advertised `v0.87.1` as the active milestone in `README.md` and omitted `v0.88` from `CHANGELOG.md`, even though the repo is already executing `v0.88` closeout and reviewer-facing proof work. A 3rd-party reviewer starting from the repo root would get the wrong release context before even reaching the milestone docs.

Disposition: corrected in this issue.

### F2. Native provider invocation artifact writing is not concurrency-safe
`adl/src/provider/http_family.rs` appends to the provider invocation artifact by reading the whole JSON file, mutating it in memory, and rewriting it through a fixed sibling temp path. When two provider calls write the same artifact concurrently, one writer can overwrite the other or collide on the shared `.tmp` path. That makes the audit artifact lossy exactly in the parallel/provider-comparison situations where it is most valuable.

Disposition: defer to `WP-18` for code remediation.

### F3. `pr finish` always runs the full Rust check stack, even for docs-only review issues
`adl/src/cli/pr_cmd.rs` unconditionally runs `cargo fmt`, `cargo clippy --all-targets`, and full `cargo test` inside `real_pr_finish` whenever checks are enabled. That means even a bounded docs/review issue pays the full repo-validation cost during publish, which increases latency, lock contention, and the chance of unrelated failures blocking closeout. This is an operability problem rather than a release-blocking correctness bug, but it is still real and visible in current workflow behavior.

Disposition: defer to `WP-18` or later tooling cleanup.

### F4. Stale closeout-state truth for `WP-14` and `WP-15`
Reviewer-facing `v0.88` docs still presented `#1652` and `#1658` as open closeout issues after both issues had already merged. This appeared in the WBS issue column, active closeout lists, and the seeded issue-wave artifact. That drift would make the review tail look less complete than it really is and force the reviewer to reconcile milestone truth manually.

Disposition: corrected in this issue.

### F5. Quality-gate checklist lag
`docs/milestones/v0.88/MILESTONE_CHECKLIST_v0.88.md` still marked the canonical quality gate as undefined even though `docs/milestones/v0.88/QUALITY_GATE_v0.88.md` now exists and is part of the reviewer package.

Disposition: corrected in this issue.

## Review Corrections Applied In This Issue
- Updated `README.md` and `CHANGELOG.md` so the repo-root release surfaces now identify `v0.88` as the active milestone.
- Updated `README.md`, `SPRINT_v0.88.md`, `WBS_v0.88.md`, `RELEASE_NOTES_v0.88.md`, and `WP_ISSUE_WAVE_v0.88.yaml` so the closeout tail now starts at `WP-16` and correctly treats `WP-14` / `WP-15` as closed.
- Updated `MILESTONE_CHECKLIST_v0.88.md` so the canonical quality-gate definition item reflects the current tracked package truth.
- Added this internal review record to the milestone package.

## Scope Limits
- This issue records the internal-review pass for the `v0.88` release package and repo-wide release-readiness scan.
- It does not perform the 3rd-party review.
- It does not widen into substantial implementation remediation that belongs in `WP-18`.

## Outcome
- The internal review found both release-surface truth drift and real implementation/tooling findings.
- The release-surface truth drift was corrected in the same issue.
- The remaining implementation/tooling findings are now explicit and bounded for remediation rather than being left for a 3rd-party reviewer to discover first.
- A tracked-file audit did not find obvious secret files, temp directories, or other misplaced repository artifacts; the odd quoted `adl/tests/fixtures/...` paths are legitimate Unicode test fixtures rather than stray top-level files.
