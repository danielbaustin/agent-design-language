# v0-87-1-swarm-make-remote-transport-explicitly-https-only-in-code-and-docs

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1477
Run ID: issue-1477
Version: v0.87.1
Title: [v0.87.1][swarm] Make remote transport explicitly HTTPS-only in code and docs
Branch: codex/1477-v0-87-1-swarm-make-remote-transport-explicitly-https-only-in-code-and-docs
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5.4
- Provider: Codex desktop
- Start Time: 2026-04-08T20:25:00Z
- End Time: 2026-04-08T20:25:00Z

## Summary
Made the remote provider transport boundary explicit: real remote endpoints must use HTTPS, local loopback HTTP remains allowed for local harnesses/tests, and the provider substrate feature doc now states that boundary directly.

## Artifacts produced
- updated remote endpoint validation in `adl/src/adl/validation.rs`
- updated provider endpoint enforcement and error messaging in `adl/src/provider.rs`
- new regression coverage in `adl/src/adl/tests.rs` and `adl/tests/provider_tests.rs`
- updated provider substrate documentation in `docs/milestones/v0.87/features/PROVIDER_SUBSTRATE_FEATURE.md`

## Actions taken
- started issue `1477` through the repo run-phase flow in its bound worktree
- tightened provider/profile endpoint checks so remote endpoints must use `https://`
- preserved plaintext `http://localhost` / `127.0.0.1` / `::1` for local development and test harnesses
- updated validation and provider-construction error messages to describe the supported boundary truthfully
- added focused tests for remote HTTP rejection and local loopback HTTP acceptance
- updated the canonical provider substrate feature doc to state the HTTPS-only remote transport boundary

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1481.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1481.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1481.
- Verification performed:
  - `git status --short`
    - verified the diff is bounded to the five intended tracked paths
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
    - verified formatting is clean
  - `cargo test --manifest-path adl/Cargo.toml validate_provider_http_ -- --nocapture`
    - verified schema validation rejects plaintext remote HTTP while allowing loopback HTTP
  - `cargo test --manifest-path adl/Cargo.toml http_provider_ -- --nocapture`
    - verified provider construction/runtime tests still pass, including the loopback harness path and the new remote-HTTP rejection case
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
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  - verified formatting is clean
- `cargo test --manifest-path adl/Cargo.toml validate_provider_http_ -- --nocapture`
  - verified provider schema validation enforces the HTTPS-only remote boundary with a loopback exception
- `cargo test --manifest-path adl/Cargo.toml http_provider_ -- --nocapture`
  - verified provider construction/runtime behavior still passes for HTTPS and local loopback harnesses while rejecting plaintext remote HTTP
- Results:
  - formatting passed
  - targeted validation tests passed
  - targeted provider tests passed

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
      - cargo fmt --manifest-path adl/Cargo.toml --all --check
      - cargo test --manifest-path adl/Cargo.toml validate_provider_http_ -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml http_provider_ -- --nocapture
  determinism:
    status: PASS
    replay_verified: false
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
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: the focused `validate_provider_http_` and `http_provider_` test sets
- Fixtures or scripts used: existing provider/unit fixtures plus loopback harnesses in the provider tests
- Replay verification (same inputs -> same artifacts/order): not run as a separate replay harness; this issue is a bounded validation/transport-rule change
- Ordering guarantees (sorting / tie-break rules used): not affected by this change; endpoint acceptance logic is direct and deterministic
- Artifact stability notes: no artifact schema or output-shape changes were introduced

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of changed tracked files and test fixtures; no credentials or token-bearing files were introduced
- Prompt / tool argument redaction verified: yes; no prompts or tool arguments were added to tracked surfaces
- Absolute path leakage check: repository-relative references only in tracked surfaces and this output card
- Sandbox / policy invariants preserved: yes

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: `adl/src/adl/validation.rs`, `adl/src/provider.rs`, and the focused validation/provider test cases
- Required artifacts present: yes
- Artifact schema/version checks: not applicable
- Hash/byte-stability checks: not applicable
- Missing/optional artifacts and rationale: no demo or trace artifact was required because this issue hardens a transport support boundary and its docs/tests

## Decisions / Deviations
- preserved `http` / `http_remote` kind aliases for compatibility instead of renaming the provider kind surface in the same issue
- allowed plaintext loopback HTTP only for local harnesses and tests to avoid introducing unnecessary local TLS complexity
- kept the docs update scoped to the canonical provider substrate feature doc already present on `main`

## Follow-ups / Deferred work
- `#1474` can later extend its generated setup docs to reflect this same HTTPS-only remote boundary once that PR lands
- `#1468` remains the correct issue for actual provider demos/tests built on top of the transport and credential setup surfaces

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
