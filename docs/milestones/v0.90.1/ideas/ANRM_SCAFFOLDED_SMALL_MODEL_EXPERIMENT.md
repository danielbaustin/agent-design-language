# ANRM Scaffolded Small-Model Experiment

## Status

v0.90.1 sidecar experiment plan.

This note does not make ANRM part of the v0.90.1 Runtime v2 implementation
surface. It preserves the ANRM training thesis and defines the smallest useful
experiment for deciding whether ADL should invest in a Gemma-family
ADL-native reasoning model.

## Thesis

ANRM is valuable if ADL can make smaller or local models useful for serious
runtime work by surrounding them with ADL structure.

The first claim to test is:

> ADL scaffolding increases the effective aptitude of a smaller model on a
> bounded operational task.

The second claim to test is:

> The CSM runtime may need a dedicated shepherd model that watches state,
> invariants, trace, and operator intent without being the same model that
> performs ordinary task work.

The training claim remains live:

> If scaffolded Gemma improves under ADL structure, a Gemma-family model may be
> fine-tuned on ADL trace-derived examples to internalize some of that scaffold.

## Why This Matters

If ANRM works, ADL gets a practical cognitive economy:

- frontier models can focus on high-judgment work
- smaller models can handle shepherding, routing, review preflight, test
  generation, trace reading, and maintenance tasks
- local/private execution becomes more credible for customer repositories
- Aptitude Atlas gets a real way to measure scaffolded aptitude rather than
  raw model reputation
- Runtime v2 can separate kernel guarantees from semantic shepherding

The goal is not to prove that a small model is a frontier model. The goal is to
measure whether ADL structure makes a smaller model reliable enough for bounded
roles.

## Experiment Boundary

This pass should not train a production ANRM.

This pass should produce:

- a task-family choice
- a subject matrix
- a scaffold definition
- a scoring rubric
- a proof packet or rehearsal packet
- a Gemma training path
- a CSM shepherd-role assessment

Training remains important, but it should start only after the baseline and
scaffolded comparison are clear enough to evaluate whether training helped.

## First Task Family

Recommended first task family: contract-following with CSM shepherd flavor.

The fixture should ask the subject to inspect a small Runtime v2 event packet
and classify whether the runtime should proceed, pause, reject, or ask for
operator input.

Why this beats a pure repo-review first run:

- it is closer to the shepherd-model hypothesis
- it tests instruction following, invariant awareness, and refusal quality
- it is small enough for local models
- it does not duplicate CodeBuddy
- it still feeds Aptitude Atlas as an aptitude test family

Fixture requirements:

- one valid event that should proceed
- one invariant violation that should be rejected
- one ambiguous event that should pause for operator input
- one tempting false positive that should not be rejected
- one trace or card-truth wrinkle that should be reported but not overclaimed

## Subject Matrix

The first comparison should include:

| Subject | Purpose | Expected Output |
| --- | --- | --- |
| Raw Gemma-family model | Baseline small-model capability | Classification and rationale without ADL scaffold |
| ADL-scaffolded Gemma-family model | Test scaffolded aptitude | Classification, invariant references, evidence, caveats |
| Optional frontier model | Reference ceiling | Same output shape for comparison |
| Optional ADL review/synthesis team | Multi-agent scaffold reference | Same output shape plus role-specific caveats |

The Gemma-family model should be treated as the primary training candidate.

## Scaffold Definition

The ADL scaffold should include:

- role: CSM shepherd candidate
- task contract: classify event as proceed, pause, reject, or ask operator
- invariant packet: the small subset of Runtime v2 invariants relevant to the
  fixture
- trace packet: event id, source, timestamp, causal parent, proposed mutation
- policy packet: allowed actions, forbidden actions, escalation rules
- output schema: decision, severity, evidence, invariant references,
  uncertainty, recommended next action
- review loop: second pass checks hallucinated evidence, missing caveats, and
  overbroad rejection

