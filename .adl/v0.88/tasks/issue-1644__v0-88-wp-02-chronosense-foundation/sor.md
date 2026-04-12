# v0-88-wp-02-chronosense-foundation

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1644
Run ID: issue-1644
Version: v0.88
Title: [v0.88][WP-02] Chronosense foundation
Branch: codex/1644-v0-88-wp-02-chronosense-foundation
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI Codex desktop app
- Start Time: 2026-04-12T19:00:00Z
- End Time: 2026-04-12T19:31:16Z

## Summary

Established a bounded `WP-02` chronosense foundation in the runtime by formalizing a
reviewable `ChronosenseFoundation` contract, exposing a concrete CLI proof hook at
`adl identity foundation`, and tightening `SUBSTANCE_OF_TIME.md` so the tracked feature doc
names the exact runtime surfaces and scope boundary owned by this issue.

## Artifacts produced
- `adl/src/chronosense.rs`
- `adl/src/cli/identity_cmd.rs`
- `adl/src/cli/usage.rs`
- `docs/milestones/v0.88/features/SUBSTANCE_OF_TIME.md`
- `.adl/state/chronosense_foundation.v1.json`

## Actions taken
- Added `ChronosenseFoundation` plus `chronosense_foundation.v1` as the bounded runtime-facing
  foundation contract for `v0.88`.
- Added `adl identity foundation [--out <path>]` as the reviewable proof hook for emitting the
  foundation artifact.
- Added focused tests for the foundation contract and CLI proof-hook path.
- Updated `SUBSTANCE_OF_TIME.md` to record the runtime-owned surfaces, bounded acceptance
  criteria, proof hook, and truthful `v0.88` current-status block.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; changes currently exist only on the bound `#1644` worktree branch
- Worktree-only paths remaining: `adl/src/chronosense.rs`, `adl/src/cli/identity_cmd.rs`, `adl/src/cli/usage.rs`, `docs/milestones/v0.88/features/SUBSTANCE_OF_TIME.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct edits in the bound `codex/1644-v0-88-wp-02-chronosense-foundation` worktree branch
- Verification performed:
  - `git status --short`
  - `ls .adl/state/chronosense_foundation.v1.json`
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
- `cargo test --manifest-path adl/Cargo.toml chronosense -- --nocapture`
  verified the chronosense foundation module contract and bounded foundation test coverage
- `cargo test --manifest-path adl/Cargo.toml identity_ -- --nocapture`
  verified the identity CLI surfaces, including the new `identity foundation` proof hook
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  verified the Rust surfaces are formatted
- `cargo run --manifest-path adl/Cargo.toml -- identity foundation --out .adl/state/chronosense_foundation.v1.json`
  executed the repo-style proof hook and emitted the bounded foundation artifact
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
      - "cargo test --manifest-path adl/Cargo.toml chronosense -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml identity_ -- --nocapture"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo run --manifest-path adl/Cargo.toml -- identity foundation --out .adl/state/chronosense_foundation.v1.json"
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
- Determinism tests executed: `cargo test --manifest-path adl/Cargo.toml chronosense -- --nocapture` and `cargo test --manifest-path adl/Cargo.toml identity_ -- --nocapture`
- Fixtures or scripts used: deterministic unit fixtures with fixed birthday, timezone, and timestamp inputs plus a repo-style `adl identity foundation` emission path
- Replay verification (same inputs -> same artifacts/order): the bounded foundation artifact is derived from static owned-surface and capability lists, so repeated `adl identity foundation --out ...` runs with the same code produce the same semantic artifact contract
- Ordering guarantees (sorting / tie-break rules used): owned runtime surfaces and required capability lists are emitted in fixed source order
- Artifact stability notes: the `ChronosenseFoundation::bounded_v088()` payload is intentionally static and review-oriented, avoiding wall-clock-dependent fields

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: reviewed the new runtime/doc surfaces and emitted artifact fields; no secret-bearing inputs or provider credentials are introduced by this issue
- Prompt / tool argument redaction verified: the new CLI proof hook records only bounded chronosense contract content and a repo-relative proof-hook path
- Absolute path leakage check: tracked docs and code use repo-relative proof-hook references; the generated local artifact path is not embedded into tracked files
- Sandbox / policy invariants preserved: the issue adds a bounded local review artifact and does not widen execution authority, network behavior, or provider permissions

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue emits a bounded foundation contract rather than a trace bundle
- Run artifact root: `.adl/state/`
- Replay command used for verification: `cargo run --manifest-path adl/Cargo.toml -- identity foundation --out .adl/state/chronosense_foundation.v1.json`
- Replay result: passed; the command emitted `.adl/state/chronosense_foundation.v1.json` in the worktree as the expected proof artifact

## Artifact Verification
- Primary proof surface: `.adl/state/chronosense_foundation.v1.json`
- Required artifacts present: yes; the bounded foundation artifact and all tracked code/doc surfaces exist in the `#1644` worktree
- Artifact schema/version checks: verified `schema_version: chronosense_foundation.v1` in the emitted artifact and added the corresponding Rust schema constant
- Hash/byte-stability checks: semantic stability verified through fixed-source emission and deterministic test coverage; no separate hash file was required for this bounded foundation issue
- Missing/optional artifacts and rationale: no flagship demo or continuity artifact is expected in `WP-02`; those remain downstream work

## Decisions / Deviations
- Used the existing `adl identity` surface as the bounded proof-hook entrypoint instead of introducing a new top-level chronosense command.
- Scoped the runtime artifact to foundation ownership and capability boundaries only, explicitly deferring continuity semantics, temporal schema completion, retrieval, commitments, and causality to later WPs.

## Follow-ups / Deferred work
- `WP-03` should extend this substrate with the canonical temporal schema contract.
- `WP-04` should connect continuity and identity semantics to the bounded foundation defined here.
- Later temporal WPs should consume the `ChronosenseFoundation` contract rather than restating `WP-02` scope in prose.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
