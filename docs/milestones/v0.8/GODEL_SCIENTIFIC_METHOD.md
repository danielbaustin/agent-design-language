# Gödel Agent Scientific Method

For cross-document alignment, see `GODEL_LOOP_INTEGRATION_V0.8.md` for the canonical v0.8 stage order and artifact mapping.

## Purpose

The Gödel Agent introduces a **scientific reasoning loop** into ADL. Instead of merely executing workflows, the system can analyze prior runs, form hypotheses about improvements, test those hypotheses deterministically, and adopt or reject changes based on evidence.

This mechanism enables **controlled self‑improvement** while preserving the core ADL guarantees:

- deterministic replay
- auditability
- policy‑gated behavior
- privacy‑safe artifacts

The Gödel Agent therefore acts as the "scientist" of the ADL runtime.

---

# Architectural Context

The scientific loop builds on infrastructure delivered in earlier milestones.

```
Runtime Execution
        ↓
Deterministic Trace Bundles
        ↓
ObsMem (Operational Memory)
        ↓
Gödel Scientific Loop
```

Where:

- **Trace bundles** capture deterministic evidence of system behavior.
- **ObsMem** stores indexed knowledge derived from traces.
- **Gödel** reasons over that evidence and proposes improvements.

## Canonical Schema/Spec Artifacts (Design Stage)

The following machine-readable schema/spec artifacts are canonical for v0.8 design work:

- `docs/milestones/v0.8/agent_profile.v1.json`
- `docs/milestones/v0.8/mutation.v1.json`

These are design-stage schema artifacts now and are candidates for later promotion into a runtime schema directory.
Any historical planning copies are non-canonical; source of truth is `docs/milestones/v0.8/`.

---

# Core Concept: Experiment Records

To support safe recursive improvement, ADL introduces a canonical artifact:

**ExperimentRecord**

This artifact records a complete improvement experiment.

## Fields

experiment_id
: Stable identifier for the experiment.

baseline_run_id
: The run used as the comparison baseline.

variant_run_id
: The run executed with the proposed change.

hypothesis
: Natural language description of the proposed improvement.

mutation
: Machine‑readable summary of the change applied.

Examples:

- retry strategy modification
- provider switch
- prompt rewrite
- workflow structural change

evaluation_plan
: Deterministic procedure used to judge success.

Examples:

- run verification hooks
- compile checks
- schema validation
- metric comparison

evidence
: Structured result of evaluation.

Includes:

- pass/fail
- metric deltas
- artifact comparisons

decision
: Outcome of the experiment.

Possible values:

- adopt
- reject
- requires_human_review

policy_approval_ref
: Reference to the delegation policy decision that permitted the mutation.

---

# Canonical Evidence Model

Gödel reasoning must operate on **canonical artifacts**, not raw runtime outputs.

This ensures replay determinism.

## Canonical View

A canonical representation of run evidence consists of:

- activation log (schema frozen)
- trace bundle manifest
- canonical artifact hashes
- failure codes
- verification results

Volatile fields (timestamps, run IDs, host paths) are excluded from comparison.

---

# Scientific Loop

The Gödel Agent executes a deterministic improvement loop.

```
1. Observe

   Query ObsMem for prior runs, failures, and patterns.

2. Hypothesis

   Generate a candidate improvement.

3. Experiment

   Execute a variant run with the mutation applied.

4. Verify

   Run deterministic verification hooks.

5. Evaluate

   Compare canonical evidence against the baseline.

6. Decide

   Adopt, reject, or request human approval.
```

Canonical stage naming for v0.8 docs:

- failure -> hypothesis -> mutation -> experiment -> evaluation -> record -> indexing

Each step emits trace events and produces artifacts.

## v0.8 Scope Boundary

This document describes the deterministic scientific-loop architecture and artifact contracts. It does not claim that autonomous self-modifying runtime execution is already implemented in v0.8.

---

# Hypothesis Generation Sources

A critical question for the Gödel Agent is **where improvement hypotheses originate**.  
The system must not generate arbitrary mutations; hypotheses must arise from
observable evidence in prior executions.

