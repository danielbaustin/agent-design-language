# ADL Output Card

Task ID: issue-0999
Run ID: issue-0999
Version: v0.85
Title: dependable-execution-fixture
Branch: codex/999-dependable-execution-fixture
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: local workspace
- Start Time: 2026-03-18T07:00:00Z
- End Time: 2026-03-18T07:05:00Z

## Summary

Fixture output record for dependable-execution completed-phase validation.

## Artifacts produced
- `docs/tooling/examples/workflow-state/good_output_record.md`

## Actions taken
- Recorded a valid completed output state.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `docs/tooling/examples/workflow-state/good_output_record.md`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: worktree branch committed and opened as draft PR
- Verification performed:
  - `git status`
- Result: PASS

## Validation
- Tests / checks run:
  - `ruby swarm/tools/validate_structured_prompt.rb --type sor --phase completed --input docs/tooling/examples/workflow-state/good_output_record.md`
- Results:
  - pass

## Verification Summary
```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "completed output validation fixture"
  determinism:
    status: PASS
    replay_verified: unknown
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
- Determinism tests executed: completed-phase validator
- Replay verification (same inputs -> same artifacts/order): not applicable
- Ordering guarantees (sorting / tie-break rules used): fixed section and field ordering
- Artifact stability notes: fixture is checked in for deterministic reruns

## Security / Privacy Checks
- Secret leakage scan performed: yes
- Prompt / tool argument redaction verified: yes
- Absolute path leakage check: pass
- Sandbox / policy invariants preserved: yes

## Replay Artifacts
- Trace bundle path(s):
- Run artifact root:
- Replay command used for verification:
- Replay result:

## Artifact Verification
- Required artifacts present: yes
- Artifact schema/version checks: completed-phase validator pass
- Hash/byte-stability checks: not run
- Missing/optional artifacts and rationale: replay artifacts are not required for this fixture

## Decisions / Deviations
- None.

## Follow-ups / Deferred work
- None.
