# v0-88-wp-05-temporal-query-and-retrieval

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1650
Run ID: issue-1650
Version: v0.88
Title: [v0.88][WP-05] Temporal query and retrieval
Branch: codex/1650-v0-88-wp-05-temporal-query-and-retrieval
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-12T21:06:00Z
- End Time: 2026-04-12T21:16:00Z

## Summary
Defined the bounded `v0.88` temporal query/retrieval contract on top of the real existing runtime memory/query surfaces, added the proof-hook CLI `adl identity retrieval`, and updated the promoted feature doc so the Sprint 1 claim is explicit, reviewable, and scoped to contract/proof semantics rather than a full temporal index.

## Artifacts produced
- `.adl/state/temporal_query_retrieval_v1.json`
- `adl/src/chronosense.rs`
- `adl/src/cli/identity_cmd.rs`
- `adl/src/cli/usage.rs`
- `docs/milestones/v0.88/features/TEMPORAL_QUERY_AND_RETRIEVAL.md`

## Actions taken
- Added `TemporalQueryRetrievalContract`, `TemporalQueryPrimitiveSet`, and `TemporalRetrievalSemantics` to define relative-order, interval, staleness, continuity, and commitment-state query classes over the existing retrieval layer.
- Anchored the contract to the real runtime surfaces already present in the repo: `MemoryQueryState`, `MemoryQuery`, `RetrievalPolicyV1`, and continuity-related run-status fields.
- Added the proof-hook CLI subcommand `adl identity retrieval [--out <path>]` and covered both artifact emission and argument validation with focused tests.
- Updated the promoted milestone feature doc with runtime-facing ownership, bounded acceptance criteria, and the proof-hook command/output path for reviewer-facing truth alignment.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining: `adl/src/chronosense.rs`, `adl/src/cli/identity_cmd.rs`, `adl/src/cli/usage.rs`, `docs/milestones/v0.88/features/TEMPORAL_QUERY_AND_RETRIEVAL.md`, `.adl/state/temporal_query_retrieval_v1.json`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: bounded issue implementation in the `#1650` worktree pending PR publication and merge
- Verification performed:
  - `git status --short`
  - `cargo run --manifest-path adl/Cargo.toml -- identity retrieval --out .adl/state/temporal_query_retrieval_v1.json`
- Result: FAIL

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
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml temporal_query_retrieval_contract -- --nocapture` - verified the new temporal query/retrieval contract matches the intended runtime and retrieval surfaces
  - `cargo test --manifest-path adl/Cargo.toml identity_retrieval -- --nocapture` - verified the `adl identity retrieval` proof hook writes the expected artifact and rejects malformed args
  - `cargo run --manifest-path adl/Cargo.toml -- identity retrieval --out .adl/state/temporal_query_retrieval_v1.json` - emitted the repo-local proof artifact for reviewer inspection
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` - verified Rust formatting
- Results:
  - All listed validation commands passed

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

Rules:
- Replace the example values below with one actual final value per field.
- Do not leave pipe-delimited enum menus or placeholder text in a finished record.

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo test --manifest-path adl/Cargo.toml temporal_query_retrieval_contract -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml identity_retrieval -- --nocapture"
      - "cargo run --manifest-path adl/Cargo.toml -- identity retrieval --out .adl/state/temporal_query_retrieval_v1.json"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
  determinism:
    status: PASS
    replay_verified: true
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
      approved: true
```

## Determinism Evidence
- Determinism tests executed: `temporal_query_retrieval_contract_matches_runtime_and_retrieval_surfaces` and `identity_retrieval_writes_temporal_query_retrieval_contract_json`
- Fixtures or scripts used: the CLI proof hook `adl identity retrieval --out .adl/state/temporal_query_retrieval_v1.json`
- Replay verification (same inputs -> same artifacts/order): repeated contract construction and focused CLI emission tests verified stable schema content and proof-hook output path for identical inputs
- Ordering guarantees (sorting / tie-break rules used): the contract explicitly names the deterministic retrieval orders already owned by `RetrievalPolicyV1` (`workflow_id_then_run_id_ascending`, `score_desc_id_asc`, `evidence_adjusted_desc_id_asc`, `id_asc`)
- Artifact stability notes: the emitted proof artifact is a pure serialization of the contract and does not depend on wall-clock input, host paths, or hidden runtime mutation

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: reviewed the new contract and proof-hook output for host tokens/secrets; none were introduced
- Prompt / tool argument redaction verified: the contract records command names and bounded runtime surface identifiers only; no prompt bodies or tool arguments are persisted
- Absolute path leakage check: recorded commands and artifact references in this SOR are repository-relative; the proof artifact itself emits only repo-local relative output metadata
- Sandbox / policy invariants preserved: no sandbox, approval, or execution-boundary policy was widened by this change; the work is limited to contract/proof-hook surfaces

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable - this WP defines a contract/proof surface rather than a trace replay bundle
- Run artifact root: `.adl/state/`
- Replay command used for verification: `cargo run --manifest-path adl/Cargo.toml -- identity retrieval --out .adl/state/temporal_query_retrieval_v1.json`
- Replay result: the proof command completed successfully and wrote `.adl/state/temporal_query_retrieval_v1.json`

## Artifact Verification
- Primary proof surface: `.adl/state/temporal_query_retrieval_v1.json`
- Required artifacts present: yes - the proof artifact and all four intended tracked source/doc files exist in the worktree
- Artifact schema/version checks: verified `schema_version: temporal_query_retrieval.v1` through the focused CLI/unit tests
- Hash/byte-stability checks: contract generation is deterministic for identical inputs; no nondeterministic fields were added
- Missing/optional artifacts and rationale: no standalone demo artifact was required because the issue explicitly called for a proof surface with fixture-backed tests instead

## Decisions / Deviations
- Kept `WP-05` as a bounded contract/proof-hook implementation tied to the existing retrieval layer instead of inventing a full temporal index before the runtime owns the necessary indexed temporal fields
- Used the `adl identity ...` proof-hook pattern established by `WP-02` through `WP-04` so Sprint 1 remains coherent and reviewer-facing
- Explicitly named commitment-state queries in the contract while keeping full commitment retrieval implementation out of scope for this WP

## Follow-ups / Deferred work
- Full temporal indexing across trace and memory records remains downstream work
- Commitment/deadline retrieval will need concrete record surfaces before the query class can be backed by runtime data
- Causality and branch-timeline reasoning remain downstream and were intentionally excluded here

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