- failure analysis (stable error codes)
- retry pattern observation (sticktoitiveness traces)
- performance regression signals
- cross-workflow learning via ObsMem
- human-seeded hypotheses

## Hypothesis Generation Mechanisms

This section describes **how** Gödel turns evidence into candidate hypotheses.

### Mechanism A: Failure clustering → repair hypotheses

1. Collect recent failures from the canonical trace model (stable failure codes).
2. Cluster by `(failure_code, step_id, tool_id/provider_profile)`.
3. For the top clusters, generate a bounded set of candidate repairs.

Examples of bounded repairs:

- adjust retry strategy for a specific failure code
- switch provider profile for a specific step
- insert a verification hook before/after a risky step
- tighten/relax a sandbox permission that is blocking execution (policy-gated)

Output:

- `hypothesis`: short natural language statement
- `mutation`: machine-readable patch summary (see Mutation format below)
- `evaluation_plan`: deterministic checks for success

### Mechanism B: Counterfactual replay → minimal mutation search

1. Select a baseline run with a stable replay.
2. Enumerate a small, policy-approved mutation set.
3. Run replay/variant runs under strict determinism.
4. Choose the smallest mutation that improves the canonical evidence score.

This is effectively "unit-testing" candidate improvements against a replayable baseline.

### Mechanism C: Pattern mining across runs

1. Index run summaries and experiment records in ObsMem.
2. Mine for associations:
   - "provider X tends to succeed on task type Y"
   - "verification hook Z catches error class W early"
3. Convert associations into reusable hypotheses for new workflows.

### Mechanism D: Human-seeded hypotheses with automated verification

Humans can propose high-leverage changes (architecture, prompts, workflow structure).

Gödel’s role is to:

- translate the suggestion into a bounded mutation
- build an evaluation plan
- run a deterministic experiment
- record the result as an ExperimentRecord

### Mechanism E: Novelty / uncertainty triggers

Even without failure, Gödel may propose experiments when signals cross thresholds:

- rising attempt counts
- rising latency
- increasing artifact churn
- oscillating strategies (no convergence)

These triggers should be conservative in v1 and always produce policy-gated, bounded experiments.

---

# Gödel Schema Artifacts (Design‑Stage)

The Gödel planning work references two **canonical JSON schema artifacts** that define
key structures used by the experimental self‑improvement system.

Canonical locations (v0.8 milestone docs):

- `docs/milestones/v0.8/agent_profile.v1.json`
- `docs/milestones/v0.8/mutation.v1.json`

These schemas currently represent **design‑stage artifacts** for the planned
Gödel evolution surface. They are used to document the structure of:

- evolvable agent behavior profiles
- bounded mutation records used in improvement experiments

At this stage they are treated as **documentation‑level schemas** rather than
runtime‑enforced contracts.

Future milestones may promote these artifacts into runtime schemas under
`swarm/schemas/` once the Gödel runtime components are implemented.

The legacy planning copies under `.adl/docs/v07planning/` remain for historical
reference, but the canonical reference for v0.8 documentation is the
`docs/milestones/v0.8/` location.

---

## Mutation format (v1)

To avoid arbitrary changes, Gödel mutations should use a compact structured representation.

A mutation record MUST include:

- `mutation_id` (stable)
- `scope` (step/tool/workflow)
- `type` (retry_strategy | provider_switch | prompt_edit | workflow_edit | permission_request | verify_hook)
- `summary` (human-readable)
- `patch` (machine-readable change descriptor)

Rules:

- mutations must be **bounded** (small diff, explicit scope)
- mutations must be **policy-gated** (Delegation Policy Surface)
- mutations must be **replay-evaluable** (canonical evidence)

---

## Evaluation plan format (v1)

An evaluation plan MUST specify deterministic checks, for example:

- compile / fmt / clippy / tests
- schema validation
- demo matrix run for the impacted demo(s)
- artifact hash comparison (canonical view)
- failure code stability check

Plans should be minimal and targeted, but sufficient to justify an adopt/reject decision.

