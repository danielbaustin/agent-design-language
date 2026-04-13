# v0-88-tools-generate-wp-issue-waves-from-canonical-wbs-surfaces

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1667
Run ID: issue-1667
Version: v0.88
Title: [v0.88][tools] Generate WP issue waves from canonical WBS surfaces
Branch: codex/1667-v0-88-tools-generate-wp-issue-waves-from-canonical-wbs-surfaces
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5-codex
- Provider: OpenAI
- Start Time: 2026-04-12T23:55:00Z
- End Time: 2026-04-13T00:33:05Z

## Summary
Added a deterministic `adl tooling generate-wp-issue-wave` control-plane generator that derives a stable WP issue-wave plan from the canonical milestone WBS and sprint docs, along with a checked-in `v0.88` proof artifact and operator documentation. The command is intentionally bounded: it emits planning/bootstrap metadata only and does not create issues, branches, or worktrees.

## Artifacts produced
- `adl/src/cli/tooling_cmd/wp_issue_wave.rs`
- `adl/src/cli/tooling_cmd.rs`
- `adl/src/cli/tooling_cmd/tests.rs`
- `docs/tooling/WP_ISSUE_WAVE_GENERATION.md`
- `docs/tooling/README.md`
- `docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml`

## Actions taken
- Added a new public tooling command, `adl tooling generate-wp-issue-wave`, with explicit `--version`, `--wbs`, `--sprint`, and `--out` handling.
- Implemented deterministic parsing of the canonical WBS work-package table and the sprint-overview table, including support for `WP-02 through WP-08` range expansion.
- Derived stable issue-wave metadata for all WBS rows whose Issue column still says the issue is to be seeded, including title, slug, labels, sprint assignment, dependency notes, and execution-vs-closeout classification.
- Checked in a generated `docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml` proof artifact produced from the live `v0.88` milestone package.
- Added operator-facing documentation describing the command contract, truth boundary, and deterministic behavior.
- Verified the conductor twice around this issue: once before execution, where it correctly routed `pre_run -> pr-run`, and once after implementation, where it still classified the issue as `run_bound` and asked for operator escalation instead of routing to `pr-finish`.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet
- Worktree-only paths remaining: `adl/src/cli/tooling_cmd/wp_issue_wave.rs`, `adl/src/cli/tooling_cmd.rs`, `adl/src/cli/tooling_cmd/tests.rs`, `docs/tooling/WP_ISSUE_WAVE_GENERATION.md`, `docs/tooling/README.md`, `docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct edits in the bound issue worktree before publication
- Verification performed:
  - `git status --short` verified the intended tracked changes are limited to the issue-scoped code/docs/artifact surfaces.
  - `ls docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml` / equivalent path checks verified the generated proof artifact exists in the issue worktree.
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
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml generate_wave_doc_from_v088_surfaces_is_deterministic_and_complete -- --nocapture` verified the generator deterministically derives the full `v0.88` wave from the live canonical WBS and sprint docs.
  - `cargo test --manifest-path adl/Cargo.toml tooling_dispatch_and_help_paths_cover_public_entrypoint -- --nocapture` verified the new command is reachable through the public `adl tooling` dispatch surface.
  - `cargo fmt --manifest-path adl/Cargo.toml --all -- --check` verified the Rust changes are formatted canonically.
  - `cargo run --manifest-path adl/Cargo.toml -- tooling generate-wp-issue-wave --version v0.88 --out <tmp>` followed by `diff -u <tmp> docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml` verified the checked-in `v0.88` wave artifact matches a fresh deterministic regeneration.
  - `python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input.json> --artifact-path .adl/reviews/workflow-conductor-1667-post-implementation.md` verified the conductor can still route the live issue through its canonical schema-driven entrypoint, and surfaced the current post-implementation classification gap.
  - `git diff --check` verified there are no whitespace or malformed patch artifacts in the tracked changes.
