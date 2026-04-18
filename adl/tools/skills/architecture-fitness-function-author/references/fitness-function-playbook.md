# Architecture Fitness Function Playbook

Use this reference when turning architecture findings into executable checks.

## Candidate Types

- `dependency_rule`: dependency direction, package boundary, forbidden import,
  or module ownership rule.
- `contract_test`: runtime lifecycle, state transition, provider boundary, or
  artifact contract that can be checked with a focused test.
- `docs_check`: documented command, architecture claim, or milestone truth check.
- `ci_gate`: wrapper that runs existing deterministic checks in CI.
- `repo_policy_check`: repo-specific script or lint that enforces local policy.
- `manual_review_gate`: rule needs human architectural judgment before
  automation.

## Classification Rules

- Use `machine_checkable` when a deterministic local command can pass/fail the
  invariant without external services or human interpretation.
- Use `human_judgment` when the invariant needs an ADR, architecture decision,
  or reviewer interpretation before automation.
- Use `deferred` when evidence is insufficient, the target path is unknown, or
  implementation would require production services, credentials, network, or
  unbounded mutation.

## Failure-Mode Guidance

- Dependency rule: name the forbidden source and target direction.
- Contract test: name the expected transition, state, or artifact field.
- Docs check: name the stale claim or command truth being guarded.
- CI gate: name the command and why a failure should block merge.
- Repo policy check: name the local policy and expected diagnostic.
