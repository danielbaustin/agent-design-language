# v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend

Canonical Template Source: `docs/templates/prompts/1.0.0/sor.md`

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-3562
Run ID: issue-3562
Version: v0.91.5
Title: [v0.91.5][review-provider][tools] Add external review provider lane for CodeFriend
Branch: codex/3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend
Card Status: ready
Status: DONE
Generated: 2026-06-01T03:18:59Z

Execution:
- Actor: codex
- Model: gpt-5.5-codex
- Provider: codex
- Start Time: 2026-06-04T17:58:00Z
- End Time: 2026-06-04T18:18:00Z

## Summary

Implemented the first ReviewProviderV1 contract slice for #3562. The work adds typed Rust review-provider request/result/run-record structures on top of the existing provider communication substrate and a tracked contract/runbook for CodeFriend ingestion and future CLI work.

## Artifacts produced

- `adl/src/provider_communication.rs`
- `docs/milestones/v0.91.5/review/review_provider/REVIEW_PROVIDER_V1_CONTRACT_3562.md`

## Actions taken

- Added review-provider role, status, redaction, severity, request, result, finding, and run-record contract types.
- Added `validate_review_provider_request` for schema version, authority boundary, embedded provider request, and non-empty review scope.
- Added `validate_review_provider_result` so failed, blocked, or skipped provider runs cannot carry scored findings.
- Added focused Rust tests for the contract, fail-closed request validation, provider-failure separation, and review-status/finding consistency.
- Added a tracked design/runbook describing the authority boundary, CLI proposal, CodeFriend ingestion path, provider sequencing, validation, follow-ons, and non-claims.
- Ran bounded pre-PR subagent review, fixed all findings, and reran focused validation.

## Main Repo Integration (REQUIRED)

- Main-repo paths updated: tracked branch changes are ready for PR publication
- Worktree-only paths remaining: tracked change still on PR branch
- Integration state: pr_open
- Verification scope: pr_branch
- Integration method used: issue-bound worktree branch `codex/3562-v0-91-5-review-provider-tools-add-external-review-provider-lane-for-codefriend`
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml review_provider --lib`
    Verified focused ReviewProviderV1 tests.
  - `cargo test --manifest-path adl/Cargo.toml provider_communication --lib`
    Verified the full provider communication unit-test slice.
  - `cargo fmt --manifest-path adl/Cargo.toml`
    Verified Rust formatting was applied.
  - `git diff --check`
    Verified patch whitespace cleanliness.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout BRANCH -- PATH` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation

- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml review_provider --lib`
    Verified four focused ReviewProviderV1 tests after reviewer fixes.
  - `cargo test --manifest-path adl/Cargo.toml provider_communication --lib`
    Verified thirteen provider communication tests, including ReviewProviderV1 and existing provider identity/logging/failure tests.
  - `cargo fmt --manifest-path adl/Cargo.toml`
    Applied Rust formatting.
  - `git diff --check`
    Verified whitespace cleanliness.
- Results:
  - PASS: `review_provider --lib`, `4` passed, `0` failed.
  - PASS: `provider_communication --lib`, `13` passed, `0` failed.
  - PASS: `cargo fmt --manifest-path adl/Cargo.toml`.
  - PASS: `git diff --check`.

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo test --manifest-path adl/Cargo.toml review_provider --lib"
      - "cargo test --manifest-path adl/Cargo.toml provider_communication --lib"
      - "cargo fmt --manifest-path adl/Cargo.toml"
      - "git diff --check"
  determinism:
    status: PASS
    replay_verified: not_applicable
    ordering_guarantees_verified: not_applicable
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
      approved: issue_scope
```

## Determinism Evidence

- Determinism tests executed: focused unit tests are deterministic and use fixture values only.
- Fixtures or scripts used: in-tree Rust unit-test fixtures in `provider_communication.rs`.
- Replay verification (same inputs -> same artifacts/order): not_applicable for pure contract/type additions.
- Ordering guarantees (sorting / tie-break rules used): not_applicable; no ordering-sensitive runtime behavior changed.
- Artifact stability notes: review-provider contract uses explicit schema version and fail-closed validation.

## Security / Privacy Checks

- Secret leakage scan performed: yes, by design review and focused diff review; no credential material was added.
- Prompt / tool argument redaction verified: existing provider logger redaction tests remain in the provider communication slice.
- Absolute path leakage check: repository-relative paths recorded in tracked artifacts and cards.
- Sandbox / policy invariants preserved: yes; external provider output remains advisory and requires CodeFriend synthesis.

## Replay Artifacts

- Trace bundle path(s): not_applicable; this issue adds contract types and docs, not a live review-provider run.
- Run artifact root: not_applicable for this slice.
- Replay command used for verification: not_run; unit tests are the proof surface.
- Replay result: NOT_RUN with rationale above.

## Artifact Verification

- Primary proof surface: `adl/src/provider_communication.rs` tests and `REVIEW_PROVIDER_V1_CONTRACT_3562.md`.
- Required artifacts present: yes.
- Artifact schema/version checks: schema-version validation added for ReviewProviderV1 request envelopes.
- Hash/byte-stability checks: not_run; not needed for this contract slice.
- Missing/optional artifacts and rationale: CLI executor and live provider review proof are follow-on slices, not part of #3562.

## Decisions / Deviations

- Used `--allow-open-pr-wave` to bind #3562 because open draft PR #3653 was unrelated and the operator explicitly allowed this issue to proceed.
- Kept this issue to contract and design/runbook work; no live review-provider executor was implemented.
- Enforced the first-slice authority boundary exactly rather than accepting arbitrary advisory text.

## Follow-ups / Deferred work

- Add `adl review-provider run` CLI using the existing provider adapter.
- Add one hosted provider smoke proof and one Ollama/mock proof.
- Add CodeFriend ingestion for `ReviewProviderResultV1` artifacts.
- Add JSON schema export for review-provider objects.
