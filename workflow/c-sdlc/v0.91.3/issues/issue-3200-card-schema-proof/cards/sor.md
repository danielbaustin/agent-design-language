# v0-91-3-wp-02-cognitive-transition-schema

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-3200
Run ID: issue-3200
Version: v0.91.3
Title: [v0.91.3][WP-02][docs/tools] Cognitive Transition schema
Branch: codex/3200-v0-91-3-wp-02-cognitive-transition-schema
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5
- Provider: openai-codex
- Start Time: 2026-05-21T20:23:47Z
- End Time: 2026-05-21T20:23:47Z

## Summary

Completed the bounded WP-02 implementation pass for `#3200`.

The issue now:

- adds a first machine-checkable cognitive transition manifest schema surface
  under `adl/src/cognitive_transition_schema.rs`
- validates bounded lifecycle states, required seed roles, GitHub issue URL
  shape, and repo-relative path discipline
- ships tracked valid and invalid JSON fixtures plus updated milestone/docs
  proof surfaces that point at the new validator-backed implementation

## Artifacts produced

- New tracked Rust schema surface:
  - `adl/src/cognitive_transition_schema.rs`
- Updated tracked Rust module export:
  - `adl/src/lib.rs`
- Updated tracked docs:
  - `docs/cognitive-sdlc/transition-schema.md`
  - `docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md`
  - `docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md`
  - `docs/milestones/v0.91.3/FEATURE_PROOF_COVERAGE_v0.91.3.md`
- New tracked JSON fixtures:
  - `docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`
  - `docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`
- Updated local ignored issue cards:
  - `.adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/spp.md`
  - `.adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sor.md`

## Actions taken

- Bound `#3200` to the issue worktree and reviewed the WP-02 source prompt,
  STP/SIP surfaces, milestone docs, and transition-schema target docs.
- Implemented a new Rust manifest schema module with exported schema JSON,
  validator, valid fixture helper, and focused unit coverage.
- Added tracked valid and invalid JSON fixture files to make the first schema
  slice reviewable outside the Rust test helper alone.
- Updated the cognitive-sdlc and milestone proof/demo docs so the WP-02 story
  now points at the real code, fixtures, and focused proof command.
- Ran focused validation for schema behavior, JSON fixture parseability,
  formatting cleanliness, and diff hygiene.

## Main Repo Integration (REQUIRED)

- Main-repo paths updated: merged PR `#3235` integrated the tracked changes into `main`.
  - `adl/src/cognitive_transition_schema.rs`
  - `adl/src/lib.rs`
  - `docs/cognitive-sdlc/transition-schema.md`
  - `docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md`
  - `docs/milestones/v0.91.3/DEMO_MATRIX_v0.91.3.md`
  - `docs/milestones/v0.91.3/FEATURE_PROOF_COVERAGE_v0.91.3.md`
  - `docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`
  - `docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`
- Worktree-only paths remaining: none
- Integration state: merged
- Verification scope: main_repo
- Integration method used: merged PR `#3235` against `main`; retained local ignored closeout records remain intentionally local where applicable.
- Verification performed:
  - `bash adl/tools/pr.sh run 3200 --version v0.91.3`
    Verified the issue bound cleanly to the expected branch and worktree before implementation.
  - `cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture`
    Verified the new manifest schema validator, fixture helper, and focused edge-case coverage.
  - `python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`
    Verified the tracked valid fixture is well-formed JSON.
  - `python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`
    Verified the tracked invalid fixture is well-formed JSON for review and later negative-path use.
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
    Verified Rust formatting cleanliness after implementation.
  - `git diff --check`
    Verified whitespace cleanliness for the tracked diff.
  - `gh pr view 3235 --json url,isDraft,state,mergeStateStatus,headRefName,baseRefName,title`
    Verified merged PR #3235 targeted `main` from the expected issue branch before closeout.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- `pr_open` should pair with truthful `Worktree-only paths remaining` content; list those paths when they still exist only in the worktree or say `none` only when the branch contents are fully represented in the main repository path.
- If `Integration state` is `pr_open`, verify the actual proof artifacts rather than only the containing directory or card path.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation

