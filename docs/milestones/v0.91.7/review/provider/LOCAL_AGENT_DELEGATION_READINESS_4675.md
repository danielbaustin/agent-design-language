# Local-Agent Delegation Readiness Proof (#4675)

## Scope

This packet records the v0.91.7 WP-05 local-agent delegation readiness slice for
issue `#4675`.

The implemented surface is the cognitive scheduler's local-agent delegation
readiness contract. It consumes the existing provider profile, model
suitability, and cheapest validated outcome inputs from WP-05, then emits a
role-specific local-agent readiness result inside the scheduler decision.

## Implemented Contract

- New scheduler bundle schema:
  `adl.scheduler.economics_input_bundle.provider_cheapest_validated_local_agent_delegation.v1`
- New local delegation context schema:
  `adl.scheduler.local_agent_delegation_readiness_context.v1`
- New scheduler decision schemas:
  - `adl.scheduler.decision.local_agent_delegation.v1`
  - `adl.scheduler.decision.provider_cheapest_validated_local_agent_delegation.v1`

The context is fail-closed. A candidate is rejected before planning when it:

- is not present in the model suitability candidates
- does not match the model suitability provider/model/runtime identity
- lacks retained evidence
- uses a runtime surface outside the explicit local-agent allowlist
- declares a delegation role not already proven by model suitability
- declares a task role that differs from the task's model-suitability
  requirement
- is not advisory-only
- claims autonomous execution, repo mutation, closeout, or merge authority

When no candidate is eligible for a task, the scheduler emits blocked local
delegation readiness instead of silently granting authority.

## Proof Fixture

Input fixture:

- `adl/tests/fixtures/scheduler/local_agent_delegation_readiness_inputs_v1.json`

Expected decision surface for `first-pass-review`:

- provider route remains present
- cheapest validated outcome model selection remains present
- local-agent readiness is present
- selected local delegate is `local:gemma4-e2b`
- readiness is `shadow_only`
- delegation mode is `shadow_mode`
- `advisory_authority_only` is `true`
- `can_execute_autonomously`, `can_mutate_repo`, and `can_close_or_merge` are
  all `false`

## Validation

Focused scheduler tests:

```text
cargo test --manifest-path adl/Cargo.toml scheduler::tests::local_agent_delegation --lib -- --nocapture
```

Result:

```text
4 passed; 0 failed; 0 ignored; 1566 filtered out
```

The first attempt exposed cold dependency rebuild behavior in the issue
worktree. The dependency warm-cache wrapper then linked 5,863 candidate files
into the issue target before the focused test was rerun.

## Non-Claims

- This does not grant local models review, merge, closeout, conductor, janitor,
  or autonomous repo-mutation authority.
- This does not prove live Ollama runtime quality for future local models.
- This does not replace provider/model suitability evidence; it composes with
  that evidence and records local delegation readiness separately.