- Results: all listed validation commands passed.

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
      - "cargo test --manifest-path adl/Cargo.toml generate_wave_doc_from_v088_surfaces_is_deterministic_and_complete -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml tooling_dispatch_and_help_paths_cover_public_entrypoint -- --nocapture"
      - "cargo fmt --manifest-path adl/Cargo.toml --all -- --check"
      - "cargo run --manifest-path adl/Cargo.toml -- tooling generate-wp-issue-wave --version v0.88 --out <tmp> && diff -u <tmp> docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml"
      - "python3 adl/tools/skills/workflow-conductor/scripts/route_workflow.py --input <tmp-input.json> --artifact-path .adl/reviews/workflow-conductor-1667-post-implementation.md"
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
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: reran the generator-specific Rust test and regenerated the checked-in `v0.88` wave artifact into a temporary path before diffing it against the committed YAML.
- Fixtures or scripts used: `cargo test --manifest-path adl/Cargo.toml generate_wave_doc_from_v088_surfaces_is_deterministic_and_complete -- --nocapture`, `cargo run --manifest-path adl/Cargo.toml -- tooling generate-wp-issue-wave --version v0.88 --out <tmp>`, and `diff -u`.
- Replay verification (same inputs -> same artifacts/order): identical WBS and sprint inputs produce the same ordered wave entries and the same emitted YAML content for `docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml`.
- Ordering guarantees (sorting / tie-break rules used): WBS rows preserve canonical WBS order, sprint mapping expands range notation deterministically, and range-derived dependency refs are emitted in ascending WP order.
- Artifact stability notes: the command does not read GitHub or timestamps, so the output is controlled entirely by the tracked WBS and sprint docs.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the new tooling module, generated YAML, and docs for credentials or tokens; none were introduced.
- Prompt / tool argument redaction verified: yes; the generated wave artifact records only milestone planning metadata and does not persist prompts, secrets, or arbitrary tool arguments.
- Absolute path leakage check: `git diff --check` passed, and the tracked docs/artifact use repository-relative paths only.
- Sandbox / policy invariants preserved: yes; the generator remains planning-only and does not create issues, bind worktrees, or mutate GitHub state.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this tooling issue does not produce a runtime trace bundle.
- Run artifact root: `docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml`
- Replay command used for verification: `cargo run --manifest-path adl/Cargo.toml -- tooling generate-wp-issue-wave --version v0.88 --out <tmp>` followed by `diff -u <tmp> docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml`
- Replay result: pass; the checked-in artifact matched a fresh regeneration byte-for-byte.

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.88/WP_ISSUE_WAVE_v0.88.yaml`
- Required artifacts present: yes; the new tooling module, dispatch/docs updates, and the generated `v0.88` wave artifact are present in the issue worktree.
- Artifact schema/version checks: the generated artifact declares `schema: adl.wp_issue_wave.v1`, and the public docs describe the bounded input/output contract.
- Hash/byte-stability checks: no separate hash file was added; deterministic regeneration and `diff -u` were used as the byte-stability check.
- Missing/optional artifacts and rationale: no GitHub-created issue bundle is expected here because issue creation/init enforcement belongs to follow-on control-plane issues in the same tranche.

## Decisions / Deviations
- Kept the new generator under the existing `adl tooling` public surface rather than embedding it into the conductor or `pr.sh`, so the output remains a bounded planning artifact instead of a hidden execution engine.
- Emitted only rows whose WBS Issue column still says they are to be seeded, which keeps the wave artifact focused on the still-pending canonical work-package issues.
- Preserved the existing `v0.88` public issue wave as repo truth instead of trying to retroactively mutate GitHub from this issue.
- The source issue prompt still references `.adl/docs/TBD/V0_88_WP_READINESS_QUEUE.md`, but that file does not exist in current repo state; the implementation used the live WBS and sprint docs directly and records that stale input reference here as a repo-process note.
- The conductor remained helpful before execution, but after implementation it still classified the live issue as `run_bound` with `open_pr_wave_only` and routed to `pr-run`/`ask_operator` instead of `pr-finish`; that maturity gap is recorded rather than widened into another conductor refactor here.

## Follow-ups / Deferred work
- Create/init enforcement that consumes the generated wave and turns it into ready bootstrap issue surfaces remains owned by follow-on issues in the `#1665` control-plane tranche.
- The conductor still needs stronger post-implementation/publication-state detection so completed issues route to `pr-finish` more reliably without operator-shaped payloads.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
