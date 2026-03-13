# Godel Experiment Workflow Template v1

## Purpose
Defines the canonical, deterministic ADL workflow-template contract for the Godel scientific loop:

failure -> hypothesis -> mutation -> experiment -> evaluation -> record

This is now a bounded executable contract as well as a spec artifact.

In v0.8, the runtime consumes the canonical template to derive the executable Gödel stage sequence for:
- stage-loop transition ordering
- runtime/demo stage-order reporting
- record-stage experiment-record contract validation

The template is authoritative for the six scientific-loop stages:

`failure -> hypothesis -> mutation -> experiment -> evaluation -> record`

The runtime then appends a single explicit runtime-managed suffix stage:

`indexing`

This means the template is not a full standalone execution engine, but it does now directly constrain executable behavior and test outcomes.

## Scope and Invariants
- Ordered stages are fixed and explicit.
- Every stage reads declared inputs and emits declared outputs only.
- No hidden state outside declared artifacts.
- Artifacts are replay/audit compatible and repo-relative.
- Policy/control logic is bounded and declarative.

## Canonical Stage Order
1. `failure`
2. `hypothesis`
3. `mutation`
4. `experiment`
5. `evaluation`
6. `record`

## Stage Contracts

### 1) failure
Purpose: normalize failure signal into canonical evidence seed.

Inputs:
- replay-compatible run artifacts
- activation/trace summary artifacts

Outputs:
- `canonical_evidence_view.v1` artifact reference
- failure context summary (deterministic, redacted)

Schema integration:
- `Canonical Evidence View v1` (#610)

### 2) hypothesis
Purpose: derive bounded candidate hypotheses from canonical evidence.

Inputs:
- output from `failure`
- optional prior indexed evidence references (read-only)

Outputs:
- deterministic hypothesis list (ordered, bounded)

Schema integration:
- evidence references remain `canonical_evidence_view` compatible

### 3) mutation
Purpose: map selected hypothesis to bounded candidate mutation(s).

Inputs:
- selected hypothesis
- allowed mutation policy surface

Outputs:
- one or more `mutation.v1` artifacts (deterministically ordered)

Schema integration:
- `Mutation format v1` (#611)

### 4) experiment
Purpose: execute candidate variant experiment plan against baseline context.

Inputs:
- baseline run reference
- candidate `mutation.v1`
- execution constraints/policy gates

Outputs:
- candidate run reference(s)
- experiment evidence references

Schema integration:
- prepares inputs for `evaluation_plan` and `experiment_record`

### 5) evaluation
Purpose: compute deterministic outcome from baseline vs candidate evidence.

Inputs:
- baseline/candidate evidence references
- `evaluation_plan.v1`
- candidate `mutation.v1`

Outputs:
- deterministic decision outcome (`adopt`, `reject`, `requires_human_review`)
- metric and rule evaluation summary

Schema integration:
- `EvaluationPlan v1` (#612)
- `Canonical Evidence View v1` (#610)
- `Mutation format v1` (#611)

### 6) record
Purpose: persist auditable experiment decision record.

Inputs:
- evaluation outputs
- mutation reference
- evidence references
- baseline/candidate run references

Outputs:
- `ExperimentRecord v1` artifact

Schema integration:
- `ExperimentRecord v1` (#609)

## Determinism Requirements
- Stage execution order is fixed and non-branching at template level.
- Input list ordering is explicit and stable.
- Candidate mutation/evaluation ordering is deterministic (explicit tie-break rules).
- Decision classification is declared by `evaluation_plan.v1` only.
- All references are artifact-based; no ambient process state.

## Replay and Audit Compatibility
- Each stage emits replay-relevant artifacts or references.
- Stage outputs are serializable and versioned.
- Stage transitions are auditable by declared input/output references.
- No absolute host paths, secrets, tokens, raw prompts, or tool arguments.

## Integration with #614 and #615
- #614 (ObsMem indexing): template output surfaces are structured for deterministic indexing (`experiment_record`, evidence, mutation references).
- #615 (demo path): this template is directly reusable as the canonical workflow skeleton for demo implementation.

## Non-goals
- Replacing the bounded runtime with a fully template-driven execution engine.
- Mutation strategy optimization algorithms.
- Autonomous policy expansion beyond bounded contracts.