---

## Scoring and selection (v1)

Gödel must select among candidate hypotheses deterministically.

A simple v1 selection rule:

1. Prefer hypotheses that change **the smallest scope**.
2. Prefer hypotheses with **strongest prior evidence** (from ExperimentRecords).
3. Prefer hypotheses that improve **correctness** over performance.
4. Use a stable tie-breaker (lexicographic by `mutation_id`).

This keeps the system reproducible and avoids hidden randomness.

---

# Example ExperimentRecord (v1)

This example shows a complete, replay-evaluable experiment that fixes a deterministic failure by inserting a verification hook and tightening canonicalization.

## Scenario

- Baseline run fails with a stable error code: `SIGN_POLICY_UNSIGNED_REQUIRED`.
- Hypothesis: the workflow should either be signed (production) or explicitly run with a dev allowance flag.
- Mutation: insert a verification hook that enforces a policy-consistent run mode and records the decision.

## ExperimentRecord (illustrative)

```yaml
experiment_id: exp_0007_sign_policy_dev_mode
baseline_run_id: run_2026_03_03T171233Z_3f8c
variant_run_id: run_2026_03_03T171945Z_9a21
hypothesis: >-
  If the workflow is unsigned, then running under an explicit dev allowance
  (or signing the workflow) will eliminate SIGN_POLICY_UNSIGNED_REQUIRED while
  preserving deterministic replay and auditability.
mutation:
  mutation_id: mut_0007_insert_verify_hook_signed_or_allow
  scope: workflow
  type: verify_hook
  summary: >-
    Insert verify hook to enforce signature policy handling (signed required)
    with an explicit dev-mode allowance path for local examples.
  patch:
    kind: insert_step
    before_step_id: run_workflow
    step:
      step_id: verify_signature_policy
      tool: adl.verify
      args:
        policy: signature
        mode: dev_allow_unsigned_for_examples
        record_decision: true
        emit_failure_code_on_violation: SIGN_POLICY_UNSIGNED_REQUIRED

evaluation_plan:
  plan_id: eval_0007_sign_policy
  checks:
    - kind: demo_matrix
      demos:
        - v0-7-hierarchical-planner
    - kind: replay
      mode: strict
      invariants:
        - stable_failure_code
        - stable_activation_log_order
        - stable_artifact_hashes
    - kind: workspace
      commands:
        - cargo fmt --all
        - cargo clippy --workspace --all-targets -- -D warnings
        - cargo test --workspace

evidence:
  result: pass
  baseline:
    status: failed
    stable_failure_code: SIGN_POLICY_UNSIGNED_REQUIRED
    key_artifacts:
      activation_log_sha256: 6a2d...c19
      trace_bundle_manifest_sha256: 1d9b...a02
  variant:
    status: succeeded
    stable_failure_code: null
    key_artifacts:
      activation_log_sha256: 6a2d...c19
      trace_bundle_manifest_sha256: 4b77...9ef
  comparisons:
    - kind: canonical_artifact_hashes
      verdict: pass
      notes: >-
        Canonical outputs match determinism invariants; volatile fields excluded.
    - kind: activation_log_order
      verdict: pass
    - kind: replay
      verdict: pass

decision: adopt
policy_approval_ref:
  policy_decision_id: pol_00041
  outcome: approved
  rationale: >-
    Mutation does not widen capabilities; it adds a verification step and
    constrains execution to an explicit, audited mode.
```

## Canonical evidence note

In this example, the *baseline* and *variant* may differ in volatile fields (run IDs, timestamps). The evidence comparison must use the **Canonical View** so Gödel’s adopt/reject decision is reproducible.

## Connection to Hadamard's Model of Discovery

The structure of this system parallels the stages of creative discovery
described by mathematician **Jacques Hadamard**. This frames Gödel as an operationalized creativity-and-verification loop, echoing Hadamard’s account in *The Psychology of Invention in the Mathematical Field*:

- **Preparation** — gathering evidence from prior runs (ObsMem)
- **Incubation** — pattern detection across traces
- **Illumination** — generation of a candidate hypothesis
- **Verification** — deterministic experiment and evaluation

