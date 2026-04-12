# v0-88-wp-03-temporal-schema

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1646
Run ID: issue-1646
Version: v0.88
Title: [v0.88][WP-03] Temporal schema
Branch: codex/1646-v0-88-wp-03-temporal-schema
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI Codex desktop app
- Start Time: 2026-04-12T19:41:00Z
- End Time: 2026-04-12T19:46:30Z

## Summary
Defined a bounded canonical temporal schema contract for `WP-03` by formalizing
`TemporalSchemaContract` and related anchor/policy/cost schema surfaces in the runtime,
exposing a concrete `adl identity schema` proof-hook command, and tightening
`TEMPORAL_SCHEMA_V01.md` so the tracked doc cites the same owned runtime surfaces and scope.

## Artifacts produced
- `adl/src/chronosense.rs`
- `adl/src/cli/identity_cmd.rs`
- `adl/src/cli/usage.rs`
- `docs/milestones/v0.88/features/TEMPORAL_SCHEMA_V01.md`
- `.adl/state/temporal_schema_v01.json`

## Actions taken
- Added `TemporalSchemaContract` plus bounded temporal anchor, subjective-time,
  execution-policy, execution-realization, cost-vector, and reference-frame schema types.
- Added `adl identity schema [--out <path>]` as the reviewable proof hook for the temporal
  schema contract.
- Added targeted tests for the schema contract and CLI emission path.
- Updated `TEMPORAL_SCHEMA_V01.md` to record runtime-facing ownership, bounded acceptance
  criteria, and the canonical proof hook for `WP-03`.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; changes currently exist only on the bound `#1646` worktree branch
- Worktree-only paths remaining: `adl/src/chronosense.rs`, `adl/src/cli/identity_cmd.rs`, `adl/src/cli/usage.rs`, `docs/milestones/v0.88/features/TEMPORAL_SCHEMA_V01.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct edits in the bound `codex/1646-v0-88-wp-03-temporal-schema` worktree branch
- Verification performed:
  - `git status --short`
  - `ls .adl/state/temporal_schema_v01.json`
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
- `cargo test --manifest-path adl/Cargo.toml temporal_schema_contract -- --nocapture`
  verified the bounded temporal schema contract and trace-hook linkage assertions
- `cargo test --manifest-path adl/Cargo.toml identity_schema -- --nocapture`
  verified the `adl identity schema` CLI proof-hook path and emitted artifact shape
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  verified the Rust surfaces are formatted
- `cargo run --manifest-path adl/Cargo.toml -- identity schema --out .adl/state/temporal_schema_v01.json`
  executed the repo-style temporal-schema proof hook and emitted the bounded schema artifact
- `git diff --check`
  verified the branch has no whitespace or patch-format defects
- Results:
  all listed validation commands passed

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
      - "cargo test --manifest-path adl/Cargo.toml temporal_schema_contract -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml identity_schema -- --nocapture"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo run --manifest-path adl/Cargo.toml -- identity schema --out .adl/state/temporal_schema_v01.json"
      - "git diff --check"
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
- Determinism tests executed: `cargo test --manifest-path adl/Cargo.toml temporal_schema_contract -- --nocapture` and `cargo test --manifest-path adl/Cargo.toml identity_schema -- --nocapture`
- Fixtures or scripts used: static schema-contract fixtures with fixed owned-surface and trace-hook lists plus the repo-style `adl identity schema` emission path
- Replay verification (same inputs -> same artifacts/order): repeated `adl identity schema --out ...` runs with the same code produce the same contract-shaped schema artifact
- Ordering guarantees (sorting / tie-break rules used): runtime surface lists, trace-hook lists, and reference-frame lists are emitted in fixed source order
- Artifact stability notes: the temporal schema contract intentionally describes required fields and trace joins without embedding wall-clock-dependent event data

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: reviewed new schema fields and emitted artifact content; no secrets, provider credentials, or host-specific material are introduced by this issue
- Prompt / tool argument redaction verified: the schema proof hook records only bounded field-contract and trace-hook metadata
- Absolute path leakage check: tracked docs and code use repo-relative proof-hook references; the generated local artifact path is not embedded into tracked files
- Sandbox / policy invariants preserved: the issue adds a reviewable schema contract and does not widen runtime authority, provider execution, or network permissions

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue emits a bounded temporal schema contract rather than a trace bundle
- Run artifact root: `.adl/state/`
- Replay command used for verification: `cargo run --manifest-path adl/Cargo.toml -- identity schema --out .adl/state/temporal_schema_v01.json`
- Replay result: passed; the command emitted `.adl/state/temporal_schema_v01.json` in the worktree as the expected proof artifact

## Artifact Verification
- Primary proof surface: `.adl/state/temporal_schema_v01.json`
- Required artifacts present: yes; the bounded temporal schema artifact and all tracked code/doc surfaces exist in the `#1646` worktree
- Artifact schema/version checks: verified `schema_version: temporal_schema.v0_1` in the emitted artifact and added the corresponding Rust schema constant
- Hash/byte-stability checks: semantic stability verified through fixed-source emission and deterministic test coverage; no separate hash file was required for this bounded schema issue
- Missing/optional artifacts and rationale: no standalone demo or continuity-validation artifact is expected in `WP-03`; those remain downstream work

## Decisions / Deviations
- Used the existing `adl identity` family for schema proof emission so the temporal band stays anchored to the same bounded runtime/identity surface as `WP-02`.
- Represented execution-policy and cost reviewability as canonical schema fields plus explicit trace-hook references rather than attempting to retrofit all runtime artifacts in this issue.

## Follow-ups / Deferred work
- `WP-04` should bind continuity and identity semantics to this temporal schema contract.
- Later temporal issues should consume the canonical temporal anchor and policy/cost field contract instead of inventing parallel temporal shapes.
- Future runtime work can wire additional trace artifacts more deeply to this schema without changing `WP-03` ownership.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
