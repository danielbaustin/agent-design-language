# Cheapest Validated Outcome Policy Proof - #4674

## Summary

Issue `#4674` implements a bounded cheapest-validated-outcome policy inside the combined scheduler/provider/model suitability path.

The policy is not a live price lookup and not autonomous authority. It is a deterministic scheduling policy that selects the lowest-cost candidate only after the candidate is already eligible for the requested model-suitability role and has retained validation evidence.

## Integrated Surfaces

- `adl/src/scheduler.rs`
  - Adds `adl.scheduler.economics_input_bundle.cheapest_validated_outcome.v1`.
  - Adds `adl.scheduler.economics_input_bundle.provider_cheapest_validated_outcome.v1`.
  - Adds `adl.scheduler.cheapest_validated_outcome_policy.v1`.
  - Adds `adl.scheduler.decision.cheapest_validated_outcome.v1`.
  - Adds `adl.scheduler.decision.provider_cheapest_validated_outcome.v1`.
  - Validates retained cost/validation evidence before scheduling.
  - Requires policy cost evidence to name the candidate's own retained model-suitability source.
  - Allows the combined provider-route, model-suitability, and cheapest-validated policy path in one scheduler bundle.
  - Selects cheapest validated eligible candidates before falling back to suitability priority tie-breaks.
- `adl/src/chronosense/long_running_proof.rs`
  - Keeps existing scheduler bundle construction explicit by setting the new policy field to `None`.
- `adl/tests/fixtures/scheduler/cheapest_validated_outcome_inputs_v1.json`
  - Proves the combined provider-route plus cheapest policy path.
  - Proves the policy chooses the cheaper validated Gemini candidate over the higher-priority OpenRouter candidate.
- `docs/milestones/v0.91.7/review/provider/artifacts/cheapest_validated_cost_table_4674.json`
  - Retains the bounded cost-tier evidence used by the fixture.
- `docs/milestones/v0.91.7/review/provider/artifacts/cheapest_validated_outcome_plan_4674.json`
  - Retains the generated scheduler plan proof.

## Policy Behavior

For each task policy, the scheduler now requires:

- a matching model-suitability task requirement
- retained candidate evidence
- retained validation reference
- `validated_outcome=true`
- candidate source reference matching the candidate's own retained model-suitability source
- candidate cost at or below the task policy maximum
- an approved bounded policy claim boundary

The selected candidate is ordered by:

1. lowest retained outcome cost tier
2. strongest model-suitability classification
3. highest suitability priority
4. deterministic provider/model/candidate tie-breakers

## Proof Result

The retained proof fixture contains three reviewer candidates:

- `openrouter:gpt-5.4`: useful with limits, high suitability priority, high cost
- `gemini:gemini-2.5-flash`: useful with limits, lower suitability priority, low cost
- `local:gemma4-e2b`: low cost, but historical-only classification

The generated decision keeps a reviewer provider route for `http:gemini-2.5-flash` and selects `gemini:gemini-2.5-flash` as the model-suitability candidate because it is the cheapest validated candidate that still satisfies the reviewer role requirement.

## Validation

Local focused validation:

```text
cargo test --manifest-path adl/Cargo.toml scheduler::tests::cheapest_validated_outcome --lib -- --nocapture
```

Result: `4 passed`.

Local scheduler regression validation:

```text
cargo test --manifest-path adl/Cargo.toml scheduler::tests --lib -- --nocapture
```

Result: `34 passed`.

Local CLI proof:

```text
ADL_OBSERVABILITY_LOG=$TMPDIR/adl-4674-cheapest-policy.log \
  adl/target/debug/adl scheduler plan \
  --input adl/tests/fixtures/scheduler/cheapest_validated_outcome_inputs_v1.json \
  --out docs/milestones/v0.91.7/review/provider/artifacts/cheapest_validated_outcome_plan_4674.json \
  --json
```

Result: generated `cheapest_validated_outcome_plan_4674.json`.

JSON assertion:

```text
cheapest_validated_outcome_policy_assertions=pass
```

## Non-Claims

- No live provider pricing lookup is claimed.
- No hosted provider invocation is claimed.
- No autonomous merge, closeout, or operator-authority delegation is claimed.
- Cost tiers are bounded retained proof inputs, not exact prices.