The Gödel Agent operationalizes this model in software: hypotheses are not
mystical insights but **trace‑grounded proposals that can be experimentally
tested**.

---

# Integration with Sticktoitiveness

The Gödel loop complements the **Sticktoitiveness Engine**.

Sticktoitiveness handles:

- retry logic
- strategy mutation
- verification hooks

Gödel provides:

- higher‑level reasoning
- experiment planning
- cross‑run learning

Together they form a layered improvement system.

```
Retry Engine
    ↓
Verification Hooks
    ↓
Experiment Records
    ↓
Gödel Reasoning
```

---

# Safety Properties

The design preserves the core ADL guarantees.

## Determinism

All experiments must be replayable.

Strict determinism mode ensures identical canonical artifacts across runs.

## Policy Gating

Mutations are allowed only if approved by the Delegation Policy Surface.

Examples:

- provider change
- tool capability change
- workflow structural mutation

## Privacy

Artifacts must never contain:

- prompts
- tool arguments
- secrets
- absolute filesystem paths

## Auditability

Every mutation is recorded via ExperimentRecord.

This creates a complete improvement history.

---

# Relationship to Popper's Three Worlds

This architecture introduces a form of **system self‑reflection**.

World 1
: Runtime execution and artifacts.

World 3
: Workflows, policies, and experiment records.

Gödel introduces a bridge toward **World 2‑like internal reasoning**, where the system forms and evaluates hypotheses about its own behavior.

This does not imply consciousness, but it creates a structural substrate for introspective reasoning.

---


# Experiment Execution Model

Gödel experiments are executed as **ADL workflows themselves**.  
This preserves the core ADL properties of determinism, traceability, and policy control.

Instead of embedding experimental logic inside the runtime, the Gödel Agent
constructs and launches a structured **experiment workflow**.

This makes the scientific loop transparent and auditable.

## Experiment Workflow Structure

A Gödel experiment workflow typically contains the following phases:

1. Baseline Retrieval

   Retrieve the baseline run referenced by `baseline_run_id`.

   Required artifacts:

   - canonical trace bundle
   - canonical artifact hashes
   - verification results

2. Mutation Application

   Apply the proposed mutation to the workflow configuration.

   Examples:

   - modify retry strategy
   - switch provider profile
   - alter prompt template
   - adjust workflow structure

   This mutation must be approved by the **Delegation Policy Surface**.

3. Variant Execution

   Execute the modified workflow under deterministic conditions.

   The run produces a **variant trace bundle**.

4. Verification Phase

   Run deterministic verification hooks.

   Examples:

   - compile checks
   - test suites
   - schema validation
   - artifact comparisons

5. Evidence Comparison

   Compare baseline and variant canonical evidence.

   Metrics may include:

   - success/failure status
   - execution attempts
   - latency
   - artifact differences

6. Decision Recording

   Produce the final **ExperimentRecord** artifact.

   This artifact records:

   - hypothesis
   - mutation
   - evidence
   - decision

## Deterministic Experiment Requirements

All Gödel experiments must satisfy the ADL determinism guarantees.

Required properties:

- canonical artifacts must be stable
- experiment workflows must be replayable
- evidence comparisons must ignore volatile fields

This ensures experiments remain **scientifically reproducible**.

## Recursive Operation

Because experiments are ADL workflows, the Gödel Agent can reason about
its own improvement experiments.

This enables a recursive improvement structure:

```
workflow execution
        ↓
trace bundle
        ↓
ObsMem indexing
        ↓
Gödel hypothesis
        ↓
experiment workflow
        ↓
ExperimentRecord
```

Over time this produces a **library of verified improvements**.

This knowledge base becomes a form of accumulated system understanding,
allowing future Gödel reasoning to build upon prior experiments.

---

# Future Work (v0.8+)

Potential extensions include:

- experiment prioritization
- novelty detection
- strategy confidence tracking
- cross‑workflow learning
- recursive improvement planning

These capabilities build on the same scientific loop described above.