- Validation commands and their purpose:
  - `bash adl/tools/pr.sh doctor 3200 --version v0.91.3 --json`
    Verified the issue bundle was structurally ready before binding.
  - `bash adl/tools/pr.sh run 3200 --version v0.91.3`
    Bound the issue branch and worktree for WP-02 implementation.
  - `cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture`
    Verified the new manifest schema module and focused unit tests.
  - `python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`
    Verified the tracked valid fixture remains parseable JSON.
  - `python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`
    Verified the tracked invalid fixture remains parseable JSON.
  - `cargo fmt --manifest-path adl/Cargo.toml --all`
    Applied Rust formatting after the focused test pass.
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
    Verified formatting cleanliness after the formatting pass.
  - `git diff --check`
    Verified whitespace cleanliness.
  - `gh pr view 3235 --json url,isDraft,state,mergeStateStatus,headRefName,baseRefName,title`
    Verified the merged PR #3235 is closed and its tracked changes are integrated into `main`.
- Results:
  - PASS

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "bash adl/tools/pr.sh doctor 3200 --version v0.91.3 --json"
      - "bash adl/tools/pr.sh run 3200 --version v0.91.3"
      - "cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture"
      - "python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json"
      - "python3 -m json.tool docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json"
      - "cargo fmt --manifest-path adl/Cargo.toml --all"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "git diff --check"
      - "gh pr view 3235 --json url,isDraft,state,mergeStateStatus,headRefName,baseRefName,title"
  determinism:
    status: PASS
    replay_verified: partial
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: issue_scoped
```

## Determinism Evidence

- Determinism tests executed: focused schema tests plus explicit JSON fixture parse checks.
- Fixtures or scripts used:
  `wp02_cognitive_transition_manifest_valid_fixture()`,
  `docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`,
  and
  `docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`.
- Replay verification (same inputs -> same artifacts/order): partial; the schema fixture helper and checked-in JSON fixtures provide stable review surfaces, but no byte-level replay harness was added in WP-02.
- Ordering guarantees (sorting / tie-break rules used): validator membership checks rely on explicit state and role lists rather than ambient ordering.
- Artifact stability notes: all new fixtures, docs, and recorded commands use repository-relative paths.

## Security / Privacy Checks

- Secret leakage scan performed through manual diff review of the new code, fixtures, and docs.
- Prompt / tool argument redaction verified: yes; no provider tokens, external credentials, or machine-local absolute paths were added to tracked surfaces.
- Absolute path leakage check: pass for the new fixtures, docs, Rust module, and recorded commands.
- Sandbox / policy invariants preserved: yes; tracked edits stayed in the bound worktree and local ignored card updates stayed under the repo-local `.adl` tree.

## Replay Artifacts

- Trace bundle path(s): not_applicable
- Run artifact root: `docs/milestones/v0.91.3/review/transition_manifest/fixtures/`
- Replay command used for verification:
  `cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture`
- Replay result: PASS for the focused validator surface

## Artifact Verification

- Primary proof surfaces:
  - `adl/src/cognitive_transition_schema.rs`
  - `docs/cognitive-sdlc/transition-schema.md`
  - `docs/milestones/v0.91.3/features/COGNITIVE_TRANSITION_MANIFEST.md`
  - `docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json`
  - `docs/milestones/v0.91.3/review/transition_manifest/fixtures/invalid_cognitive_transition_manifest_v1_missing_seed_role.json`
- Required artifacts present: yes
- Artifact schema/version checks: the Rust validator enforces `cognitive_transition_manifest.v1` and the exported schema JSON exposes the expected object properties.
- Hash/byte-stability checks: not_run
- Missing/optional artifacts and rationale: transition DAG, shard-plan, signed-trace, and ObsMem artifacts remain intentionally out of scope for WP-02.

## Decisions / Deviations

- Kept the first schema slice intentionally narrow: machine-checkable manifest surface, seed roles, lifecycle states, fixtures, and docs only.
- Added tracked JSON fixtures under the milestone review tree so WP-02 has a reviewable artifact outside the Rust test helper alone.
- No demo lane was added because the milestone demo expectation for WP-02 is proof-oriented validator output rather than a standalone operator-facing demo command.

## Follow-ups / Deferred work

- No further CI watch or closeout normalization is required for integration state; PR #3235 is already merged.
- No further integration-state normalization is required; future changes belong in a new follow-on issue.
- Let downstream issues extend transition DAG, shard-plan, signed-trace, and ObsMem integration once the first manifest surface is landed.