The scaffold should not provide the answer. It should provide the cognitive
rails that a shepherd model would have in the real runtime.

## Scoring Rubric

Score each subject on:

- correct decision
- invariant awareness
- evidence quality
- false-positive restraint
- refusal or pause quality
- uncertainty handling
- schema compliance
- repair burden
- latency and cost when measurable

Do not collapse these into one mythology score. Aptitude Atlas can later decide
how to present weighted summaries.

## Proof Packet Shape

The first proof packet should include:

- subject manifest
- fixture manifest
- scaffold manifest
- raw output
- scaffolded output
- evaluator notes
- scorecard
- CSM shepherd assessment
- Gemma training path
- demo classification: proving, non-proving, skipped, or failed

If no live model run happens in this issue, classify the result as non-proving
and leave a runnable protocol.

## Gemma Training Path

Training is not discarded. It is the likely second step if the scaffolded run
shows promise.

Minimum useful Gemma fine-tuning experiment:

1. Select base model
   - Gemma-family instruct model that fits available local hardware.
   - Prefer a size that can be trained with LoRA or QLoRA on a 24 GB GPU.

2. Build a tiny ADL-native dataset
   - 100 to 500 examples first.
   - Sources: validated trace snippets, task cards, review findings, demo proof
     packets, and CSM fixture decisions.
   - Each example must include input, expected output, evidence references, and
     why the output is correct.

3. Split the dataset
   - train
   - validation
   - held-out evaluation
   - adversarial or trap cases

4. Train the smallest useful adapter
   - LoRA or QLoRA first.
   - Do not train a full model until adapter evidence is strong.

5. Evaluate against three baselines
   - raw Gemma
   - prompted/scaffolded Gemma
   - fine-tuned plus scaffolded Gemma

6. Promotion gate
   - Fine-tuning is useful only if it reduces repair burden or improves
     decision accuracy beyond scaffold-only prompting on held-out fixtures.

Do not train on arbitrary web text. The ANRM thesis is trace-derived ADL
cognition, not generic language modeling.

## CSM Shepherd Model Assessment

A dedicated CSM shepherd model would not be the kernel.

The kernel owns hard guarantees:

- state mutation ordering
- trace append
- invariant enforcement hooks
- snapshot and rehydrate mechanics
- capability and permission checks

The shepherd model would own semantic monitoring and operator-facing judgment:

- explain why a proposed event is risky
- notice when an invariant check needs human review
- summarize manifold health
- classify ambiguous transitions
- recommend pause, reject, or ask-operator actions
- inspect trace for stale or contradictory state
- help schedule attention across citizens

The shepherd must never silently mutate manifold state. It recommends and
explains; the kernel decides what can happen.

Initial assessment:

- CSM probably does need a shepherd role.
- ANRM is a plausible candidate for that role because shepherding is repetitive,
  structured, evidence-heavy, and strongly tied to ADL trace semantics.
- We do not yet have enough evidence to require ANRM as a Runtime v2 component.

## Follow-On Issues

If the experiment is promising, split follow-up work into:

- CSM shepherd fixture and evaluator implementation
- Gemma local training feasibility spike
- ADL trace-to-Gemma dataset builder
- ANRM LoRA or QLoRA proof run
- Aptitude Atlas scaffolded-model scorecard integration
- Runtime v2 shepherd-report surface

## Non-Claims

- This does not prove first true Gödel-agent birth.
- This does not make ANRM a production model.
- This does not replace frontier models.
- This does not make CSM dependent on an untrained shepherd model.
- This does not complete the ANRM trace extractor or dataset pipeline.

## Decision Rule

Proceed toward Gemma training only if the first scaffolded comparison shows one
of these:

- materially better decision accuracy than raw Gemma
- materially lower repair burden
- stronger schema compliance
- better refusal or pause behavior
- better evidence references

If scaffold-only prompting is enough for shepherding, training can wait. If
training clearly helps, ANRM becomes a serious v0.90.2 or v0.91 candidate.
