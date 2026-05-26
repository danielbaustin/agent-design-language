# example-sor-repair

Canonical Template Source: `docs/templates/prompts/1.0.0/sor.md`

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-4005
Run ID: issue-4005
Version: v0.91.4
Title: [example][SOR] Repaired PR-open truth
Branch: codex/4005-example-sor-repair
Card Status: ready
Status: DONE
Generated: 2026-05-26T12:00:00Z

Execution:
- Actor: codex
- Model: gpt-5
- Provider: OpenAI Codex desktop
- Start Time: 2026-05-26T12:00:00Z
- End Time: 2026-05-26T12:15:00Z

## Summary

Illustrative repaired execution record showing the correct `pr_open` shape
without prematurely claiming merge or closeout. This is an example surface, not
a live issue closeout record.

## Artifacts produced
- Updated repair-example docs:
  - `docs/tooling/csdlc-prompt-editor/repair_examples/README.md`
  - `docs/tooling/csdlc-prompt-editor/repair_examples/sor_repaired_pr_open.md`
- Focused proof script:
  - `adl/tools/test_card_editor_repair_examples.sh`

## Actions taken
- Normalized the output record to the correct `pr_open` shape for an
  illustrative repaired example.
- Removed premature completion language and absolute-path leakage.
- Recorded only the commands that actually ran for the example proof lane.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet; this illustrative example intentionally stops before merge or closeout truth
- Worktree-only paths remaining: example tracked changes still model a branch-local `pr_open` state
  - `docs/tooling/csdlc-prompt-editor/repair_examples/README.md`
  - `adl/tools/test_card_editor_repair_examples.sh`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: illustrative repaired `SOR` shape showing how an open-PR execution record should read before terminal closeout; no live GitHub issue or PR is implied by this example
- Verification performed:
  - `bash adl/tools/test_card_editor_repair_examples.sh`
    Verified the repaired examples pass validation and the overclaim cases fail closed.
  - `git diff --check`
    Verified the example diff is patch-clean.
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
  - `bash adl/tools/test_card_editor_repair_examples.sh`
    Verified repaired-card examples for SIP, STP, SPP, SRP, and SOR, plus fail-closed overclaim cases for SRP and SOR.
  - `git diff --check`
    Verified no whitespace or patch-formatting defects in the tracked example surfaces.
- Results:
  - PASS

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
      - "bash adl/tools/test_card_editor_repair_examples.sh"
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
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: focused repair-example proof script.
- Fixtures or scripts used: `adl/tools/test_card_editor_repair_examples.sh`.
- Replay verification (same inputs -> same artifacts/order): verified through repeated validator checks over tracked examples and deterministic fail-closed mutations.
- Ordering guarantees (sorting / tie-break rules used): the script validates the same example set in a fixed order each run.
- Artifact stability notes: the examples use repository-relative paths only and avoid machine-local assumptions.

## Security / Privacy Checks
- Secret leakage scan performed: bounded content review only; no secrets or credentials were introduced.
- Prompt / tool argument redaction verified: yes; recorded commands use repository-relative paths only.
- Absolute path leakage check: pass; no unjustified host absolute paths are recorded in this example.
- Sandbox / policy invariants preserved: yes; example remains at `pr_open` and does not invent merge truth.

## Replay Artifacts
- Trace bundle path(s): not_applicable
- Run artifact root: not_applicable
- Replay command used for verification: not_applicable; the focused proof script already covers deterministic replayability for this example surface.
- Replay result: PASS

## Artifact Verification
- Primary proof surface: the focused repair-example validator script.
- Required artifacts present: yes; all tracked repaired examples and the proof script exist.
- Artifact schema/version checks: the structured prompt validator passed for the repaired examples.
- Hash/byte-stability checks: not_run
- Missing/optional artifacts and rationale: no browser demo or production artifact is required for this example-only proof surface.

## Decisions / Deviations
- This example intentionally stops at `pr_open` shape to show the correct non-terminal SOR state after repair.
- It does not claim merge, closeout, or main-branch integration.

## Follow-ups / Deferred work
- Use the real issue-local `SOR` during publication and closeout instead of reusing this example.
