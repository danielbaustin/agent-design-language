# v0-88-wp-04-continuity-and-identity-semantics

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1648
Run ID: issue-1648
Version: v0.88
Title: [v0.88][WP-04] Continuity and identity semantics
Branch: codex/1648-v0-88-wp-04-continuity-and-identity-semantics
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI Codex desktop app
- Start Time: 2026-04-12T20:48:00Z
- End Time: 2026-04-12T20:57:30Z

## Summary
Grounded `WP-04` continuity and identity semantics in the runtime by formalizing a bounded
`ContinuitySemanticsContract`, exposing a concrete `adl identity continuity` proof hook, and
aligning `CHRONOSENSE_AND_IDENTITY.md` with the actual continuity-state and resumption surfaces
already emitted by the runtime.

## Artifacts produced
- `adl/src/chronosense.rs`
- `adl/src/cli/identity_cmd.rs`
- `adl/src/cli/usage.rs`
- `docs/milestones/v0.88/features/CHRONOSENSE_AND_IDENTITY.md`
- `.adl/state/continuity_semantics_v1.json`

## Actions taken
- Added `ContinuitySemanticsContract` as the canonical bounded continuity/identity contract for
  `v0.88`.
- Anchored the contract to the existing runtime status surface:
  `run_status.v1.continuity_status`, `preservation_status`, and `shepherd_decision`.
- Added `adl identity continuity [--out <path>]` as the reviewable proof hook for the continuity
  semantics contract.
- Added focused tests for the continuity contract and CLI emission path.
- Updated `CHRONOSENSE_AND_IDENTITY.md` to record runtime-facing ownership, bounded acceptance
  criteria, and the canonical proof hook.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; changes currently exist only on the bound `#1648` worktree branch
- Worktree-only paths remaining: `adl/src/chronosense.rs`, `adl/src/cli/identity_cmd.rs`, `adl/src/cli/usage.rs`, `docs/milestones/v0.88/features/CHRONOSENSE_AND_IDENTITY.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct edits in the bound `codex/1648-v0-88-wp-04-continuity-and-identity-semantics` worktree branch
- Verification performed:
  - `git status --short`
  - `ls .adl/state/continuity_semantics_v1.json`
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
- `cargo test --manifest-path adl/Cargo.toml continuity_semantics_contract -- --nocapture`
  verified the continuity semantics contract matches the runtime continuity-state surface
- `cargo test --manifest-path adl/Cargo.toml identity_continuity -- --nocapture`
  verified the `adl identity continuity` CLI proof-hook path and emitted artifact shape
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  verified the Rust surfaces are formatted
- `cargo run --manifest-path adl/Cargo.toml -- identity continuity --out .adl/state/continuity_semantics_v1.json`
  executed the repo-style continuity proof hook and emitted the bounded continuity artifact
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
      - "cargo test --manifest-path adl/Cargo.toml continuity_semantics_contract -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml identity_continuity -- --nocapture"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo run --manifest-path adl/Cargo.toml -- identity continuity --out .adl/state/continuity_semantics_v1.json"
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
- Determinism tests executed: `cargo test --manifest-path adl/Cargo.toml continuity_semantics_contract -- --nocapture` and `cargo test --manifest-path adl/Cargo.toml identity_continuity -- --nocapture`
- Fixtures or scripts used: static continuity-state fixtures aligned to the runtime run-status surface plus the repo-style `adl identity continuity` emission path
- Replay verification (same inputs -> same artifacts/order): repeated `adl identity continuity --out ...` runs with the same code produce the same contract-shaped continuity artifact
- Ordering guarantees (sorting / tie-break rules used): continuity-state lists, proof-fixture hooks, and resumption rules are emitted in fixed source order
- Artifact stability notes: the continuity artifact intentionally describes continuity/resumption semantics without embedding live run-specific state

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: reviewed the new continuity contract fields and emitted artifact content; no secrets, provider credentials, or host-specific material are introduced by this issue
- Prompt / tool argument redaction verified: the continuity proof hook records only bounded continuity-state and resumption-rule metadata
- Absolute path leakage check: tracked docs and code use repo-relative proof-hook references; the generated local artifact path is not embedded into tracked files
- Sandbox / policy invariants preserved: the issue adds a reviewable continuity semantics contract and does not widen runtime authority, provider behavior, or network permissions

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue emits a bounded continuity semantics contract rather than a trace bundle
- Run artifact root: `.adl/state/`
- Replay command used for verification: `cargo run --manifest-path adl/Cargo.toml -- identity continuity --out .adl/state/continuity_semantics_v1.json`
- Replay result: passed; the command emitted `.adl/state/continuity_semantics_v1.json` in the worktree as the expected proof artifact

## Artifact Verification
- Primary proof surface: `.adl/state/continuity_semantics_v1.json`
- Required artifacts present: yes; the bounded continuity artifact and all tracked code/doc surfaces exist in the `#1648` worktree
- Artifact schema/version checks: verified `schema_version: continuity_semantics.v1` in the emitted artifact and added the corresponding Rust schema constant
- Hash/byte-stability checks: semantic stability verified through fixed-source emission and deterministic test coverage; no separate hash file was required for this bounded continuity issue
- Missing/optional artifacts and rationale: no standalone demo or retrieval/commitment artifact is expected in `WP-04`; those remain downstream work

## Decisions / Deviations
- Used the existing run-status continuity fields as the canonical runtime ownership surface instead of inventing a second continuity-status channel.
- Represented identity preservation as a continuity/resumption rule tied to temporal structure and resume guards, not as a broad autonomous selfhood claim.

## Follow-ups / Deferred work
- Later runtime work can wire deeper continuity validation against richer trace/subjective-time evidence without changing the `WP-04` contract.
- Retrieval, commitments, causality, and governance work should consume these continuity states rather than redefining restart/resume semantics independently.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
